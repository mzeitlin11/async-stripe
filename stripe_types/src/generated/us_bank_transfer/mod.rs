#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct UsBankTransfer {
    /// The banking network used for this funding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<UsBankTransferNetwork>,
    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
}
/// The banking network used for this funding.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsBankTransferNetwork {
    Ach,
    DomesticWireUs,
    Swift,
}

impl UsBankTransferNetwork {
    pub fn as_str(self) -> &'static str {
        use UsBankTransferNetwork::*;
        match self {
            Ach => "ach",
            DomesticWireUs => "domestic_wire_us",
            Swift => "swift",
        }
    }
}

impl std::str::FromStr for UsBankTransferNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UsBankTransferNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "domestic_wire_us" => Ok(DomesticWireUs),
            "swift" => Ok(Swift),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UsBankTransferNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UsBankTransferNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UsBankTransferNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UsBankTransferNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for UsBankTransferNetwork"))
    }
}
