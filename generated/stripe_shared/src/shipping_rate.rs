/// Shipping rates describe the price of shipping presented to your customers and
/// applied to a purchase.
/// For more information, see [Charge for shipping](https://stripe.com/docs/payments/during-payment/charge-shipping).
///
/// For more details see <<https://stripe.com/docs/api/shipping_rates/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ShippingRate {
    /// Whether the shipping rate can be used for new purchases. Defaults to `true`.
    pub active: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub delivery_estimate: Option<stripe_shared::ShippingRateDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub display_name: Option<String>,
    pub fixed_amount: Option<stripe_shared::ShippingRateFixedAmount>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ShippingRateId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: Option<stripe_shared::ShippingRateTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    /// The Shipping tax code is `txcd_92010001`.
    pub tax_code: Option<stripe_types::Expandable<stripe_shared::TaxCode>>,
    /// The type of calculation to use on the shipping rate. Can only be `fixed_amount` for now.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: stripe_shared::ShippingRateType,
}
#[cfg(feature = "min-ser")]
pub struct ShippingRateBuilder {
    active: Option<bool>,
    created: Option<stripe_types::Timestamp>,
    delivery_estimate: Option<Option<stripe_shared::ShippingRateDeliveryEstimate>>,
    display_name: Option<Option<String>>,
    fixed_amount: Option<Option<stripe_shared::ShippingRateFixedAmount>>,
    id: Option<stripe_shared::ShippingRateId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    tax_behavior: Option<Option<stripe_shared::ShippingRateTaxBehavior>>,
    tax_code: Option<Option<stripe_types::Expandable<stripe_shared::TaxCode>>>,
    type_: Option<stripe_shared::ShippingRateType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ShippingRate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ShippingRate>,
        builder: ShippingRateBuilder,
    }

    impl Visitor for Place<ShippingRate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ShippingRateBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ShippingRateBuilder {
        type Out = ShippingRate;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "active" => Ok(Deserialize::begin(&mut self.active)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "delivery_estimate" => Ok(Deserialize::begin(&mut self.delivery_estimate)),
                "display_name" => Ok(Deserialize::begin(&mut self.display_name)),
                "fixed_amount" => Ok(Deserialize::begin(&mut self.fixed_amount)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "tax_behavior" => Ok(Deserialize::begin(&mut self.tax_behavior)),
                "tax_code" => Ok(Deserialize::begin(&mut self.tax_code)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                created: Deserialize::default(),
                delivery_estimate: Deserialize::default(),
                display_name: Deserialize::default(),
                fixed_amount: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                tax_behavior: Deserialize::default(),
                tax_code: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let active = self.active.take()?;
            let created = self.created.take()?;
            let delivery_estimate = self.delivery_estimate.take()?;
            let display_name = self.display_name.take()?;
            let fixed_amount = self.fixed_amount.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let tax_behavior = self.tax_behavior.take()?;
            let tax_code = self.tax_code.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { active, created, delivery_estimate, display_name, fixed_amount, id, livemode, metadata, tax_behavior, tax_code, type_ })
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

    impl ObjectDeser for ShippingRate {
        type Builder = ShippingRateBuilder;
    }
};
impl stripe_types::Object for ShippingRate {
    type Id = stripe_shared::ShippingRateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ShippingRateId, "shr_");
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ShippingRateTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl ShippingRateTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use ShippingRateTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for ShippingRateTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShippingRateTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ShippingRateTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ShippingRateTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ShippingRateTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ShippingRateTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRateTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ShippingRateTaxBehavior"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingRateTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ShippingRateTaxBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingRateTaxBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ShippingRateType {
    FixedAmount,
}
impl ShippingRateType {
    pub fn as_str(self) -> &'static str {
        use ShippingRateType::*;
        match self {
            FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for ShippingRateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShippingRateType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ShippingRateType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ShippingRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ShippingRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ShippingRateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ShippingRateType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingRateType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ShippingRateType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingRateType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
