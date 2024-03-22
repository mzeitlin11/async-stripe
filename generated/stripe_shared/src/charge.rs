/// The `Charge` object represents a single attempt to move money into your Stripe account.
/// PaymentIntent confirmation is the most common way to create Charges, but transferring
/// money to a different Stripe account through Connect also creates Charges.
/// Some legacy payment flows create Charges directly, which is not recommended for new integrations.
///
/// For more details see <<https://stripe.com/docs/api/charges/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Charge {
    /// Amount intended to be collected by this payment.
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    /// Amount in cents (or local equivalent) captured (can be less than the amount attribute on the charge if a partial capture was made).
    pub amount_captured: i64,
    /// Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the charge if a partial refund was issued).
    pub amount_refunded: i64,
    /// ID of the Connect application that created the charge.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The application fee (if any) for the charge.
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee: Option<stripe_types::Expandable<stripe_shared::ApplicationFee>>,
    /// The amount of the application fee (if any) requested for the charge.
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee_amount: Option<i64>,
    /// Authorization code on the charge.
    pub authorization_code: Option<String>,
    /// ID of the balance transaction that describes the impact of this charge on your account balance (not including refunds or disputes).
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    pub billing_details: stripe_shared::BillingDetails,
    /// The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card and bank statements.
    /// Allows you to see what the statement descriptor looks like after the static and dynamic portions are combined.
    pub calculated_statement_descriptor: Option<String>,
    /// If the charge was created without capturing, this Boolean represents whether it is still uncaptured or has since been captured.
    pub captured: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the customer this charge is for if one exists.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Whether the charge has been disputed.
    pub disputed: bool,
    /// ID of the balance transaction that describes the reversal of the balance on your account due to payment failure.
    pub failure_balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/error-codes) for a list of codes).
    pub failure_code: Option<String>,
    /// Message to user further explaining reason for charge failure if available.
    pub failure_message: Option<String>,
    /// Information on fraud assessments for the charge.
    pub fraud_details: Option<stripe_shared::ChargeFraudDetails>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ChargeId,
    /// ID of the invoice this charge is for if one exists.
    pub invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    pub level3: Option<stripe_shared::Level3>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The account (if any) the charge was made on behalf of without triggering an automatic transfer.
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// Details about whether the payment was accepted, and why.
    /// See [understanding declines](https://stripe.com/docs/declines) for details.
    pub outcome: Option<stripe_shared::ChargeOutcome>,
    /// `true` if the charge succeeded, or was successfully authorized for later capture.
    pub paid: bool,
    /// ID of the PaymentIntent associated with this charge, if one exists.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    /// ID of the payment method used in this charge.
    pub payment_method: Option<String>,
    /// Details about the payment method at the time of the transaction.
    pub payment_method_details: Option<stripe_shared::PaymentMethodDetails>,
    pub radar_options: Option<stripe_shared::RadarRadarOptions>,
    /// This is the email address that the receipt for this charge was sent to.
    pub receipt_email: Option<String>,
    /// This is the transaction number that appears on email receipts sent for this charge.
    /// This attribute will be `null` until a receipt has been sent.
    pub receipt_number: Option<String>,
    /// This is the URL to view the receipt for this charge.
    /// The receipt is kept up-to-date to the latest state of the charge, including any refunds.
    /// If the charge is for an Invoice, the receipt will be stylized as an Invoice receipt.
    pub receipt_url: Option<String>,
    /// Whether the charge has been fully refunded.
    /// If the charge is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    /// A list of refunds that have been applied to the charge.
    pub refunds: Option<stripe_types::List<stripe_shared::Refund>>,
    /// ID of the review associated with this charge if one exists.
    pub review: Option<stripe_types::Expandable<stripe_shared::Review>>,
    /// Shipping information for the charge.
    pub shipping: Option<stripe_shared::Shipping>,
    /// This is a legacy field that will be removed in the future.
    /// It contains the Source, Card, or BankAccount object used for the charge.
    /// For details about the payment method used for this charge, refer to `payment_method` or `payment_method_details` instead.
    pub source: Option<stripe_shared::PaymentSource>,
    /// The transfer ID which created this charge.
    /// Only present if the charge came from another Stripe account.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub source_transfer: Option<stripe_types::Expandable<stripe_shared::Transfer>>,
    /// For card charges, use `statement_descriptor_suffix` instead.
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,
    /// Provides information about the charge that customers see on their statements.
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,
    /// The status of the payment is either `succeeded`, `pending`, or `failed`.
    pub status: ChargeStatus,
    /// ID of the transfer to the `destination` account (only applicable if the charge was created using the `destination` parameter).
    pub transfer: Option<stripe_types::Expandable<stripe_shared::Transfer>>,
    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub transfer_data: Option<stripe_shared::ChargeTransferData>,
    /// A string that identifies this transaction as part of a group.
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct ChargeBuilder {
    amount: Option<i64>,
    amount_captured: Option<i64>,
    amount_refunded: Option<i64>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    application_fee: Option<Option<stripe_types::Expandable<stripe_shared::ApplicationFee>>>,
    application_fee_amount: Option<Option<i64>>,
    authorization_code: Option<Option<String>>,
    balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    billing_details: Option<stripe_shared::BillingDetails>,
    calculated_statement_descriptor: Option<Option<String>>,
    captured: Option<bool>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    description: Option<Option<String>>,
    disputed: Option<bool>,
    failure_balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    failure_code: Option<Option<String>>,
    failure_message: Option<Option<String>>,
    fraud_details: Option<Option<stripe_shared::ChargeFraudDetails>>,
    id: Option<stripe_shared::ChargeId>,
    invoice: Option<Option<stripe_types::Expandable<stripe_shared::Invoice>>>,
    level3: Option<Option<stripe_shared::Level3>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    outcome: Option<Option<stripe_shared::ChargeOutcome>>,
    paid: Option<bool>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    payment_method: Option<Option<String>>,
    payment_method_details: Option<Option<stripe_shared::PaymentMethodDetails>>,
    radar_options: Option<Option<stripe_shared::RadarRadarOptions>>,
    receipt_email: Option<Option<String>>,
    receipt_number: Option<Option<String>>,
    receipt_url: Option<Option<String>>,
    refunded: Option<bool>,
    refunds: Option<Option<stripe_types::List<stripe_shared::Refund>>>,
    review: Option<Option<stripe_types::Expandable<stripe_shared::Review>>>,
    shipping: Option<Option<stripe_shared::Shipping>>,
    source: Option<Option<stripe_shared::PaymentSource>>,
    source_transfer: Option<Option<stripe_types::Expandable<stripe_shared::Transfer>>>,
    statement_descriptor: Option<Option<String>>,
    statement_descriptor_suffix: Option<Option<String>>,
    status: Option<ChargeStatus>,
    transfer: Option<Option<stripe_types::Expandable<stripe_shared::Transfer>>>,
    transfer_data: Option<Option<stripe_shared::ChargeTransferData>>,
    transfer_group: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Charge {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Charge>,
        builder: ChargeBuilder,
    }

    impl Visitor for Place<Charge> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ChargeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ChargeBuilder {
        type Out = Charge;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_captured" => Ok(Deserialize::begin(&mut self.amount_captured)),
                "amount_refunded" => Ok(Deserialize::begin(&mut self.amount_refunded)),
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "application_fee" => Ok(Deserialize::begin(&mut self.application_fee)),
                "application_fee_amount" => Ok(Deserialize::begin(&mut self.application_fee_amount)),
                "authorization_code" => Ok(Deserialize::begin(&mut self.authorization_code)),
                "balance_transaction" => Ok(Deserialize::begin(&mut self.balance_transaction)),
                "billing_details" => Ok(Deserialize::begin(&mut self.billing_details)),
                "calculated_statement_descriptor" => Ok(Deserialize::begin(&mut self.calculated_statement_descriptor)),
                "captured" => Ok(Deserialize::begin(&mut self.captured)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "disputed" => Ok(Deserialize::begin(&mut self.disputed)),
                "failure_balance_transaction" => Ok(Deserialize::begin(&mut self.failure_balance_transaction)),
                "failure_code" => Ok(Deserialize::begin(&mut self.failure_code)),
                "failure_message" => Ok(Deserialize::begin(&mut self.failure_message)),
                "fraud_details" => Ok(Deserialize::begin(&mut self.fraud_details)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "invoice" => Ok(Deserialize::begin(&mut self.invoice)),
                "level3" => Ok(Deserialize::begin(&mut self.level3)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "on_behalf_of" => Ok(Deserialize::begin(&mut self.on_behalf_of)),
                "outcome" => Ok(Deserialize::begin(&mut self.outcome)),
                "paid" => Ok(Deserialize::begin(&mut self.paid)),
                "payment_intent" => Ok(Deserialize::begin(&mut self.payment_intent)),
                "payment_method" => Ok(Deserialize::begin(&mut self.payment_method)),
                "payment_method_details" => Ok(Deserialize::begin(&mut self.payment_method_details)),
                "radar_options" => Ok(Deserialize::begin(&mut self.radar_options)),
                "receipt_email" => Ok(Deserialize::begin(&mut self.receipt_email)),
                "receipt_number" => Ok(Deserialize::begin(&mut self.receipt_number)),
                "receipt_url" => Ok(Deserialize::begin(&mut self.receipt_url)),
                "refunded" => Ok(Deserialize::begin(&mut self.refunded)),
                "refunds" => Ok(Deserialize::begin(&mut self.refunds)),
                "review" => Ok(Deserialize::begin(&mut self.review)),
                "shipping" => Ok(Deserialize::begin(&mut self.shipping)),
                "source" => Ok(Deserialize::begin(&mut self.source)),
                "source_transfer" => Ok(Deserialize::begin(&mut self.source_transfer)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),
                "statement_descriptor_suffix" => Ok(Deserialize::begin(&mut self.statement_descriptor_suffix)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "transfer" => Ok(Deserialize::begin(&mut self.transfer)),
                "transfer_data" => Ok(Deserialize::begin(&mut self.transfer_data)),
                "transfer_group" => Ok(Deserialize::begin(&mut self.transfer_group)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_captured: Deserialize::default(),
                amount_refunded: Deserialize::default(),
                application: Deserialize::default(),
                application_fee: Deserialize::default(),
                application_fee_amount: Deserialize::default(),
                authorization_code: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                billing_details: Deserialize::default(),
                calculated_statement_descriptor: Deserialize::default(),
                captured: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                description: Deserialize::default(),
                disputed: Deserialize::default(),
                failure_balance_transaction: Deserialize::default(),
                failure_code: Deserialize::default(),
                failure_message: Deserialize::default(),
                fraud_details: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                level3: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                outcome: Deserialize::default(),
                paid: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_method: Deserialize::default(),
                payment_method_details: Deserialize::default(),
                radar_options: Deserialize::default(),
                receipt_email: Deserialize::default(),
                receipt_number: Deserialize::default(),
                receipt_url: Deserialize::default(),
                refunded: Deserialize::default(),
                refunds: Deserialize::default(),
                review: Deserialize::default(),
                shipping: Deserialize::default(),
                source: Deserialize::default(),
                source_transfer: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                statement_descriptor_suffix: Deserialize::default(),
                status: Deserialize::default(),
                transfer: Deserialize::default(),
                transfer_data: Deserialize::default(),
                transfer_group: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_captured = self.amount_captured.take()?;
            let amount_refunded = self.amount_refunded.take()?;
            let application = self.application.take()?;
            let application_fee = self.application_fee.take()?;
            let application_fee_amount = self.application_fee_amount.take()?;
            let authorization_code = self.authorization_code.take()?;
            let balance_transaction = self.balance_transaction.take()?;
            let billing_details = self.billing_details.take()?;
            let calculated_statement_descriptor = self.calculated_statement_descriptor.take()?;
            let captured = self.captured.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let customer = self.customer.take()?;
            let description = self.description.take()?;
            let disputed = self.disputed.take()?;
            let failure_balance_transaction = self.failure_balance_transaction.take()?;
            let failure_code = self.failure_code.take()?;
            let failure_message = self.failure_message.take()?;
            let fraud_details = self.fraud_details.take()?;
            let id = self.id.take()?;
            let invoice = self.invoice.take()?;
            let level3 = self.level3.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let on_behalf_of = self.on_behalf_of.take()?;
            let outcome = self.outcome.take()?;
            let paid = self.paid.take()?;
            let payment_intent = self.payment_intent.take()?;
            let payment_method = self.payment_method.take()?;
            let payment_method_details = self.payment_method_details.take()?;
            let radar_options = self.radar_options.take()?;
            let receipt_email = self.receipt_email.take()?;
            let receipt_number = self.receipt_number.take()?;
            let receipt_url = self.receipt_url.take()?;
            let refunded = self.refunded.take()?;
            let refunds = self.refunds.take()?;
            let review = self.review.take()?;
            let shipping = self.shipping.take()?;
            let source = self.source.take()?;
            let source_transfer = self.source_transfer.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;
            let statement_descriptor_suffix = self.statement_descriptor_suffix.take()?;
            let status = self.status.take()?;
            let transfer = self.transfer.take()?;
            let transfer_data = self.transfer_data.take()?;
            let transfer_group = self.transfer_group.take()?;

            Some(Self::Out {
                amount,
                amount_captured,
                amount_refunded,
                application,
                application_fee,
                application_fee_amount,
                authorization_code,
                balance_transaction,
                billing_details,
                calculated_statement_descriptor,
                captured,
                created,
                currency,
                customer,
                description,
                disputed,
                failure_balance_transaction,
                failure_code,
                failure_message,
                fraud_details,
                id,
                invoice,
                level3,
                livemode,
                metadata,
                on_behalf_of,
                outcome,
                paid,
                payment_intent,
                payment_method,
                payment_method_details,
                radar_options,
                receipt_email,
                receipt_number,
                receipt_url,
                refunded,
                refunds,
                review,
                shipping,
                source,
                source_transfer,
                statement_descriptor,
                statement_descriptor_suffix,
                status,
                transfer,
                transfer_data,
                transfer_group,
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

    impl ObjectDeser for Charge {
        type Builder = ChargeBuilder;
    }
};
/// The status of the payment is either `succeeded`, `pending`, or `failed`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ChargeStatus {
    Failed,
    Pending,
    Succeeded,
}
impl ChargeStatus {
    pub fn as_str(self) -> &'static str {
        use ChargeStatus::*;
        match self {
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for ChargeStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChargeStatus::*;
        match s {
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ChargeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ChargeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ChargeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ChargeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ChargeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ChargeStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ChargeStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ChargeStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ChargeStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Charge {
    type Id = stripe_shared::ChargeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ChargeId, "ch_" | "py_");
