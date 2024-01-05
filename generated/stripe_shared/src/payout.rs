/// A `Payout` object is created when you receive funds from Stripe, or when you
/// initiate a payout to either a bank account or debit card of a [connected
/// Stripe account](/docs/connect/bank-debit-card-payouts). You can retrieve individual payouts,
/// and list all payouts. Payouts are made on [varying
/// schedules](/docs/connect/manage-payout-schedule), depending on your country and
/// industry.
///
/// Related guide: [Receiving payouts](https://stripe.com/docs/payouts)
///
/// For more details see <<https://stripe.com/docs/api/payouts/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Payout {
    /// The amount (in cents (or local equivalent)) that transfers to your bank account or debit card.
    pub amount: i64,
    /// Date that you can expect the payout to arrive in the bank.
    /// This factors in delays to account for weekends or bank holidays.
    pub arrival_date: stripe_types::Timestamp,
    /// Returns `true` if the payout is created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule) and `false` if it's [requested manually](https://stripe.com/docs/payouts#manual-payouts).
    pub automatic: bool,
    /// ID of the balance transaction that describes the impact of this payout on your account balance.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// ID of the bank account or card the payout is sent to.
    pub destination: Option<stripe_types::Expandable<stripe_shared::ExternalAccount>>,
    /// If the payout fails or cancels, this is the ID of the balance transaction that reverses the initial balance transaction and returns the funds from the failed payout back in your balance.
    pub failure_balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Error code that provides a reason for a payout failure, if available.
    /// View our [list of failure codes](https://stripe.com/docs/api#payout_failures).
    pub failure_code: Option<String>,
    /// Message that provides the reason for a payout failure, if available.
    pub failure_message: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PayoutId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The method used to send this payout, which can be `standard` or `instant`.
    /// `instant` is supported for payouts to debit cards and bank accounts in certain countries.
    /// Learn more about [bank support for Instant Payouts](https://stripe.com/docs/payouts/instant-payouts-banks).
    pub method: String,
    /// If the payout reverses another, this is the ID of the original payout.
    pub original_payout: Option<stripe_types::Expandable<stripe_shared::Payout>>,
    /// If `completed`, you can use the [Balance Transactions API](https://stripe.com/docs/api/balance_transactions/list#balance_transaction_list-payout) to list all balance transactions that are paid out in this payout.
    pub reconciliation_status: PayoutReconciliationStatus,
    /// If the payout reverses, this is the ID of the payout that reverses this payout.
    pub reversed_by: Option<stripe_types::Expandable<stripe_shared::Payout>>,
    /// The source balance this payout came from, which can be one of the following: `card`, `fpx`, or `bank_account`.
    pub source_type: String,
    /// Extra information about a payout that displays on the user's bank statement.
    pub statement_descriptor: Option<String>,
    /// Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`.
    /// A payout is `pending` until it's submitted to the bank, when it becomes `in_transit`.
    /// The status changes to `paid` if the transaction succeeds, or to `failed` or `canceled` (within 5 business days).
    /// Some payouts that fail might initially show as `paid`, then change to `failed`.
    pub status: String,
    /// Can be `bank_account` or `card`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PayoutType,
}
#[cfg(feature = "min-ser")]
pub struct PayoutBuilder {
    amount: Option<i64>,
    arrival_date: Option<stripe_types::Timestamp>,
    automatic: Option<bool>,
    balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    destination: Option<Option<stripe_types::Expandable<stripe_shared::ExternalAccount>>>,
    failure_balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    failure_code: Option<Option<String>>,
    failure_message: Option<Option<String>>,
    id: Option<stripe_shared::PayoutId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    method: Option<String>,
    original_payout: Option<Option<stripe_types::Expandable<stripe_shared::Payout>>>,
    reconciliation_status: Option<PayoutReconciliationStatus>,
    reversed_by: Option<Option<stripe_types::Expandable<stripe_shared::Payout>>>,
    source_type: Option<String>,
    statement_descriptor: Option<Option<String>>,
    status: Option<String>,
    type_: Option<PayoutType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Payout {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Payout>,
        builder: PayoutBuilder,
    }

    impl Visitor for Place<Payout> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PayoutBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PayoutBuilder {
        type Out = Payout;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "arrival_date" => Ok(Deserialize::begin(&mut self.arrival_date)),
                "automatic" => Ok(Deserialize::begin(&mut self.automatic)),
                "balance_transaction" => Ok(Deserialize::begin(&mut self.balance_transaction)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "destination" => Ok(Deserialize::begin(&mut self.destination)),
                "failure_balance_transaction" => Ok(Deserialize::begin(&mut self.failure_balance_transaction)),
                "failure_code" => Ok(Deserialize::begin(&mut self.failure_code)),
                "failure_message" => Ok(Deserialize::begin(&mut self.failure_message)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "method" => Ok(Deserialize::begin(&mut self.method)),
                "original_payout" => Ok(Deserialize::begin(&mut self.original_payout)),
                "reconciliation_status" => Ok(Deserialize::begin(&mut self.reconciliation_status)),
                "reversed_by" => Ok(Deserialize::begin(&mut self.reversed_by)),
                "source_type" => Ok(Deserialize::begin(&mut self.source_type)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                arrival_date: Deserialize::default(),
                automatic: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                destination: Deserialize::default(),
                failure_balance_transaction: Deserialize::default(),
                failure_code: Deserialize::default(),
                failure_message: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                method: Deserialize::default(),
                original_payout: Deserialize::default(),
                reconciliation_status: Deserialize::default(),
                reversed_by: Deserialize::default(),
                source_type: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let arrival_date = self.arrival_date.take()?;
            let automatic = self.automatic.take()?;
            let balance_transaction = self.balance_transaction.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let description = self.description.take()?;
            let destination = self.destination.take()?;
            let failure_balance_transaction = self.failure_balance_transaction.take()?;
            let failure_code = self.failure_code.take()?;
            let failure_message = self.failure_message.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let method = self.method.take()?;
            let original_payout = self.original_payout.take()?;
            let reconciliation_status = self.reconciliation_status.take()?;
            let reversed_by = self.reversed_by.take()?;
            let source_type = self.source_type.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;
            let status = self.status.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out {
                amount,
                arrival_date,
                automatic,
                balance_transaction,
                created,
                currency,
                description,
                destination,
                failure_balance_transaction,
                failure_code,
                failure_message,
                id,
                livemode,
                metadata,
                method,
                original_payout,
                reconciliation_status,
                reversed_by,
                source_type,
                statement_descriptor,
                status,
                type_,
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

    impl ObjectDeser for Payout {
        type Builder = PayoutBuilder;
    }
};
/// If `completed`, you can use the [Balance Transactions API](https://stripe.com/docs/api/balance_transactions/list#balance_transaction_list-payout) to list all balance transactions that are paid out in this payout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PayoutReconciliationStatus {
    Completed,
    InProgress,
    NotApplicable,
}
impl PayoutReconciliationStatus {
    pub fn as_str(self) -> &'static str {
        use PayoutReconciliationStatus::*;
        match self {
            Completed => "completed",
            InProgress => "in_progress",
            NotApplicable => "not_applicable",
        }
    }
}

impl std::str::FromStr for PayoutReconciliationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PayoutReconciliationStatus::*;
        match s {
            "completed" => Ok(Completed),
            "in_progress" => Ok(InProgress),
            "not_applicable" => Ok(NotApplicable),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PayoutReconciliationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PayoutReconciliationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PayoutReconciliationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PayoutReconciliationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PayoutReconciliationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PayoutReconciliationStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PayoutReconciliationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PayoutReconciliationStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PayoutReconciliationStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Can be `bank_account` or `card`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PayoutType {
    BankAccount,
    Card,
}
impl PayoutType {
    pub fn as_str(self) -> &'static str {
        use PayoutType::*;
        match self {
            BankAccount => "bank_account",
            Card => "card",
        }
    }
}

impl std::str::FromStr for PayoutType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PayoutType::*;
        match s {
            "bank_account" => Ok(BankAccount),
            "card" => Ok(Card),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PayoutType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PayoutType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PayoutType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PayoutType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PayoutType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PayoutType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PayoutType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Payout {
    type Id = stripe_shared::PayoutId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PayoutId, "po_");
