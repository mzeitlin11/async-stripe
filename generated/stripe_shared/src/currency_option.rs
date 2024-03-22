#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CurrencyOption {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    pub custom_unit_amount: Option<stripe_shared::CustomUnitAmount>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub tax_behavior: Option<CurrencyOptionTaxBehavior>,
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    pub tiers: Option<Vec<stripe_shared::PriceTier>>,
    /// The unit amount in cents (or local equivalent) to be charged, represented as a whole integer if possible.
    /// Only set if `billing_scheme=per_unit`.
    pub unit_amount: Option<i64>,
    /// The unit amount in cents (or local equivalent) to be charged, represented as a decimal string with at most 12 decimal places.
    /// Only set if `billing_scheme=per_unit`.
    pub unit_amount_decimal: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct CurrencyOptionBuilder {
    custom_unit_amount: Option<Option<stripe_shared::CustomUnitAmount>>,
    tax_behavior: Option<Option<CurrencyOptionTaxBehavior>>,
    tiers: Option<Option<Vec<stripe_shared::PriceTier>>>,
    unit_amount: Option<Option<i64>>,
    unit_amount_decimal: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CurrencyOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CurrencyOption>,
        builder: CurrencyOptionBuilder,
    }

    impl Visitor for Place<CurrencyOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CurrencyOptionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CurrencyOptionBuilder {
        type Out = CurrencyOption;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "custom_unit_amount" => Ok(Deserialize::begin(&mut self.custom_unit_amount)),
                "tax_behavior" => Ok(Deserialize::begin(&mut self.tax_behavior)),
                "tiers" => Ok(Deserialize::begin(&mut self.tiers)),
                "unit_amount" => Ok(Deserialize::begin(&mut self.unit_amount)),
                "unit_amount_decimal" => Ok(Deserialize::begin(&mut self.unit_amount_decimal)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                custom_unit_amount: Deserialize::default(),
                tax_behavior: Deserialize::default(),
                tiers: Deserialize::default(),
                unit_amount: Deserialize::default(),
                unit_amount_decimal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let custom_unit_amount = self.custom_unit_amount.take()?;
            let tax_behavior = self.tax_behavior.take()?;
            let tiers = self.tiers.take()?;
            let unit_amount = self.unit_amount.take()?;
            let unit_amount_decimal = self.unit_amount_decimal.take()?;

            Some(Self::Out { custom_unit_amount, tax_behavior, tiers, unit_amount, unit_amount_decimal })
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

    impl ObjectDeser for CurrencyOption {
        type Builder = CurrencyOptionBuilder;
    }
};
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CurrencyOptionTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CurrencyOptionTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CurrencyOptionTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CurrencyOptionTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CurrencyOptionTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CurrencyOptionTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CurrencyOptionTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CurrencyOptionTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CurrencyOptionTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CurrencyOptionTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CurrencyOptionTaxBehavior"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CurrencyOptionTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CurrencyOptionTaxBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CurrencyOptionTaxBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
