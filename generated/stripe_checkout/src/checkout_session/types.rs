/// A Checkout Session represents your customer's session as they pay for
/// one-time purchases or subscriptions through [Checkout](https://stripe.com/docs/payments/checkout)
/// or [Payment Links](https://stripe.com/docs/payments/payment-links). We recommend creating a
/// new Session each time your customer attempts to pay.
///
/// Once payment is successful, the Checkout Session will contain a reference
/// to the [Customer](https://stripe.com/docs/api/customers), and either the successful
/// [PaymentIntent](https://stripe.com/docs/api/payment_intents) or an active
/// [Subscription](https://stripe.com/docs/api/subscriptions).
///
/// You can create a Checkout Session on your server and redirect to its URL
/// to begin Checkout.
///
/// Related guide: [Checkout quickstart](https://stripe.com/docs/checkout/quickstart)
///
/// For more details see <<https://stripe.com/docs/api/checkout/sessions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CheckoutSession {
    /// When set, provides configuration for actions to take if this Checkout Session expires.
    pub after_expiration: Option<stripe_checkout::PaymentPagesCheckoutSessionAfterExpiration>,
    /// Enables user redeemable promotion codes.
    pub allow_promotion_codes: Option<bool>,
    /// Total of all items before discounts or taxes are applied.
    pub amount_subtotal: Option<i64>,
    /// Total of all items after discounts and taxes are applied.
    pub amount_total: Option<i64>,
    pub automatic_tax: stripe_checkout::PaymentPagesCheckoutSessionAutomaticTax,
    /// Describes whether Checkout should collect the customer's billing address.
    pub billing_address_collection: Option<stripe_checkout::CheckoutSessionBillingAddressCollection>,
    /// If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment and return to your website.
    pub cancel_url: Option<String>,
    /// A unique string to reference the Checkout Session. This can be a
    /// customer ID, a cart ID, or similar, and can be used to reconcile the
    /// Session with your internal systems.
    pub client_reference_id: Option<String>,
    /// Client secret to be used when initializing Stripe.js embedded checkout.
    pub client_secret: Option<String>,
    /// Results of `consent_collection` for this session.
    pub consent: Option<stripe_checkout::PaymentPagesCheckoutSessionConsent>,
    /// When set, provides configuration for the Checkout Session to gather active consent from customers.
    pub consent_collection: Option<stripe_checkout::PaymentPagesCheckoutSessionConsentCollection>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// Currency conversion details for automatic currency conversion sessions
    pub currency_conversion: Option<stripe_checkout::PaymentPagesCheckoutSessionCurrencyConversion>,
    /// Collect additional information from your customer using custom fields.
    /// Up to 2 fields are supported.
    pub custom_fields: Vec<stripe_checkout::PaymentPagesCheckoutSessionCustomFields>,
    pub custom_text: stripe_checkout::PaymentPagesCheckoutSessionCustomText,
    /// The ID of the customer for this Session.
    /// For Checkout Sessions in `subscription` mode or Checkout Sessions with `customer_creation` set as `always` in `payment` mode, Checkout.
    /// will create a new customer object based on information provided
    /// during the payment flow unless an existing customer was provided when
    /// the Session was created.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
    pub customer_creation: Option<CheckoutSessionCustomerCreation>,
    /// The customer details including the customer's tax exempt status and the customer's tax IDs.
    /// Only the customer's email is present on Sessions in `setup` mode.
    pub customer_details: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomerDetails>,
    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file. To access information about the customer once the payment flow is
    /// complete, use the `customer` attribute.
    pub customer_email: Option<String>,
    /// The timestamp at which the Checkout Session will expire.
    pub expires_at: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_checkout::CheckoutSessionId,
    /// ID of the invoice created by the Checkout Session, if it exists.
    pub invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    /// Details on the state of invoice creation for the Checkout Session.
    pub invoice_creation: Option<stripe_checkout::PaymentPagesCheckoutSessionInvoiceCreation>,
    /// The line items purchased by the customer.
    pub line_items: Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The IETF language tag of the locale Checkout is displayed in.
    /// If blank or `auto`, the browser's locale is used.
    pub locale: Option<stripe_checkout::CheckoutSessionLocale>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The mode of the Checkout Session.
    pub mode: stripe_checkout::CheckoutSessionMode,
    /// The ID of the PaymentIntent for Checkout Sessions in `payment` mode.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    /// The ID of the Payment Link that created this Session.
    pub payment_link: Option<stripe_types::Expandable<stripe_shared::PaymentLink>>,
    /// Configure whether a Checkout Session should collect a payment method.
    pub payment_method_collection: Option<CheckoutSessionPaymentMethodCollection>,
    /// Information about the payment method configuration used for this Checkout session if using dynamic payment methods.
    pub payment_method_configuration_details: Option<stripe_shared::PaymentMethodConfigBizPaymentMethodConfigurationDetails>,
    /// Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession.
    pub payment_method_options: Option<stripe_checkout::CheckoutSessionPaymentMethodOptions>,
    /// A list of the types of payment methods (e.g. card) this Checkout
    /// Session is allowed to accept.
    pub payment_method_types: Vec<String>,
    /// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
    /// You can use this value to decide when to fulfill your customer's order.
    pub payment_status: CheckoutSessionPaymentStatus,
    pub phone_number_collection: Option<stripe_checkout::PaymentPagesCheckoutSessionPhoneNumberCollection>,
    /// The ID of the original expired Checkout Session that triggered the recovery flow.
    pub recovered_from: Option<String>,
    /// Applies to Checkout Sessions with `ui_mode: embedded`.
    /// By default, Stripe will always redirect to your return_url after a successful confirmation.
    /// If you set `redirect_on_completion: 'if_required'`, then we will only redirect if your user chooses a redirect-based payment method.
    pub redirect_on_completion: Option<stripe_checkout::CheckoutSessionRedirectOnCompletion>,
    /// Applies to Checkout Sessions with `ui_mode: embedded`.
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    pub return_url: Option<String>,
    /// The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    pub setup_intent: Option<stripe_types::Expandable<stripe_shared::SetupIntent>>,
    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    pub shipping_address_collection: Option<stripe_checkout::PaymentPagesCheckoutSessionShippingAddressCollection>,
    /// The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<stripe_checkout::PaymentPagesCheckoutSessionShippingCost>,
    /// Shipping information for this Checkout Session.
    pub shipping_details: Option<stripe_shared::Shipping>,
    /// The shipping rate options applied to this Session.
    pub shipping_options: Vec<stripe_checkout::PaymentPagesCheckoutSessionShippingOption>,
    /// The status of the Checkout Session, one of `open`, `complete`, or `expired`.
    pub status: Option<stripe_checkout::CheckoutSessionStatus>,
    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button. `submit_type` can only be
    /// specified on Checkout Sessions in `payment` mode, but not Checkout Sessions
    /// in `subscription` or `setup` mode.
    pub submit_type: Option<stripe_checkout::CheckoutSessionSubmitType>,
    /// The ID of the subscription for Checkout Sessions in `subscription` mode.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// The URL the customer will be directed to after the payment or
    /// subscription creation is successful.
    pub success_url: Option<String>,
    pub tax_id_collection: Option<stripe_checkout::PaymentPagesCheckoutSessionTaxIdCollection>,
    /// Tax and discount details for the computed total amount.
    pub total_details: Option<stripe_checkout::PaymentPagesCheckoutSessionTotalDetails>,
    /// The UI mode of the Session. Can be `hosted` (default) or `embedded`.
    pub ui_mode: Option<stripe_checkout::CheckoutSessionUiMode>,
    /// The URL to the Checkout Session.
    /// Redirect customers to this URL to take them to Checkout.
    /// If you’re using [Custom Domains](https://stripe.com/docs/payments/checkout/custom-domains), the URL will use your subdomain.
    /// Otherwise, it’ll use `checkout.stripe.com.`.
    /// This value is only present when the session is active.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct CheckoutSessionBuilder {
    after_expiration: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionAfterExpiration>>,
    allow_promotion_codes: Option<Option<bool>>,
    amount_subtotal: Option<Option<i64>>,
    amount_total: Option<Option<i64>>,
    automatic_tax: Option<stripe_checkout::PaymentPagesCheckoutSessionAutomaticTax>,
    billing_address_collection: Option<Option<stripe_checkout::CheckoutSessionBillingAddressCollection>>,
    cancel_url: Option<Option<String>>,
    client_reference_id: Option<Option<String>>,
    client_secret: Option<Option<String>>,
    consent: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionConsent>>,
    consent_collection: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionConsentCollection>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<Option<stripe_types::Currency>>,
    currency_conversion: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCurrencyConversion>>,
    custom_fields: Option<Vec<stripe_checkout::PaymentPagesCheckoutSessionCustomFields>>,
    custom_text: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomText>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_creation: Option<Option<CheckoutSessionCustomerCreation>>,
    customer_details: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomerDetails>>,
    customer_email: Option<Option<String>>,
    expires_at: Option<stripe_types::Timestamp>,
    id: Option<stripe_checkout::CheckoutSessionId>,
    invoice: Option<Option<stripe_types::Expandable<stripe_shared::Invoice>>>,
    invoice_creation: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionInvoiceCreation>>,
    line_items: Option<Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>>,
    livemode: Option<bool>,
    locale: Option<Option<stripe_checkout::CheckoutSessionLocale>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    mode: Option<stripe_checkout::CheckoutSessionMode>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    payment_link: Option<Option<stripe_types::Expandable<stripe_shared::PaymentLink>>>,
    payment_method_collection: Option<Option<CheckoutSessionPaymentMethodCollection>>,
    payment_method_configuration_details: Option<Option<stripe_shared::PaymentMethodConfigBizPaymentMethodConfigurationDetails>>,
    payment_method_options: Option<Option<stripe_checkout::CheckoutSessionPaymentMethodOptions>>,
    payment_method_types: Option<Vec<String>>,
    payment_status: Option<CheckoutSessionPaymentStatus>,
    phone_number_collection: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionPhoneNumberCollection>>,
    recovered_from: Option<Option<String>>,
    redirect_on_completion: Option<Option<stripe_checkout::CheckoutSessionRedirectOnCompletion>>,
    return_url: Option<Option<String>>,
    setup_intent: Option<Option<stripe_types::Expandable<stripe_shared::SetupIntent>>>,
    shipping_address_collection: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionShippingAddressCollection>>,
    shipping_cost: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionShippingCost>>,
    shipping_details: Option<Option<stripe_shared::Shipping>>,
    shipping_options: Option<Vec<stripe_checkout::PaymentPagesCheckoutSessionShippingOption>>,
    status: Option<Option<stripe_checkout::CheckoutSessionStatus>>,
    submit_type: Option<Option<stripe_checkout::CheckoutSessionSubmitType>>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    success_url: Option<Option<String>>,
    tax_id_collection: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionTaxIdCollection>>,
    total_details: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionTotalDetails>>,
    ui_mode: Option<Option<stripe_checkout::CheckoutSessionUiMode>>,
    url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutSession>,
        builder: CheckoutSessionBuilder,
    }

    impl Visitor for Place<CheckoutSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CheckoutSessionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CheckoutSessionBuilder {
        type Out = CheckoutSession;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "after_expiration" => Ok(Deserialize::begin(&mut self.after_expiration)),
                "allow_promotion_codes" => Ok(Deserialize::begin(&mut self.allow_promotion_codes)),
                "amount_subtotal" => Ok(Deserialize::begin(&mut self.amount_subtotal)),
                "amount_total" => Ok(Deserialize::begin(&mut self.amount_total)),
                "automatic_tax" => Ok(Deserialize::begin(&mut self.automatic_tax)),
                "billing_address_collection" => Ok(Deserialize::begin(&mut self.billing_address_collection)),
                "cancel_url" => Ok(Deserialize::begin(&mut self.cancel_url)),
                "client_reference_id" => Ok(Deserialize::begin(&mut self.client_reference_id)),
                "client_secret" => Ok(Deserialize::begin(&mut self.client_secret)),
                "consent" => Ok(Deserialize::begin(&mut self.consent)),
                "consent_collection" => Ok(Deserialize::begin(&mut self.consent_collection)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "currency_conversion" => Ok(Deserialize::begin(&mut self.currency_conversion)),
                "custom_fields" => Ok(Deserialize::begin(&mut self.custom_fields)),
                "custom_text" => Ok(Deserialize::begin(&mut self.custom_text)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "customer_creation" => Ok(Deserialize::begin(&mut self.customer_creation)),
                "customer_details" => Ok(Deserialize::begin(&mut self.customer_details)),
                "customer_email" => Ok(Deserialize::begin(&mut self.customer_email)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "invoice" => Ok(Deserialize::begin(&mut self.invoice)),
                "invoice_creation" => Ok(Deserialize::begin(&mut self.invoice_creation)),
                "line_items" => Ok(Deserialize::begin(&mut self.line_items)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "locale" => Ok(Deserialize::begin(&mut self.locale)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "mode" => Ok(Deserialize::begin(&mut self.mode)),
                "payment_intent" => Ok(Deserialize::begin(&mut self.payment_intent)),
                "payment_link" => Ok(Deserialize::begin(&mut self.payment_link)),
                "payment_method_collection" => Ok(Deserialize::begin(&mut self.payment_method_collection)),
                "payment_method_configuration_details" => Ok(Deserialize::begin(&mut self.payment_method_configuration_details)),
                "payment_method_options" => Ok(Deserialize::begin(&mut self.payment_method_options)),
                "payment_method_types" => Ok(Deserialize::begin(&mut self.payment_method_types)),
                "payment_status" => Ok(Deserialize::begin(&mut self.payment_status)),
                "phone_number_collection" => Ok(Deserialize::begin(&mut self.phone_number_collection)),
                "recovered_from" => Ok(Deserialize::begin(&mut self.recovered_from)),
                "redirect_on_completion" => Ok(Deserialize::begin(&mut self.redirect_on_completion)),
                "return_url" => Ok(Deserialize::begin(&mut self.return_url)),
                "setup_intent" => Ok(Deserialize::begin(&mut self.setup_intent)),
                "shipping_address_collection" => Ok(Deserialize::begin(&mut self.shipping_address_collection)),
                "shipping_cost" => Ok(Deserialize::begin(&mut self.shipping_cost)),
                "shipping_details" => Ok(Deserialize::begin(&mut self.shipping_details)),
                "shipping_options" => Ok(Deserialize::begin(&mut self.shipping_options)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "submit_type" => Ok(Deserialize::begin(&mut self.submit_type)),
                "subscription" => Ok(Deserialize::begin(&mut self.subscription)),
                "success_url" => Ok(Deserialize::begin(&mut self.success_url)),
                "tax_id_collection" => Ok(Deserialize::begin(&mut self.tax_id_collection)),
                "total_details" => Ok(Deserialize::begin(&mut self.total_details)),
                "ui_mode" => Ok(Deserialize::begin(&mut self.ui_mode)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                after_expiration: Deserialize::default(),
                allow_promotion_codes: Deserialize::default(),
                amount_subtotal: Deserialize::default(),
                amount_total: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                billing_address_collection: Deserialize::default(),
                cancel_url: Deserialize::default(),
                client_reference_id: Deserialize::default(),
                client_secret: Deserialize::default(),
                consent: Deserialize::default(),
                consent_collection: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                currency_conversion: Deserialize::default(),
                custom_fields: Deserialize::default(),
                custom_text: Deserialize::default(),
                customer: Deserialize::default(),
                customer_creation: Deserialize::default(),
                customer_details: Deserialize::default(),
                customer_email: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                invoice_creation: Deserialize::default(),
                line_items: Deserialize::default(),
                livemode: Deserialize::default(),
                locale: Deserialize::default(),
                metadata: Deserialize::default(),
                mode: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_link: Deserialize::default(),
                payment_method_collection: Deserialize::default(),
                payment_method_configuration_details: Deserialize::default(),
                payment_method_options: Deserialize::default(),
                payment_method_types: Deserialize::default(),
                payment_status: Deserialize::default(),
                phone_number_collection: Deserialize::default(),
                recovered_from: Deserialize::default(),
                redirect_on_completion: Deserialize::default(),
                return_url: Deserialize::default(),
                setup_intent: Deserialize::default(),
                shipping_address_collection: Deserialize::default(),
                shipping_cost: Deserialize::default(),
                shipping_details: Deserialize::default(),
                shipping_options: Deserialize::default(),
                status: Deserialize::default(),
                submit_type: Deserialize::default(),
                subscription: Deserialize::default(),
                success_url: Deserialize::default(),
                tax_id_collection: Deserialize::default(),
                total_details: Deserialize::default(),
                ui_mode: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let after_expiration = self.after_expiration.take()?;
            let allow_promotion_codes = self.allow_promotion_codes.take()?;
            let amount_subtotal = self.amount_subtotal.take()?;
            let amount_total = self.amount_total.take()?;
            let automatic_tax = self.automatic_tax.take()?;
            let billing_address_collection = self.billing_address_collection.take()?;
            let cancel_url = self.cancel_url.take()?;
            let client_reference_id = self.client_reference_id.take()?;
            let client_secret = self.client_secret.take()?;
            let consent = self.consent.take()?;
            let consent_collection = self.consent_collection.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let currency_conversion = self.currency_conversion.take()?;
            let custom_fields = self.custom_fields.take()?;
            let custom_text = self.custom_text.take()?;
            let customer = self.customer.take()?;
            let customer_creation = self.customer_creation.take()?;
            let customer_details = self.customer_details.take()?;
            let customer_email = self.customer_email.take()?;
            let expires_at = self.expires_at.take()?;
            let id = self.id.take()?;
            let invoice = self.invoice.take()?;
            let invoice_creation = self.invoice_creation.take()?;
            let line_items = self.line_items.take()?;
            let livemode = self.livemode.take()?;
            let locale = self.locale.take()?;
            let metadata = self.metadata.take()?;
            let mode = self.mode.take()?;
            let payment_intent = self.payment_intent.take()?;
            let payment_link = self.payment_link.take()?;
            let payment_method_collection = self.payment_method_collection.take()?;
            let payment_method_configuration_details = self.payment_method_configuration_details.take()?;
            let payment_method_options = self.payment_method_options.take()?;
            let payment_method_types = self.payment_method_types.take()?;
            let payment_status = self.payment_status.take()?;
            let phone_number_collection = self.phone_number_collection.take()?;
            let recovered_from = self.recovered_from.take()?;
            let redirect_on_completion = self.redirect_on_completion.take()?;
            let return_url = self.return_url.take()?;
            let setup_intent = self.setup_intent.take()?;
            let shipping_address_collection = self.shipping_address_collection.take()?;
            let shipping_cost = self.shipping_cost.take()?;
            let shipping_details = self.shipping_details.take()?;
            let shipping_options = self.shipping_options.take()?;
            let status = self.status.take()?;
            let submit_type = self.submit_type.take()?;
            let subscription = self.subscription.take()?;
            let success_url = self.success_url.take()?;
            let tax_id_collection = self.tax_id_collection.take()?;
            let total_details = self.total_details.take()?;
            let ui_mode = self.ui_mode.take()?;
            let url = self.url.take()?;

            Some(Self::Out {
                after_expiration,
                allow_promotion_codes,
                amount_subtotal,
                amount_total,
                automatic_tax,
                billing_address_collection,
                cancel_url,
                client_reference_id,
                client_secret,
                consent,
                consent_collection,
                created,
                currency,
                currency_conversion,
                custom_fields,
                custom_text,
                customer,
                customer_creation,
                customer_details,
                customer_email,
                expires_at,
                id,
                invoice,
                invoice_creation,
                line_items,
                livemode,
                locale,
                metadata,
                mode,
                payment_intent,
                payment_link,
                payment_method_collection,
                payment_method_configuration_details,
                payment_method_options,
                payment_method_types,
                payment_status,
                phone_number_collection,
                recovered_from,
                redirect_on_completion,
                return_url,
                setup_intent,
                shipping_address_collection,
                shipping_cost,
                shipping_details,
                shipping_options,
                status,
                submit_type,
                subscription,
                success_url,
                tax_id_collection,
                total_details,
                ui_mode,
                url,
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

    impl ObjectDeser for CheckoutSession {
        type Builder = CheckoutSessionBuilder;
    }
};
/// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSessionCustomerCreation {
    Always,
    IfRequired,
}
impl CheckoutSessionCustomerCreation {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for CheckoutSessionCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionCustomerCreation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutSessionCustomerCreation"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionCustomerCreation {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionCustomerCreation> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionCustomerCreation::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Configure whether a Checkout Session should collect a payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSessionPaymentMethodCollection {
    Always,
    IfRequired,
}
impl CheckoutSessionPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for CheckoutSessionPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutSessionPaymentMethodCollection"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionPaymentMethodCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionPaymentMethodCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionPaymentMethodCollection::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
/// You can use this value to decide when to fulfill your customer's order.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSessionPaymentStatus {
    NoPaymentRequired,
    Paid,
    Unpaid,
}
impl CheckoutSessionPaymentStatus {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionPaymentStatus::*;
        match self {
            NoPaymentRequired => "no_payment_required",
            Paid => "paid",
            Unpaid => "unpaid",
        }
    }
}

impl std::str::FromStr for CheckoutSessionPaymentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionPaymentStatus::*;
        match s {
            "no_payment_required" => Ok(NoPaymentRequired),
            "paid" => Ok(Paid),
            "unpaid" => Ok(Unpaid),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionPaymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionPaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutSessionPaymentStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionPaymentStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionPaymentStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionPaymentStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for CheckoutSession {
    type Id = stripe_checkout::CheckoutSessionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CheckoutSessionId, "cs_");
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSessionBillingAddressCollection {
    Auto,
    Required,
}
impl CheckoutSessionBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionBillingAddressCollection::*;
        match self {
            Auto => "auto",
            Required => "required",
        }
    }
}

impl std::str::FromStr for CheckoutSessionBillingAddressCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionBillingAddressCollection::*;
        match s {
            "auto" => Ok(Auto),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionBillingAddressCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionBillingAddressCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutSessionBillingAddressCollection"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionBillingAddressCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionBillingAddressCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionBillingAddressCollection::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnMinusGb,
    Es,
    EsMinus419,
    Et,
    Fi,
    Fil,
    Fr,
    FrMinusCa,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    PtMinusBr,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    ZhMinusHk,
    ZhMinusTw,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CheckoutSessionLocale {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionLocale::*;
        match self {
            Auto => "auto",
            Bg => "bg",
            Cs => "cs",
            Da => "da",
            De => "de",
            El => "el",
            En => "en",
            EnMinusGb => "en-GB",
            Es => "es",
            EsMinus419 => "es-419",
            Et => "et",
            Fi => "fi",
            Fil => "fil",
            Fr => "fr",
            FrMinusCa => "fr-CA",
            Hr => "hr",
            Hu => "hu",
            Id => "id",
            It => "it",
            Ja => "ja",
            Ko => "ko",
            Lt => "lt",
            Lv => "lv",
            Ms => "ms",
            Mt => "mt",
            Nb => "nb",
            Nl => "nl",
            Pl => "pl",
            Pt => "pt",
            PtMinusBr => "pt-BR",
            Ro => "ro",
            Ru => "ru",
            Sk => "sk",
            Sl => "sl",
            Sv => "sv",
            Th => "th",
            Tr => "tr",
            Vi => "vi",
            Zh => "zh",
            ZhMinusHk => "zh-HK",
            ZhMinusTw => "zh-TW",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CheckoutSessionLocale {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionLocale::*;
        match s {
            "auto" => Ok(Auto),
            "bg" => Ok(Bg),
            "cs" => Ok(Cs),
            "da" => Ok(Da),
            "de" => Ok(De),
            "el" => Ok(El),
            "en" => Ok(En),
            "en-GB" => Ok(EnMinusGb),
            "es" => Ok(Es),
            "es-419" => Ok(EsMinus419),
            "et" => Ok(Et),
            "fi" => Ok(Fi),
            "fil" => Ok(Fil),
            "fr" => Ok(Fr),
            "fr-CA" => Ok(FrMinusCa),
            "hr" => Ok(Hr),
            "hu" => Ok(Hu),
            "id" => Ok(Id),
            "it" => Ok(It),
            "ja" => Ok(Ja),
            "ko" => Ok(Ko),
            "lt" => Ok(Lt),
            "lv" => Ok(Lv),
            "ms" => Ok(Ms),
            "mt" => Ok(Mt),
            "nb" => Ok(Nb),
            "nl" => Ok(Nl),
            "pl" => Ok(Pl),
            "pt" => Ok(Pt),
            "pt-BR" => Ok(PtMinusBr),
            "ro" => Ok(Ro),
            "ru" => Ok(Ru),
            "sk" => Ok(Sk),
            "sl" => Ok(Sl),
            "sv" => Ok(Sv),
            "th" => Ok(Th),
            "tr" => Ok(Tr),
            "vi" => Ok(Vi),
            "zh" => Ok(Zh),
            "zh-HK" => Ok(ZhMinusHk),
            "zh-TW" => Ok(ZhMinusTw),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionLocale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionLocale {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionLocale> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionLocale::from_str(s).unwrap_or(CheckoutSessionLocale::Unknown));
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSessionMode {
    Payment,
    Setup,
    Subscription,
}
impl CheckoutSessionMode {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionMode::*;
        match self {
            Payment => "payment",
            Setup => "setup",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for CheckoutSessionMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionMode::*;
        match s {
            "payment" => Ok(Payment),
            "setup" => Ok(Setup),
            "subscription" => Ok(Subscription),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutSessionMode"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionMode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSessionRedirectOnCompletion {
    Always,
    IfRequired,
    Never,
}
impl CheckoutSessionRedirectOnCompletion {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionRedirectOnCompletion::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CheckoutSessionRedirectOnCompletion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionRedirectOnCompletion::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionRedirectOnCompletion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionRedirectOnCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionRedirectOnCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionRedirectOnCompletion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionRedirectOnCompletion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutSessionRedirectOnCompletion"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionRedirectOnCompletion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionRedirectOnCompletion> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionRedirectOnCompletion::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSessionStatus {
    Complete,
    Expired,
    Open,
}
impl CheckoutSessionStatus {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionStatus::*;
        match self {
            Complete => "complete",
            Expired => "expired",
            Open => "open",
        }
    }
}

impl std::str::FromStr for CheckoutSessionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionStatus::*;
        match s {
            "complete" => Ok(Complete),
            "expired" => Ok(Expired),
            "open" => Ok(Open),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutSessionStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSessionSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}
impl CheckoutSessionSubmitType {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionSubmitType::*;
        match self {
            Auto => "auto",
            Book => "book",
            Donate => "donate",
            Pay => "pay",
        }
    }
}

impl std::str::FromStr for CheckoutSessionSubmitType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionSubmitType::*;
        match s {
            "auto" => Ok(Auto),
            "book" => Ok(Book),
            "donate" => Ok(Donate),
            "pay" => Ok(Pay),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionSubmitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionSubmitType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutSessionSubmitType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionSubmitType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionSubmitType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionSubmitType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSessionUiMode {
    Embedded,
    Hosted,
}
impl CheckoutSessionUiMode {
    pub fn as_str(self) -> &'static str {
        use CheckoutSessionUiMode::*;
        match self {
            Embedded => "embedded",
            Hosted => "hosted",
        }
    }
}

impl std::str::FromStr for CheckoutSessionUiMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionUiMode::*;
        match s {
            "embedded" => Ok(Embedded),
            "hosted" => Ok(Hosted),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutSessionUiMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutSessionUiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionUiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionUiMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSessionUiMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutSessionUiMode"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutSessionUiMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutSessionUiMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionUiMode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
