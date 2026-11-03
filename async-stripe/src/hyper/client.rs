use std::time::Duration;

use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use hyper::http::request::Builder;
use hyper::http::{HeaderName, HeaderValue};
use hyper::{Request, StatusCode};
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;
use stripe_client_core::ConfigOverride;
use stripe_client_core::{CustomizedStripeRequest, RequestBuilder, StripeMethod};
use stripe_client_core::{Outcome, RequestStrategy};

use crate::StripeError;
use crate::hyper::client_builder::{ClientBuilder, ClientConfig};

/// A client for making Stripe API requests.
#[derive(Clone, Debug)]
pub struct Client {
    client: HyperClient<crate::hyper::connector::Connector, Full<Bytes>>,
    config: ClientConfig,
}

impl Client {
    pub(crate) fn from_config(config: ClientConfig) -> Self {
        Self {
            client: HyperClient::builder(TokioExecutor::new())
                .pool_max_idle_per_host(0)
                .build(crate::hyper::connector::create()),
            config,
        }
    }

    /// Construct a `client` with the given secret key and a default configuration.
    ///
    /// # Panics
    /// This method panics if secret key is not usable as a header value.
    pub fn new(secret_key: impl Into<String>) -> Self {
        ClientBuilder::new(secret_key).build().expect("invalid secret provided")
    }

    fn get_account_id_header(
        &self,
        account_id_override: Option<String>,
    ) -> Result<Option<HeaderValue>, StripeError> {
        if let Some(overridden) = account_id_override {
            return Ok(Some(HeaderValue::from_str(overridden.as_str()).map_err(|_| {
                StripeError::ConfigError("invalid account id set in customizations".into())
            })?));
        }
        Ok(self.config.account_id.clone())
    }

    fn construct_request(
        &self,
        req: RequestBuilder,
        config: &ConfigOverride,
    ) -> Result<(Builder, Option<Bytes>), StripeError> {
        let uri = req
            .url(&self.config.api_base.to_string())
            .map_err(|error| StripeError::ConfigError(error.into()))?;
        let version = config
            .api_version
            .as_deref()
            .or_else(|| self.config.stripe_version.as_ref().and_then(|value| value.to_str().ok()))
            .or(req.api_version.as_deref())
            .or(crate::default_api_version());
        if req.is_v2() && version.is_none_or(|version| version.trim().is_empty()) {
            return Err(StripeError::ConfigError("v2 requests require Stripe-Version".into()));
        }

        let user_agent = if req.is_v2() {
            HeaderValue::from_str(
                &self
                    .config
                    .user_agent
                    .to_str()
                    .map_err(|_| StripeError::ConfigError("invalid user agent".into()))?
                    .replacen("Stripe/v1", "Stripe/v2", 1),
            )
            .map_err(|_| StripeError::ConfigError("invalid user agent".into()))?
        } else {
            self.config.user_agent.clone()
        };
        let mut builder = Request::builder()
            .method(conv_stripe_method(req.method))
            .uri(uri)
            .header(AUTHORIZATION, self.config.secret.clone())
            .header(USER_AGENT, user_agent);
        if let Some(version) = version {
            builder = builder.header("stripe-version", version);
        }
        if let Some(context) = &config.context {
            builder = builder.header("stripe-context", context);
        }

        if let Some(client_id) = &self.config.client_id {
            builder = builder.header(HeaderName::from_static("client-id"), client_id.clone());
        }
        if let Some(account_id) = self.get_account_id_header(config.account_id.clone())? {
            builder = builder.header(HeaderName::from_static("stripe-account"), account_id);
        }

        let body = if let Some(body) = req.body {
            builder = builder.header(CONTENT_TYPE, HeaderValue::from_static(req.content_type));
            Some(Bytes::from(body))
        } else {
            None
        };
        Ok((builder, body))
    }

    #[tracing::instrument(skip(self, body, req_builder), fields(strategy = ?strategy, retry_count))]
    async fn send_inner(
        &self,
        body: Option<Bytes>,
        mut req_builder: Builder,
        strategy: RequestStrategy,
        timeout: Option<Duration>,
        is_v2: bool,
    ) -> Result<Bytes, StripeError> {
        let mut tries = 0;
        let mut last_status: Option<StatusCode> = None;
        let mut last_retry_header: Option<bool> = None;
        let mut last_error = StripeError::ClientError("invalid strategy".into());

        let v2_mutation = is_v2
            && req_builder.method_ref().is_some_and(|method| {
                matches!(*method, hyper::Method::POST | hyper::Method::DELETE)
            });
        if let Some(key) = strategy
            .key_for_request(v2_mutation)
            .map_err(|error| StripeError::ConfigError(error.into()))?
        {
            const HEADER_NAME: HeaderName = HeaderName::from_static("idempotency-key");
            req_builder = req_builder.header(HEADER_NAME, key.as_str());
            tracing::debug!("using idempotency key");
        }

        let req = req_builder.body(Full::new(body.unwrap_or_default()))?;

        loop {
            tracing::Span::current().record("retry_count", tries);

            return match strategy.test(last_status.map(|s| s.as_u16()), last_retry_header, tries) {
                Outcome::Stop => {
                    tracing::warn!("request failed after {} attempts", tries);
                    Err(last_error)
                }
                Outcome::Continue(duration) => {
                    if let Some(duration) = duration {
                        tracing::debug!("backing off for {:?}", duration);
                        tokio::time::sleep(duration).await;
                    }

                    tracing::debug!("sending request (attempt {})", tries + 1);
                    let attempt = async {
                        let response = self.client.request(req.clone()).await?;
                        let status = response.status();
                        let retry = response
                            .headers()
                            .get("Stripe-Should-Retry")
                            .and_then(|s| s.to_str().ok())
                            .and_then(|s| s.parse().ok());
                        let bytes = response.into_body().collect().await?.to_bytes();
                        Ok::<_, StripeError>((status, retry, bytes))
                    };
                    let attempt_result = match timeout {
                        Some(t) => tokio::time::timeout(t, attempt)
                            .await
                            .unwrap_or_else(|_| Err(StripeError::Timeout)),
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

                        last_error = StripeError::from_api_response(status.as_u16(), &bytes, is_v2);
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

fn conv_stripe_method(method: StripeMethod) -> hyper::Method {
    match method {
        StripeMethod::Get => hyper::Method::GET,
        StripeMethod::Post => hyper::Method::POST,
        StripeMethod::Delete => hyper::Method::DELETE,
    }
}

impl stripe_client_core::StripeClient for Client {
    type Err = StripeError;

    #[tracing::instrument(skip(self, req_full), fields(method, path))]
    async fn execute(&self, req_full: CustomizedStripeRequest) -> Result<Bytes, Self::Err> {
        let (req, config) = req_full.into_pieces();

        tracing::Span::current().record("method", tracing::field::debug(&req.method));
        tracing::Span::current().record("path", &req.path);

        let is_v2 = req.is_v2();
        let (builder, body) = self.construct_request(req, &config)?;

        let request_strategy =
            config.request_strategy.unwrap_or_else(|| self.config.request_strategy.clone());
        let timeout = config.timeout.or(self.config.timeout);
        self.send_inner(body, builder, request_strategy, timeout, is_v2).await
    }
}
