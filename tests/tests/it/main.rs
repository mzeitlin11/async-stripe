// Needed for `json!` usage in tests
#![recursion_limit = "256"]
mod deser;
mod mock;
mod price;

#[cfg(feature = "async")]
mod async_tests;
#[cfg(feature = "blocking")]
mod blocking;
mod pagination_utils;
