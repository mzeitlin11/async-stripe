/// A payment link is a shareable URL that will take your customers to a hosted payment page.
/// A payment link can be shared and used multiple times.
///
/// When a customer opens a payment link it will open a new [checkout session](https://stripe.com/docs/api/checkout/sessions) to render the payment page.
/// You can use [checkout session events](https://stripe.com/docs/api/events/types#event_types-checkout.session.completed) to track payments through payment links.
///
/// Related guide: [Payment Links API](https://stripe.com/docs/payment-links)
///
/// For more details see <<https://stripe.com/docs/api/payment_links/payment_links/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLink {
    /// Whether the payment link's `url` is active.
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub active: bool,
    pub after_completion: stripe_shared::PaymentLinksResourceAfterCompletion,
    /// Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,
    /// The ID of the Connect application that created the Payment Link.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_shared::PaymentLinksResourceAutomaticTax,
    /// Configuration for collecting the customer's billing address.
    pub billing_address_collection: stripe_shared::PaymentLinkBillingAddressCollection,
    /// When set, provides configuration to gather active consent from customers.
    pub consent_collection: Option<stripe_shared::PaymentLinksResourceConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Collect additional information from your customer using custom fields.
    /// Up to 2 fields are supported.
    pub custom_fields: Vec<stripe_shared::PaymentLinksResourceCustomFields>,
    pub custom_text: stripe_shared::PaymentLinksResourceCustomText,
    /// Configuration for Customer creation during checkout.
    pub customer_creation: PaymentLinkCustomerCreation,
    /// Unique identifier for the object.
    pub id: stripe_shared::PaymentLinkId,
    /// Configuration for creating invoice for payment mode payment links.
    pub invoice_creation: Option<stripe_shared::PaymentLinksResourceInvoiceCreation>,
    /// The line items representing what is being sold.
    pub line_items: Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The account on behalf of which to charge.
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// Indicates the parameters to be passed to PaymentIntent creation during checkout.
    pub payment_intent_data: Option<stripe_shared::PaymentLinksResourcePaymentIntentData>,
    /// Configuration for collecting a payment method during checkout.
    pub payment_method_collection: PaymentLinkPaymentMethodCollection,
    /// The list of payment method types that customers can use.
    /// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>,
    pub phone_number_collection: stripe_shared::PaymentLinksResourcePhoneNumberCollection,
    /// Configuration for collecting the customer's shipping address.
    pub shipping_address_collection: Option<stripe_shared::PaymentLinksResourceShippingAddressCollection>,
    /// The shipping rate options applied to the session.
    pub shipping_options: Vec<stripe_shared::PaymentLinksResourceShippingOption>,
    /// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
    pub submit_type: stripe_shared::PaymentLinkSubmitType,
    /// When creating a subscription, the specified configuration data will be used.
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub subscription_data: Option<stripe_shared::PaymentLinksResourceSubscriptionData>,
    pub tax_id_collection: stripe_shared::PaymentLinksResourceTaxIdCollection,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub transfer_data: Option<stripe_shared::PaymentLinksResourceTransferData>,
    /// The public URL that can be shared with customers.
    pub url: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinkBuilder {
    active: Option<bool>,
    after_completion: Option<stripe_shared::PaymentLinksResourceAfterCompletion>,
    allow_promotion_codes: Option<bool>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    application_fee_amount: Option<Option<i64>>,
    application_fee_percent: Option<Option<f64>>,
    automatic_tax: Option<stripe_shared::PaymentLinksResourceAutomaticTax>,
    billing_address_collection: Option<stripe_shared::PaymentLinkBillingAddressCollection>,
    consent_collection: Option<Option<stripe_shared::PaymentLinksResourceConsentCollection>>,
    currency: Option<stripe_types::Currency>,
    custom_fields: Option<Vec<stripe_shared::PaymentLinksResourceCustomFields>>,
    custom_text: Option<stripe_shared::PaymentLinksResourceCustomText>,
    customer_creation: Option<PaymentLinkCustomerCreation>,
    id: Option<stripe_shared::PaymentLinkId>,
    invoice_creation: Option<Option<stripe_shared::PaymentLinksResourceInvoiceCreation>>,
    line_items: Option<Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    payment_intent_data: Option<Option<stripe_shared::PaymentLinksResourcePaymentIntentData>>,
    payment_method_collection: Option<PaymentLinkPaymentMethodCollection>,
    payment_method_types: Option<Option<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>>,
    phone_number_collection: Option<stripe_shared::PaymentLinksResourcePhoneNumberCollection>,
    shipping_address_collection: Option<Option<stripe_shared::PaymentLinksResourceShippingAddressCollection>>,
    shipping_options: Option<Vec<stripe_shared::PaymentLinksResourceShippingOption>>,
    submit_type: Option<stripe_shared::PaymentLinkSubmitType>,
    subscription_data: Option<Option<stripe_shared::PaymentLinksResourceSubscriptionData>>,
    tax_id_collection: Option<stripe_shared::PaymentLinksResourceTaxIdCollection>,
    transfer_data: Option<Option<stripe_shared::PaymentLinksResourceTransferData>>,
    url: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLink>,
        builder: PaymentLinkBuilder,
    }

    impl Visitor for Place<PaymentLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinkBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinkBuilder {
        type Out = PaymentLink;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "active" => Ok(Deserialize::begin(&mut self.active)),
                "after_completion" => Ok(Deserialize::begin(&mut self.after_completion)),
                "allow_promotion_codes" => Ok(Deserialize::begin(&mut self.allow_promotion_codes)),
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "application_fee_amount" => Ok(Deserialize::begin(&mut self.application_fee_amount)),
                "application_fee_percent" => Ok(Deserialize::begin(&mut self.application_fee_percent)),
                "automatic_tax" => Ok(Deserialize::begin(&mut self.automatic_tax)),
                "billing_address_collection" => Ok(Deserialize::begin(&mut self.billing_address_collection)),
                "consent_collection" => Ok(Deserialize::begin(&mut self.consent_collection)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "custom_fields" => Ok(Deserialize::begin(&mut self.custom_fields)),
                "custom_text" => Ok(Deserialize::begin(&mut self.custom_text)),
                "customer_creation" => Ok(Deserialize::begin(&mut self.customer_creation)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "invoice_creation" => Ok(Deserialize::begin(&mut self.invoice_creation)),
                "line_items" => Ok(Deserialize::begin(&mut self.line_items)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "on_behalf_of" => Ok(Deserialize::begin(&mut self.on_behalf_of)),
                "payment_intent_data" => Ok(Deserialize::begin(&mut self.payment_intent_data)),
                "payment_method_collection" => Ok(Deserialize::begin(&mut self.payment_method_collection)),
                "payment_method_types" => Ok(Deserialize::begin(&mut self.payment_method_types)),
                "phone_number_collection" => Ok(Deserialize::begin(&mut self.phone_number_collection)),
                "shipping_address_collection" => Ok(Deserialize::begin(&mut self.shipping_address_collection)),
                "shipping_options" => Ok(Deserialize::begin(&mut self.shipping_options)),
                "submit_type" => Ok(Deserialize::begin(&mut self.submit_type)),
                "subscription_data" => Ok(Deserialize::begin(&mut self.subscription_data)),
                "tax_id_collection" => Ok(Deserialize::begin(&mut self.tax_id_collection)),
                "transfer_data" => Ok(Deserialize::begin(&mut self.transfer_data)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                after_completion: Deserialize::default(),
                allow_promotion_codes: Deserialize::default(),
                application: Deserialize::default(),
                application_fee_amount: Deserialize::default(),
                application_fee_percent: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                billing_address_collection: Deserialize::default(),
                consent_collection: Deserialize::default(),
                currency: Deserialize::default(),
                custom_fields: Deserialize::default(),
                custom_text: Deserialize::default(),
                customer_creation: Deserialize::default(),
                id: Deserialize::default(),
                invoice_creation: Deserialize::default(),
                line_items: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                payment_intent_data: Deserialize::default(),
                payment_method_collection: Deserialize::default(),
                payment_method_types: Deserialize::default(),
                phone_number_collection: Deserialize::default(),
                shipping_address_collection: Deserialize::default(),
                shipping_options: Deserialize::default(),
                submit_type: Deserialize::default(),
                subscription_data: Deserialize::default(),
                tax_id_collection: Deserialize::default(),
                transfer_data: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let active = self.active.take()?;
            let after_completion = self.after_completion.take()?;
            let allow_promotion_codes = self.allow_promotion_codes.take()?;
            let application = self.application.take()?;
            let application_fee_amount = self.application_fee_amount.take()?;
            let application_fee_percent = self.application_fee_percent.take()?;
            let automatic_tax = self.automatic_tax.take()?;
            let billing_address_collection = self.billing_address_collection.take()?;
            let consent_collection = self.consent_collection.take()?;
            let currency = self.currency.take()?;
            let custom_fields = self.custom_fields.take()?;
            let custom_text = self.custom_text.take()?;
            let customer_creation = self.customer_creation.take()?;
            let id = self.id.take()?;
            let invoice_creation = self.invoice_creation.take()?;
            let line_items = self.line_items.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let on_behalf_of = self.on_behalf_of.take()?;
            let payment_intent_data = self.payment_intent_data.take()?;
            let payment_method_collection = self.payment_method_collection.take()?;
            let payment_method_types = self.payment_method_types.take()?;
            let phone_number_collection = self.phone_number_collection.take()?;
            let shipping_address_collection = self.shipping_address_collection.take()?;
            let shipping_options = self.shipping_options.take()?;
            let submit_type = self.submit_type.take()?;
            let subscription_data = self.subscription_data.take()?;
            let tax_id_collection = self.tax_id_collection.take()?;
            let transfer_data = self.transfer_data.take()?;
            let url = self.url.take()?;

            Some(Self::Out {
                active,
                after_completion,
                allow_promotion_codes,
                application,
                application_fee_amount,
                application_fee_percent,
                automatic_tax,
                billing_address_collection,
                consent_collection,
                currency,
                custom_fields,
                custom_text,
                customer_creation,
                id,
                invoice_creation,
                line_items,
                livemode,
                metadata,
                on_behalf_of,
                payment_intent_data,
                payment_method_collection,
                payment_method_types,
                phone_number_collection,
                shipping_address_collection,
                shipping_options,
                submit_type,
                subscription_data,
                tax_id_collection,
                transfer_data,
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

    impl ObjectDeser for PaymentLink {
        type Builder = PaymentLinkBuilder;
    }
};
/// Configuration for Customer creation during checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinkCustomerCreation {
    Always,
    IfRequired,
}
impl PaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for PaymentLinkCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinkCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkCustomerCreation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkCustomerCreation"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkCustomerCreation {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinkCustomerCreation> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkCustomerCreation::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Configuration for collecting a payment method during checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}
impl PaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for PaymentLinkPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinkPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkPaymentMethodCollection"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkPaymentMethodCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinkPaymentMethodCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkPaymentMethodCollection::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for PaymentLink {
    type Id = stripe_shared::PaymentLinkId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PaymentLinkId, "plink_");
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinkBillingAddressCollection {
    Auto,
    Required,
}
impl PaymentLinkBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkBillingAddressCollection::*;
        match self {
            Auto => "auto",
            Required => "required",
        }
    }
}

impl std::str::FromStr for PaymentLinkBillingAddressCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkBillingAddressCollection::*;
        match s {
            "auto" => Ok(Auto),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinkBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkBillingAddressCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkBillingAddressCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkBillingAddressCollection"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkBillingAddressCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinkBillingAddressCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkBillingAddressCollection::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinkPaymentMethodTypes {
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl PaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkPaymentMethodTypes::*;
        match self {
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Klarna => "klarna",
            Konbini => "konbini",
            Link => "link",
            Oxxo => "oxxo",
            P24 => "p24",
            Paynow => "paynow",
            Paypal => "paypal",
            Pix => "pix",
            Promptpay => "promptpay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentLinkPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkPaymentMethodTypes::*;
        match s {
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinkPaymentMethodTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkPaymentMethodTypes::from_str(s).unwrap_or(PaymentLinkPaymentMethodTypes::Unknown));
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinkSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}
impl PaymentLinkSubmitType {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkSubmitType::*;
        match self {
            Auto => "auto",
            Book => "book",
            Donate => "donate",
            Pay => "pay",
        }
    }
}

impl std::str::FromStr for PaymentLinkSubmitType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkSubmitType::*;
        match s {
            "auto" => Ok(Auto),
            "book" => Ok(Book),
            "donate" => Ok(Donate),
            "pay" => Ok(Pay),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinkSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinkSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkSubmitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkSubmitType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkSubmitType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkSubmitType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinkSubmitType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkSubmitType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
