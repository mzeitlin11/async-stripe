/// An issuing token object is created when an issued card is added to a digital wallet.
/// As a [card issuer](https://stripe.com/docs/issuing), you can [view and manage these tokens](https://stripe.com/docs/issuing/controls/token-management) through Stripe.
///
/// For more details see <<https://stripe.com/docs/api/issuing/tokens/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingToken {
    /// Card associated with this token.
    pub card: stripe_types::Expandable<stripe_shared::IssuingCard>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The hashed ID derived from the device ID from the card network associated with the token
    pub device_fingerprint: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingTokenId,
    /// The last four digits of the token.
    pub last4: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The token service provider / card network associated with the token.
    pub network: IssuingTokenNetwork,
    pub network_data: Option<stripe_shared::IssuingNetworkTokenNetworkData>,
    /// Time at which the token was last updated by the card network.
    /// Measured in seconds since the Unix epoch.
    pub network_updated_at: stripe_types::Timestamp,
    /// The usage state of the token.
    pub status: stripe_shared::IssuingTokenStatus,
    /// The digital wallet for this token, if one was used.
    pub wallet_provider: Option<IssuingTokenWalletProvider>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTokenBuilder {
    card: Option<stripe_types::Expandable<stripe_shared::IssuingCard>>,
    created: Option<stripe_types::Timestamp>,
    device_fingerprint: Option<Option<String>>,
    id: Option<stripe_shared::IssuingTokenId>,
    last4: Option<Option<String>>,
    livemode: Option<bool>,
    network: Option<IssuingTokenNetwork>,
    network_data: Option<Option<stripe_shared::IssuingNetworkTokenNetworkData>>,
    network_updated_at: Option<stripe_types::Timestamp>,
    status: Option<stripe_shared::IssuingTokenStatus>,
    wallet_provider: Option<Option<IssuingTokenWalletProvider>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingToken {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingToken>,
        builder: IssuingTokenBuilder,
    }

    impl Visitor for Place<IssuingToken> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTokenBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTokenBuilder {
        type Out = IssuingToken;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "device_fingerprint" => Ok(Deserialize::begin(&mut self.device_fingerprint)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "network" => Ok(Deserialize::begin(&mut self.network)),
                "network_data" => Ok(Deserialize::begin(&mut self.network_data)),
                "network_updated_at" => Ok(Deserialize::begin(&mut self.network_updated_at)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "wallet_provider" => Ok(Deserialize::begin(&mut self.wallet_provider)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                card: Deserialize::default(),
                created: Deserialize::default(),
                device_fingerprint: Deserialize::default(),
                id: Deserialize::default(),
                last4: Deserialize::default(),
                livemode: Deserialize::default(),
                network: Deserialize::default(),
                network_data: Deserialize::default(),
                network_updated_at: Deserialize::default(),
                status: Deserialize::default(),
                wallet_provider: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let card = self.card.take()?;
            let created = self.created.take()?;
            let device_fingerprint = self.device_fingerprint.take()?;
            let id = self.id.take()?;
            let last4 = self.last4.take()?;
            let livemode = self.livemode.take()?;
            let network = self.network.take()?;
            let network_data = self.network_data.take()?;
            let network_updated_at = self.network_updated_at.take()?;
            let status = self.status.take()?;
            let wallet_provider = self.wallet_provider.take()?;

            Some(Self::Out { card, created, device_fingerprint, id, last4, livemode, network, network_data, network_updated_at, status, wallet_provider })
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

    impl ObjectDeser for IssuingToken {
        type Builder = IssuingTokenBuilder;
    }
};
/// The token service provider / card network associated with the token.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTokenNetwork {
    Mastercard,
    Visa,
}
impl IssuingTokenNetwork {
    pub fn as_str(self) -> &'static str {
        use IssuingTokenNetwork::*;
        match self {
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for IssuingTokenNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTokenNetwork::*;
        match s {
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingTokenNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingTokenNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTokenNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTokenNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTokenNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingTokenNetwork"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingTokenNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingTokenNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTokenNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The digital wallet for this token, if one was used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTokenWalletProvider {
    ApplePay,
    GooglePay,
    SamsungPay,
}
impl IssuingTokenWalletProvider {
    pub fn as_str(self) -> &'static str {
        use IssuingTokenWalletProvider::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            SamsungPay => "samsung_pay",
        }
    }
}

impl std::str::FromStr for IssuingTokenWalletProvider {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTokenWalletProvider::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "samsung_pay" => Ok(SamsungPay),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingTokenWalletProvider {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTokenWalletProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTokenWalletProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingTokenWalletProvider"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingTokenWalletProvider {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingTokenWalletProvider> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTokenWalletProvider::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for IssuingToken {
    type Id = stripe_shared::IssuingTokenId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IssuingTokenId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTokenStatus {
    Active,
    Deleted,
    Requested,
    Suspended,
}
impl IssuingTokenStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Requested => "requested",
            Suspended => "suspended",
        }
    }
}

impl std::str::FromStr for IssuingTokenStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "requested" => Ok(Requested),
            "suspended" => Ok(Suspended),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingTokenStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTokenStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingTokenStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingTokenStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingTokenStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTokenStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
