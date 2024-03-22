/// You can reverse some [ReceivedCredits](https://stripe.com/docs/api#received_credits) depending on their network and source flow.
/// Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryCreditReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The FinancialAccount to reverse funds from.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryCreditReversalId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: TreasuryCreditReversalNetwork,
    /// The ReceivedCredit being reversed.
    pub received_credit: String,
    /// Status of the CreditReversal
    pub status: stripe_treasury::TreasuryCreditReversalStatus,
    pub status_transitions: stripe_treasury::TreasuryReceivedCreditsResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryCreditReversalBuilder {
    amount: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    financial_account: Option<String>,
    hosted_regulatory_receipt_url: Option<Option<String>>,
    id: Option<stripe_treasury::TreasuryCreditReversalId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network: Option<TreasuryCreditReversalNetwork>,
    received_credit: Option<String>,
    status: Option<stripe_treasury::TreasuryCreditReversalStatus>,
    status_transitions: Option<stripe_treasury::TreasuryReceivedCreditsResourceStatusTransitions>,
    transaction: Option<Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryCreditReversal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryCreditReversal>,
        builder: TreasuryCreditReversalBuilder,
    }

    impl Visitor for Place<TreasuryCreditReversal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryCreditReversalBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryCreditReversalBuilder {
        type Out = TreasuryCreditReversal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "financial_account" => Ok(Deserialize::begin(&mut self.financial_account)),
                "hosted_regulatory_receipt_url" => Ok(Deserialize::begin(&mut self.hosted_regulatory_receipt_url)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "network" => Ok(Deserialize::begin(&mut self.network)),
                "received_credit" => Ok(Deserialize::begin(&mut self.received_credit)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "status_transitions" => Ok(Deserialize::begin(&mut self.status_transitions)),
                "transaction" => Ok(Deserialize::begin(&mut self.transaction)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                financial_account: Deserialize::default(),
                hosted_regulatory_receipt_url: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                network: Deserialize::default(),
                received_credit: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
                transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let financial_account = self.financial_account.take()?;
            let hosted_regulatory_receipt_url = self.hosted_regulatory_receipt_url.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let network = self.network.take()?;
            let received_credit = self.received_credit.take()?;
            let status = self.status.take()?;
            let status_transitions = self.status_transitions.take()?;
            let transaction = self.transaction.take()?;

            Some(Self::Out { amount, created, currency, financial_account, hosted_regulatory_receipt_url, id, livemode, metadata, network, received_credit, status, status_transitions, transaction })
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

    impl ObjectDeser for TreasuryCreditReversal {
        type Builder = TreasuryCreditReversalBuilder;
    }
};
/// The rails used to reverse the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryCreditReversalNetwork {
    Ach,
    Stripe,
}
impl TreasuryCreditReversalNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryCreditReversalNetwork::*;
        match self {
            Ach => "ach",
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for TreasuryCreditReversalNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryCreditReversalNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "stripe" => Ok(Stripe),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryCreditReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryCreditReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryCreditReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryCreditReversalNetwork"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryCreditReversalNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryCreditReversalNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryCreditReversalNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TreasuryCreditReversal {
    type Id = stripe_treasury::TreasuryCreditReversalId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryCreditReversalId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryCreditReversalStatus {
    Canceled,
    Posted,
    Processing,
}
impl TreasuryCreditReversalStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryCreditReversalStatus::*;
        match self {
            Canceled => "canceled",
            Posted => "posted",
            Processing => "processing",
        }
    }
}

impl std::str::FromStr for TreasuryCreditReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryCreditReversalStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "posted" => Ok(Posted),
            "processing" => Ok(Processing),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryCreditReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryCreditReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryCreditReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryCreditReversalStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryCreditReversalStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryCreditReversalStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryCreditReversalStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
