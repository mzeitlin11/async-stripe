#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceTaxTransactionShippingCost {
    /// The shipping amount in integer cents.
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for shipping, in integer cents.
    pub amount_tax: i64,
    /// The ID of an existing [ShippingRate](https://stripe.com/docs/api/shipping_rates/object).
    pub shipping_rate: Option<String>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxProductResourceTaxTransactionShippingCostTaxBehavior,
    /// Detailed account of taxes relevant to shipping cost.
    /// (It is not populated for the transaction resource object and will be removed in the next API version.).
    pub tax_breakdown: Option<Vec<stripe_misc::TaxProductResourceLineItemTaxBreakdown>>,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for shipping.
    pub tax_code: String,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceTaxTransactionShippingCostBuilder {
    amount: Option<i64>,
    amount_tax: Option<i64>,
    shipping_rate: Option<Option<String>>,
    tax_behavior: Option<TaxProductResourceTaxTransactionShippingCostTaxBehavior>,
    tax_breakdown: Option<Option<Vec<stripe_misc::TaxProductResourceLineItemTaxBreakdown>>>,
    tax_code: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxTransactionShippingCost {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxTransactionShippingCost>,
        builder: TaxProductResourceTaxTransactionShippingCostBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxTransactionShippingCost> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceTaxTransactionShippingCostBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxTransactionShippingCostBuilder {
        type Out = TaxProductResourceTaxTransactionShippingCost;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_tax" => Ok(Deserialize::begin(&mut self.amount_tax)),
                "shipping_rate" => Ok(Deserialize::begin(&mut self.shipping_rate)),
                "tax_behavior" => Ok(Deserialize::begin(&mut self.tax_behavior)),
                "tax_breakdown" => Ok(Deserialize::begin(&mut self.tax_breakdown)),
                "tax_code" => Ok(Deserialize::begin(&mut self.tax_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_tax: Deserialize::default(),
                shipping_rate: Deserialize::default(),
                tax_behavior: Deserialize::default(),
                tax_breakdown: Deserialize::default(),
                tax_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_tax = self.amount_tax.take()?;
            let shipping_rate = self.shipping_rate.take()?;
            let tax_behavior = self.tax_behavior.take()?;
            let tax_breakdown = self.tax_breakdown.take()?;
            let tax_code = self.tax_code.take()?;

            Some(Self::Out { amount, amount_tax, shipping_rate, tax_behavior, tax_breakdown, tax_code })
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

    impl ObjectDeser for TaxProductResourceTaxTransactionShippingCost {
        type Builder = TaxProductResourceTaxTransactionShippingCostBuilder;
    }
};
/// Specifies whether the `amount` includes taxes.
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
}
impl TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxTransactionShippingCostTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxTransactionShippingCostTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductResourceTaxTransactionShippingCostTaxBehavior"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductResourceTaxTransactionShippingCostTaxBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductResourceTaxTransactionShippingCostTaxBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
