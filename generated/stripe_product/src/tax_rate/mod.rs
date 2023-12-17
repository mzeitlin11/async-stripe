pub use stripe_shared::tax_rate::*;
#[cfg(feature = "tax_rate")]
mod requests;
#[cfg(feature = "tax_rate")]
pub use requests::*;
