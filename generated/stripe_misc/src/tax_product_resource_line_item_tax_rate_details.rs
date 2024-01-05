#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceLineItemTaxRateDetails {
    /// A localized display name for tax type, intended to be human-readable.
    /// For example, "Local Sales and Use Tax", "Value-added tax (VAT)", or "Umsatzsteuer (USt.)".
    pub display_name: String,
    /// The tax rate percentage as a string. For example, 8.5% is represented as "8.5".
    pub percentage_decimal: String,
    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: TaxProductResourceLineItemTaxRateDetailsTaxType,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceLineItemTaxRateDetailsBuilder {
    display_name: Option<String>,
    percentage_decimal: Option<String>,
    tax_type: Option<TaxProductResourceLineItemTaxRateDetailsTaxType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceLineItemTaxRateDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceLineItemTaxRateDetails>,
        builder: TaxProductResourceLineItemTaxRateDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceLineItemTaxRateDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceLineItemTaxRateDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceLineItemTaxRateDetailsBuilder {
        type Out = TaxProductResourceLineItemTaxRateDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "display_name" => Ok(Deserialize::begin(&mut self.display_name)),
                "percentage_decimal" => Ok(Deserialize::begin(&mut self.percentage_decimal)),
                "tax_type" => Ok(Deserialize::begin(&mut self.tax_type)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { display_name: Deserialize::default(), percentage_decimal: Deserialize::default(), tax_type: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let display_name = self.display_name.take()?;
            let percentage_decimal = self.percentage_decimal.take()?;
            let tax_type = self.tax_type.take()?;

            Some(Self::Out { display_name, percentage_decimal, tax_type })
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

    impl ObjectDeser for TaxProductResourceLineItemTaxRateDetails {
        type Builder = TaxProductResourceLineItemTaxRateDetailsBuilder;
    }
};
/// The tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceLineItemTaxRateDetailsTaxType {
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
impl TaxProductResourceLineItemTaxRateDetailsTaxType {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceLineItemTaxRateDetailsTaxType::*;
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

impl std::str::FromStr for TaxProductResourceLineItemTaxRateDetailsTaxType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceLineItemTaxRateDetailsTaxType::*;
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
impl AsRef<str> for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductResourceLineItemTaxRateDetailsTaxType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductResourceLineItemTaxRateDetailsTaxType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductResourceLineItemTaxRateDetailsTaxType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
