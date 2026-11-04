use std::time::Duration;

use async_std::task::sleep;
use http_types::{Body, Request, StatusCode};
use stripe_client_core::ConfigOverride;
use stripe_client_core::{
    CustomizedStripeRequest, Outcome, RequestBuilder, RequestStrategy, StripeMethod,
};

use crate::StripeError;
use crate::async_std::ClientBuilder;
use crate::async_std::config::ClientConfig;

/// A client for making Stripe API requests.
#[derive(Clone, Debug)]
pub struct Client {
    client: surf::Client,
    config: ClientConfig,
}

impl Client {
    /// Construct a `client` with the given secret key and a default configuration.
    ///
    /// # Panics
    /// This method panics if secret key is not usable as a header value.
    pub fn new(secret_key: impl Into<String>) -> Self {
        ClientBuilder::new(secret_key).build().expect("invalid secret provided")
    }

    pub(crate) fn from_config(config: ClientConfig) -> Self {
        Self { client: surf::Client::new(), config }
    }

    fn create_request(
        &self,
        builder: RequestBuilder,
        config: &ConfigOverride,
    ) -> Result<Request, StripeError> {
        let url = builder
            .url(self.config.api_base.as_str())
            .map_err(|error| StripeError::ConfigError(error.into()))?;
        let version = config
            .api_version
            .as_deref()
            .or(self.config.stripe_version.as_deref())
            .or(builder.api_version.as_deref())
            .or(crate::default_api_version());
        if builder.is_v2() && version.is_none_or(|version| version.trim().is_empty()) {
            return Err(StripeError::ConfigError("v2 requests require Stripe-Version".into()));
        }
        let is_v2 = builder.is_v2();
        let mut request = Request::new(
            conv_stripe_method(builder.method),
            url.parse::<http_types::Url>()
                .map_err(|error| StripeError::ConfigError(error.to_string()))?,
        );
        if let Some(body) = builder.body {
            request.set_body(Body::from_string(body));
            request.insert_header("Content-Type", builder.content_type);
        }
        let mut header = |name: &str, value: &str| -> Result<(), StripeError> {
            let value = value
                .parse::<http_types::headers::HeaderValue>()
                .map_err(|error| StripeError::ConfigError(error.to_string()))?;
            request.insert_header(name, value);
            Ok(())
        };
        header("Authorization", &format!("Bearer {}", self.config.secret))?;
        for (name, value) in self.config.to_headers_array() {
            if name != "Stripe-Version"
                && let Some(value) = value
            {
                if name == "User-Agent" && is_v2 {
                    header(name, &value.replacen("Stripe/v1", "Stripe/v2", 1))?;
                } else {
                    header(name, value)?;
                }
            }
        }
        if let Some(version) = version {
            header("Stripe-Version", version)?;
        }
        if let Some(context) = &config.context {
            header("Stripe-Context", context)?;
        }
        if let Some(event) = &builder.event_trigger {
            header("Stripe-Request-Trigger", &format!("event={event}"))?;
        }
        if let Some(account) = &config.account_id {
            header("Stripe-Account", account)?;
        }
        Ok(request)
    }

    #[tracing::instrument(skip(self, request), fields(strategy = ?strategy, retry_count))]
    async fn send_inner(
        &self,
        mut request: Request,
        strategy: RequestStrategy,
        timeout: Option<Duration>,
        is_v2: bool,
    ) -> Result<Vec<u8>, StripeError> {
        let mut tries = 0;
        let mut last_status: Option<StatusCode> = None;
        let mut last_retry_header: Option<bool> = None;

        // if we have no last error, then the strategy is invalid
        let mut last_error = StripeError::ClientError("Invalid strategy".to_string());

        let v2_mutation = is_v2
            && matches!(request.method(), http_types::Method::Post | http_types::Method::Delete);
        if let Some(key) = strategy
            .key_for_request(v2_mutation)
            .map_err(|error| StripeError::ConfigError(error.into()))?
        {
            request.insert_header("idempotency-key", key.as_str());
            tracing::debug!("using idempotency key");
        }

        let body = request.body_bytes().await?;

        loop {
            tracing::Span::current().record("retry_count", tries);

            return match strategy.test(last_status.map(|s| s.into()), last_retry_header, tries) {
                Outcome::Stop => {
                    tracing::warn!("request failed after {} attempts", tries);
                    Err(last_error)
                }
                Outcome::Continue(duration) => {
                    if let Some(duration) = duration {
                        tracing::debug!("backing off for {:?}", duration);
                        sleep(duration).await;
                    }

                    // we need to clone the request before sending it so we can
                    // re-use it if we need to retry. ditto for the body
                    let mut request = request.clone();
                    request.set_body(body.clone());

                    tracing::debug!("sending request (attempt {})", tries + 1);
                    let attempt = async {
                        let mut response = self.client.send(request).await?;
                        let status = response.status();
                        let retry = response
                            .header("Stripe-Should-Retry")
                            .and_then(|s| s.last().as_str().parse().ok());
                        let bytes = response.body_bytes().await?;
                        Ok::<_, StripeError>((status, retry, bytes))
                    };
                    let attempt_result = match timeout {
                        Some(t) => async_std::future::timeout(t, attempt)
                            .await
                            .unwrap_or(Err(StripeError::Timeout)),
                        None => attempt.await,
                    };
                    let (status, retry, bytes) = match attempt_result {
                        Ok(parts) => parts,
                        Err(err) => {
                            tracing::warn!("request failed: {}", err);
                            last_error = err;
                            tries += 1;
                            continue;
                        }
                    };

                    if !status.is_success() {
                        tries += 1;
                        tracing::warn!("request returned error status: {}", status);
                        last_error = StripeError::from_api_response(status.into(), &bytes, is_v2);
                        last_status = Some(status);
                        last_retry_header = retry;

                        continue;
                    }

                    tracing::debug!("request successful");
                    Ok(bytes)
                }
            };
        }
    }
}

fn conv_stripe_method(method: StripeMethod) -> http_types::Method {
    match method {
        StripeMethod::Get => http_types::Method::Get,
        StripeMethod::Post => http_types::Method::Post,
        StripeMethod::Delete => http_types::Method::Delete,
    }
}

impl stripe_client_core::StripeClient for Client {
    type Err = StripeError;

    #[tracing::instrument(skip(self, req_full), fields(method, path))]
    async fn execute(&self, req_full: CustomizedStripeRequest) -> Result<bytes::Bytes, Self::Err> {
        let (req, config) = req_full.into_pieces();

        tracing::Span::current().record("method", tracing::field::debug(&req.method));
        tracing::Span::current().record("path", &req.path);

        let is_v2 = req.is_v2();
        let req = self.create_request(req, &config)?;
        let request_strategy =
            config.request_strategy.unwrap_or_else(|| self.config.request_strategy.clone());
        let timeout = config.timeout.or(self.config.timeout);
        let bytes = self.send_inner(req, request_strategy, timeout, is_v2).await?;
        Ok(bytes::Bytes::from(bytes))
    }
}
