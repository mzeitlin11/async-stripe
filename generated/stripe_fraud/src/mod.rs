#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Fraud` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_fraud;
pub mod deleted_radar_value_list;
pub use deleted_radar_value_list::DeletedRadarValueList;
pub mod deleted_radar_value_list_item;
pub use deleted_radar_value_list_item::DeletedRadarValueListItem;
pub mod radar_early_fraud_warning;
pub use radar_early_fraud_warning::RadarEarlyFraudWarning;
pub mod radar_value_list;
pub use radar_value_list::RadarValueList;
pub mod radar_value_list_item;
pub use radar_value_list_item::RadarValueListItem;
pub use stripe_shared::RadarReviewResourceLocation;
pub mod radar_review_resource_location;
pub use stripe_shared::RadarReviewResourceSession;
pub mod radar_review_resource_session;
pub use stripe_shared::Review;
pub mod review;
pub use stripe_shared::Rule;
pub mod rule;
