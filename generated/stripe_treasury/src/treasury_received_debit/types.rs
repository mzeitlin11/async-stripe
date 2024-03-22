/// ReceivedDebits represent funds pulled from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts).
/// These are not initiated from the FinancialAccount.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryReceivedDebit {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: String,
    /// Reason for the failure.
    /// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
    pub failure_code: Option<TreasuryReceivedDebitFailureCode>,
    /// The FinancialAccount that funds were pulled from.
    pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryReceivedDebitId,
    pub initiating_payment_method_details: Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>,
    pub linked_flows: stripe_treasury::TreasuryReceivedDebitsResourceLinkedFlows,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The network used for the ReceivedDebit.
    pub network: TreasuryReceivedDebitNetwork,
    /// Details describing when a ReceivedDebit might be reversed.
    pub reversal_details: Option<stripe_treasury::TreasuryReceivedDebitsResourceReversalDetails>,
    /// Status of the ReceivedDebit.
    /// ReceivedDebits are created with a status of either `succeeded` (approved) or `failed` (declined).
    /// The failure reason can be found under the `failure_code`.
    pub status: stripe_treasury::TreasuryReceivedDebitStatus,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryReceivedDebitBuilder {
    amount: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<String>,
    failure_code: Option<Option<TreasuryReceivedDebitFailureCode>>,
    financial_account: Option<Option<String>>,
    hosted_regulatory_receipt_url: Option<Option<String>>,
    id: Option<stripe_treasury::TreasuryReceivedDebitId>,
    initiating_payment_method_details: Option<Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>>,
    linked_flows: Option<stripe_treasury::TreasuryReceivedDebitsResourceLinkedFlows>,
    livemode: Option<bool>,
    network: Option<TreasuryReceivedDebitNetwork>,
    reversal_details: Option<Option<stripe_treasury::TreasuryReceivedDebitsResourceReversalDetails>>,
    status: Option<stripe_treasury::TreasuryReceivedDebitStatus>,
    transaction: Option<Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryReceivedDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedDebit>,
        builder: TreasuryReceivedDebitBuilder,
    }

    impl Visitor for Place<TreasuryReceivedDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryReceivedDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryReceivedDebitBuilder {
        type Out = TreasuryReceivedDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "failure_code" => Ok(Deserialize::begin(&mut self.failure_code)),
                "financial_account" => Ok(Deserialize::begin(&mut self.financial_account)),
                "hosted_regulatory_receipt_url" => Ok(Deserialize::begin(&mut self.hosted_regulatory_receipt_url)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "initiating_payment_method_details" => Ok(Deserialize::begin(&mut self.initiating_payment_method_details)),
                "linked_flows" => Ok(Deserialize::begin(&mut self.linked_flows)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "network" => Ok(Deserialize::begin(&mut self.network)),
                "reversal_details" => Ok(Deserialize::begin(&mut self.reversal_details)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "transaction" => Ok(Deserialize::begin(&mut self.transaction)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                failure_code: Deserialize::default(),
                financial_account: Deserialize::default(),
                hosted_regulatory_receipt_url: Deserialize::default(),
                id: Deserialize::default(),
                initiating_payment_method_details: Deserialize::default(),
                linked_flows: Deserialize::default(),
                livemode: Deserialize::default(),
                network: Deserialize::default(),
                reversal_details: Deserialize::default(),
                status: Deserialize::default(),
                transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let description = self.description.take()?;
            let failure_code = self.failure_code.take()?;
            let financial_account = self.financial_account.take()?;
            let hosted_regulatory_receipt_url = self.hosted_regulatory_receipt_url.take()?;
            let id = self.id.take()?;
            let initiating_payment_method_details = self.initiating_payment_method_details.take()?;
            let linked_flows = self.linked_flows.take()?;
            let livemode = self.livemode.take()?;
            let network = self.network.take()?;
            let reversal_details = self.reversal_details.take()?;
            let status = self.status.take()?;
            let transaction = self.transaction.take()?;

            Some(Self::Out {
                amount,
                created,
                currency,
                description,
                failure_code,
                financial_account,
                hosted_regulatory_receipt_url,
                id,
                initiating_payment_method_details,
                linked_flows,
                livemode,
                network,
                reversal_details,
                status,
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

    impl ObjectDeser for TreasuryReceivedDebit {
        type Builder = TreasuryReceivedDebitBuilder;
    }
};
/// Reason for the failure.
/// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitFailureCode {
    AccountClosed,
    AccountFrozen,
    InsufficientFunds,
    Other,
}
impl TreasuryReceivedDebitFailureCode {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitFailureCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            InsufficientFunds => "insufficient_funds",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitFailureCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitFailureCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "insufficient_funds" => Ok(InsufficientFunds),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryReceivedDebitFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryReceivedDebitFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitFailureCode"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryReceivedDebitFailureCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryReceivedDebitFailureCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedDebitFailureCode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The network used for the ReceivedDebit.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitNetwork {
    Ach,
    Card,
    Stripe,
}
impl TreasuryReceivedDebitNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            "stripe" => Ok(Stripe),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryReceivedDebitNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitNetwork"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryReceivedDebitNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryReceivedDebitNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedDebitNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TreasuryReceivedDebit {
    type Id = stripe_treasury::TreasuryReceivedDebitId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryReceivedDebitId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitStatus {
    Failed,
    Succeeded,
}
impl TreasuryReceivedDebitStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitStatus::*;
        match self {
            Failed => "failed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitStatus::*;
        match s {
            "failed" => Ok(Failed),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryReceivedDebitStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryReceivedDebitStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryReceivedDebitStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedDebitStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
