/// The `Charge` object represents a single attempt to move money into your Stripe account.
/// PaymentIntent confirmation is the most common way to create Charges, but transferring
/// money to a different Stripe account through Connect also creates Charges.
/// Some legacy payment flows create Charges directly, which is not recommended for new integrations.
///
/// For more details see <<https://stripe.com/docs/api/charges/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub failure_balance_transaction:
        Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<stripe_types::Expandable<stripe_shared::Transfer>>,
    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub transfer_data: Option<stripe_shared::ChargeTransferData>,
    /// A string that identifies this transaction as part of a group.
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}
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
impl stripe_types::Object for Charge {
    type Id = stripe_shared::ChargeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ChargeId, "ch_" | "py_");
