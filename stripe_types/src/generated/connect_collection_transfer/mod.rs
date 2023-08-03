#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectCollectionTransfer {
    /// Amount transferred, in %s.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the account that funds are being collected for.
    pub destination: stripe_types::Expandable<stripe_types::Account>,
    /// Unique identifier for the object.
    pub id: stripe_types::connect_collection_transfer::ConnectCollectionTransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ConnectCollectionTransferObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConnectCollectionTransferObject {
    ConnectCollectionTransfer,
}

impl ConnectCollectionTransferObject {
    pub fn as_str(self) -> &'static str {
        use ConnectCollectionTransferObject::*;
        match self {
            ConnectCollectionTransfer => "connect_collection_transfer",
        }
    }
}

impl std::str::FromStr for ConnectCollectionTransferObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConnectCollectionTransferObject::*;
        match s {
            "connect_collection_transfer" => Ok(ConnectCollectionTransfer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ConnectCollectionTransferObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConnectCollectionTransferObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConnectCollectionTransferObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConnectCollectionTransferObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConnectCollectionTransferObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for ConnectCollectionTransferObject"))
    }
}
impl stripe_types::Object for ConnectCollectionTransfer {
    type Id = stripe_types::connect_collection_transfer::ConnectCollectionTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ConnectCollectionTransferId, "connct_");
