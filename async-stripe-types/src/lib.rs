//! This crate provides Rust bindings for core types to the Stripe HTTP API.
#![warn(clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![deny(missing_docs, missing_debug_implementations)]
#![forbid(unsafe_code)]

mod currency;
mod deferred_fields;
mod error;
mod expandable;
mod expanded_value;
mod ids;
mod object_value;
mod pagination;
mod params;
mod record_value;

pub use currency::Currency;
pub use deferred_fields::DeferredFields;
pub use error::StripeParseError;
pub use expandable::*;
pub use expanded_value::ExpandedValue;
pub use object_value::{ExpandableValue, ObjectDecodeError, ObjectType, ObjectValue};
pub use pagination::*;
pub use params::*;
pub use record_value::{RecordDecodeError, RecordValue};
pub use serde_helpers::{with_serde_json, with_serde_json_opt};

miniserde::make_place!(Place);

#[doc(hidden)]
pub mod miniserde_helpers;

#[doc(hidden)]
pub mod serde_helpers;

// Allow generated code to use absolute paths starting with `stripe_types` instead of `crate`
extern crate self as stripe_types;
