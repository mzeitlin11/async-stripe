#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsDefaults {
    /// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
    /// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
    pub tax_behavior: Option<TaxProductResourceTaxSettingsDefaultsTaxBehavior>,
    /// Default [tax code](https://stripe.com/docs/tax/tax-categories) used to classify your products and prices.
    pub tax_code: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceTaxSettingsDefaultsBuilder {
    tax_behavior: Option<Option<TaxProductResourceTaxSettingsDefaultsTaxBehavior>>,
    tax_code: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxSettingsDefaults {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxSettingsDefaults>,
        builder: TaxProductResourceTaxSettingsDefaultsBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxSettingsDefaults> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceTaxSettingsDefaultsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxSettingsDefaultsBuilder {
        type Out = TaxProductResourceTaxSettingsDefaults;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "tax_behavior" => Ok(Deserialize::begin(&mut self.tax_behavior)),
                "tax_code" => Ok(Deserialize::begin(&mut self.tax_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { tax_behavior: Deserialize::default(), tax_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let tax_behavior = self.tax_behavior.take()?;
            let tax_code = self.tax_code.take()?;

            Some(Self::Out { tax_behavior, tax_code })
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

    impl ObjectDeser for TaxProductResourceTaxSettingsDefaults {
        type Builder = TaxProductResourceTaxSettingsDefaultsBuilder;
    }
};
/// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
/// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
}
impl TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxSettingsDefaultsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            InferredByCurrency => "inferred_by_currency",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxSettingsDefaultsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "inferred_by_currency" => Ok(InferredByCurrency),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductResourceTaxSettingsDefaultsTaxBehavior"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductResourceTaxSettingsDefaultsTaxBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductResourceTaxSettingsDefaultsTaxBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
