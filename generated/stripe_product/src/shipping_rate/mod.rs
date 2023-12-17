pub use stripe_shared::shipping_rate::*;
#[cfg(feature = "shipping_rate")]
mod requests;
#[cfg(feature = "shipping_rate")]
pub use requests::*;
