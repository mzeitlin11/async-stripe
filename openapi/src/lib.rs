pub mod codegen;
mod components;
mod crate_inference;
mod crate_table;
pub mod crates;
mod graph;
mod ids;
mod object_writing;
mod overrides;
mod printable;
mod requests;
mod rust_object;
mod rust_type;
pub mod spec;
pub mod spec_fetch;
mod spec_inference;
mod stripe_object;
mod templates;
mod types;
pub mod url_finder;
pub mod utils;
mod visitor;
mod webhook;

pub const STRIPE_TYPES: &str = "stripe_types";
