/// Use OutboundTransfers to transfer funds from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) to a PaymentMethod belonging to the same entity.
/// To send funds to a different party, use [OutboundPayments](https://stripe.com/docs/api#outbound_payments) instead.
/// You can send funds over ACH rails or through a domestic wire transfer to a user's own external bank account.
///
/// Simulate OutboundTransfer state changes with the `/v1/test_helpers/treasury/outbound_transfers` endpoints.
/// These methods can only be called on test mode objects.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryOutboundTransfer {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Returns `true` if the object can be canceled, and `false` otherwise.
    pub cancelable: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// The PaymentMethod used as the payment instrument for an OutboundTransfer.
    pub destination_payment_method: Option<String>,
    pub destination_payment_method_details: stripe_treasury::OutboundTransfersPaymentMethodDetails,
    /// The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: stripe_types::Timestamp,
    /// The FinancialAccount that funds were pulled from.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryOutboundTransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about a returned OutboundTransfer. Only set when the status is `returned`.
    pub returned_details: Option<stripe_treasury::TreasuryOutboundTransfersResourceReturnedDetails>,
    /// Information about the OutboundTransfer to be sent to the recipient account.
    pub statement_descriptor: String,
    /// Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`.
    /// An OutboundTransfer is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
    pub status: stripe_treasury::TreasuryOutboundTransferStatus,
    pub status_transitions: stripe_treasury::TreasuryOutboundTransfersResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransaction>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryOutboundTransferBuilder {
    amount: Option<i64>,
    cancelable: Option<bool>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    destination_payment_method: Option<Option<String>>,
    destination_payment_method_details: Option<stripe_treasury::OutboundTransfersPaymentMethodDetails>,
    expected_arrival_date: Option<stripe_types::Timestamp>,
    financial_account: Option<String>,
    hosted_regulatory_receipt_url: Option<Option<String>>,
    id: Option<stripe_treasury::TreasuryOutboundTransferId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    returned_details: Option<Option<stripe_treasury::TreasuryOutboundTransfersResourceReturnedDetails>>,
    statement_descriptor: Option<String>,
    status: Option<stripe_treasury::TreasuryOutboundTransferStatus>,
    status_transitions: Option<stripe_treasury::TreasuryOutboundTransfersResourceStatusTransitions>,
    transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryOutboundTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundTransfer>,
        builder: TreasuryOutboundTransferBuilder,
    }

    impl Visitor for Place<TreasuryOutboundTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryOutboundTransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryOutboundTransferBuilder {
        type Out = TreasuryOutboundTransfer;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "cancelable" => Ok(Deserialize::begin(&mut self.cancelable)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "destination_payment_method" => Ok(Deserialize::begin(&mut self.destination_payment_method)),
                "destination_payment_method_details" => Ok(Deserialize::begin(&mut self.destination_payment_method_details)),
                "expected_arrival_date" => Ok(Deserialize::begin(&mut self.expected_arrival_date)),
                "financial_account" => Ok(Deserialize::begin(&mut self.financial_account)),
                "hosted_regulatory_receipt_url" => Ok(Deserialize::begin(&mut self.hosted_regulatory_receipt_url)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "returned_details" => Ok(Deserialize::begin(&mut self.returned_details)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "status_transitions" => Ok(Deserialize::begin(&mut self.status_transitions)),
                "transaction" => Ok(Deserialize::begin(&mut self.transaction)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                cancelable: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                destination_payment_method: Deserialize::default(),
                destination_payment_method_details: Deserialize::default(),
                expected_arrival_date: Deserialize::default(),
                financial_account: Deserialize::default(),
                hosted_regulatory_receipt_url: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                returned_details: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
                transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let cancelable = self.cancelable.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let description = self.description.take()?;
            let destination_payment_method = self.destination_payment_method.take()?;
            let destination_payment_method_details = self.destination_payment_method_details.take()?;
            let expected_arrival_date = self.expected_arrival_date.take()?;
            let financial_account = self.financial_account.take()?;
            let hosted_regulatory_receipt_url = self.hosted_regulatory_receipt_url.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let returned_details = self.returned_details.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;
            let status = self.status.take()?;
            let status_transitions = self.status_transitions.take()?;
            let transaction = self.transaction.take()?;

            Some(Self::Out {
                amount,
                cancelable,
                created,
                currency,
                description,
                destination_payment_method,
                destination_payment_method_details,
                expected_arrival_date,
                financial_account,
                hosted_regulatory_receipt_url,
                id,
                livemode,
                metadata,
                returned_details,
                statement_descriptor,
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

    impl ObjectDeser for TreasuryOutboundTransfer {
        type Builder = TreasuryOutboundTransferBuilder;
    }
};
impl stripe_types::Object for TreasuryOutboundTransfer {
    type Id = stripe_treasury::TreasuryOutboundTransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryOutboundTransferId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryOutboundTransferStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}
impl TreasuryOutboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryOutboundTransferStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Posted => "posted",
            Processing => "processing",
            Returned => "returned",
        }
    }
}

impl std::str::FromStr for TreasuryOutboundTransferStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundTransferStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "posted" => Ok(Posted),
            "processing" => Ok(Processing),
            "returned" => Ok(Returned),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryOutboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryOutboundTransferStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryOutboundTransferStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryOutboundTransferStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryOutboundTransferStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryOutboundTransferStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryOutboundTransferStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
