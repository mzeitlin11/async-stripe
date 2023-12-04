#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Products` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_product;
pub use stripe_types::Coupon;
pub mod coupon;
pub use stripe_types::CouponAppliesTo;
pub mod coupon_applies_to;
pub use stripe_types::CouponCurrencyOption;
pub mod coupon_currency_option;
pub use stripe_types::CurrencyOption;
pub mod currency_option;
pub use stripe_types::CustomUnitAmount;
pub mod custom_unit_amount;
pub use stripe_types::DeletedCoupon;
pub mod deleted_coupon;
pub use stripe_types::DeletedDiscount;
pub mod deleted_discount;
pub use stripe_types::DeletedProduct;
pub mod deleted_product;
pub use stripe_types::Discount;
pub mod discount;
pub use stripe_types::LineItemsDiscountAmount;
pub mod line_items_discount_amount;
pub use stripe_types::LineItemsTaxAmount;
pub mod line_items_tax_amount;
pub use stripe_types::PackageDimensions;
pub mod package_dimensions;
pub use stripe_types::Price;
pub mod price;
pub use stripe_types::PriceTier;
pub mod price_tier;
pub use stripe_types::Product;
pub mod product;
pub use stripe_types::ProductFeature;
pub mod product_feature;
pub use stripe_types::PromotionCode;
pub mod promotion_code;
pub use stripe_types::PromotionCodeCurrencyOption;
pub mod promotion_code_currency_option;
pub use stripe_types::PromotionCodesResourceRestrictions;
pub mod promotion_codes_resource_restrictions;
pub use stripe_types::Recurring;
pub mod recurring;
pub use stripe_types::ShippingRate;
pub mod shipping_rate;
pub use stripe_types::TaxCode;
pub mod tax_code;
pub use stripe_types::TaxRate;
pub mod tax_rate;
pub use stripe_types::TransformQuantity;
pub mod transform_quantity;
