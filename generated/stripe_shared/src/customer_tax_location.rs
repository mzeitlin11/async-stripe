#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerTaxLocation {
    /// The customer's country as identified by Stripe Tax.
    pub country: String,
    /// The data source used to infer the customer's location.
    pub source: CustomerTaxLocationSource,
    /// The customer's state, county, province, or region as identified by Stripe Tax.
    pub state: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct CustomerTaxLocationBuilder {
    country: Option<String>,
    source: Option<CustomerTaxLocationSource>,
    state: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerTaxLocation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerTaxLocation>,
        builder: CustomerTaxLocationBuilder,
    }

    impl Visitor for Place<CustomerTaxLocation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomerTaxLocationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomerTaxLocationBuilder {
        type Out = CustomerTaxLocation;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "source" => Ok(Deserialize::begin(&mut self.source)),
                "state" => Ok(Deserialize::begin(&mut self.state)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { country: Deserialize::default(), source: Deserialize::default(), state: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let country = self.country.take()?;
            let source = self.source.take()?;
            let state = self.state.take()?;

            Some(Self::Out { country, source, state })
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

    impl ObjectDeser for CustomerTaxLocation {
        type Builder = CustomerTaxLocationBuilder;
    }
};
/// The data source used to infer the customer's location.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerTaxLocationSource {
    BillingAddress,
    IpAddress,
    PaymentMethod,
    ShippingDestination,
}
impl CustomerTaxLocationSource {
    pub fn as_str(self) -> &'static str {
        use CustomerTaxLocationSource::*;
        match self {
            BillingAddress => "billing_address",
            IpAddress => "ip_address",
            PaymentMethod => "payment_method",
            ShippingDestination => "shipping_destination",
        }
    }
}

impl std::str::FromStr for CustomerTaxLocationSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerTaxLocationSource::*;
        match s {
            "billing_address" => Ok(BillingAddress),
            "ip_address" => Ok(IpAddress),
            "payment_method" => Ok(PaymentMethod),
            "shipping_destination" => Ok(ShippingDestination),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CustomerTaxLocationSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CustomerTaxLocationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerTaxLocationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CustomerTaxLocationSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerTaxLocationSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerTaxLocationSource"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerTaxLocationSource {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CustomerTaxLocationSource> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerTaxLocationSource::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
