//! This crate provides shared utilities for implementing clients
//! capable of making Stripe API requests.

#![warn(clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![deny(missing_docs, missing_debug_implementations)]
#![forbid(unsafe_code)]
mod config;
mod event_notification;
mod pagination;
mod request_strategy;
mod stripe_request;
mod v2_pagination;

pub use config::{ConfigOverride, SharedConfigBuilder};
pub use event_notification::{EventNotification, RelatedObject};
pub use pagination::*;
pub use request_strategy::*;
pub use stripe_request::*;
#[cfg(feature = "v1")]
pub use stripe_shared::version::VERSION;
#[cfg(feature = "v1")]
pub use stripe_shared::{AccountId, ApiVersion, ApplicationId};
pub use v2_pagination::*;
