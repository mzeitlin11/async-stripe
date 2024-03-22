#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct UsBankAccountNetworks {
    /// The preferred network.
    pub preferred: Option<String>,
    /// All supported networks.
    pub supported: Vec<UsBankAccountNetworksSupported>,
}
#[cfg(feature = "min-ser")]
pub struct UsBankAccountNetworksBuilder {
    preferred: Option<Option<String>>,
    supported: Option<Vec<UsBankAccountNetworksSupported>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for UsBankAccountNetworks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<UsBankAccountNetworks>,
        builder: UsBankAccountNetworksBuilder,
    }

    impl Visitor for Place<UsBankAccountNetworks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: UsBankAccountNetworksBuilder::deser_default() }))
        }
    }

    impl MapBuilder for UsBankAccountNetworksBuilder {
        type Out = UsBankAccountNetworks;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "preferred" => Ok(Deserialize::begin(&mut self.preferred)),
                "supported" => Ok(Deserialize::begin(&mut self.supported)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { preferred: Deserialize::default(), supported: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let preferred = self.preferred.take()?;
            let supported = self.supported.take()?;

            Some(Self::Out { preferred, supported })
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

    impl ObjectDeser for UsBankAccountNetworks {
        type Builder = UsBankAccountNetworksBuilder;
    }
};
/// All supported networks.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UsBankAccountNetworksSupported {
    Ach,
    UsDomesticWire,
}
impl UsBankAccountNetworksSupported {
    pub fn as_str(self) -> &'static str {
        use UsBankAccountNetworksSupported::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for UsBankAccountNetworksSupported {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UsBankAccountNetworksSupported::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UsBankAccountNetworksSupported {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UsBankAccountNetworksSupported {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UsBankAccountNetworksSupported {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UsBankAccountNetworksSupported"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for UsBankAccountNetworksSupported {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<UsBankAccountNetworksSupported> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(UsBankAccountNetworksSupported::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
