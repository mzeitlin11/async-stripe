pub use stripe_shared::coupon::*;
#[cfg(feature = "coupon")]
mod requests;
#[cfg(feature = "coupon")]
pub use requests::*;
