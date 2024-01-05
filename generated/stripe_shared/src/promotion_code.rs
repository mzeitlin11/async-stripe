/// A Promotion Code represents a customer-redeemable code for a [coupon](https://stripe.com/docs/api#coupons).
/// It can be used to.
/// create multiple codes for a single coupon.
///
/// For more details see <<https://stripe.com/docs/api/promotion_codes/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PromotionCode {
    /// Whether the promotion code is currently active.
    /// A promotion code is only active if the coupon is also valid.
    pub active: bool,
    /// The customer-facing code.
    /// Regardless of case, this code must be unique across all active promotion codes for each customer.
    pub code: String,
    pub coupon: stripe_shared::Coupon,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The customer that this promotion code can be used by.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Date at which the promotion code can no longer be redeemed.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PromotionCodeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Maximum number of times this promotion code can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub restrictions: stripe_shared::PromotionCodesResourceRestrictions,
    /// Number of times this promotion code has been used.
    pub times_redeemed: i64,
}
#[cfg(feature = "min-ser")]
pub struct PromotionCodeBuilder {
    active: Option<bool>,
    code: Option<String>,
    coupon: Option<stripe_shared::Coupon>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_shared::PromotionCodeId>,
    livemode: Option<bool>,
    max_redemptions: Option<Option<i64>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    restrictions: Option<stripe_shared::PromotionCodesResourceRestrictions>,
    times_redeemed: Option<i64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PromotionCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PromotionCode>,
        builder: PromotionCodeBuilder,
    }

    impl Visitor for Place<PromotionCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PromotionCodeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PromotionCodeBuilder {
        type Out = PromotionCode;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "active" => Ok(Deserialize::begin(&mut self.active)),
                "code" => Ok(Deserialize::begin(&mut self.code)),
                "coupon" => Ok(Deserialize::begin(&mut self.coupon)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "max_redemptions" => Ok(Deserialize::begin(&mut self.max_redemptions)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "restrictions" => Ok(Deserialize::begin(&mut self.restrictions)),
                "times_redeemed" => Ok(Deserialize::begin(&mut self.times_redeemed)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                code: Deserialize::default(),
                coupon: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                max_redemptions: Deserialize::default(),
                metadata: Deserialize::default(),
                restrictions: Deserialize::default(),
                times_redeemed: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let active = self.active.take()?;
            let code = self.code.take()?;
            let coupon = self.coupon.take()?;
            let created = self.created.take()?;
            let customer = self.customer.take()?;
            let expires_at = self.expires_at.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let max_redemptions = self.max_redemptions.take()?;
            let metadata = self.metadata.take()?;
            let restrictions = self.restrictions.take()?;
            let times_redeemed = self.times_redeemed.take()?;

            Some(Self::Out { active, code, coupon, created, customer, expires_at, id, livemode, max_redemptions, metadata, restrictions, times_redeemed })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PromotionCode {
        type Builder = PromotionCodeBuilder;
    }
};
impl stripe_types::Object for PromotionCode {
    type Id = stripe_shared::PromotionCodeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PromotionCodeId, "promo_");
