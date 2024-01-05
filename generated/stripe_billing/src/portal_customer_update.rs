#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalCustomerUpdate {
    /// The types of customer updates that are supported. When empty, customers are not updateable.
    pub allowed_updates: Vec<PortalCustomerUpdateAllowedUpdates>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
#[cfg(feature = "min-ser")]
pub struct PortalCustomerUpdateBuilder {
    allowed_updates: Option<Vec<PortalCustomerUpdateAllowedUpdates>>,
    enabled: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalCustomerUpdate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalCustomerUpdate>,
        builder: PortalCustomerUpdateBuilder,
    }

    impl Visitor for Place<PortalCustomerUpdate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalCustomerUpdateBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalCustomerUpdateBuilder {
        type Out = PortalCustomerUpdate;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "allowed_updates" => Ok(Deserialize::begin(&mut self.allowed_updates)),
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { allowed_updates: Deserialize::default(), enabled: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let allowed_updates = self.allowed_updates.take()?;
            let enabled = self.enabled.take()?;

            Some(Self::Out { allowed_updates, enabled })
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

    impl ObjectDeser for PortalCustomerUpdate {
        type Builder = PortalCustomerUpdateBuilder;
    }
};
/// The types of customer updates that are supported. When empty, customers are not updateable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
}
impl PortalCustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        use PortalCustomerUpdateAllowedUpdates::*;
        match self {
            Address => "address",
            Email => "email",
            Name => "name",
            Phone => "phone",
            Shipping => "shipping",
            TaxId => "tax_id",
        }
    }
}

impl std::str::FromStr for PortalCustomerUpdateAllowedUpdates {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalCustomerUpdateAllowedUpdates::*;
        match s {
            "address" => Ok(Address),
            "email" => Ok(Email),
            "name" => Ok(Name),
            "phone" => Ok(Phone),
            "shipping" => Ok(Shipping),
            "tax_id" => Ok(TaxId),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PortalCustomerUpdateAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PortalCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalCustomerUpdateAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalCustomerUpdateAllowedUpdates {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PortalCustomerUpdateAllowedUpdates"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PortalCustomerUpdateAllowedUpdates {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PortalCustomerUpdateAllowedUpdates> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalCustomerUpdateAllowedUpdates::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
