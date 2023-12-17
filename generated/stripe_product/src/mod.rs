#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Products` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_product;
pub use stripe_shared::Coupon;
pub mod coupon;
pub use stripe_shared::CouponAppliesTo;
pub mod coupon_applies_to;
pub use stripe_shared::CouponCurrencyOption;
pub mod coupon_currency_option;
pub use stripe_shared::CurrencyOption;
pub mod currency_option;
pub use stripe_shared::CustomUnitAmount;
pub mod custom_unit_amount;
pub use stripe_shared::DeletedCoupon;
pub mod deleted_coupon;
pub use stripe_shared::DeletedDiscount;
pub mod deleted_discount;
pub use stripe_shared::DeletedProduct;
pub mod deleted_product;
pub use stripe_shared::Discount;
pub mod discount;
pub use stripe_shared::LineItemsDiscountAmount;
pub mod line_items_discount_amount;
pub use stripe_shared::LineItemsTaxAmount;
pub mod line_items_tax_amount;
pub use stripe_shared::PackageDimensions;
pub mod package_dimensions;
pub use stripe_shared::Price;
pub mod price;
pub use stripe_shared::PriceTier;
pub mod price_tier;
pub use stripe_shared::Product;
pub mod product;
pub use stripe_shared::ProductFeature;
pub mod product_feature;
pub use stripe_shared::PromotionCode;
pub mod promotion_code;
pub use stripe_shared::PromotionCodeCurrencyOption;
pub mod promotion_code_currency_option;
pub use stripe_shared::PromotionCodesResourceRestrictions;
pub mod promotion_codes_resource_restrictions;
pub use stripe_shared::Recurring;
pub mod recurring;
pub use stripe_shared::ShippingRate;
pub mod shipping_rate;
pub use stripe_shared::TaxCode;
pub mod tax_code;
pub use stripe_shared::TaxRate;
pub mod tax_rate;
pub use stripe_shared::TransformQuantity;
pub mod transform_quantity;
