/// You can reverse some [ReceivedDebits](https://stripe.com/docs/api#received_debits) depending on their network and source flow.
/// Reversing a ReceivedDebit leads to the creation of a new object known as a DebitReversal.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryDebitReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The FinancialAccount to reverse funds from.
    pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryDebitReversalId,
    /// Other flows linked to a DebitReversal.
    pub linked_flows: Option<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: TreasuryDebitReversalNetwork,
    /// The ReceivedDebit being reversed.
    pub received_debit: String,
    /// Status of the DebitReversal
    pub status: TreasuryDebitReversalStatus,
    pub status_transitions: stripe_treasury::TreasuryReceivedDebitsResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryDebitReversalBuilder {
    amount: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    financial_account: Option<Option<String>>,
    hosted_regulatory_receipt_url: Option<Option<String>>,
    id: Option<stripe_treasury::TreasuryDebitReversalId>,
    linked_flows: Option<Option<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network: Option<TreasuryDebitReversalNetwork>,
    received_debit: Option<String>,
    status: Option<TreasuryDebitReversalStatus>,
    status_transitions: Option<stripe_treasury::TreasuryReceivedDebitsResourceStatusTransitions>,
    transaction: Option<Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryDebitReversal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryDebitReversal>,
        builder: TreasuryDebitReversalBuilder,
    }

    impl Visitor for Place<TreasuryDebitReversal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryDebitReversalBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryDebitReversalBuilder {
        type Out = TreasuryDebitReversal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "financial_account" => Ok(Deserialize::begin(&mut self.financial_account)),
                "hosted_regulatory_receipt_url" => Ok(Deserialize::begin(&mut self.hosted_regulatory_receipt_url)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "linked_flows" => Ok(Deserialize::begin(&mut self.linked_flows)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "network" => Ok(Deserialize::begin(&mut self.network)),
                "received_debit" => Ok(Deserialize::begin(&mut self.received_debit)),
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
                linked_flows: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                network: Deserialize::default(),
                received_debit: Deserialize::default(),
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
            let linked_flows = self.linked_flows.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let network = self.network.take()?;
            let received_debit = self.received_debit.take()?;
            let status = self.status.take()?;
            let status_transitions = self.status_transitions.take()?;
            let transaction = self.transaction.take()?;

            Some(Self::Out {
                amount,
                created,
                currency,
                financial_account,
                hosted_regulatory_receipt_url,
                id,
                linked_flows,
                livemode,
                metadata,
                network,
                received_debit,
                status,
                status_transitions,
                transaction,
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

    impl ObjectDeser for TreasuryDebitReversal {
        type Builder = TreasuryDebitReversalBuilder;
    }
};
/// The rails used to reverse the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryDebitReversalNetwork {
    Ach,
    Card,
}
impl TreasuryDebitReversalNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryDebitReversalNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
        }
    }
}

impl std::str::FromStr for TreasuryDebitReversalNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryDebitReversalNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryDebitReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryDebitReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryDebitReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryDebitReversalNetwork"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryDebitReversalNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryDebitReversalNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryDebitReversalNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Status of the DebitReversal
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryDebitReversalStatus {
    Failed,
    Processing,
    Succeeded,
}
impl TreasuryDebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryDebitReversalStatus::*;
        match self {
            Failed => "failed",
            Processing => "processing",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryDebitReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryDebitReversalStatus::*;
        match s {
            "failed" => Ok(Failed),
            "processing" => Ok(Processing),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryDebitReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryDebitReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryDebitReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryDebitReversalStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryDebitReversalStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryDebitReversalStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryDebitReversalStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TreasuryDebitReversal {
    type Id = stripe_treasury::TreasuryDebitReversalId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryDebitReversalId);
