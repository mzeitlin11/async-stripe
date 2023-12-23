#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListSubscriptionSchedule<'a> {
    /// Only return subscription schedules that were created canceled the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<stripe_types::RangeQueryTs>,
    /// Only return subscription schedules that completed during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<stripe_types::RangeQueryTs>,
    /// Only return subscription schedules that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return subscription schedules for the given customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return subscription schedules that were released during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<stripe_types::RangeQueryTs>,
    /// Only return subscription schedules that have not started yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<bool>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListSubscriptionSchedule<'a> {
    /// Retrieves the list of your subscription schedules.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::SubscriptionSchedule>> {
        client.get_query("/subscription_schedules", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_shared::SubscriptionSchedule>> {
        stripe::ListPaginator::from_list_params("/subscription_schedules", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionSchedule<'a> {
    /// The identifier of the customer to create the subscription schedule for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<CreateSubscriptionScheduleDefaultSettings<'a>>,
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<CreateSubscriptionScheduleEndBehavior>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Migrate an existing subscription to be managed by a subscription schedule.
    /// If this parameter is set, a subscription schedule will be created using the subscription's item(s), set to auto-renew using the subscription's interval.
    /// When using this parameter, other parameters (such as phase values) cannot be set.
    /// To create a subscription schedule with other modifications, we recommend making two separate API calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_subscription: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// List representing phases of the subscription schedule.
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<&'a [CreateSubscriptionSchedulePhases<'a>]>,
    /// When the subscription schedule starts.
    /// We recommend using `now` so that it starts the subscription immediately.
    /// You can also use a Unix timestamp to backdate the subscription so that it starts on a past date, or set a future date for the subscription to start on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<CreateSubscriptionScheduleStartDate>,
}
impl<'a> CreateSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Object representing the subscription schedule's default settings.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettings<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Default settings for automatic tax computation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateSubscriptionScheduleDefaultSettingsAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<CreateSubscriptionScheduleDefaultSettingsBillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CreateSubscriptionScheduleDefaultSettingsCollectionMethod>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateSubscriptionScheduleDefaultSettingsInvoiceSettings>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateSubscriptionScheduleDefaultSettingsTransferData<'a>>,
}
impl<'a> CreateSubscriptionScheduleDefaultSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default settings for automatic tax computation.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}
impl CreateSubscriptionScheduleDefaultSettingsAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// Pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl CreateSubscriptionScheduleDefaultSettingsBillingThresholds {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically` on creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionScheduleDefaultSettingsCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionScheduleDefaultSettingsCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}
impl CreateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> CreateSubscriptionScheduleDefaultSettingsTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount_percent: None, destination }
    }
}
/// Behavior of the subscription schedule and underlying subscription when it ends.
/// Possible values are `release` or `cancel` with the default being `release`.
/// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}
impl CreateSubscriptionScheduleEndBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionScheduleEndBehavior::*;
        match self {
            Cancel => "cancel",
            None => "none",
            Release => "release",
            Renew => "renew",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionScheduleEndBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionScheduleEndBehavior::*;
        match s {
            "cancel" => Ok(Cancel),
            "none" => Ok(None),
            "release" => Ok(Release),
            "renew" => Ok(Renew),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateSubscriptionScheduleEndBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateSubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionScheduleEndBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// List representing phases of the subscription schedule.
/// Each phase can be customized to have different durations, plans, and coupons.
/// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhases<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<&'a [CreateSubscriptionSchedulePhasesAddInvoiceItems<'a>]>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateSubscriptionSchedulePhasesAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<CreateSubscriptionSchedulePhasesBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<CreateSubscriptionSchedulePhasesBillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CreateSubscriptionSchedulePhasesCollectionMethod>,
    /// The identifier of the coupon to apply to this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will set the Subscription's [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates), which means they will be the Invoice's [`default_tax_rates`](https://stripe.com/docs/api/invoices/create#create_invoice-default_tax_rates) for any Invoices issued by the Subscription during this Phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The date at which this phase of the subscription schedule ends.
    /// If set, `iterations` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateSubscriptionSchedulePhasesInvoiceSettings>,
    /// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
    pub items: &'a [CreateSubscriptionSchedulePhasesItems<'a>],
    /// Integer representing the multiplier applied to the price interval.
    /// For example, `iterations=2` applied to a price with `interval=month` and `interval_count=3` results in a phase of duration `2 * 3 months = 6 months`.
    /// If set, `end_date` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered, adding new keys and replacing existing keys in the subscription's `metadata`.
    /// Individual keys in the subscription's `metadata` can be unset by posting an empty value to them in the phase's `metadata`.
    /// To unset all keys in the subscription's `metadata`, update the subscription directly or unset every key individually from the phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
    /// The default value is `create_prorations`.
    /// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
    /// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateSubscriptionSchedulePhasesProrationBehavior>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateSubscriptionSchedulePhasesTransferData<'a>>,
    /// If set to true the entire phase is counted as a trial and the customer will not be charged for any fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,
    /// Sets the phase to trialing from the start date to this date.
    /// Must be before the phase end date, can not be combined with `trial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<stripe_types::Timestamp>,
}
impl<'a> CreateSubscriptionSchedulePhases<'a> {
    pub fn new(items: &'a [CreateSubscriptionSchedulePhasesItems<'a>]) -> Self {
        Self {
            add_invoice_items: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_thresholds: None,
            collection_method: None,
            coupon: None,
            currency: None,
            default_payment_method: None,
            default_tax_rates: None,
            description: None,
            end_date: None,
            invoice_settings: None,
            items,
            iterations: None,
            metadata: None,
            on_behalf_of: None,
            proration_behavior: None,
            transfer_data: None,
            trial: None,
            trial_end: None,
        }
    }
}
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
/// You may pass up to 20 items.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesAddInvoiceItems<'a> {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceData<'a>>,
    /// Quantity for this item. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateSubscriptionSchedulePhasesAddInvoiceItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceData<'a> {
    pub fn new(currency: stripe_types::Currency, product: &'a str) -> Self {
        Self { currency, product, tax_behavior: None, unit_amount: None, unit_amount_decimal: None }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Automatic tax settings for this phase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}
impl CreateSubscriptionSchedulePhasesAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// Pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl CreateSubscriptionSchedulePhasesBillingThresholds {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically` on creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl CreateSubscriptionSchedulePhasesCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateSubscriptionSchedulePhasesCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesInvoiceSettings {
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}
impl CreateSubscriptionSchedulePhasesInvoiceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesItems<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<CreateSubscriptionSchedulePhasesItemsBillingThresholds>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a configuration item.
    /// Metadata on a configuration item will update the underlying subscription item's `metadata` when the phase is entered, adding new keys and replacing existing keys.
    /// Individual keys in the subscription item's `metadata` can be unset by posting an empty value to them in the configuration item's `metadata`.
    /// To unset all keys in the subscription item's `metadata`, update the subscription item directly or unset every key individually from the configuration item's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The plan ID to subscribe to. You may specify the same ID in `plan` and `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateSubscriptionSchedulePhasesItemsPriceData<'a>>,
    /// Quantity for the given price.
    /// Can be set only if the price's `usage_type` is `licensed` and not `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateSubscriptionSchedulePhasesItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// When updating, pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsBillingThresholds {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}
impl CreateSubscriptionSchedulePhasesItemsBillingThresholds {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: CreateSubscriptionSchedulePhasesItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateSubscriptionSchedulePhasesItemsPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: CreateSubscriptionSchedulePhasesItemsPriceDataRecurring,
    ) -> Self {
        Self {
            currency,
            product,
            recurring,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    pub fn new(interval: CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
/// The default value is `create_prorations`.
/// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
/// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl CreateSubscriptionSchedulePhasesProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateSubscriptionSchedulePhasesProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> CreateSubscriptionSchedulePhasesTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount_percent: None, destination }
    }
}
/// When the subscription schedule starts.
/// We recommend using `now` so that it starts the subscription immediately.
/// You can also use a Unix timestamp to backdate the subscription so that it starts on a past date, or set a future date for the subscription to start on.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum CreateSubscriptionScheduleStartDate {
    Timestamp(stripe_types::Timestamp),
    Now,
}
impl<'a> CreateSubscriptionSchedule<'a> {
    /// Creates a new subscription schedule object.
    /// Each customer can have up to 500 active or scheduled subscriptions.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_shared::SubscriptionSchedule> {
        client.send_form("/subscription_schedules", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveSubscriptionSchedule<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveSubscriptionSchedule<'a> {
    /// Retrieves the details of an existing subscription schedule.
    /// You only need to supply the unique subscription schedule identifier that was returned upon subscription schedule creation.
    pub fn send(
        &self,
        client: &stripe::Client,
        schedule: &stripe_shared::SubscriptionScheduleId,
    ) -> stripe::Response<stripe_shared::SubscriptionSchedule> {
        client.get_query(&format!("/subscription_schedules/{schedule}"), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionSchedule<'a> {
    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<UpdateSubscriptionScheduleDefaultSettings<'a>>,
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<UpdateSubscriptionScheduleEndBehavior>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// List representing phases of the subscription schedule.
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    /// Note that past phases can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<&'a [UpdateSubscriptionSchedulePhases<'a>]>,
    /// If the update changes the current phase, indicates whether the changes should be prorated.
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpdateSubscriptionScheduleProrationBehavior>,
}
impl<'a> UpdateSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Object representing the subscription schedule's default settings.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettings<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Default settings for automatic tax computation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateSubscriptionScheduleDefaultSettingsAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpdateSubscriptionScheduleDefaultSettingsBillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<UpdateSubscriptionScheduleDefaultSettingsCollectionMethod>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdateSubscriptionScheduleDefaultSettingsInvoiceSettings>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdateSubscriptionScheduleDefaultSettingsTransferData<'a>>,
}
impl<'a> UpdateSubscriptionScheduleDefaultSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default settings for automatic tax computation.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}
impl UpdateSubscriptionScheduleDefaultSettingsAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// Pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl UpdateSubscriptionScheduleDefaultSettingsBillingThresholds {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically` on creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionScheduleDefaultSettingsCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionScheduleDefaultSettingsCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}
impl UpdateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> UpdateSubscriptionScheduleDefaultSettingsTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount_percent: None, destination }
    }
}
/// Behavior of the subscription schedule and underlying subscription when it ends.
/// Possible values are `release` or `cancel` with the default being `release`.
/// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}
impl UpdateSubscriptionScheduleEndBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionScheduleEndBehavior::*;
        match self {
            Cancel => "cancel",
            None => "none",
            Release => "release",
            Renew => "renew",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionScheduleEndBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionScheduleEndBehavior::*;
        match s {
            "cancel" => Ok(Cancel),
            "none" => Ok(None),
            "release" => Ok(Release),
            "renew" => Ok(Renew),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionScheduleEndBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionScheduleEndBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// List representing phases of the subscription schedule.
/// Each phase can be customized to have different durations, plans, and coupons.
/// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
/// Note that past phases can be omitted.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhases<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<&'a [UpdateSubscriptionSchedulePhasesAddInvoiceItems<'a>]>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateSubscriptionSchedulePhasesAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpdateSubscriptionSchedulePhasesBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpdateSubscriptionSchedulePhasesBillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<UpdateSubscriptionSchedulePhasesCollectionMethod>,
    /// The identifier of the coupon to apply to this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will set the Subscription's [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates), which means they will be the Invoice's [`default_tax_rates`](https://stripe.com/docs/api/invoices/create#create_invoice-default_tax_rates) for any Invoices issued by the Subscription during this Phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The date at which this phase of the subscription schedule ends.
    /// If set, `iterations` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<UpdateSubscriptionSchedulePhasesEndDate>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdateSubscriptionSchedulePhasesInvoiceSettings>,
    /// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
    pub items: &'a [UpdateSubscriptionSchedulePhasesItems<'a>],
    /// Integer representing the multiplier applied to the price interval.
    /// For example, `iterations=2` applied to a price with `interval=month` and `interval_count=3` results in a phase of duration `2 * 3 months = 6 months`.
    /// If set, `end_date` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered, adding new keys and replacing existing keys in the subscription's `metadata`.
    /// Individual keys in the subscription's `metadata` can be unset by posting an empty value to them in the phase's `metadata`.
    /// To unset all keys in the subscription's `metadata`, update the subscription directly or unset every key individually from the phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
    /// The default value is `create_prorations`.
    /// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
    /// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpdateSubscriptionSchedulePhasesProrationBehavior>,
    /// The date at which this phase of the subscription schedule starts or `now`.
    /// Must be set on the first phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<UpdateSubscriptionSchedulePhasesStartDate>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdateSubscriptionSchedulePhasesTransferData<'a>>,
    /// If set to true the entire phase is counted as a trial and the customer will not be charged for any fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,
    /// Sets the phase to trialing from the start date to this date.
    /// Must be before the phase end date, can not be combined with `trial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<UpdateSubscriptionSchedulePhasesTrialEnd>,
}
impl<'a> UpdateSubscriptionSchedulePhases<'a> {
    pub fn new(items: &'a [UpdateSubscriptionSchedulePhasesItems<'a>]) -> Self {
        Self {
            add_invoice_items: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_thresholds: None,
            collection_method: None,
            coupon: None,
            currency: None,
            default_payment_method: None,
            default_tax_rates: None,
            description: None,
            end_date: None,
            invoice_settings: None,
            items,
            iterations: None,
            metadata: None,
            on_behalf_of: None,
            proration_behavior: None,
            start_date: None,
            transfer_data: None,
            trial: None,
            trial_end: None,
        }
    }
}
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
/// You may pass up to 20 items.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesAddInvoiceItems<'a> {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceData<'a>>,
    /// Quantity for this item. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpdateSubscriptionSchedulePhasesAddInvoiceItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceData<'a> {
    pub fn new(currency: stripe_types::Currency, product: &'a str) -> Self {
        Self { currency, product, tax_behavior: None, unit_amount: None, unit_amount_decimal: None }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Automatic tax settings for this phase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}
impl UpdateSubscriptionSchedulePhasesAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// Pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl UpdateSubscriptionSchedulePhasesBillingThresholds {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically` on creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl UpdateSubscriptionSchedulePhasesCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionSchedulePhasesCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The date at which this phase of the subscription schedule ends.
/// If set, `iterations` must not be set.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpdateSubscriptionSchedulePhasesEndDate {
    Timestamp(stripe_types::Timestamp),
    Now,
}
/// All invoices will be billed using the specified settings.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesInvoiceSettings {
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}
impl UpdateSubscriptionSchedulePhasesInvoiceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItems<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpdateSubscriptionSchedulePhasesItemsBillingThresholds>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a configuration item.
    /// Metadata on a configuration item will update the underlying subscription item's `metadata` when the phase is entered, adding new keys and replacing existing keys.
    /// Individual keys in the subscription item's `metadata` can be unset by posting an empty value to them in the configuration item's `metadata`.
    /// To unset all keys in the subscription item's `metadata`, update the subscription item directly or unset every key individually from the configuration item's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The plan ID to subscribe to. You may specify the same ID in `plan` and `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateSubscriptionSchedulePhasesItemsPriceData<'a>>,
    /// Quantity for the given price.
    /// Can be set only if the price's `usage_type` is `licensed` and not `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpdateSubscriptionSchedulePhasesItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
/// When updating, pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsBillingThresholds {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}
impl UpdateSubscriptionSchedulePhasesItemsBillingThresholds {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateSubscriptionSchedulePhasesItemsPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring,
    ) -> Self {
        Self {
            currency,
            product,
            recurring,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    pub fn new(interval: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
/// The default value is `create_prorations`.
/// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
/// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpdateSubscriptionSchedulePhasesProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionSchedulePhasesProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The date at which this phase of the subscription schedule starts or `now`.
/// Must be set on the first phase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpdateSubscriptionSchedulePhasesStartDate {
    Timestamp(stripe_types::Timestamp),
    Now,
}
/// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> UpdateSubscriptionSchedulePhasesTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount_percent: None, destination }
    }
}
/// Sets the phase to trialing from the start date to this date.
/// Must be before the phase end date, can not be combined with `trial`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpdateSubscriptionSchedulePhasesTrialEnd {
    Timestamp(stripe_types::Timestamp),
    Now,
}
/// If the update changes the current phase, indicates whether the changes should be prorated.
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionScheduleProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpdateSubscriptionScheduleProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionScheduleProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionScheduleProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionScheduleProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateSubscriptionScheduleProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateSubscriptionScheduleProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionScheduleProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionScheduleProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> UpdateSubscriptionSchedule<'a> {
    /// Updates an existing subscription schedule.
    pub fn send(
        &self,
        client: &stripe::Client,
        schedule: &stripe_shared::SubscriptionScheduleId,
    ) -> stripe::Response<stripe_shared::SubscriptionSchedule> {
        client.send_form(
            &format!("/subscription_schedules/{schedule}"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelSubscriptionSchedule<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// If the subscription schedule is `active`, indicates if a final invoice will be generated that contains any un-invoiced metered usage and new/pending proration invoice items.
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_now: Option<bool>,
    /// If the subscription schedule is `active`, indicates if the cancellation should be prorated.
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
}
impl<'a> CancelSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelSubscriptionSchedule<'a> {
    /// Cancels a subscription schedule and its associated subscription immediately (if the subscription schedule has an active subscription).
    /// A subscription schedule can only be canceled if its status is `not_started` or `active`.
    pub fn send(
        &self,
        client: &stripe::Client,
        schedule: &stripe_shared::SubscriptionScheduleId,
    ) -> stripe::Response<stripe_shared::SubscriptionSchedule> {
        client.send_form(
            &format!("/subscription_schedules/{schedule}/cancel"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReleaseSubscriptionSchedule<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Keep any cancellation on the subscription that the schedule has set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_cancel_date: Option<bool>,
}
impl<'a> ReleaseSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ReleaseSubscriptionSchedule<'a> {
    /// Releases the subscription schedule immediately, which will stop scheduling of its phases, but leave any existing subscription in place.
    /// A schedule can only be released if its status is `not_started` or `active`.
    /// If the subscription schedule is currently associated with a subscription, releasing it will remove its `subscription` property and set the subscription’s ID to the `released_subscription` property.
    pub fn send(
        &self,
        client: &stripe::Client,
        schedule: &stripe_shared::SubscriptionScheduleId,
    ) -> stripe::Response<stripe_shared::SubscriptionSchedule> {
        client.send_form(
            &format!("/subscription_schedules/{schedule}/release"),
            self,
            http_types::Method::Post,
        )
    }
}
