#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingNetworkTokenNetworkData {
    pub device: Option<stripe_shared::IssuingNetworkTokenDevice>,
    pub mastercard: Option<stripe_shared::IssuingNetworkTokenMastercard>,
    /// The network that the token is associated with.
    /// An additional hash is included with a name matching this value, containing tokenization data specific to the card network.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: IssuingNetworkTokenNetworkDataType,
    pub visa: Option<stripe_shared::IssuingNetworkTokenVisa>,
    pub wallet_provider: Option<stripe_shared::IssuingNetworkTokenWalletProvider>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingNetworkTokenNetworkDataBuilder {
    device: Option<Option<stripe_shared::IssuingNetworkTokenDevice>>,
    mastercard: Option<Option<stripe_shared::IssuingNetworkTokenMastercard>>,
    type_: Option<IssuingNetworkTokenNetworkDataType>,
    visa: Option<Option<stripe_shared::IssuingNetworkTokenVisa>>,
    wallet_provider: Option<Option<stripe_shared::IssuingNetworkTokenWalletProvider>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingNetworkTokenNetworkData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenNetworkData>,
        builder: IssuingNetworkTokenNetworkDataBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenNetworkData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingNetworkTokenNetworkDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenNetworkDataBuilder {
        type Out = IssuingNetworkTokenNetworkData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "device" => Ok(Deserialize::begin(&mut self.device)),
                "mastercard" => Ok(Deserialize::begin(&mut self.mastercard)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "visa" => Ok(Deserialize::begin(&mut self.visa)),
                "wallet_provider" => Ok(Deserialize::begin(&mut self.wallet_provider)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { device: Deserialize::default(), mastercard: Deserialize::default(), type_: Deserialize::default(), visa: Deserialize::default(), wallet_provider: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let device = self.device.take()?;
            let mastercard = self.mastercard.take()?;
            let type_ = self.type_.take()?;
            let visa = self.visa.take()?;
            let wallet_provider = self.wallet_provider.take()?;

            Some(Self::Out { device, mastercard, type_, visa, wallet_provider })
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

    impl ObjectDeser for IssuingNetworkTokenNetworkData {
        type Builder = IssuingNetworkTokenNetworkDataBuilder;
    }
};
/// The network that the token is associated with.
/// An additional hash is included with a name matching this value, containing tokenization data specific to the card network.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenNetworkDataType {
    Mastercard,
    Visa,
}
impl IssuingNetworkTokenNetworkDataType {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenNetworkDataType::*;
        match self {
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenNetworkDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenNetworkDataType::*;
        match s {
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingNetworkTokenNetworkDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenNetworkDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenNetworkDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingNetworkTokenNetworkDataType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingNetworkTokenNetworkDataType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingNetworkTokenNetworkDataType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingNetworkTokenNetworkDataType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
