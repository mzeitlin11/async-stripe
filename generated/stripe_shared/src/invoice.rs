/// Invoices are statements of amounts owed by a customer, and are either
/// generated one-off, or generated periodically from a subscription.
///
/// They contain [invoice items](https://stripe.com/docs/api#invoiceitems), and proration adjustments
/// that may be caused by subscription upgrades/downgrades (if necessary).
///
/// If your invoice is configured to be billed through automatic charges,
/// Stripe automatically finalizes your invoice and attempts payment. Note
/// that finalizing the invoice,
/// [when automatic](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection), does.
/// not happen immediately as the invoice is created. Stripe waits
/// until one hour after the last webhook was successfully sent (or the last
/// webhook timed out after failing). If you (and the platforms you may have
/// connected to) have no webhooks configured, Stripe waits one hour after
/// creation to finalize the invoice.
///
/// If your invoice is configured to be billed by sending an email, then based on your
/// [email settings](https://dashboard.stripe.com/account/billing/automatic),
/// Stripe will email the invoice to your customer and await payment. These
/// emails can contain a link to a hosted page to pay the invoice.
///
/// Stripe applies any customer credit on the account before determining the
/// amount due for the invoice (i.e., the amount that will be actually
/// charged). If the amount due for the invoice is less than Stripe's [minimum allowed charge
/// per currency](/docs/currencies#minimum-and-maximum-charge-amounts), the
/// invoice is automatically marked paid, and we add the amount due to the
/// customer's credit balance which is applied to the next invoice.
///
/// More details on the customer's credit balance are
/// [here](https://stripe.com/docs/billing/customer/balance).
///
/// Related guide: [Send invoices to customers](https://stripe.com/docs/billing/invoices/sending)
///
/// For more details see <<https://stripe.com/docs/api/invoices/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Invoice {
    /// The country of the business associated with this invoice, most often the business creating the invoice.
    pub account_country: Option<String>,
    /// The public name of the business associated with this invoice, most often the business creating the invoice.
    pub account_name: Option<String>,
    /// The account tax IDs associated with the invoice. Only editable when the invoice is a draft.
    pub account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>,
    /// Final amount due at this time for this invoice.
    /// If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0.
    /// If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account.
    /// The charge that gets generated for the invoice will be for the amount specified in `amount_due`.
    pub amount_due: i64,
    /// The amount, in cents (or local equivalent), that was paid.
    pub amount_paid: i64,
    /// The difference between amount_due and amount_paid, in cents (or local equivalent).
    pub amount_remaining: i64,
    /// This is the sum of all the shipping amounts.
    pub amount_shipping: i64,
    /// ID of the Connect Application that created the invoice.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account when the invoice is paid.
    pub application_fee_amount: Option<i64>,
    /// Number of payment attempts made for this invoice, from the perspective of the payment retry schedule.
    /// Any payment attempt counts as the first attempt, and subsequently only automatic retries increment the attempt count.
    /// In other words, manual payment attempts after the first attempt do not affect the retry schedule.
    pub attempt_count: u64,
    /// Whether an attempt has been made to pay the invoice.
    /// An invoice is not attempted until 1 hour after the `invoice.created` webhook, for example, so you might not want to display that invoice as unpaid to your users.
    pub attempted: bool,
    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    /// If `false`, the invoice's state doesn't automatically advance without an explicit action.
    pub auto_advance: Option<bool>,
    pub automatic_tax: stripe_shared::AutomaticTax,
    /// Indicates the reason why the invoice was created.
    ///
    /// * `manual`: Unrelated to a subscription, for example, created via the invoice editor.
    /// * `subscription`: No longer in use.
    /// Applies to subscriptions from before May 2018 where no distinction was made between updates, cycles, and thresholds.
    /// * `subscription_create`: A new subscription was created.
    /// * `subscription_cycle`: A subscription advanced into a new period.
    /// * `subscription_threshold`: A subscription reached a billing threshold.
    /// * `subscription_update`: A subscription was updated.
    /// * `upcoming`: Reserved for simulated invoices, per the upcoming invoice endpoint.
    pub billing_reason: Option<InvoiceBillingReason>,
    /// ID of the latest charge generated for this invoice, if any.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    pub collection_method: stripe_shared::InvoiceCollectionMethod,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Custom fields displayed on the invoice.
    pub custom_fields: Option<Vec<stripe_shared::InvoiceSettingCustomField>>,
    /// The ID of the customer who will be billed.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// The customer's address.
    /// Until the invoice is finalized, this field will equal `customer.address`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_address: Option<stripe_shared::Address>,
    /// The customer's email.
    /// Until the invoice is finalized, this field will equal `customer.email`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_email: Option<String>,
    /// The customer's name.
    /// Until the invoice is finalized, this field will equal `customer.name`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_name: Option<String>,
    /// The customer's phone number.
    /// Until the invoice is finalized, this field will equal `customer.phone`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_phone: Option<String>,
    /// The customer's shipping information.
    /// Until the invoice is finalized, this field will equal `customer.shipping`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_shipping: Option<stripe_shared::Shipping>,
    /// The customer's tax exempt status.
    /// Until the invoice is finalized, this field will equal `customer.tax_exempt`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_tax_exempt: Option<InvoiceCustomerTaxExempt>,
    /// The customer's tax IDs.
    /// Until the invoice is finalized, this field will contain the same tax IDs as `customer.tax_ids`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_tax_ids: Option<Vec<stripe_shared::InvoicesResourceInvoiceTaxId>>,
    /// ID of the default payment method for the invoice.
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// ID of the default payment source for the invoice.
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    pub default_source: Option<stripe_types::Expandable<stripe_shared::PaymentSource>>,
    /// The tax rates applied to this invoice, if any.
    pub default_tax_rates: Vec<stripe_shared::TaxRate>,
    /// An arbitrary string attached to the object.
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    pub description: Option<String>,
    /// Describes the current discount applied to this invoice, if there is one.
    /// Not populated if there are multiple discounts.
    pub discount: Option<stripe_shared::Discount>,
    /// The discounts applied to the invoice.
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>,
    /// The date on which payment for this invoice is due.
    /// This value will be `null` for invoices where `collection_method=charge_automatically`.
    pub due_date: Option<stripe_types::Timestamp>,
    /// The date when this invoice is in effect.
    /// Same as `finalized_at` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the invoice PDF and receipt.
    pub effective_at: Option<stripe_types::Timestamp>,
    /// Ending customer balance after the invoice is finalized.
    /// Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice.
    /// If the invoice has not been finalized yet, this will be null.
    pub ending_balance: Option<i64>,
    /// Footer displayed on the invoice.
    pub footer: Option<String>,
    /// Details of the invoice that was cloned.
    /// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    pub from_invoice: Option<stripe_shared::InvoicesFromInvoice>,
    /// The URL for the hosted invoice page, which allows customers to view and pay an invoice.
    /// If the invoice has not been finalized yet, this will be null.
    pub hosted_invoice_url: Option<String>,
    /// Unique identifier for the object.
    /// This property is always present unless the invoice is an upcoming invoice.
    /// See [Retrieve an upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) for more details.
    pub id: Option<stripe_shared::InvoiceId>,
    /// The link to download the PDF for the invoice.
    /// If the invoice has not been finalized yet, this will be null.
    pub invoice_pdf: Option<String>,
    /// The error encountered during the previous attempt to finalize the invoice.
    /// This field is cleared when the invoice is successfully finalized.
    pub last_finalization_error: Option<Box<stripe_shared::ApiErrors>>,
    /// The ID of the most recent non-draft revision of this invoice
    pub latest_revision: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    /// The individual line items that make up the invoice.
    /// `lines` is sorted as follows: (1) pending invoice items (including prorations) in reverse chronological order, (2) subscription items in reverse chronological order, and (3) invoice items added after invoice creation in chronological order.
    pub lines: stripe_types::List<stripe_shared::InvoiceLineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The time at which payment will next be attempted.
    /// This value will be `null` for invoices where `collection_method=send_invoice`.
    pub next_payment_attempt: Option<stripe_types::Timestamp>,
    /// A unique, identifying string that appears on emails sent to the customer for this invoice.
    /// This starts with the customer's unique invoice_prefix if it is specified.
    pub number: Option<String>,
    /// The account (if any) for which the funds of the invoice payment are intended.
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// Whether payment was successfully collected for this invoice.
    /// An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance.
    pub paid: bool,
    /// Returns true if the invoice was manually marked paid, returns false if the invoice hasn't been paid yet or was paid on Stripe.
    pub paid_out_of_band: bool,
    /// The PaymentIntent associated with this invoice.
    /// The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice.
    /// Note that voiding an invoice will cancel the PaymentIntent.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    pub payment_settings: stripe_shared::InvoicesPaymentSettings,
    /// End of the usage period during which invoice items were added to this invoice.
    pub period_end: stripe_types::Timestamp,
    /// Start of the usage period during which invoice items were added to this invoice.
    pub period_start: stripe_types::Timestamp,
    /// Total amount of all post-payment credit notes issued for this invoice.
    pub post_payment_credit_notes_amount: i64,
    /// Total amount of all pre-payment credit notes issued for this invoice.
    pub pre_payment_credit_notes_amount: i64,
    /// The quote this invoice was generated from.
    pub quote: Option<stripe_types::Expandable<stripe_shared::Quote>>,
    /// This is the transaction number that appears on email receipts sent for this invoice.
    pub receipt_number: Option<String>,
    /// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
    pub rendering: Option<stripe_shared::InvoicesInvoiceRendering>,
    /// This is a legacy field that will be removed soon.
    /// For details about `rendering_options`, refer to `rendering` instead.
    /// Options for invoice PDF rendering.
    pub rendering_options: Option<stripe_shared::InvoiceSettingRenderingOptions>,
    /// The details of the cost of shipping, including the ShippingRate applied on the invoice.
    pub shipping_cost: Option<stripe_shared::InvoicesShippingCost>,
    /// Shipping details for the invoice.
    /// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
    pub shipping_details: Option<stripe_shared::Shipping>,
    /// Starting customer balance before the invoice is finalized.
    /// If the invoice has not been finalized yet, this will be the current customer balance.
    /// For revision invoices, this also includes any customer balance that was applied to the original invoice.
    pub starting_balance: i64,
    /// Extra information about an invoice for the customer's credit card statement.
    pub statement_descriptor: Option<String>,
    /// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
    /// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    pub status: Option<stripe_shared::InvoiceStatus>,
    pub status_transitions: stripe_shared::InvoicesStatusTransitions,
    /// The subscription that this invoice was prepared for, if any.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// Details about the subscription that created this invoice.
    pub subscription_details: Option<stripe_shared::SubscriptionDetailsData>,
    /// Only set for upcoming invoices that preview prorations. The time used to calculate prorations.
    pub subscription_proration_date: Option<stripe_types::Timestamp>,
    /// Total of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusive tax is applied.
    /// Item discounts are already incorporated.
    pub subtotal: i64,
    /// The integer amount in cents (or local equivalent) representing the subtotal of the invoice before any invoice level discount or tax is applied.
    /// Item discounts are already incorporated.
    pub subtotal_excluding_tax: Option<i64>,
    /// The amount of tax on this invoice. This is the sum of all the tax amounts on this invoice.
    pub tax: Option<i64>,
    /// ID of the test clock this invoice belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
    pub threshold_reason: Option<stripe_shared::InvoiceThresholdReason>,
    /// Total after discounts and taxes.
    pub total: i64,
    /// The aggregate amounts calculated per discount across all line items.
    pub total_discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
    /// The integer amount in cents (or local equivalent) representing the total amount of the invoice including all discounts but excluding all tax.
    pub total_excluding_tax: Option<i64>,
    /// The aggregate amounts calculated per tax rate for all line items.
    pub total_tax_amounts: Vec<stripe_shared::InvoiceTaxAmount>,
    /// The account (if any) the payment will be attributed to for tax reporting, and where funds from the payment will be transferred to for the invoice.
    pub transfer_data: Option<stripe_shared::InvoiceTransferData>,
    /// Invoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts have [been exhausted](https://stripe.com/docs/billing/webhooks#understand).
    /// This field tracks the time when webhooks for this invoice were successfully delivered.
    /// If the invoice had no webhooks to deliver, this will be set while the invoice is being created.
    pub webhooks_delivered_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceBuilder {
    account_country: Option<Option<String>>,
    account_name: Option<Option<String>>,
    account_tax_ids: Option<Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>>,
    amount_due: Option<i64>,
    amount_paid: Option<i64>,
    amount_remaining: Option<i64>,
    amount_shipping: Option<i64>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    application_fee_amount: Option<Option<i64>>,
    attempt_count: Option<u64>,
    attempted: Option<bool>,
    auto_advance: Option<Option<bool>>,
    automatic_tax: Option<stripe_shared::AutomaticTax>,
    billing_reason: Option<Option<InvoiceBillingReason>>,
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    collection_method: Option<stripe_shared::InvoiceCollectionMethod>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    custom_fields: Option<Option<Vec<stripe_shared::InvoiceSettingCustomField>>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_address: Option<Option<stripe_shared::Address>>,
    customer_email: Option<Option<String>>,
    customer_name: Option<Option<String>>,
    customer_phone: Option<Option<String>>,
    customer_shipping: Option<Option<stripe_shared::Shipping>>,
    customer_tax_exempt: Option<Option<InvoiceCustomerTaxExempt>>,
    customer_tax_ids: Option<Option<Vec<stripe_shared::InvoicesResourceInvoiceTaxId>>>,
    default_payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    default_source: Option<Option<stripe_types::Expandable<stripe_shared::PaymentSource>>>,
    default_tax_rates: Option<Vec<stripe_shared::TaxRate>>,
    description: Option<Option<String>>,
    discount: Option<Option<stripe_shared::Discount>>,
    discounts: Option<Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>>,
    due_date: Option<Option<stripe_types::Timestamp>>,
    effective_at: Option<Option<stripe_types::Timestamp>>,
    ending_balance: Option<Option<i64>>,
    footer: Option<Option<String>>,
    from_invoice: Option<Option<stripe_shared::InvoicesFromInvoice>>,
    hosted_invoice_url: Option<Option<String>>,
    id: Option<Option<stripe_shared::InvoiceId>>,
    invoice_pdf: Option<Option<String>>,
    last_finalization_error: Option<Option<Box<stripe_shared::ApiErrors>>>,
    latest_revision: Option<Option<stripe_types::Expandable<stripe_shared::Invoice>>>,
    lines: Option<stripe_types::List<stripe_shared::InvoiceLineItem>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    next_payment_attempt: Option<Option<stripe_types::Timestamp>>,
    number: Option<Option<String>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    paid: Option<bool>,
    paid_out_of_band: Option<bool>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    payment_settings: Option<stripe_shared::InvoicesPaymentSettings>,
    period_end: Option<stripe_types::Timestamp>,
    period_start: Option<stripe_types::Timestamp>,
    post_payment_credit_notes_amount: Option<i64>,
    pre_payment_credit_notes_amount: Option<i64>,
    quote: Option<Option<stripe_types::Expandable<stripe_shared::Quote>>>,
    receipt_number: Option<Option<String>>,
    rendering: Option<Option<stripe_shared::InvoicesInvoiceRendering>>,
    rendering_options: Option<Option<stripe_shared::InvoiceSettingRenderingOptions>>,
    shipping_cost: Option<Option<stripe_shared::InvoicesShippingCost>>,
    shipping_details: Option<Option<stripe_shared::Shipping>>,
    starting_balance: Option<i64>,
    statement_descriptor: Option<Option<String>>,
    status: Option<Option<stripe_shared::InvoiceStatus>>,
    status_transitions: Option<stripe_shared::InvoicesStatusTransitions>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    subscription_details: Option<Option<stripe_shared::SubscriptionDetailsData>>,
    subscription_proration_date: Option<Option<stripe_types::Timestamp>>,
    subtotal: Option<i64>,
    subtotal_excluding_tax: Option<Option<i64>>,
    tax: Option<Option<i64>>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
    threshold_reason: Option<Option<stripe_shared::InvoiceThresholdReason>>,
    total: Option<i64>,
    total_discount_amounts: Option<Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>>,
    total_excluding_tax: Option<Option<i64>>,
    total_tax_amounts: Option<Vec<stripe_shared::InvoiceTaxAmount>>,
    transfer_data: Option<Option<stripe_shared::InvoiceTransferData>>,
    webhooks_delivered_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Invoice {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Invoice>,
        builder: InvoiceBuilder,
    }

    impl Visitor for Place<Invoice> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceBuilder {
        type Out = Invoice;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "account_country" => Ok(Deserialize::begin(&mut self.account_country)),
                "account_name" => Ok(Deserialize::begin(&mut self.account_name)),
                "account_tax_ids" => Ok(Deserialize::begin(&mut self.account_tax_ids)),
                "amount_due" => Ok(Deserialize::begin(&mut self.amount_due)),
                "amount_paid" => Ok(Deserialize::begin(&mut self.amount_paid)),
                "amount_remaining" => Ok(Deserialize::begin(&mut self.amount_remaining)),
                "amount_shipping" => Ok(Deserialize::begin(&mut self.amount_shipping)),
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "application_fee_amount" => Ok(Deserialize::begin(&mut self.application_fee_amount)),
                "attempt_count" => Ok(Deserialize::begin(&mut self.attempt_count)),
                "attempted" => Ok(Deserialize::begin(&mut self.attempted)),
                "auto_advance" => Ok(Deserialize::begin(&mut self.auto_advance)),
                "automatic_tax" => Ok(Deserialize::begin(&mut self.automatic_tax)),
                "billing_reason" => Ok(Deserialize::begin(&mut self.billing_reason)),
                "charge" => Ok(Deserialize::begin(&mut self.charge)),
                "collection_method" => Ok(Deserialize::begin(&mut self.collection_method)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "custom_fields" => Ok(Deserialize::begin(&mut self.custom_fields)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "customer_address" => Ok(Deserialize::begin(&mut self.customer_address)),
                "customer_email" => Ok(Deserialize::begin(&mut self.customer_email)),
                "customer_name" => Ok(Deserialize::begin(&mut self.customer_name)),
                "customer_phone" => Ok(Deserialize::begin(&mut self.customer_phone)),
                "customer_shipping" => Ok(Deserialize::begin(&mut self.customer_shipping)),
                "customer_tax_exempt" => Ok(Deserialize::begin(&mut self.customer_tax_exempt)),
                "customer_tax_ids" => Ok(Deserialize::begin(&mut self.customer_tax_ids)),
                "default_payment_method" => Ok(Deserialize::begin(&mut self.default_payment_method)),
                "default_source" => Ok(Deserialize::begin(&mut self.default_source)),
                "default_tax_rates" => Ok(Deserialize::begin(&mut self.default_tax_rates)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "discount" => Ok(Deserialize::begin(&mut self.discount)),
                "discounts" => Ok(Deserialize::begin(&mut self.discounts)),
                "due_date" => Ok(Deserialize::begin(&mut self.due_date)),
                "effective_at" => Ok(Deserialize::begin(&mut self.effective_at)),
                "ending_balance" => Ok(Deserialize::begin(&mut self.ending_balance)),
                "footer" => Ok(Deserialize::begin(&mut self.footer)),
                "from_invoice" => Ok(Deserialize::begin(&mut self.from_invoice)),
                "hosted_invoice_url" => Ok(Deserialize::begin(&mut self.hosted_invoice_url)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "invoice_pdf" => Ok(Deserialize::begin(&mut self.invoice_pdf)),
                "last_finalization_error" => Ok(Deserialize::begin(&mut self.last_finalization_error)),
                "latest_revision" => Ok(Deserialize::begin(&mut self.latest_revision)),
                "lines" => Ok(Deserialize::begin(&mut self.lines)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "next_payment_attempt" => Ok(Deserialize::begin(&mut self.next_payment_attempt)),
                "number" => Ok(Deserialize::begin(&mut self.number)),
                "on_behalf_of" => Ok(Deserialize::begin(&mut self.on_behalf_of)),
                "paid" => Ok(Deserialize::begin(&mut self.paid)),
                "paid_out_of_band" => Ok(Deserialize::begin(&mut self.paid_out_of_band)),
                "payment_intent" => Ok(Deserialize::begin(&mut self.payment_intent)),
                "payment_settings" => Ok(Deserialize::begin(&mut self.payment_settings)),
                "period_end" => Ok(Deserialize::begin(&mut self.period_end)),
                "period_start" => Ok(Deserialize::begin(&mut self.period_start)),
                "post_payment_credit_notes_amount" => Ok(Deserialize::begin(&mut self.post_payment_credit_notes_amount)),
                "pre_payment_credit_notes_amount" => Ok(Deserialize::begin(&mut self.pre_payment_credit_notes_amount)),
                "quote" => Ok(Deserialize::begin(&mut self.quote)),
                "receipt_number" => Ok(Deserialize::begin(&mut self.receipt_number)),
                "rendering" => Ok(Deserialize::begin(&mut self.rendering)),
                "rendering_options" => Ok(Deserialize::begin(&mut self.rendering_options)),
                "shipping_cost" => Ok(Deserialize::begin(&mut self.shipping_cost)),
                "shipping_details" => Ok(Deserialize::begin(&mut self.shipping_details)),
                "starting_balance" => Ok(Deserialize::begin(&mut self.starting_balance)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "status_transitions" => Ok(Deserialize::begin(&mut self.status_transitions)),
                "subscription" => Ok(Deserialize::begin(&mut self.subscription)),
                "subscription_details" => Ok(Deserialize::begin(&mut self.subscription_details)),
                "subscription_proration_date" => Ok(Deserialize::begin(&mut self.subscription_proration_date)),
                "subtotal" => Ok(Deserialize::begin(&mut self.subtotal)),
                "subtotal_excluding_tax" => Ok(Deserialize::begin(&mut self.subtotal_excluding_tax)),
                "tax" => Ok(Deserialize::begin(&mut self.tax)),
                "test_clock" => Ok(Deserialize::begin(&mut self.test_clock)),
                "threshold_reason" => Ok(Deserialize::begin(&mut self.threshold_reason)),
                "total" => Ok(Deserialize::begin(&mut self.total)),
                "total_discount_amounts" => Ok(Deserialize::begin(&mut self.total_discount_amounts)),
                "total_excluding_tax" => Ok(Deserialize::begin(&mut self.total_excluding_tax)),
                "total_tax_amounts" => Ok(Deserialize::begin(&mut self.total_tax_amounts)),
                "transfer_data" => Ok(Deserialize::begin(&mut self.transfer_data)),
                "webhooks_delivered_at" => Ok(Deserialize::begin(&mut self.webhooks_delivered_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account_country: Deserialize::default(),
                account_name: Deserialize::default(),
                account_tax_ids: Deserialize::default(),
                amount_due: Deserialize::default(),
                amount_paid: Deserialize::default(),
                amount_remaining: Deserialize::default(),
                amount_shipping: Deserialize::default(),
                application: Deserialize::default(),
                application_fee_amount: Deserialize::default(),
                attempt_count: Deserialize::default(),
                attempted: Deserialize::default(),
                auto_advance: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                billing_reason: Deserialize::default(),
                charge: Deserialize::default(),
                collection_method: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                custom_fields: Deserialize::default(),
                customer: Deserialize::default(),
                customer_address: Deserialize::default(),
                customer_email: Deserialize::default(),
                customer_name: Deserialize::default(),
                customer_phone: Deserialize::default(),
                customer_shipping: Deserialize::default(),
                customer_tax_exempt: Deserialize::default(),
                customer_tax_ids: Deserialize::default(),
                default_payment_method: Deserialize::default(),
                default_source: Deserialize::default(),
                default_tax_rates: Deserialize::default(),
                description: Deserialize::default(),
                discount: Deserialize::default(),
                discounts: Deserialize::default(),
                due_date: Deserialize::default(),
                effective_at: Deserialize::default(),
                ending_balance: Deserialize::default(),
                footer: Deserialize::default(),
                from_invoice: Deserialize::default(),
                hosted_invoice_url: Deserialize::default(),
                id: Deserialize::default(),
                invoice_pdf: Deserialize::default(),
                last_finalization_error: Deserialize::default(),
                latest_revision: Deserialize::default(),
                lines: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                next_payment_attempt: Deserialize::default(),
                number: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                paid: Deserialize::default(),
                paid_out_of_band: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_settings: Deserialize::default(),
                period_end: Deserialize::default(),
                period_start: Deserialize::default(),
                post_payment_credit_notes_amount: Deserialize::default(),
                pre_payment_credit_notes_amount: Deserialize::default(),
                quote: Deserialize::default(),
                receipt_number: Deserialize::default(),
                rendering: Deserialize::default(),
                rendering_options: Deserialize::default(),
                shipping_cost: Deserialize::default(),
                shipping_details: Deserialize::default(),
                starting_balance: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
                subscription: Deserialize::default(),
                subscription_details: Deserialize::default(),
                subscription_proration_date: Deserialize::default(),
                subtotal: Deserialize::default(),
                subtotal_excluding_tax: Deserialize::default(),
                tax: Deserialize::default(),
                test_clock: Deserialize::default(),
                threshold_reason: Deserialize::default(),
                total: Deserialize::default(),
                total_discount_amounts: Deserialize::default(),
                total_excluding_tax: Deserialize::default(),
                total_tax_amounts: Deserialize::default(),
                transfer_data: Deserialize::default(),
                webhooks_delivered_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_country = self.account_country.take()?;
            let account_name = self.account_name.take()?;
            let account_tax_ids = self.account_tax_ids.take()?;
            let amount_due = self.amount_due.take()?;
            let amount_paid = self.amount_paid.take()?;
            let amount_remaining = self.amount_remaining.take()?;
            let amount_shipping = self.amount_shipping.take()?;
            let application = self.application.take()?;
            let application_fee_amount = self.application_fee_amount.take()?;
            let attempt_count = self.attempt_count.take()?;
            let attempted = self.attempted.take()?;
            let auto_advance = self.auto_advance.take()?;
            let automatic_tax = self.automatic_tax.take()?;
            let billing_reason = self.billing_reason.take()?;
            let charge = self.charge.take()?;
            let collection_method = self.collection_method.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let custom_fields = self.custom_fields.take()?;
            let customer = self.customer.take()?;
            let customer_address = self.customer_address.take()?;
            let customer_email = self.customer_email.take()?;
            let customer_name = self.customer_name.take()?;
            let customer_phone = self.customer_phone.take()?;
            let customer_shipping = self.customer_shipping.take()?;
            let customer_tax_exempt = self.customer_tax_exempt.take()?;
            let customer_tax_ids = self.customer_tax_ids.take()?;
            let default_payment_method = self.default_payment_method.take()?;
            let default_source = self.default_source.take()?;
            let default_tax_rates = self.default_tax_rates.take()?;
            let description = self.description.take()?;
            let discount = self.discount.take()?;
            let discounts = self.discounts.take()?;
            let due_date = self.due_date.take()?;
            let effective_at = self.effective_at.take()?;
            let ending_balance = self.ending_balance.take()?;
            let footer = self.footer.take()?;
            let from_invoice = self.from_invoice.take()?;
            let hosted_invoice_url = self.hosted_invoice_url.take()?;
            let id = self.id.take()?;
            let invoice_pdf = self.invoice_pdf.take()?;
            let last_finalization_error = self.last_finalization_error.take()?;
            let latest_revision = self.latest_revision.take()?;
            let lines = self.lines.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let next_payment_attempt = self.next_payment_attempt.take()?;
            let number = self.number.take()?;
            let on_behalf_of = self.on_behalf_of.take()?;
            let paid = self.paid.take()?;
            let paid_out_of_band = self.paid_out_of_band.take()?;
            let payment_intent = self.payment_intent.take()?;
            let payment_settings = self.payment_settings.take()?;
            let period_end = self.period_end.take()?;
            let period_start = self.period_start.take()?;
            let post_payment_credit_notes_amount = self.post_payment_credit_notes_amount.take()?;
            let pre_payment_credit_notes_amount = self.pre_payment_credit_notes_amount.take()?;
            let quote = self.quote.take()?;
            let receipt_number = self.receipt_number.take()?;
            let rendering = self.rendering.take()?;
            let rendering_options = self.rendering_options.take()?;
            let shipping_cost = self.shipping_cost.take()?;
            let shipping_details = self.shipping_details.take()?;
            let starting_balance = self.starting_balance.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;
            let status = self.status.take()?;
            let status_transitions = self.status_transitions.take()?;
            let subscription = self.subscription.take()?;
            let subscription_details = self.subscription_details.take()?;
            let subscription_proration_date = self.subscription_proration_date.take()?;
            let subtotal = self.subtotal.take()?;
            let subtotal_excluding_tax = self.subtotal_excluding_tax.take()?;
            let tax = self.tax.take()?;
            let test_clock = self.test_clock.take()?;
            let threshold_reason = self.threshold_reason.take()?;
            let total = self.total.take()?;
            let total_discount_amounts = self.total_discount_amounts.take()?;
            let total_excluding_tax = self.total_excluding_tax.take()?;
            let total_tax_amounts = self.total_tax_amounts.take()?;
            let transfer_data = self.transfer_data.take()?;
            let webhooks_delivered_at = self.webhooks_delivered_at.take()?;

            Some(Self::Out {
                account_country,
                account_name,
                account_tax_ids,
                amount_due,
                amount_paid,
                amount_remaining,
                amount_shipping,
                application,
                application_fee_amount,
                attempt_count,
                attempted,
                auto_advance,
                automatic_tax,
                billing_reason,
                charge,
                collection_method,
                created,
                currency,
                custom_fields,
                customer,
                customer_address,
                customer_email,
                customer_name,
                customer_phone,
                customer_shipping,
                customer_tax_exempt,
                customer_tax_ids,
                default_payment_method,
                default_source,
                default_tax_rates,
                description,
                discount,
                discounts,
                due_date,
                effective_at,
                ending_balance,
                footer,
                from_invoice,
                hosted_invoice_url,
                id,
                invoice_pdf,
                last_finalization_error,
                latest_revision,
                lines,
                livemode,
                metadata,
                next_payment_attempt,
                number,
                on_behalf_of,
                paid,
                paid_out_of_band,
                payment_intent,
                payment_settings,
                period_end,
                period_start,
                post_payment_credit_notes_amount,
                pre_payment_credit_notes_amount,
                quote,
                receipt_number,
                rendering,
                rendering_options,
                shipping_cost,
                shipping_details,
                starting_balance,
                statement_descriptor,
                status,
                status_transitions,
                subscription,
                subscription_details,
                subscription_proration_date,
                subtotal,
                subtotal_excluding_tax,
                tax,
                test_clock,
                threshold_reason,
                total,
                total_discount_amounts,
                total_excluding_tax,
                total_tax_amounts,
                transfer_data,
                webhooks_delivered_at,
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

    impl ObjectDeser for Invoice {
        type Builder = InvoiceBuilder;
    }
};
/// Indicates the reason why the invoice was created.
///
/// * `manual`: Unrelated to a subscription, for example, created via the invoice editor.
/// * `subscription`: No longer in use.
/// Applies to subscriptions from before May 2018 where no distinction was made between updates, cycles, and thresholds.
/// * `subscription_create`: A new subscription was created.
/// * `subscription_cycle`: A subscription advanced into a new period.
/// * `subscription_threshold`: A subscription reached a billing threshold.
/// * `subscription_update`: A subscription was updated.
/// * `upcoming`: Reserved for simulated invoices, per the upcoming invoice endpoint.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceBillingReason {
    AutomaticPendingInvoiceItemInvoice,
    Manual,
    QuoteAccept,
    Subscription,
    SubscriptionCreate,
    SubscriptionCycle,
    SubscriptionThreshold,
    SubscriptionUpdate,
    Upcoming,
}
impl InvoiceBillingReason {
    pub fn as_str(self) -> &'static str {
        use InvoiceBillingReason::*;
        match self {
            AutomaticPendingInvoiceItemInvoice => "automatic_pending_invoice_item_invoice",
            Manual => "manual",
            QuoteAccept => "quote_accept",
            Subscription => "subscription",
            SubscriptionCreate => "subscription_create",
            SubscriptionCycle => "subscription_cycle",
            SubscriptionThreshold => "subscription_threshold",
            SubscriptionUpdate => "subscription_update",
            Upcoming => "upcoming",
        }
    }
}

impl std::str::FromStr for InvoiceBillingReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceBillingReason::*;
        match s {
            "automatic_pending_invoice_item_invoice" => Ok(AutomaticPendingInvoiceItemInvoice),
            "manual" => Ok(Manual),
            "quote_accept" => Ok(QuoteAccept),
            "subscription" => Ok(Subscription),
            "subscription_create" => Ok(SubscriptionCreate),
            "subscription_cycle" => Ok(SubscriptionCycle),
            "subscription_threshold" => Ok(SubscriptionThreshold),
            "subscription_update" => Ok(SubscriptionUpdate),
            "upcoming" => Ok(Upcoming),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoiceBillingReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoiceBillingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceBillingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceBillingReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceBillingReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoiceBillingReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceBillingReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoiceBillingReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceBillingReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The customer's tax exempt status.
/// Until the invoice is finalized, this field will equal `customer.tax_exempt`.
/// Once the invoice is finalized, this field will no longer be updated.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceCustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}
impl InvoiceCustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        use InvoiceCustomerTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for InvoiceCustomerTaxExempt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceCustomerTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoiceCustomerTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoiceCustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceCustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceCustomerTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceCustomerTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoiceCustomerTaxExempt"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceCustomerTaxExempt {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoiceCustomerTaxExempt> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceCustomerTaxExempt::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Invoice {
    type Id = Option<stripe_shared::InvoiceId>;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(InvoiceId, "in_");
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl InvoiceCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use InvoiceCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for InvoiceCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoiceCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoiceCollectionMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoiceCollectionMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceCollectionMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}
impl InvoiceStatus {
    pub fn as_str(self) -> &'static str {
        use InvoiceStatus::*;
        match self {
            Draft => "draft",
            Open => "open",
            Paid => "paid",
            Uncollectible => "uncollectible",
            Void => "void",
        }
    }
}

impl std::str::FromStr for InvoiceStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceStatus::*;
        match s {
            "draft" => Ok(Draft),
            "open" => Ok(Open),
            "paid" => Ok(Paid),
            "uncollectible" => Ok(Uncollectible),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoiceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoiceStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoiceStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
