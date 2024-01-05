/// ReceivedCredits represent funds sent to a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) (for example, via ACH or wire).
/// These money movements are not initiated from the FinancialAccount.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryReceivedCredit {
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
    /// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
    pub failure_code: Option<TreasuryReceivedCreditFailureCode>,
    /// The FinancialAccount that received the funds.
    pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryReceivedCreditId,
    pub initiating_payment_method_details: stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    pub linked_flows: stripe_treasury::TreasuryReceivedCreditsResourceLinkedFlows,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The rails used to send the funds.
    pub network: TreasuryReceivedCreditNetwork,
    /// Details describing when a ReceivedCredit may be reversed.
    pub reversal_details: Option<stripe_treasury::TreasuryReceivedCreditsResourceReversalDetails>,
    /// Status of the ReceivedCredit.
    /// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
    /// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
    pub status: stripe_treasury::TreasuryReceivedCreditStatus,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryReceivedCreditBuilder {
    amount: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<String>,
    failure_code: Option<Option<TreasuryReceivedCreditFailureCode>>,
    financial_account: Option<Option<String>>,
    hosted_regulatory_receipt_url: Option<Option<String>>,
    id: Option<stripe_treasury::TreasuryReceivedCreditId>,
    initiating_payment_method_details: Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>,
    linked_flows: Option<stripe_treasury::TreasuryReceivedCreditsResourceLinkedFlows>,
    livemode: Option<bool>,
    network: Option<TreasuryReceivedCreditNetwork>,
    reversal_details: Option<Option<stripe_treasury::TreasuryReceivedCreditsResourceReversalDetails>>,
    status: Option<stripe_treasury::TreasuryReceivedCreditStatus>,
    transaction: Option<Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryReceivedCredit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCredit>,
        builder: TreasuryReceivedCreditBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCredit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryReceivedCreditBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryReceivedCreditBuilder {
        type Out = TreasuryReceivedCredit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
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

    impl ObjectDeser for TreasuryReceivedCredit {
        type Builder = TreasuryReceivedCreditBuilder;
    }
};
/// Reason for the failure.
/// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    Other,
}
impl TreasuryReceivedCreditFailureCode {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditFailureCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditFailureCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditFailureCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryReceivedCreditFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditFailureCode"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryReceivedCreditFailureCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryReceivedCreditFailureCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedCreditFailureCode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The rails used to send the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditNetwork {
    Ach,
    Card,
    Stripe,
    UsDomesticWire,
}
impl TreasuryReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
            Stripe => "stripe",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            "stripe" => Ok(Stripe),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryReceivedCreditNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditNetwork"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryReceivedCreditNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryReceivedCreditNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedCreditNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TreasuryReceivedCredit {
    type Id = stripe_treasury::TreasuryReceivedCreditId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryReceivedCreditId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditStatus {
    Failed,
    Succeeded,
}
impl TreasuryReceivedCreditStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditStatus::*;
        match self {
            Failed => "failed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditStatus::*;
        match s {
            "failed" => Ok(Failed),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryReceivedCreditStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryReceivedCreditStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryReceivedCreditStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedCreditStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
