#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceCustomerDetails {
    /// The customer's postal address (for example, home or business location).
    pub address: Option<stripe_misc::TaxProductResourcePostalAddress>,
    /// The type of customer address provided.
    pub address_source: Option<TaxProductResourceCustomerDetailsAddressSource>,
    /// The customer's IP address (IPv4 or IPv6).
    pub ip_address: Option<String>,
    /// The customer's tax IDs (for example, EU VAT numbers).
    pub tax_ids: Vec<stripe_misc::TaxProductResourceCustomerDetailsResourceTaxId>,
    /// The taxability override used for taxation.
    pub taxability_override: TaxProductResourceCustomerDetailsTaxabilityOverride,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceCustomerDetailsBuilder {
    address: Option<Option<stripe_misc::TaxProductResourcePostalAddress>>,
    address_source: Option<Option<TaxProductResourceCustomerDetailsAddressSource>>,
    ip_address: Option<Option<String>>,
    tax_ids: Option<Vec<stripe_misc::TaxProductResourceCustomerDetailsResourceTaxId>>,
    taxability_override: Option<TaxProductResourceCustomerDetailsTaxabilityOverride>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceCustomerDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceCustomerDetails>,
        builder: TaxProductResourceCustomerDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceCustomerDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceCustomerDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceCustomerDetailsBuilder {
        type Out = TaxProductResourceCustomerDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),
                "address_source" => Ok(Deserialize::begin(&mut self.address_source)),
                "ip_address" => Ok(Deserialize::begin(&mut self.ip_address)),
                "tax_ids" => Ok(Deserialize::begin(&mut self.tax_ids)),
                "taxability_override" => Ok(Deserialize::begin(&mut self.taxability_override)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                address_source: Deserialize::default(),
                ip_address: Deserialize::default(),
                tax_ids: Deserialize::default(),
                taxability_override: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;
            let address_source = self.address_source.take()?;
            let ip_address = self.ip_address.take()?;
            let tax_ids = self.tax_ids.take()?;
            let taxability_override = self.taxability_override.take()?;

            Some(Self::Out { address, address_source, ip_address, tax_ids, taxability_override })
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

    impl ObjectDeser for TaxProductResourceCustomerDetails {
        type Builder = TaxProductResourceCustomerDetailsBuilder;
    }
};
/// The type of customer address provided.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceCustomerDetailsAddressSource {
    Billing,
    Shipping,
}
impl TaxProductResourceCustomerDetailsAddressSource {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceCustomerDetailsAddressSource::*;
        match self {
            Billing => "billing",
            Shipping => "shipping",
        }
    }
}

impl std::str::FromStr for TaxProductResourceCustomerDetailsAddressSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceCustomerDetailsAddressSource::*;
        match s {
            "billing" => Ok(Billing),
            "shipping" => Ok(Shipping),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductResourceCustomerDetailsAddressSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceCustomerDetailsAddressSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceCustomerDetailsAddressSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductResourceCustomerDetailsAddressSource"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductResourceCustomerDetailsAddressSource {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductResourceCustomerDetailsAddressSource> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductResourceCustomerDetailsAddressSource::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The taxability override used for taxation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
}
impl TaxProductResourceCustomerDetailsTaxabilityOverride {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceCustomerDetailsTaxabilityOverride::*;
        match self {
            CustomerExempt => "customer_exempt",
            None => "none",
            ReverseCharge => "reverse_charge",
        }
    }
}

impl std::str::FromStr for TaxProductResourceCustomerDetailsTaxabilityOverride {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceCustomerDetailsTaxabilityOverride::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "none" => Ok(None),
            "reverse_charge" => Ok(ReverseCharge),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductResourceCustomerDetailsTaxabilityOverride"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductResourceCustomerDetailsTaxabilityOverride> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductResourceCustomerDetailsTaxabilityOverride::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
