use std::fmt::Display;

use stripe_client_core::StripeClientErr;
#[cfg(feature = "v1")]
use stripe_shared::ApiErrors;
use thiserror::Error;

/// An error encountered when communicating with the Stripe API.
#[derive(Debug, Error)]
pub enum StripeError {
    /// Stripe returned a client error.
    #[error("error reported by stripe: {0:#?}, status code: {1}")]
    #[cfg(feature = "v1")]
    Stripe(Box<ApiErrors>, u16),
    /// A model-independent API error, including v2 error responses.
    #[error("error reported by stripe: {0:?}, status code: {1}")]
    Api(Box<ApiError>, u16),
    /// An error occurred when parsing the Stripe response.
    #[error("error deserializing a request: {0}")]
    JSONDeserialize(String),
    /// An error occurred communicating with Stripe.
    #[error("error communicating with stripe: {0}")]
    ClientError(String),
    /// The client configuration was invalid.
    #[error("configuration error: {0}")]
    ConfigError(String),
    /// A blocking request timed out
    #[error("timeout communicating with stripe")]
    Timeout,
}

impl StripeClientErr for StripeError {
    fn deserialize_err(msg: impl Display) -> Self {
        Self::JSONDeserialize(msg.to_string())
    }
}

#[cfg(feature = "__hyper")]
impl From<hyper::Error> for StripeError {
    fn from(err: hyper::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}

#[cfg(feature = "__hyper")]
impl From<hyper_util::client::legacy::Error> for StripeError {
    fn from(err: hyper_util::client::legacy::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}

#[cfg(feature = "__hyper")]
impl From<hyper::http::Error> for StripeError {
    fn from(err: hyper::http::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}

#[cfg(feature = "async-std-surf")]
impl From<http_types::Error> for StripeError {
    fn from(err: http_types::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}

/// API error information without dependencies on resource models.
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
pub struct ApiError {
    /// Stripe's machine-readable error code.
    pub code: Option<String>,
    /// Description of the error.
    pub message: Option<String>,
    /// Full response body, retaining additional error information.
    pub body: String,
}

#[cfg(any(feature = "__hyper", feature = "async-std-surf"))]
#[derive(miniserde::Deserialize)]
struct ErrorEnvelope {
    error: ErrorFields,
}

#[cfg(any(feature = "__hyper", feature = "async-std-surf"))]
#[derive(miniserde::Deserialize)]
struct ErrorFields {
    code: Option<String>,
    message: Option<String>,
}

#[cfg(any(feature = "__hyper", feature = "async-std-surf"))]
impl StripeError {
    pub(crate) fn from_api_response(status: u16, bytes: &[u8], is_v2: bool) -> Self {
        let Ok(body) = std::str::from_utf8(bytes) else {
            return Self::JSONDeserialize("Response was not valid UTF-8".into());
        };
        #[cfg(feature = "v1")]
        if !is_v2 {
            return miniserde::json::from_str::<stripe_shared::Error>(body)
                .map(|response| Self::Stripe(response.error, status))
                .unwrap_or_else(|_| {
                    Self::JSONDeserialize("error deserializing Stripe error".into())
                });
        }
        #[cfg(not(feature = "v1"))]
        let _ = is_v2;
        miniserde::json::from_str::<ErrorEnvelope>(body)
            .map(|response| {
                Self::Api(
                    Box::new(ApiError {
                        code: response.error.code,
                        message: response.error.message,
                        body: body.to_owned(),
                    }),
                    status,
                )
            })
            .unwrap_or_else(|_| Self::JSONDeserialize("error deserializing Stripe error".into()))
    }
}

#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiError").finish_non_exhaustive()
    }
}
