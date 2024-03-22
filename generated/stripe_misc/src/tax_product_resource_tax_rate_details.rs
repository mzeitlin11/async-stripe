#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceTaxRateDetails {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    /// The tax rate percentage as a string. For example, 8.5% is represented as `"8.5"`.
    pub percentage_decimal: String,
    /// State, county, province, or region.
    pub state: Option<String>,
    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<TaxProductResourceTaxRateDetailsTaxType>,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceTaxRateDetailsBuilder {
    country: Option<Option<String>>,
    percentage_decimal: Option<String>,
    state: Option<Option<String>>,
    tax_type: Option<Option<TaxProductResourceTaxRateDetailsTaxType>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxRateDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxRateDetails>,
        builder: TaxProductResourceTaxRateDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxRateDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceTaxRateDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxRateDetailsBuilder {
        type Out = TaxProductResourceTaxRateDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "percentage_decimal" => Ok(Deserialize::begin(&mut self.percentage_decimal)),
                "state" => Ok(Deserialize::begin(&mut self.state)),
                "tax_type" => Ok(Deserialize::begin(&mut self.tax_type)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { country: Deserialize::default(), percentage_decimal: Deserialize::default(), state: Deserialize::default(), tax_type: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let country = self.country.take()?;
            let percentage_decimal = self.percentage_decimal.take()?;
            let state = self.state.take()?;
            let tax_type = self.tax_type.take()?;

            Some(Self::Out { country, percentage_decimal, state, tax_type })
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

    impl ObjectDeser for TaxProductResourceTaxRateDetails {
        type Builder = TaxProductResourceTaxRateDetailsBuilder;
    }
};
/// The tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxRateDetailsTaxType {
    AmusementTax,
    CommunicationsTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}
impl TaxProductResourceTaxRateDetailsTaxType {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxRateDetailsTaxType::*;
        match self {
            AmusementTax => "amusement_tax",
            CommunicationsTax => "communications_tax",
            Gst => "gst",
            Hst => "hst",
            Igst => "igst",
            Jct => "jct",
            LeaseTax => "lease_tax",
            Pst => "pst",
            Qst => "qst",
            Rst => "rst",
            SalesTax => "sales_tax",
            Vat => "vat",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxRateDetailsTaxType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxRateDetailsTaxType::*;
        match s {
            "amusement_tax" => Ok(AmusementTax),
            "communications_tax" => Ok(CommunicationsTax),
            "gst" => Ok(Gst),
            "hst" => Ok(Hst),
            "igst" => Ok(Igst),
            "jct" => Ok(Jct),
            "lease_tax" => Ok(LeaseTax),
            "pst" => Ok(Pst),
            "qst" => Ok(Qst),
            "rst" => Ok(Rst),
            "sales_tax" => Ok(SalesTax),
            "vat" => Ok(Vat),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductResourceTaxRateDetailsTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductResourceTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceTaxRateDetailsTaxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxRateDetailsTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductResourceTaxRateDetailsTaxType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductResourceTaxRateDetailsTaxType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductResourceTaxRateDetailsTaxType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductResourceTaxRateDetailsTaxType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
