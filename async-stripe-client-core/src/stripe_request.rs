use std::fmt::Display;
use std::marker::PhantomData;
use std::time::Duration;

use bytes::Bytes;
use miniserde::json::from_str;
use serde::Serialize;

use crate::ConfigOverride;
use crate::request_strategy::RequestStrategy;

/// Encode an opaque v2 identifier as one URL path segment.
#[cfg(feature = "v2")]
pub fn encode_path_segment(value: &str) -> String {
    const ENCODE: &percent_encoding::AsciiSet =
        &percent_encoding::NON_ALPHANUMERIC.remove(b'-').remove(b'.').remove(b'_').remove(b'~');
    percent_encoding::utf8_percent_encode(value, ENCODE).to_string()
}

/// REST API methods used by Stripe.
#[derive(Debug, Copy, Clone)]
pub enum StripeMethod {
    /// GET
    Get,
    /// POST
    Post,
    /// DELETE
    Delete,
}

/// This trait allows clients implementing `StripeClient` to define their own error type,
/// while ensuring they can receive deserialization errors.
pub trait StripeClientErr {
    /// Raised when we cannot deserialize the bytes received from a request into the
    /// specified type.
    fn deserialize_err(msg: impl Display) -> Self;
}

/// An abstraction for defining HTTP clients capable of making Stripe API requests compatible
/// with request information generated in the stripe request crates.
pub trait StripeClient {
    /// The error returned, either if the request failed due to an issue with client
    /// communicating with Stripe, or a client error returned by the Stripe API.
    type Err: StripeClientErr;

    /// Make the API call.
    ///
    /// # Errors
    /// If the request failed due to an issue with client communication with Stripe,
    /// or a client error returned by the Stripe API.
    fn execute(
        &self,
        req: CustomizedStripeRequest,
    ) -> impl std::future::Future<Output = Result<Bytes, Self::Err>> + Send;
}

/// An abstraction for defining HTTP clients capable of making blocking Stripe API requests compatible
/// with request information generated in the stripe request crates.
pub trait StripeBlockingClient {
    /// The error returned, either if the request failed due to an issue with client
    /// communicating with Stripe, or a client error returned by the Stripe API.
    type Err: StripeClientErr;

    /// Make a blocking API call.
    ///
    /// # Errors
    /// If the request failed due to an issue with client communication with Stripe,
    /// or a client error returned by the Stripe API.
    fn execute(&self, req: CustomizedStripeRequest) -> Result<Bytes, Self::Err>;
}

/// Define how to convert structs into the data required to make a specific Stripe API call.
pub trait StripeRequest {
    /// The data returned from the eventual API call.
    type Output;

    /// Convert the struct into library-agnostic data that can be used by compatible
    /// clients to make API calls.
    fn build(&self) -> RequestBuilder;

    /// Convert to a builder allowing per-request customization.
    fn customize(&self) -> CustomizableStripeRequest<Self::Output> {
        CustomizableStripeRequest::new(self.build())
    }
}

/// A `CustomizableStripeRequest` allows for configuring per-request behavior that overrides
/// default configuration values.
#[derive(Debug)]
pub struct CustomizableStripeRequest<T> {
    pub(crate) inner: CustomizedStripeRequest,
    _output: PhantomData<T>,
}

/// The request specification used by a compatible Stripe client to make a request.
#[derive(Clone, Debug)]
pub struct CustomizedStripeRequest {
    pub(crate) request: RequestBuilder,
    pub(crate) config_override: ConfigOverride,
}

impl CustomizedStripeRequest {
    /// Split the request specification into the request itself and any configuration override.
    pub fn into_pieces(self) -> (RequestBuilder, ConfigOverride) {
        (self.request, self.config_override)
    }
}

impl<T> CustomizableStripeRequest<T> {
    pub(crate) fn new(req_builder: RequestBuilder) -> Self {
        Self {
            _output: PhantomData,
            inner: CustomizedStripeRequest {
                request: req_builder,
                config_override: ConfigOverride::new(),
            },
        }
    }

    /// Set a strategy to use for this request, overriding the default set
    /// during configuration.
    pub fn request_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.inner.config_override.request_strategy = Some(strategy);
        self
    }

    /// Set an account id to use for this request, overriding the default set
    /// during configuration.
    pub fn account_id(mut self, account_id: impl AsRef<str>) -> Self {
        self.inner.config_override.account_id = Some(account_id.as_ref().to_owned());
        self
    }

    /// Set the authentication context for an event or related resource.
    pub fn context(mut self, context: impl Into<String>) -> Self {
        self.inner.config_override.context = Some(context.into());
        self
    }

    /// Override the dated API revision for this request.
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.inner.config_override.api_version = Some(version.into());
        self
    }

    /// Set a per-attempt timeout to use for this request, overriding the
    /// default set during configuration. The timeout applies to each
    /// individual HTTP attempt; the request strategy may still retry a
    /// timed-out attempt.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.inner.config_override.timeout = Some(timeout);
        self
    }
}

impl<T: miniserde::Deserialize> CustomizableStripeRequest<T> {
    /// Sends the request and returns the response.
    ///
    /// # Errors
    /// If the request failed due to an issue with client communication with Stripe,
    /// or a client error returned by the Stripe API.
    pub async fn send<C: StripeClient>(self, client: &C) -> Result<T, C::Err> {
        let bytes = client.execute(self.inner).await?;
        deserialize_bytes(bytes)
    }

    /// Sends the request, blocking the main thread until the response is returned.
    ///
    /// # Errors
    /// If the request failed due to an issue with client communication with Stripe,
    /// or a client error returned by the Stripe API.
    pub fn send_blocking<C: StripeBlockingClient>(self, client: &C) -> Result<T, C::Err> {
        let bytes = client.execute(self.inner)?;
        deserialize_bytes(bytes)
    }
}

fn deserialize_bytes<T: miniserde::Deserialize, Err: StripeClientErr>(
    bytes: Bytes,
) -> Result<T, Err> {
    let str = std::str::from_utf8(bytes.as_ref())
        .map_err(|_| Err::deserialize_err("Response was not valid UTF-8"))?;
    from_str(str).map_err(|_| Err::deserialize_err("error deserializing request data"))
}

/// A builder for specifying the possible pieces of a Stripe API request.
#[derive(Clone, Debug)]
pub struct RequestBuilder {
    /// Event responsible for fetching this resource, for Stripe request correlation.
    pub event_trigger: Option<String>,
    /// The current query string to use, if provided.
    pub query: Option<String>,
    /// The serialized body to send, if provided.
    pub body: Option<String>,
    /// The media type of the serialized body.
    pub content_type: &'static str,
    /// Default dated API revision supplied by generated bindings.
    pub api_version: Option<String>,
    /// The API endpoint to send the request to.
    pub path: String,
    /// The method type.
    pub method: StripeMethod,
}

impl RequestBuilder {
    /// Construct a new `RequestBuilder`.
    pub fn new(method: StripeMethod, path: impl Into<String>) -> Self {
        Self {
            event_trigger: None,
            path: path.into(),
            method,
            query: None,
            body: None,
            content_type: "application/x-www-form-urlencoded",
            api_version: None,
        }
    }

    /// Set the default dated API revision for these bindings.
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = Some(version.into());
        self
    }

    /// Associate this fetch with a delivered event notification.
    pub fn event_trigger(mut self, event: impl Into<String>) -> Self {
        self.event_trigger = Some(event.into());
        self
    }

    /// Whether this request addresses the v2 API namespace.
    pub fn is_v2(&self) -> bool {
        self.path.starts_with("/v2/")
    }

    /// Resolve a relative API target. Legacy unqualified paths use v1.
    ///
    /// # Errors
    /// Rejects targets containing an authority, fragment, or control characters.
    pub fn url(&self, base: &str) -> Result<String, &'static str> {
        if !self.path.starts_with('/')
            || self.path.starts_with("//")
            || self.path.contains("://")
            || self.path.contains('#')
            || self.path.chars().any(char::is_control)
        {
            return Err("request target must be a relative API path");
        }
        let prefix = if self.path.starts_with("/v1/") || self.is_v2() { "" } else { "/v1" };
        let mut url = format!("{}{prefix}{}", base.trim_end_matches('/'), self.path);
        if let Some(query) = &self.query {
            url.push(if url.contains('?') { '&' } else { '?' });
            url.push_str(query);
        }
        Ok(url)
    }

    /// Construct a JSON request body.
    #[allow(clippy::missing_panics_doc)]
    pub fn json<P: Serialize>(mut self, params: &P) -> Self {
        self.body = Some(serde_json::to_string(params).expect("valid serialization"));
        self.content_type = "application/json";
        self
    }

    /// Construct an empty JSON object body.
    pub fn json_empty(mut self) -> Self {
        self.body = Some("{}".to_owned());
        self.content_type = "application/json";
        self
    }

    /// Set a query by serializing the params.
    #[allow(clippy::missing_panics_doc)]
    pub fn query<P: Serialize>(mut self, params: &P) -> Self {
        self.query = Some(serde_qs::to_string(params).expect("valid serialization"));
        self
    }

    /// Construct a serialized, form-encoded body.
    #[allow(clippy::missing_panics_doc)]
    pub fn form<F: Serialize>(mut self, form: &F) -> Self {
        self.body = Some(serde_qs::to_string(form).expect("valid serialization"));
        self.content_type = "application/x-www-form-urlencoded";
        self
    }

    /// Convert this request into a `CustomizableStripeRequest` to allow per-request
    /// customization.
    pub fn customize<T>(self) -> CustomizableStripeRequest<T> {
        CustomizableStripeRequest::new(self)
    }
}
