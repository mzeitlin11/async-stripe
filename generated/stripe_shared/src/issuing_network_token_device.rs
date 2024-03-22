#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingNetworkTokenDevice {
    /// An obfuscated ID derived from the device ID.
    pub device_fingerprint: Option<String>,
    /// The IP address of the device at provisioning time.
    pub ip_address: Option<String>,
    /// The geographic latitude/longitude coordinates of the device at provisioning time.
    /// The format is [+-]decimal/[+-]decimal.
    pub location: Option<String>,
    /// The name of the device used for tokenization.
    pub name: Option<String>,
    /// The phone number of the device used for tokenization.
    pub phone_number: Option<String>,
    /// The type of device used for tokenization.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: Option<IssuingNetworkTokenDeviceType>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingNetworkTokenDeviceBuilder {
    device_fingerprint: Option<Option<String>>,
    ip_address: Option<Option<String>>,
    location: Option<Option<String>>,
    name: Option<Option<String>>,
    phone_number: Option<Option<String>>,
    type_: Option<Option<IssuingNetworkTokenDeviceType>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingNetworkTokenDevice {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenDevice>,
        builder: IssuingNetworkTokenDeviceBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenDevice> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingNetworkTokenDeviceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenDeviceBuilder {
        type Out = IssuingNetworkTokenDevice;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "device_fingerprint" => Ok(Deserialize::begin(&mut self.device_fingerprint)),
                "ip_address" => Ok(Deserialize::begin(&mut self.ip_address)),
                "location" => Ok(Deserialize::begin(&mut self.location)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "phone_number" => Ok(Deserialize::begin(&mut self.phone_number)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                device_fingerprint: Deserialize::default(),
                ip_address: Deserialize::default(),
                location: Deserialize::default(),
                name: Deserialize::default(),
                phone_number: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let device_fingerprint = self.device_fingerprint.take()?;
            let ip_address = self.ip_address.take()?;
            let location = self.location.take()?;
            let name = self.name.take()?;
            let phone_number = self.phone_number.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { device_fingerprint, ip_address, location, name, phone_number, type_ })
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

    impl ObjectDeser for IssuingNetworkTokenDevice {
        type Builder = IssuingNetworkTokenDeviceBuilder;
    }
};
/// The type of device used for tokenization.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenDeviceType {
    Other,
    Phone,
    Watch,
}
impl IssuingNetworkTokenDeviceType {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenDeviceType::*;
        match self {
            Other => "other",
            Phone => "phone",
            Watch => "watch",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenDeviceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenDeviceType::*;
        match s {
            "other" => Ok(Other),
            "phone" => Ok(Phone),
            "watch" => Ok(Watch),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingNetworkTokenDeviceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenDeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenDeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingNetworkTokenDeviceType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingNetworkTokenDeviceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingNetworkTokenDeviceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingNetworkTokenDeviceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
