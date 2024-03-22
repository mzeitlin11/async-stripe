#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,
    pub rate: stripe_shared::TaxRate,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<LineItemsTaxAmountTaxabilityReason>,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct LineItemsTaxAmountBuilder {
    amount: Option<i64>,
    rate: Option<stripe_shared::TaxRate>,
    taxability_reason: Option<Option<LineItemsTaxAmountTaxabilityReason>>,
    taxable_amount: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for LineItemsTaxAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LineItemsTaxAmount>,
        builder: LineItemsTaxAmountBuilder,
    }

    impl Visitor for Place<LineItemsTaxAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: LineItemsTaxAmountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for LineItemsTaxAmountBuilder {
        type Out = LineItemsTaxAmount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "rate" => Ok(Deserialize::begin(&mut self.rate)),
                "taxability_reason" => Ok(Deserialize::begin(&mut self.taxability_reason)),
                "taxable_amount" => Ok(Deserialize::begin(&mut self.taxable_amount)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), rate: Deserialize::default(), taxability_reason: Deserialize::default(), taxable_amount: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let rate = self.rate.take()?;
            let taxability_reason = self.taxability_reason.take()?;
            let taxable_amount = self.taxable_amount.take()?;

            Some(Self::Out { amount, rate, taxability_reason, taxable_amount })
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

    impl ObjectDeser for LineItemsTaxAmount {
        type Builder = LineItemsTaxAmountBuilder;
    }
};
/// The reasoning behind this tax, for example, if the product is tax exempt.
/// The possible values for this field may be extended as new tax rules are supported.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum LineItemsTaxAmountTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl LineItemsTaxAmountTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        use LineItemsTaxAmountTaxabilityReason::*;
        match self {
            CustomerExempt => "customer_exempt",
            NotCollecting => "not_collecting",
            NotSubjectToTax => "not_subject_to_tax",
            NotSupported => "not_supported",
            PortionProductExempt => "portion_product_exempt",
            PortionReducedRated => "portion_reduced_rated",
            PortionStandardRated => "portion_standard_rated",
            ProductExempt => "product_exempt",
            ProductExemptHoliday => "product_exempt_holiday",
            ProportionallyRated => "proportionally_rated",
            ReducedRated => "reduced_rated",
            ReverseCharge => "reverse_charge",
            StandardRated => "standard_rated",
            TaxableBasisReduced => "taxable_basis_reduced",
            ZeroRated => "zero_rated",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for LineItemsTaxAmountTaxabilityReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LineItemsTaxAmountTaxabilityReason::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "not_collecting" => Ok(NotCollecting),
            "not_subject_to_tax" => Ok(NotSubjectToTax),
            "not_supported" => Ok(NotSupported),
            "portion_product_exempt" => Ok(PortionProductExempt),
            "portion_reduced_rated" => Ok(PortionReducedRated),
            "portion_standard_rated" => Ok(PortionStandardRated),
            "product_exempt" => Ok(ProductExempt),
            "product_exempt_holiday" => Ok(ProductExemptHoliday),
            "proportionally_rated" => Ok(ProportionallyRated),
            "reduced_rated" => Ok(ReducedRated),
            "reverse_charge" => Ok(ReverseCharge),
            "standard_rated" => Ok(StandardRated),
            "taxable_basis_reduced" => Ok(TaxableBasisReduced),
            "zero_rated" => Ok(ZeroRated),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for LineItemsTaxAmountTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for LineItemsTaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for LineItemsTaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for LineItemsTaxAmountTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LineItemsTaxAmountTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LineItemsTaxAmountTaxabilityReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<LineItemsTaxAmountTaxabilityReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(LineItemsTaxAmountTaxabilityReason::from_str(s).unwrap_or(LineItemsTaxAmountTaxabilityReason::Unknown));
        Ok(())
    }
}
