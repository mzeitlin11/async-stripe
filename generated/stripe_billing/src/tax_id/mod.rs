pub use stripe_shared::tax_id::*;
#[cfg(feature = "tax_id")]
mod requests;
#[cfg(feature = "tax_id")]
pub use requests::*;
