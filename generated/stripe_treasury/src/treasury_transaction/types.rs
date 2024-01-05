/// Transactions represent changes to a [FinancialAccount's](https://stripe.com/docs/api#financial_accounts) balance.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryTransaction {
    /// Amount (in cents) transferred.
    pub amount: i64,
    pub balance_impact: stripe_treasury::TreasuryTransactionsResourceBalanceImpact,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: String,
    /// A list of TransactionEntries that are part of this Transaction.
    /// This cannot be expanded in any list endpoints.
    pub entries: Option<stripe_types::List<stripe_treasury::TreasuryTransactionEntry>>,
    /// The FinancialAccount associated with this object.
    pub financial_account: String,
    /// ID of the flow that created the Transaction.
    pub flow: Option<String>,
    /// Details of the flow that created the Transaction.
    pub flow_details: Option<stripe_treasury::TreasuryTransactionsResourceFlowDetails>,
    /// Type of the flow that created the Transaction.
    pub flow_type: TreasuryTransactionFlowType,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Status of the Transaction.
    pub status: stripe_treasury::TreasuryTransactionStatus,
    pub status_transitions: stripe_treasury::TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryTransactionBuilder {
    amount: Option<i64>,
    balance_impact: Option<stripe_treasury::TreasuryTransactionsResourceBalanceImpact>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<String>,
    entries: Option<Option<stripe_types::List<stripe_treasury::TreasuryTransactionEntry>>>,
    financial_account: Option<String>,
    flow: Option<Option<String>>,
    flow_details: Option<Option<stripe_treasury::TreasuryTransactionsResourceFlowDetails>>,
    flow_type: Option<TreasuryTransactionFlowType>,
    id: Option<stripe_treasury::TreasuryTransactionId>,
    livemode: Option<bool>,
    status: Option<stripe_treasury::TreasuryTransactionStatus>,
    status_transitions: Option<stripe_treasury::TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryTransaction>,
        builder: TreasuryTransactionBuilder,
    }

    impl Visitor for Place<TreasuryTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryTransactionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryTransactionBuilder {
        type Out = TreasuryTransaction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "balance_impact" => Ok(Deserialize::begin(&mut self.balance_impact)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "entries" => Ok(Deserialize::begin(&mut self.entries)),
                "financial_account" => Ok(Deserialize::begin(&mut self.financial_account)),
                "flow" => Ok(Deserialize::begin(&mut self.flow)),
                "flow_details" => Ok(Deserialize::begin(&mut self.flow_details)),
                "flow_type" => Ok(Deserialize::begin(&mut self.flow_type)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "status_transitions" => Ok(Deserialize::begin(&mut self.status_transitions)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                balance_impact: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                entries: Deserialize::default(),
                financial_account: Deserialize::default(),
                flow: Deserialize::default(),
                flow_details: Deserialize::default(),
                flow_type: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let balance_impact = self.balance_impact.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let description = self.description.take()?;
            let entries = self.entries.take()?;
            let financial_account = self.financial_account.take()?;
            let flow = self.flow.take()?;
            let flow_details = self.flow_details.take()?;
            let flow_type = self.flow_type.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let status = self.status.take()?;
            let status_transitions = self.status_transitions.take()?;

            Some(Self::Out { amount, balance_impact, created, currency, description, entries, financial_account, flow, flow_details, flow_type, id, livemode, status, status_transitions })
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

    impl ObjectDeser for TreasuryTransaction {
        type Builder = TreasuryTransactionBuilder;
    }
};
/// Type of the flow that created the Transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryTransactionFlowType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}
impl TreasuryTransactionFlowType {
    pub fn as_str(self) -> &'static str {
        use TreasuryTransactionFlowType::*;
        match self {
            CreditReversal => "credit_reversal",
            DebitReversal => "debit_reversal",
            InboundTransfer => "inbound_transfer",
            IssuingAuthorization => "issuing_authorization",
            Other => "other",
            OutboundPayment => "outbound_payment",
            OutboundTransfer => "outbound_transfer",
            ReceivedCredit => "received_credit",
            ReceivedDebit => "received_debit",
        }
    }
}

impl std::str::FromStr for TreasuryTransactionFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionFlowType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "debit_reversal" => Ok(DebitReversal),
            "inbound_transfer" => Ok(InboundTransfer),
            "issuing_authorization" => Ok(IssuingAuthorization),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "outbound_transfer" => Ok(OutboundTransfer),
            "received_credit" => Ok(ReceivedCredit),
            "received_debit" => Ok(ReceivedDebit),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryTransactionFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryTransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryTransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryTransactionFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryTransactionFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryTransactionFlowType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryTransactionFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryTransactionFlowType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryTransactionFlowType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TreasuryTransaction {
    type Id = stripe_treasury::TreasuryTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryTransactionId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryTransactionStatus {
    Open,
    Posted,
    Void,
}
impl TreasuryTransactionStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryTransactionStatus::*;
        match self {
            Open => "open",
            Posted => "posted",
            Void => "void",
        }
    }
}

impl std::str::FromStr for TreasuryTransactionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionStatus::*;
        match s {
            "open" => Ok(Open),
            "posted" => Ok(Posted),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryTransactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryTransactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryTransactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryTransactionStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryTransactionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryTransactionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryTransactionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
