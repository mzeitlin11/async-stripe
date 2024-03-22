/// Subscriptions allow you to charge a customer on a recurring basis.
///
/// Related guide: [Creating subscriptions](https://stripe.com/docs/billing/subscriptions/creating)
///
/// For more details see <<https://stripe.com/docs/api/subscriptions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Subscription {
    /// ID of the Connect Application that created the subscription.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_shared::SubscriptionAutomaticTax,
    /// Determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// The timestamp is in UTC format.
    pub billing_cycle_anchor: stripe_types::Timestamp,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionBillingThresholds>,
    /// A date in the future at which the subscription will automatically get canceled
    pub cancel_at: Option<stripe_types::Timestamp>,
    /// If the subscription has been canceled with the `at_period_end` flag set to `true`, `cancel_at_period_end` on the subscription will be true.
    /// You can use this attribute to determine whether a subscription that has a status of active is scheduled to be canceled at the end of the current period.
    pub cancel_at_period_end: bool,
    /// If the subscription has been canceled, the date of that cancellation.
    /// If the subscription was canceled with `cancel_at_period_end`, `canceled_at` will reflect the time of the most recent update request, not the end of the subscription period when the subscription is automatically moved to a canceled state.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Details about why this subscription was cancelled
    pub cancellation_details: Option<stripe_shared::CancellationDetails>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: stripe_shared::SubscriptionCollectionMethod,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// End of the current period that the subscription has been invoiced for.
    /// At the end of this period, a new invoice will be created.
    pub current_period_end: stripe_types::Timestamp,
    /// Start of the current period that the subscription has been invoiced for.
    pub current_period_start: stripe_types::Timestamp,
    /// ID of the customer who owns the subscription.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// Number of days a customer has to pay invoices generated by this subscription.
    /// This value will be `null` for subscriptions where `collection_method=charge_automatically`.
    pub days_until_due: Option<u32>,
    /// ID of the default payment method for the subscription.
    /// It must belong to the customer associated with the subscription.
    /// This takes precedence over `default_source`.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    pub default_payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// ID of the default payment source for the subscription.
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If `default_payment_method` is also set, `default_payment_method` will take precedence.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    pub default_source: Option<stripe_types::Expandable<stripe_shared::PaymentSource>>,
    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    pub default_tax_rates: Option<Vec<stripe_shared::TaxRate>>,
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// Describes the current discount applied to this subscription, if there is one.
    /// When billing, a discount applied to a subscription overrides a discount applied on a customer-wide basis.
    pub discount: Option<stripe_shared::Discount>,
    /// If the subscription has ended, the date the subscription ended.
    pub ended_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionId,
    /// List of subscription items, each with an attached price.
    pub items: stripe_types::List<stripe_shared::SubscriptionItem>,
    /// The most recent invoice this subscription has generated.
    pub latest_invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Specifies the approximate timestamp on which any pending invoice items will be billed according to the schedule provided at `pending_invoice_item_interval`.
    pub next_pending_invoice_item_invoice: Option<stripe_types::Timestamp>,
    /// The account (if any) the charge was made on behalf of for charges associated with this subscription.
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// If specified, payment collection for this subscription will be paused.
    pub pause_collection: Option<stripe_shared::SubscriptionsResourcePauseCollection>,
    /// Payment settings passed on to invoices created by the subscription.
    pub payment_settings: Option<stripe_shared::SubscriptionsResourcePaymentSettings>,
    /// Specifies an interval for how often to bill for any pending invoice items.
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    pub pending_invoice_item_interval: Option<stripe_shared::SubscriptionPendingInvoiceItemInterval>,
    /// You can use this [SetupIntent](https://stripe.com/docs/api/setup_intents) to collect user authentication when creating a subscription without immediate payment or updating a subscription's payment method, allowing you to optimize for off-session payments.
    /// Learn more in the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication#scenario-2).
    pub pending_setup_intent: Option<stripe_types::Expandable<stripe_shared::SetupIntent>>,
    /// If specified, [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates) that will be applied to the subscription once the `latest_invoice` has been paid.
    pub pending_update: Option<stripe_shared::SubscriptionsResourcePendingUpdate>,
    /// The schedule attached to the subscription
    pub schedule: Option<stripe_types::Expandable<stripe_shared::SubscriptionSchedule>>,
    /// Date when the subscription was first created.
    /// The date might differ from the `created` date due to backdating.
    pub start_date: stripe_types::Timestamp,
    /// Possible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`.
    ///
    ///
    /// For `collection_method=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails.
    /// A subscription in this state can only have metadata and default_source updated.
    /// Once the first invoice is paid, the subscription moves into an `active` state.
    /// If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`.
    /// This is a terminal state, the open invoice will be voided and no further invoices will be generated.
    ///
    ///
    /// A subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over.
    ///
    ///
    /// If subscription `collection_method=charge_automatically`, it becomes `past_due` when payment is required but cannot be paid (due to failed payment or awaiting additional user actions).
    /// Once Stripe has exhausted all payment retry attempts, the subscription will become `canceled` or `unpaid` (depending on your subscriptions settings).
    ///
    ///
    /// If subscription `collection_method=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that.
    /// Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed).
    /// After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices.
    pub status: SubscriptionStatus,
    /// ID of the test clock this subscription belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
    /// The account (if any) the subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<stripe_shared::SubscriptionTransferData>,
    /// If the subscription has a trial, the end of that trial.
    pub trial_end: Option<stripe_types::Timestamp>,
    /// Settings related to subscription trials.
    pub trial_settings: Option<stripe_shared::SubscriptionsTrialsResourceTrialSettings>,
    /// If the subscription has a trial, the beginning of that trial.
    pub trial_start: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionBuilder {
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    application_fee_percent: Option<Option<f64>>,
    automatic_tax: Option<stripe_shared::SubscriptionAutomaticTax>,
    billing_cycle_anchor: Option<stripe_types::Timestamp>,
    billing_thresholds: Option<Option<stripe_shared::SubscriptionBillingThresholds>>,
    cancel_at: Option<Option<stripe_types::Timestamp>>,
    cancel_at_period_end: Option<bool>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    cancellation_details: Option<Option<stripe_shared::CancellationDetails>>,
    collection_method: Option<stripe_shared::SubscriptionCollectionMethod>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    current_period_end: Option<stripe_types::Timestamp>,
    current_period_start: Option<stripe_types::Timestamp>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    days_until_due: Option<Option<u32>>,
    default_payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    default_source: Option<Option<stripe_types::Expandable<stripe_shared::PaymentSource>>>,
    default_tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
    description: Option<Option<String>>,
    discount: Option<Option<stripe_shared::Discount>>,
    ended_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_shared::SubscriptionId>,
    items: Option<stripe_types::List<stripe_shared::SubscriptionItem>>,
    latest_invoice: Option<Option<stripe_types::Expandable<stripe_shared::Invoice>>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    next_pending_invoice_item_invoice: Option<Option<stripe_types::Timestamp>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    pause_collection: Option<Option<stripe_shared::SubscriptionsResourcePauseCollection>>,
    payment_settings: Option<Option<stripe_shared::SubscriptionsResourcePaymentSettings>>,
    pending_invoice_item_interval: Option<Option<stripe_shared::SubscriptionPendingInvoiceItemInterval>>,
    pending_setup_intent: Option<Option<stripe_types::Expandable<stripe_shared::SetupIntent>>>,
    pending_update: Option<Option<stripe_shared::SubscriptionsResourcePendingUpdate>>,
    schedule: Option<Option<stripe_types::Expandable<stripe_shared::SubscriptionSchedule>>>,
    start_date: Option<stripe_types::Timestamp>,
    status: Option<SubscriptionStatus>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
    transfer_data: Option<Option<stripe_shared::SubscriptionTransferData>>,
    trial_end: Option<Option<stripe_types::Timestamp>>,
    trial_settings: Option<Option<stripe_shared::SubscriptionsTrialsResourceTrialSettings>>,
    trial_start: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Subscription {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Subscription>,
        builder: SubscriptionBuilder,
    }

    impl Visitor for Place<Subscription> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionBuilder {
        type Out = Subscription;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "application_fee_percent" => Ok(Deserialize::begin(&mut self.application_fee_percent)),
                "automatic_tax" => Ok(Deserialize::begin(&mut self.automatic_tax)),
                "billing_cycle_anchor" => Ok(Deserialize::begin(&mut self.billing_cycle_anchor)),
                "billing_thresholds" => Ok(Deserialize::begin(&mut self.billing_thresholds)),
                "cancel_at" => Ok(Deserialize::begin(&mut self.cancel_at)),
                "cancel_at_period_end" => Ok(Deserialize::begin(&mut self.cancel_at_period_end)),
                "canceled_at" => Ok(Deserialize::begin(&mut self.canceled_at)),
                "cancellation_details" => Ok(Deserialize::begin(&mut self.cancellation_details)),
                "collection_method" => Ok(Deserialize::begin(&mut self.collection_method)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "current_period_end" => Ok(Deserialize::begin(&mut self.current_period_end)),
                "current_period_start" => Ok(Deserialize::begin(&mut self.current_period_start)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "days_until_due" => Ok(Deserialize::begin(&mut self.days_until_due)),
                "default_payment_method" => Ok(Deserialize::begin(&mut self.default_payment_method)),
                "default_source" => Ok(Deserialize::begin(&mut self.default_source)),
                "default_tax_rates" => Ok(Deserialize::begin(&mut self.default_tax_rates)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "discount" => Ok(Deserialize::begin(&mut self.discount)),
                "ended_at" => Ok(Deserialize::begin(&mut self.ended_at)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "items" => Ok(Deserialize::begin(&mut self.items)),
                "latest_invoice" => Ok(Deserialize::begin(&mut self.latest_invoice)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "next_pending_invoice_item_invoice" => Ok(Deserialize::begin(&mut self.next_pending_invoice_item_invoice)),
                "on_behalf_of" => Ok(Deserialize::begin(&mut self.on_behalf_of)),
                "pause_collection" => Ok(Deserialize::begin(&mut self.pause_collection)),
                "payment_settings" => Ok(Deserialize::begin(&mut self.payment_settings)),
                "pending_invoice_item_interval" => Ok(Deserialize::begin(&mut self.pending_invoice_item_interval)),
                "pending_setup_intent" => Ok(Deserialize::begin(&mut self.pending_setup_intent)),
                "pending_update" => Ok(Deserialize::begin(&mut self.pending_update)),
                "schedule" => Ok(Deserialize::begin(&mut self.schedule)),
                "start_date" => Ok(Deserialize::begin(&mut self.start_date)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "test_clock" => Ok(Deserialize::begin(&mut self.test_clock)),
                "transfer_data" => Ok(Deserialize::begin(&mut self.transfer_data)),
                "trial_end" => Ok(Deserialize::begin(&mut self.trial_end)),
                "trial_settings" => Ok(Deserialize::begin(&mut self.trial_settings)),
                "trial_start" => Ok(Deserialize::begin(&mut self.trial_start)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                application: Deserialize::default(),
                application_fee_percent: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                billing_cycle_anchor: Deserialize::default(),
                billing_thresholds: Deserialize::default(),
                cancel_at: Deserialize::default(),
                cancel_at_period_end: Deserialize::default(),
                canceled_at: Deserialize::default(),
                cancellation_details: Deserialize::default(),
                collection_method: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                current_period_end: Deserialize::default(),
                current_period_start: Deserialize::default(),
                customer: Deserialize::default(),
                days_until_due: Deserialize::default(),
                default_payment_method: Deserialize::default(),
                default_source: Deserialize::default(),
                default_tax_rates: Deserialize::default(),
                description: Deserialize::default(),
                discount: Deserialize::default(),
                ended_at: Deserialize::default(),
                id: Deserialize::default(),
                items: Deserialize::default(),
                latest_invoice: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                next_pending_invoice_item_invoice: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                pause_collection: Deserialize::default(),
                payment_settings: Deserialize::default(),
                pending_invoice_item_interval: Deserialize::default(),
                pending_setup_intent: Deserialize::default(),
                pending_update: Deserialize::default(),
                schedule: Deserialize::default(),
                start_date: Deserialize::default(),
                status: Deserialize::default(),
                test_clock: Deserialize::default(),
                transfer_data: Deserialize::default(),
                trial_end: Deserialize::default(),
                trial_settings: Deserialize::default(),
                trial_start: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let application = self.application.take()?;
            let application_fee_percent = self.application_fee_percent.take()?;
            let automatic_tax = self.automatic_tax.take()?;
            let billing_cycle_anchor = self.billing_cycle_anchor.take()?;
            let billing_thresholds = self.billing_thresholds.take()?;
            let cancel_at = self.cancel_at.take()?;
            let cancel_at_period_end = self.cancel_at_period_end.take()?;
            let canceled_at = self.canceled_at.take()?;
            let cancellation_details = self.cancellation_details.take()?;
            let collection_method = self.collection_method.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let current_period_end = self.current_period_end.take()?;
            let current_period_start = self.current_period_start.take()?;
            let customer = self.customer.take()?;
            let days_until_due = self.days_until_due.take()?;
            let default_payment_method = self.default_payment_method.take()?;
            let default_source = self.default_source.take()?;
            let default_tax_rates = self.default_tax_rates.take()?;
            let description = self.description.take()?;
            let discount = self.discount.take()?;
            let ended_at = self.ended_at.take()?;
            let id = self.id.take()?;
            let items = self.items.take()?;
            let latest_invoice = self.latest_invoice.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let next_pending_invoice_item_invoice = self.next_pending_invoice_item_invoice.take()?;
            let on_behalf_of = self.on_behalf_of.take()?;
            let pause_collection = self.pause_collection.take()?;
            let payment_settings = self.payment_settings.take()?;
            let pending_invoice_item_interval = self.pending_invoice_item_interval.take()?;
            let pending_setup_intent = self.pending_setup_intent.take()?;
            let pending_update = self.pending_update.take()?;
            let schedule = self.schedule.take()?;
            let start_date = self.start_date.take()?;
            let status = self.status.take()?;
            let test_clock = self.test_clock.take()?;
            let transfer_data = self.transfer_data.take()?;
            let trial_end = self.trial_end.take()?;
            let trial_settings = self.trial_settings.take()?;
            let trial_start = self.trial_start.take()?;

            Some(Self::Out {
                application,
                application_fee_percent,
                automatic_tax,
                billing_cycle_anchor,
                billing_thresholds,
                cancel_at,
                cancel_at_period_end,
                canceled_at,
                cancellation_details,
                collection_method,
                created,
                currency,
                current_period_end,
                current_period_start,
                customer,
                days_until_due,
                default_payment_method,
                default_source,
                default_tax_rates,
                description,
                discount,
                ended_at,
                id,
                items,
                latest_invoice,
                livemode,
                metadata,
                next_pending_invoice_item_invoice,
                on_behalf_of,
                pause_collection,
                payment_settings,
                pending_invoice_item_interval,
                pending_setup_intent,
                pending_update,
                schedule,
                start_date,
                status,
                test_clock,
                transfer_data,
                trial_end,
                trial_settings,
                trial_start,
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

    impl ObjectDeser for Subscription {
        type Builder = SubscriptionBuilder;
    }
};
/// Possible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`.
///
///
/// For `collection_method=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails.
/// A subscription in this state can only have metadata and default_source updated.
/// Once the first invoice is paid, the subscription moves into an `active` state.
/// If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`.
/// This is a terminal state, the open invoice will be voided and no further invoices will be generated.
///
///
/// A subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over.
///
///
/// If subscription `collection_method=charge_automatically`, it becomes `past_due` when payment is required but cannot be paid (due to failed payment or awaiting additional user actions).
/// Once Stripe has exhausted all payment retry attempts, the subscription will become `canceled` or `unpaid` (depending on your subscriptions settings).
///
///
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SubscriptionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Subscription {
    type Id = stripe_shared::SubscriptionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SubscriptionId, "sub_");
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionCollectionMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SubscriptionCollectionMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionCollectionMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
