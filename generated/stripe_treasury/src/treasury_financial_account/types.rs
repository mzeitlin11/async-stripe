/// Stripe Treasury provides users with a container for money called a FinancialAccount that is separate from their Payments balance.
/// FinancialAccounts serve as the source and destination of Treasury’s money movement APIs.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryFinancialAccount {
    /// The array of paths to active Features in the Features hash.
    pub active_features: Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>,
    pub balance: stripe_treasury::TreasuryFinancialAccountsResourceBalance,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub features: Option<stripe_treasury::TreasuryFinancialAccountFeatures>,
    /// The set of credentials that resolve to a FinancialAccount.
    pub financial_addresses: Vec<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddress>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryFinancialAccountId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The array of paths to pending Features in the Features hash.
    pub pending_features: Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    pub platform_restrictions: Option<stripe_treasury::TreasuryFinancialAccountsResourcePlatformRestrictions>,
    /// The array of paths to restricted Features in the Features hash.
    pub restricted_features: Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>,
    /// The enum specifying what state the account is in.
    pub status: TreasuryFinancialAccountStatus,
    pub status_details: stripe_treasury::TreasuryFinancialAccountsResourceStatusDetails,
    /// The currencies the FinancialAccount can hold a balance in.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    pub supported_currencies: Vec<String>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryFinancialAccountBuilder {
    active_features: Option<Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>>,
    balance: Option<stripe_treasury::TreasuryFinancialAccountsResourceBalance>,
    country: Option<String>,
    created: Option<stripe_types::Timestamp>,
    features: Option<Option<stripe_treasury::TreasuryFinancialAccountFeatures>>,
    financial_addresses: Option<Vec<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddress>>,
    id: Option<stripe_treasury::TreasuryFinancialAccountId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    pending_features: Option<Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>>,
    platform_restrictions: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourcePlatformRestrictions>>,
    restricted_features: Option<Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>>,
    status: Option<TreasuryFinancialAccountStatus>,
    status_details: Option<stripe_treasury::TreasuryFinancialAccountsResourceStatusDetails>,
    supported_currencies: Option<Vec<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccount>,
        builder: TreasuryFinancialAccountBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryFinancialAccountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountBuilder {
        type Out = TreasuryFinancialAccount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "active_features" => Ok(Deserialize::begin(&mut self.active_features)),
                "balance" => Ok(Deserialize::begin(&mut self.balance)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "features" => Ok(Deserialize::begin(&mut self.features)),
                "financial_addresses" => Ok(Deserialize::begin(&mut self.financial_addresses)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "pending_features" => Ok(Deserialize::begin(&mut self.pending_features)),
                "platform_restrictions" => Ok(Deserialize::begin(&mut self.platform_restrictions)),
                "restricted_features" => Ok(Deserialize::begin(&mut self.restricted_features)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "status_details" => Ok(Deserialize::begin(&mut self.status_details)),
                "supported_currencies" => Ok(Deserialize::begin(&mut self.supported_currencies)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                active_features: Deserialize::default(),
                balance: Deserialize::default(),
                country: Deserialize::default(),
                created: Deserialize::default(),
                features: Deserialize::default(),
                financial_addresses: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                pending_features: Deserialize::default(),
                platform_restrictions: Deserialize::default(),
                restricted_features: Deserialize::default(),
                status: Deserialize::default(),
                status_details: Deserialize::default(),
                supported_currencies: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let active_features = self.active_features.take()?;
            let balance = self.balance.take()?;
            let country = self.country.take()?;
            let created = self.created.take()?;
            let features = self.features.take()?;
            let financial_addresses = self.financial_addresses.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let pending_features = self.pending_features.take()?;
            let platform_restrictions = self.platform_restrictions.take()?;
            let restricted_features = self.restricted_features.take()?;
            let status = self.status.take()?;
            let status_details = self.status_details.take()?;
            let supported_currencies = self.supported_currencies.take()?;

            Some(Self::Out {
                active_features,
                balance,
                country,
                created,
                features,
                financial_addresses,
                id,
                livemode,
                metadata,
                pending_features,
                platform_restrictions,
                restricted_features,
                status,
                status_details,
                supported_currencies,
            })
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

    impl ObjectDeser for TreasuryFinancialAccount {
        type Builder = TreasuryFinancialAccountBuilder;
    }
};
/// The enum specifying what state the account is in.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountStatus {
    Closed,
    Open,
}
impl TreasuryFinancialAccountStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountStatus::*;
        match self {
            Closed => "closed",
            Open => "open",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountStatus::*;
        match s {
            "closed" => Ok(Closed),
            "open" => Ok(Open),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryFinancialAccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryFinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryFinancialAccountStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryFinancialAccountStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryFinancialAccountStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TreasuryFinancialAccount {
    type Id = stripe_treasury::TreasuryFinancialAccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryFinancialAccountId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountArray {
    CardIssuing,
    DepositInsurance,
    FinancialAddressesAba,
    InboundTransfersAch,
    IntraStripeFlows,
    OutboundPaymentsAch,
    OutboundPaymentsUsDomesticWire,
    OutboundTransfersAch,
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}
impl TreasuryFinancialAccountArray {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountArray::*;
        match self {
            CardIssuing => "card_issuing",
            DepositInsurance => "deposit_insurance",
            FinancialAddressesAba => "financial_addresses.aba",
            InboundTransfersAch => "inbound_transfers.ach",
            IntraStripeFlows => "intra_stripe_flows",
            OutboundPaymentsAch => "outbound_payments.ach",
            OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            OutboundTransfersAch => "outbound_transfers.ach",
            OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            RemoteDepositCapture => "remote_deposit_capture",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountArray {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountArray::*;
        match s {
            "card_issuing" => Ok(CardIssuing),
            "deposit_insurance" => Ok(DepositInsurance),
            "financial_addresses.aba" => Ok(FinancialAddressesAba),
            "inbound_transfers.ach" => Ok(InboundTransfersAch),
            "intra_stripe_flows" => Ok(IntraStripeFlows),
            "outbound_payments.ach" => Ok(OutboundPaymentsAch),
            "outbound_payments.us_domestic_wire" => Ok(OutboundPaymentsUsDomesticWire),
            "outbound_transfers.ach" => Ok(OutboundTransfersAch),
            "outbound_transfers.us_domestic_wire" => Ok(OutboundTransfersUsDomesticWire),
            "remote_deposit_capture" => Ok(RemoteDepositCapture),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryFinancialAccountArray {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryFinancialAccountArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountArray {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountArray"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryFinancialAccountArray {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryFinancialAccountArray> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryFinancialAccountArray::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
