/// A coupon contains information about a percent-off or amount-off discount you
/// might want to apply to a customer.
/// Coupons may be applied to [subscriptions](https://stripe.com/docs/api#subscriptions), [invoices](https://stripe.com/docs/api#invoices),.
/// [checkout sessions](https://stripe.com/docs/api/checkout/sessions), [quotes](https://stripe.com/docs/api#quotes), and more.
/// Coupons do not work with conventional one-off [charges](https://stripe.com/docs/api#create_charge) or [payment intents](https://stripe.com/docs/api/payment_intents).
///
/// For more details see <<https://stripe.com/docs/api/coupons/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Coupon {
    /// Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: Option<i64>,
    pub applies_to: Option<stripe_shared::CouponAppliesTo>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If `amount_off` has been set, the three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the amount to take off.
    pub currency: Option<stripe_types::Currency>,
    /// Coupons defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options: Option<std::collections::HashMap<stripe_types::Currency, stripe_shared::CouponCurrencyOption>>,
    /// One of `forever`, `once`, and `repeating`.
    /// Describes how long a customer who applies this coupon will get the discount.
    pub duration: stripe_shared::CouponDuration,
    /// If `duration` is `repeating`, the number of months the coupon applies.
    /// Null if coupon `duration` is `forever` or `once`.
    pub duration_in_months: Option<i64>,
    /// Unique identifier for the object.
    pub id: stripe_shared::CouponId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Maximum number of times this coupon can be redeemed, in total, across all customers, before it is no longer valid.
    pub max_redemptions: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Name of the coupon displayed to customers on for instance invoices or receipts.
    pub name: Option<String>,
    /// Percent that will be taken off the subtotal of any invoices for this customer for the duration of the coupon.
    /// For example, a coupon with percent_off of 50 will make a $ (or local equivalent)100 invoice $ (or local equivalent)50 instead.
    pub percent_off: Option<f64>,
    /// Date after which the coupon can no longer be redeemed.
    pub redeem_by: Option<stripe_types::Timestamp>,
    /// Number of times this coupon has been applied to a customer.
    pub times_redeemed: i64,
    /// Taking account of the above properties, whether this coupon can still be applied to a customer.
    pub valid: bool,
}
#[cfg(feature = "min-ser")]
pub struct CouponBuilder {
    amount_off: Option<Option<i64>>,
    applies_to: Option<Option<stripe_shared::CouponAppliesTo>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<Option<stripe_types::Currency>>,
    currency_options: Option<Option<std::collections::HashMap<stripe_types::Currency, stripe_shared::CouponCurrencyOption>>>,
    duration: Option<stripe_shared::CouponDuration>,
    duration_in_months: Option<Option<i64>>,
    id: Option<stripe_shared::CouponId>,
    livemode: Option<bool>,
    max_redemptions: Option<Option<i64>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    name: Option<Option<String>>,
    percent_off: Option<Option<f64>>,
    redeem_by: Option<Option<stripe_types::Timestamp>>,
    times_redeemed: Option<i64>,
    valid: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Coupon {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Coupon>,
        builder: CouponBuilder,
    }

    impl Visitor for Place<Coupon> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CouponBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CouponBuilder {
        type Out = Coupon;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_off" => Ok(Deserialize::begin(&mut self.amount_off)),
                "applies_to" => Ok(Deserialize::begin(&mut self.applies_to)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "currency_options" => Ok(Deserialize::begin(&mut self.currency_options)),
                "duration" => Ok(Deserialize::begin(&mut self.duration)),
                "duration_in_months" => Ok(Deserialize::begin(&mut self.duration_in_months)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "max_redemptions" => Ok(Deserialize::begin(&mut self.max_redemptions)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "percent_off" => Ok(Deserialize::begin(&mut self.percent_off)),
                "redeem_by" => Ok(Deserialize::begin(&mut self.redeem_by)),
                "times_redeemed" => Ok(Deserialize::begin(&mut self.times_redeemed)),
                "valid" => Ok(Deserialize::begin(&mut self.valid)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount_off: Deserialize::default(),
                applies_to: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                currency_options: Deserialize::default(),
                duration: Deserialize::default(),
                duration_in_months: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                max_redemptions: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
                percent_off: Deserialize::default(),
                redeem_by: Deserialize::default(),
                times_redeemed: Deserialize::default(),
                valid: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_off = self.amount_off.take()?;
            let applies_to = self.applies_to.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let currency_options = self.currency_options.take()?;
            let duration = self.duration.take()?;
            let duration_in_months = self.duration_in_months.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let max_redemptions = self.max_redemptions.take()?;
            let metadata = self.metadata.take()?;
            let name = self.name.take()?;
            let percent_off = self.percent_off.take()?;
            let redeem_by = self.redeem_by.take()?;
            let times_redeemed = self.times_redeemed.take()?;
            let valid = self.valid.take()?;

            Some(Self::Out {
                amount_off,
                applies_to,
                created,
                currency,
                currency_options,
                duration,
                duration_in_months,
                id,
                livemode,
                max_redemptions,
                metadata,
                name,
                percent_off,
                redeem_by,
                times_redeemed,
                valid,
            })
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

    impl ObjectDeser for Coupon {
        type Builder = CouponBuilder;
    }
};
impl stripe_types::Object for Coupon {
    type Id = stripe_shared::CouponId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CouponId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CouponDuration {
    Forever,
    Once,
    Repeating,
}
impl CouponDuration {
    pub fn as_str(self) -> &'static str {
        use CouponDuration::*;
        match self {
            Forever => "forever",
            Once => "once",
            Repeating => "repeating",
        }
    }
}

impl std::str::FromStr for CouponDuration {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CouponDuration::*;
        match s {
            "forever" => Ok(Forever),
            "once" => Ok(Once),
            "repeating" => Ok(Repeating),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CouponDuration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CouponDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CouponDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CouponDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CouponDuration {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CouponDuration"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CouponDuration {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CouponDuration> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CouponDuration::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
