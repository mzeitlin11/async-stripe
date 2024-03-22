/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceFinancialAddress {
    pub aba: Option<stripe_treasury::TreasuryFinancialAccountsResourceAbaRecord>,
    /// The list of networks that the address supports
    pub supported_networks: Option<Vec<TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks>>,
    /// The type of financial address
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: TreasuryFinancialAccountsResourceFinancialAddressType,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryFinancialAccountsResourceFinancialAddressBuilder {
    aba: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceAbaRecord>>,
    supported_networks: Option<Option<Vec<TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks>>>,
    type_: Option<TreasuryFinancialAccountsResourceFinancialAddressType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountsResourceFinancialAddress {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceFinancialAddress>,
        builder: TreasuryFinancialAccountsResourceFinancialAddressBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceFinancialAddress> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryFinancialAccountsResourceFinancialAddressBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceFinancialAddressBuilder {
        type Out = TreasuryFinancialAccountsResourceFinancialAddress;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "aba" => Ok(Deserialize::begin(&mut self.aba)),
                "supported_networks" => Ok(Deserialize::begin(&mut self.supported_networks)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { aba: Deserialize::default(), supported_networks: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let aba = self.aba.take()?;
            let supported_networks = self.supported_networks.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { aba, supported_networks, type_ })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceFinancialAddress {
        type Builder = TreasuryFinancialAccountsResourceFinancialAddressBuilder;
    }
};
/// The list of networks that the address supports
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    Ach,
    UsDomesticWire,
}
impl TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The type of financial address
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceFinancialAddressType {
    Aba,
}
impl TreasuryFinancialAccountsResourceFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceFinancialAddressType::*;
        match self {
            Aba => "aba",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceFinancialAddressType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceFinancialAddressType::*;
        match s {
            "aba" => Ok(Aba),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceFinancialAddressType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryFinancialAccountsResourceFinancialAddressType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryFinancialAccountsResourceFinancialAddressType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
