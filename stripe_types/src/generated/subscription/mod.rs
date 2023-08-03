/// Subscriptions allow you to charge a customer on a recurring basis.
///
/// Related guide: [Creating subscriptions](https://stripe.com/docs/billing/subscriptions/creating).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Subscription {
    /// ID of the Connect Application that created the subscription.
    pub application: Option<stripe_types::Expandable<stripe_types::Application>>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_types::SubscriptionAutomaticTax,
    /// Determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    ///
    /// The timestamp is in UTC format.
    pub billing_cycle_anchor: stripe_types::Timestamp,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_types::SubscriptionBillingThresholds>,
    /// A date in the future at which the subscription will automatically get canceled.
    pub cancel_at: Option<stripe_types::Timestamp>,
    /// If the subscription has been canceled with the `at_period_end` flag set to `true`, `cancel_at_period_end` on the subscription will be true.
    ///
    /// You can use this attribute to determine whether a subscription that has a status of active is scheduled to be canceled at the end of the current period.
    pub cancel_at_period_end: bool,
    /// If the subscription has been canceled, the date of that cancellation.
    ///
    /// If the subscription was canceled with `cancel_at_period_end`, `canceled_at` will reflect the time of the most recent update request, not the end of the subscription period when the subscription is automatically moved to a canceled state.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Details about why this subscription was cancelled.
    pub cancellation_details: Option<stripe_types::CancellationDetails>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: SubscriptionCollectionMethod,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// End of the current period that the subscription has been invoiced for.
    ///
    /// At the end of this period, a new invoice will be created.
    pub current_period_end: stripe_types::Timestamp,
    /// Start of the current period that the subscription has been invoiced for.
    pub current_period_start: stripe_types::Timestamp,
    /// ID of the customer who owns the subscription.
    pub customer: stripe_types::Expandable<stripe_types::Customer>,
    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// This value will be `null` for subscriptions where `collection_method=charge_automatically`.
    pub days_until_due: Option<u32>,
    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// This takes precedence over `default_source`.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    pub default_payment_method: Option<stripe_types::Expandable<stripe_types::PaymentMethod>>,
    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If `default_payment_method` is also set, `default_payment_method` will take precedence.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    pub default_source: Option<stripe_types::Expandable<stripe_types::PaymentSource>>,
    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<stripe_types::TaxRate>>,
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces.
    pub description: Option<String>,
    /// Describes the current discount applied to this subscription, if there is one.
    ///
    /// When billing, a discount applied to a subscription overrides a discount applied on a customer-wide basis.
    pub discount: Option<stripe_types::Discount>,
    /// If the subscription has ended, the date the subscription ended.
    pub ended_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_types::subscription::SubscriptionId,
    /// List of subscription items, each with an attached price.
    pub items: stripe_types::List<stripe_types::SubscriptionItem>,
    /// The most recent invoice this subscription has generated.
    pub latest_invoice: Option<stripe_types::Expandable<stripe_types::Invoice>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Specifies the approximate timestamp on which any pending invoice items will be billed according to the schedule provided at `pending_invoice_item_interval`.
    pub next_pending_invoice_item_invoice: Option<stripe_types::Timestamp>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SubscriptionObject,
    /// The account (if any) the charge was made on behalf of for charges associated with this subscription.
    ///
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_types::Account>>,
    /// If specified, payment collection for this subscription will be paused.
    pub pause_collection: Option<stripe_types::SubscriptionsResourcePauseCollection>,
    /// Payment settings passed on to invoices created by the subscription.
    pub payment_settings: Option<stripe_types::SubscriptionsResourcePaymentSettings>,
    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    pub pending_invoice_item_interval: Option<stripe_types::SubscriptionPendingInvoiceItemInterval>,
    /// You can use this [SetupIntent](https://stripe.com/docs/api/setup_intents) to collect user authentication when creating a subscription without immediate payment or updating a subscription's payment method, allowing you to optimize for off-session payments.
    ///
    /// Learn more in the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication#scenario-2).
    pub pending_setup_intent: Option<stripe_types::Expandable<stripe_types::SetupIntent>>,
    /// If specified, [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates) that will be applied to the subscription once the `latest_invoice` has been paid.
    pub pending_update: Option<stripe_types::SubscriptionsResourcePendingUpdate>,
    /// The schedule attached to the subscription.
    pub schedule: Option<stripe_types::Expandable<stripe_types::SubscriptionSchedule>>,
    /// Date when the subscription was first created.
    ///
    /// The date might differ from the `created` date due to backdating.
    pub start_date: stripe_types::Timestamp,
    /// Possible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`.
    ///
    /// For `collection_method=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails.
    /// A subscription in this state can only have metadata and default_source updated.
    /// Once the first invoice is paid, the subscription moves into an `active` state.
    /// If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`.
    /// This is a terminal state, the open invoice will be voided and no further invoices will be generated.
    /// A subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over.
    /// If subscription `collection_method=charge_automatically`, it becomes `past_due` when payment is required but cannot be paid (due to failed payment or awaiting additional user actions).
    /// Once Stripe has exhausted all payment retry attempts, the subscription will become `canceled` or `unpaid` (depending on your subscriptions settings).
    /// If subscription `collection_method=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that.
    /// Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed).
    /// After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices.
    pub status: SubscriptionStatus,
    /// ID of the test clock this subscription belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_types::TestClock>>,
    /// The account (if any) the subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<stripe_types::SubscriptionTransferData>,
    /// If the subscription has a trial, the end of that trial.
    pub trial_end: Option<stripe_types::Timestamp>,
    /// Settings related to subscription trials.
    pub trial_settings: Option<stripe_types::SubscriptionsTrialsResourceTrialSettings>,
    /// If the subscription has a trial, the beginning of that trial.
    pub trial_start: Option<stripe_types::Timestamp>,
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl SubscriptionCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use SubscriptionCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for SubscriptionCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionCollectionMethod"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionObject {
    Subscription,
}

impl SubscriptionObject {
    pub fn as_str(self) -> &'static str {
        use SubscriptionObject::*;
        match self {
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for SubscriptionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionObject::*;
        match s {
            "subscription" => Ok(Subscription),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionObject"))
    }
}
/// Possible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`.
///
/// For `collection_method=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails.
/// A subscription in this state can only have metadata and default_source updated.
/// Once the first invoice is paid, the subscription moves into an `active` state.
/// If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`.
/// This is a terminal state, the open invoice will be voided and no further invoices will be generated.
/// A subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over.
/// If subscription `collection_method=charge_automatically`, it becomes `past_due` when payment is required but cannot be paid (due to failed payment or awaiting additional user actions).
/// Once Stripe has exhausted all payment retry attempts, the subscription will become `canceled` or `unpaid` (depending on your subscriptions settings).
/// If subscription `collection_method=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that.
/// Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed).
/// After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Paused,
    Trialing,
    Unpaid,
}

impl SubscriptionStatus {
    pub fn as_str(self) -> &'static str {
        use SubscriptionStatus::*;
        match self {
            Active => "active",
            Canceled => "canceled",
            Incomplete => "incomplete",
            IncompleteExpired => "incomplete_expired",
            PastDue => "past_due",
            Paused => "paused",
            Trialing => "trialing",
            Unpaid => "unpaid",
        }
    }
}

impl std::str::FromStr for SubscriptionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionStatus::*;
        match s {
            "active" => Ok(Active),
            "canceled" => Ok(Canceled),
            "incomplete" => Ok(Incomplete),
            "incomplete_expired" => Ok(IncompleteExpired),
            "past_due" => Ok(PastDue),
            "paused" => Ok(Paused),
            "trialing" => Ok(Trialing),
            "unpaid" => Ok(Unpaid),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionStatus"))
    }
}
impl stripe_types::Object for Subscription {
    type Id = stripe_types::subscription::SubscriptionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(SubscriptionId, "sub_");
