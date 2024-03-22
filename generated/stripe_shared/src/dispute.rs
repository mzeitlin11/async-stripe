/// A dispute occurs when a customer questions your charge with their card issuer.
/// When this happens, you have the opportunity to respond to the dispute with
/// evidence that shows that the charge is legitimate.
///
/// Related guide: [Disputes and fraud](https://stripe.com/docs/disputes)
///
/// For more details see <<https://stripe.com/docs/api/disputes/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Dispute {
    /// Disputed amount.
    /// Usually the amount of the charge, but it can differ (usually because of currency fluctuation or because only part of the order is disputed).
    pub amount: i64,
    /// List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
    pub balance_transactions: Vec<stripe_shared::BalanceTransaction>,
    /// ID of the charge that's disputed.
    pub charge: stripe_types::Expandable<stripe_shared::Charge>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    pub evidence: stripe_shared::DisputeEvidence,
    pub evidence_details: stripe_shared::DisputeEvidenceDetails,
    /// Unique identifier for the object.
    pub id: stripe_shared::DisputeId,
    /// If true, it's still possible to refund the disputed payment.
    /// After the payment has been fully refunded, no further funds are withdrawn from your Stripe account as a result of this dispute.
    pub is_charge_refundable: bool,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Network-dependent reason code for the dispute.
    pub network_reason_code: Option<String>,
    /// ID of the PaymentIntent that's disputed.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    pub payment_method_details: Option<stripe_shared::DisputePaymentMethodDetails>,
    /// Reason given by cardholder for dispute.
    /// Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`.
    /// Learn more about [dispute reasons](https://stripe.com/docs/disputes/categories).
    pub reason: String,
    /// Current status of dispute.
    /// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `won`, or `lost`.
    pub status: DisputeStatus,
}
#[cfg(feature = "min-ser")]
pub struct DisputeBuilder {
    amount: Option<i64>,
    balance_transactions: Option<Vec<stripe_shared::BalanceTransaction>>,
    charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    evidence: Option<stripe_shared::DisputeEvidence>,
    evidence_details: Option<stripe_shared::DisputeEvidenceDetails>,
    id: Option<stripe_shared::DisputeId>,
    is_charge_refundable: Option<bool>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network_reason_code: Option<Option<String>>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    payment_method_details: Option<Option<stripe_shared::DisputePaymentMethodDetails>>,
    reason: Option<String>,
    status: Option<DisputeStatus>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Dispute {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Dispute>,
        builder: DisputeBuilder,
    }

    impl Visitor for Place<Dispute> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DisputeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DisputeBuilder {
        type Out = Dispute;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "balance_transactions" => Ok(Deserialize::begin(&mut self.balance_transactions)),
                "charge" => Ok(Deserialize::begin(&mut self.charge)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "evidence" => Ok(Deserialize::begin(&mut self.evidence)),
                "evidence_details" => Ok(Deserialize::begin(&mut self.evidence_details)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "is_charge_refundable" => Ok(Deserialize::begin(&mut self.is_charge_refundable)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "network_reason_code" => Ok(Deserialize::begin(&mut self.network_reason_code)),
                "payment_intent" => Ok(Deserialize::begin(&mut self.payment_intent)),
                "payment_method_details" => Ok(Deserialize::begin(&mut self.payment_method_details)),
                "reason" => Ok(Deserialize::begin(&mut self.reason)),
                "status" => Ok(Deserialize::begin(&mut self.status)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                balance_transactions: Deserialize::default(),
                charge: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                evidence: Deserialize::default(),
                evidence_details: Deserialize::default(),
                id: Deserialize::default(),
                is_charge_refundable: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                network_reason_code: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_method_details: Deserialize::default(),
                reason: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let balance_transactions = self.balance_transactions.take()?;
            let charge = self.charge.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let evidence = self.evidence.take()?;
            let evidence_details = self.evidence_details.take()?;
            let id = self.id.take()?;
            let is_charge_refundable = self.is_charge_refundable.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let network_reason_code = self.network_reason_code.take()?;
            let payment_intent = self.payment_intent.take()?;
            let payment_method_details = self.payment_method_details.take()?;
            let reason = self.reason.take()?;
            let status = self.status.take()?;

            Some(Self::Out {
                amount,
                balance_transactions,
                charge,
                created,
                currency,
                evidence,
                evidence_details,
                id,
                is_charge_refundable,
                livemode,
                metadata,
                network_reason_code,
                payment_intent,
                payment_method_details,
                reason,
                status,
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

    impl ObjectDeser for Dispute {
        type Builder = DisputeBuilder;
    }
};
/// Current status of dispute.
/// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `won`, or `lost`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DisputeStatus {
    Lost,
    NeedsResponse,
    UnderReview,
    WarningClosed,
    WarningNeedsResponse,
    WarningUnderReview,
    Won,
}
impl DisputeStatus {
    pub fn as_str(self) -> &'static str {
        use DisputeStatus::*;
        match self {
            Lost => "lost",
            NeedsResponse => "needs_response",
            UnderReview => "under_review",
            WarningClosed => "warning_closed",
            WarningNeedsResponse => "warning_needs_response",
            WarningUnderReview => "warning_under_review",
            Won => "won",
        }
    }
}

impl std::str::FromStr for DisputeStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeStatus::*;
        match s {
            "lost" => Ok(Lost),
            "needs_response" => Ok(NeedsResponse),
            "under_review" => Ok(UnderReview),
            "warning_closed" => Ok(WarningClosed),
            "warning_needs_response" => Ok(WarningNeedsResponse),
            "warning_under_review" => Ok(WarningUnderReview),
            "won" => Ok(Won),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for DisputeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for DisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DisputeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisputeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for DisputeStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DisputeStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<DisputeStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputeStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Dispute {
    type Id = stripe_shared::DisputeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(DisputeId, "dp_" | "du_");
