#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPaymentLink<'a> {
    /// Only return payment links that are active or inactive (e.g., pass `false` to list all inactive payment links).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListPaymentLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListPaymentLink<'a> {
    /// Returns a list of your payment links.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::PaymentLink>> {
        client.get_query("/payment_links", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::PaymentLink> {
        stripe::ListPaginator::from_params("/payment_links", self)
    }
}
impl<'a> stripe::PaginationParams for ListPaymentLink<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePaymentLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrievePaymentLink<'a> {
    /// Retrieve a payment link.
    pub fn send(
        &self,
        client: &stripe::Client,
        payment_link: &stripe_types::payment_link::PaymentLinkId,
    ) -> stripe::Response<stripe_types::PaymentLink> {
        client
            .get_query(&format!("/payment_links/{payment_link}", payment_link = payment_link), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListLineItemsPaymentLink<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListLineItemsPaymentLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListLineItemsPaymentLink<'a> {
    /// When retrieving a payment link, there is an includable **line_items** property containing the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn send(
        &self,
        client: &stripe::Client,
        payment_link: &stripe_types::payment_link::PaymentLinkId,
    ) -> stripe::Response<stripe_types::List<stripe_types::LineItem>> {
        client.get_query(
            &format!("/payment_links/{payment_link}/line_items", payment_link = payment_link),
            self,
        )
    }
    pub fn paginate(
        self,
        payment_link: &stripe_types::payment_link::PaymentLinkId,
    ) -> stripe::ListPaginator<stripe_types::LineItem> {
        stripe::ListPaginator::from_params(
            &format!("/payment_links/{payment_link}/line_items", payment_link = payment_link),
            self,
        )
    }
}
impl<'a> stripe::PaginationParams for ListLineItemsPaymentLink<'a> {}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLink<'a> {
    /// Behavior after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<CreatePaymentLinkAfterCompletion<'a>>,
    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// Can only be applied when there are no line items with recurring prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Configuration for automatic tax collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreatePaymentLinkAutomaticTax>,
    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<CreatePaymentLinkBillingAddressCollection>,
    /// Configure fields to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<CreatePaymentLinkConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies) and supported by each line item's price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Collect additional information from your customer using custom fields.
    ///
    /// Up to 2 fields are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CreatePaymentLinkCustomFields<'a>]>,
    /// Display additional text for your customers using custom text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<CreatePaymentLinkCustomText<'a>>,
    /// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<CreatePaymentLinkCustomerCreation>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Generate a post-purchase Invoice for one-time payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<CreatePaymentLinkInvoiceCreation<'a>>,
    /// The line items representing what is being sold.
    ///
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    pub line_items: &'a [CreatePaymentLinkLineItems<'a>],
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<CreatePaymentLinkPaymentIntentData<'a>>,
    /// Specify whether Checkout should collect a payment method.
    ///
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<CreatePaymentLinkPaymentMethodCollection>,
    /// The list of payment method types that customers can use.
    ///
    /// If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [CreatePaymentLinkPaymentMethodTypes]>,
    /// Controls phone number collection settings during checkout.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<CreatePaymentLinkPhoneNumberCollection>,
    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<CreatePaymentLinkShippingAddressCollection<'a>>,
    /// The shipping rate options to apply to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<&'a [CreatePaymentLinkShippingOptions<'a>]>,
    /// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
    ///
    /// Changing this value will also affect the hostname in the [url](https://stripe.com/docs/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CreatePaymentLinkSubmitType>,
    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreatePaymentLinkSubscriptionData<'a>>,
    /// Controls tax ID collection during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<CreatePaymentLinkTaxIdCollection>,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreatePaymentLinkTransferData<'a>>,
}
impl<'a> CreatePaymentLink<'a> {
    pub fn new(line_items: &'a [CreatePaymentLinkLineItems<'a>]) -> Self {
        Self {
            after_completion: Default::default(),
            allow_promotion_codes: Default::default(),
            application_fee_amount: Default::default(),
            application_fee_percent: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            consent_collection: Default::default(),
            currency: Default::default(),
            custom_fields: Default::default(),
            custom_text: Default::default(),
            customer_creation: Default::default(),
            expand: Default::default(),
            invoice_creation: Default::default(),
            line_items,
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            payment_intent_data: Default::default(),
            payment_method_collection: Default::default(),
            payment_method_types: Default::default(),
            phone_number_collection: Default::default(),
            shipping_address_collection: Default::default(),
            shipping_options: Default::default(),
            submit_type: Default::default(),
            subscription_data: Default::default(),
            tax_id_collection: Default::default(),
            transfer_data: Default::default(),
        }
    }
}
/// Behavior after the purchase is complete.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkAfterCompletion<'a> {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<CreatePaymentLinkAfterCompletionHostedConfirmation<'a>>,
    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<CreatePaymentLinkAfterCompletionRedirect<'a>>,
    /// The specified behavior after the purchase is complete.
    ///
    /// Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkAfterCompletionType,
}
impl<'a> CreatePaymentLinkAfterCompletion<'a> {
    pub fn new(type_: CreatePaymentLinkAfterCompletionType) -> Self {
        Self { hosted_confirmation: Default::default(), redirect: Default::default(), type_ }
    }
}
/// Configuration when `type=hosted_confirmation`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkAfterCompletionHostedConfirmation<'a> {
    /// A custom message to display to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<&'a str>,
}
impl<'a> CreatePaymentLinkAfterCompletionHostedConfirmation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration when `type=redirect`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkAfterCompletionRedirect<'a> {
    /// The URL the customer will be redirected to after the purchase is complete.
    ///
    /// You can embed `{CHECKOUT_SESSION_ID}` into the URL to have the `id` of the completed [checkout session](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-id) included.
    pub url: &'a str,
}
impl<'a> CreatePaymentLinkAfterCompletionRedirect<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
}
/// The specified behavior after the purchase is complete.
///
/// Either `redirect` or `hosted_confirmation`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl CreatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkAfterCompletionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "redirect" => Ok(Redirect),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configuration for automatic tax collection.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkAutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}
impl CreatePaymentLinkAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Configuration for collecting the customer's billing address.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkBillingAddressCollection {
    Auto,
    Required,
}

impl CreatePaymentLinkBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkBillingAddressCollection::*;
        match self {
            Auto => "auto",
            Required => "required",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkBillingAddressCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkBillingAddressCollection::*;
        match s {
            "auto" => Ok(Auto),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkBillingAddressCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configure fields to gather active consent from customers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkConsentCollection {
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    ///
    /// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
    /// Only available to US merchants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<CreatePaymentLinkConsentCollectionPromotions>,
    /// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
    /// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<CreatePaymentLinkConsentCollectionTermsOfService>,
}
impl CreatePaymentLinkConsentCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If set to `auto`, enables the collection of customer consent for promotional communications.
///
/// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
/// Only available to US merchants.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkConsentCollectionPromotions {
    Auto,
    None,
}

impl CreatePaymentLinkConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkConsentCollectionPromotions::*;
        match self {
            Auto => "auto",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkConsentCollectionPromotions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkConsentCollectionPromotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
/// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkConsentCollectionTermsOfService {
    None,
    Required,
}

impl CreatePaymentLinkConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkConsentCollectionTermsOfService::*;
        match self {
            None => "none",
            Required => "required",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkConsentCollectionTermsOfService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkConsentCollectionTermsOfService::*;
        match s {
            "none" => Ok(None),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Collect additional information from your customer using custom fields.
///
/// Up to 2 fields are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomFields<'a> {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CreatePaymentLinkCustomFieldsDropdown<'a>>,
    /// String of your choice that your integration can use to reconcile this field.
    ///
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: &'a str,
    /// The label for the field, displayed to the customer.
    pub label: CreatePaymentLinkCustomFieldsLabel<'a>,
    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<CreatePaymentLinkCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<CreatePaymentLinkCustomFieldsText>,
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkCustomFieldsType,
}
impl<'a> CreatePaymentLinkCustomFields<'a> {
    pub fn new(
        key: &'a str,
        label: CreatePaymentLinkCustomFieldsLabel<'a>,
        type_: CreatePaymentLinkCustomFieldsType,
    ) -> Self {
        Self {
            dropdown: Default::default(),
            key,
            label,
            numeric: Default::default(),
            optional: Default::default(),
            text: Default::default(),
            type_,
        }
    }
}
/// Configuration for `type=dropdown` fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsDropdown<'a> {
    /// The options available for the customer to select.
    ///
    /// Up to 200 options allowed.
    pub options: &'a [CreatePaymentLinkCustomFieldsDropdownOptions<'a>],
}
impl<'a> CreatePaymentLinkCustomFieldsDropdown<'a> {
    pub fn new(options: &'a [CreatePaymentLinkCustomFieldsDropdownOptions<'a>]) -> Self {
        Self { options }
    }
}
/// The options available for the customer to select.
///
/// Up to 200 options allowed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsDropdownOptions<'a> {
    /// The label for the option, displayed to the customer.
    ///
    /// Up to 100 characters.
    pub label: &'a str,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    ///
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: &'a str,
}
impl<'a> CreatePaymentLinkCustomFieldsDropdownOptions<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self { label, value }
    }
}
/// The label for the field, displayed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsLabel<'a> {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: &'a str,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkCustomFieldsLabelType,
}
impl<'a> CreatePaymentLinkCustomFieldsLabel<'a> {
    pub fn new(custom: &'a str, type_: CreatePaymentLinkCustomFieldsLabelType) -> Self {
        Self { custom, type_ }
    }
}
/// The type of the label.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkCustomFieldsLabelType {
    Custom,
}

impl CreatePaymentLinkCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkCustomFieldsLabelType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkCustomFieldsLabelType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkCustomFieldsLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkCustomFieldsLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configuration for `type=numeric` fields.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl CreatePaymentLinkCustomFieldsNumeric {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for `type=text` fields.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl CreatePaymentLinkCustomFieldsText {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of the field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

impl CreatePaymentLinkCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkCustomFieldsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkCustomFieldsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Display additional text for your customers using custom text.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkCustomText<'a> {
    /// Custom text that should be displayed alongside shipping address collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreatePaymentLinkCustomTextShippingAddress<'a>>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<CreatePaymentLinkCustomTextSubmit<'a>>,
    /// Custom text that should be displayed in place of the default terms of service agreement text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance:
        Option<CreatePaymentLinkCustomTextTermsOfServiceAcceptance<'a>>,
}
impl<'a> CreatePaymentLinkCustomText<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Custom text that should be displayed alongside shipping address collection.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomTextShippingAddress<'a> {
    /// Text may be up to 1200 characters in length.
    pub message: &'a str,
}
impl<'a> CreatePaymentLinkCustomTextShippingAddress<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
/// Custom text that should be displayed alongside the payment confirmation button.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomTextSubmit<'a> {
    /// Text may be up to 1200 characters in length.
    pub message: &'a str,
}
impl<'a> CreatePaymentLinkCustomTextSubmit<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
/// Custom text that should be displayed in place of the default terms of service agreement text.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomTextTermsOfServiceAcceptance<'a> {
    /// Text may be up to 1200 characters in length.
    pub message: &'a str,
}
impl<'a> CreatePaymentLinkCustomTextTermsOfServiceAcceptance<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
/// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkCustomerCreation {
    Always,
    IfRequired,
}

impl CreatePaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Generate a post-purchase Invoice for one-time payments.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreation<'a> {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Invoice PDF configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<CreatePaymentLinkInvoiceCreationInvoiceData<'a>>,
}
impl<'a> CreatePaymentLinkInvoiceCreation<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, invoice_data: Default::default() }
    }
}
/// Invoice PDF configuration.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceData<'a> {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<&'a [&'a str]>,
    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CreatePaymentLinkInvoiceCreationInvoiceDataCustomFields<'a>]>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions>,
}
impl<'a> CreatePaymentLinkInvoiceCreationInvoiceData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default custom fields to be displayed on invoices for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceDataCustomFields<'a> {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: &'a str,
    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: &'a str,
}
impl<'a> CreatePaymentLinkInvoiceCreationInvoiceDataCustomFields<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
}
impl CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
///
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The line items representing what is being sold.
///
/// Each line item represents an item being sold.
/// Up to 20 line items are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkLineItems<'a> {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreatePaymentLinkLineItemsAdjustableQuantity>,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) or [Plan](https://stripe.com/docs/api/plans) object.
    pub price: &'a str,
    /// The quantity of the line item being purchased.
    pub quantity: u64,
}
impl<'a> CreatePaymentLinkLineItems<'a> {
    pub fn new(price: &'a str, quantity: u64) -> Self {
        Self { adjustable_quantity: Default::default(), price, quantity }
    }
}
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative Integer.
    pub enabled: bool,
    /// The maximum quantity the customer can purchase.
    ///
    /// By default this value is 99.
    /// You can specify a value up to 999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity the customer can purchase.
    ///
    /// By default this value is 0.
    /// If there is only one item in the cart then that item's quantity cannot go down to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
impl CreatePaymentLinkLineItemsAdjustableQuantity {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: Default::default(), minimum: Default::default() }
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkPaymentIntentData<'a> {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreatePaymentLinkPaymentIntentDataCaptureMethod>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    ///
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
    ///
    /// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
    ///
    /// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
    ///
    /// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
    ///
    /// If Checkout does not create a Customer, the payment method is not attached to a Customer.
    ///
    /// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.  When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentLinkPaymentIntentDataSetupFutureUsage>,
    /// Extra information about the payment.
    ///
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that's set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
}
impl<'a> CreatePaymentLinkPaymentIntentData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkPaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}

impl CreatePaymentLinkPaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkPaymentIntentDataCaptureMethod::*;
        match self {
            Automatic => "automatic",
            AutomaticAsync => "automatic_async",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkPaymentIntentDataCaptureMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "automatic_async" => Ok(AutomaticAsync),
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
///
/// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
///
/// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
///
/// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
///
/// If Checkout does not create a Customer, the payment method is not attached to a Customer.
///
/// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.  When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}

impl CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkPaymentIntentDataSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkPaymentIntentDataSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Specify whether Checkout should collect a payment method.
///
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}

impl CreatePaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The list of payment method types that customers can use.
///
/// If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support)).
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentLinkPaymentMethodTypes {
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

impl CreatePaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkPaymentMethodTypes::*;
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

impl std::str::FromStr for CreatePaymentLinkPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkPaymentMethodTypes::*;
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

impl AsRef<str> for CreatePaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Controls phone number collection settings during checkout.
///
/// We recommend that you review your privacy policy and check with your legal contacts.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkPhoneNumberCollection {
    /// Set to `true` to enable phone number collection.
    pub enabled: bool,
}
impl CreatePaymentLinkPhoneNumberCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Configuration for collecting the customer's shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkShippingAddressCollection<'a> {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: &'a [CreatePaymentLinkShippingAddressCollectionAllowedCountries],
}
impl<'a> CreatePaymentLinkShippingAddressCollection<'a> {
    pub fn new(
        allowed_countries: &'a [CreatePaymentLinkShippingAddressCollectionAllowedCountries],
    ) -> Self {
        Self { allowed_countries }
    }
}
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
/// shipping locations.
///
/// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    Ac,
    Ad,
    Ae,
    Af,
    Ag,
    Ai,
    Al,
    Am,
    Ao,
    Aq,
    Ar,
    At,
    Au,
    Aw,
    Ax,
    Az,
    Ba,
    Bb,
    Bd,
    Be,
    Bf,
    Bg,
    Bh,
    Bi,
    Bj,
    Bl,
    Bm,
    Bn,
    Bo,
    Bq,
    Br,
    Bs,
    Bt,
    Bv,
    Bw,
    By,
    Bz,
    Ca,
    Cd,
    Cf,
    Cg,
    Ch,
    Ci,
    Ck,
    Cl,
    Cm,
    Cn,
    Co,
    Cr,
    Cv,
    Cw,
    Cy,
    Cz,
    De,
    Dj,
    Dk,
    Dm,
    Do,
    Dz,
    Ec,
    Ee,
    Eg,
    Eh,
    Er,
    Es,
    Et,
    Fi,
    Fj,
    Fk,
    Fo,
    Fr,
    Ga,
    Gb,
    Gd,
    Ge,
    Gf,
    Gg,
    Gh,
    Gi,
    Gl,
    Gm,
    Gn,
    Gp,
    Gq,
    Gr,
    Gs,
    Gt,
    Gu,
    Gw,
    Gy,
    Hk,
    Hn,
    Hr,
    Ht,
    Hu,
    Id,
    Ie,
    Il,
    Im,
    In,
    Io,
    Iq,
    Is,
    It,
    Je,
    Jm,
    Jo,
    Jp,
    Ke,
    Kg,
    Kh,
    Ki,
    Km,
    Kn,
    Kr,
    Kw,
    Ky,
    Kz,
    La,
    Lb,
    Lc,
    Li,
    Lk,
    Lr,
    Ls,
    Lt,
    Lu,
    Lv,
    Ly,
    Ma,
    Mc,
    Md,
    Me,
    Mf,
    Mg,
    Mk,
    Ml,
    Mm,
    Mn,
    Mo,
    Mq,
    Mr,
    Ms,
    Mt,
    Mu,
    Mv,
    Mw,
    Mx,
    My,
    Mz,
    Na,
    Nc,
    Ne,
    Ng,
    Ni,
    Nl,
    No,
    Np,
    Nr,
    Nu,
    Nz,
    Om,
    Pa,
    Pe,
    Pf,
    Pg,
    Ph,
    Pk,
    Pl,
    Pm,
    Pn,
    Pr,
    Ps,
    Pt,
    Py,
    Qa,
    Re,
    Ro,
    Rs,
    Ru,
    Rw,
    Sa,
    Sb,
    Sc,
    Se,
    Sg,
    Sh,
    Si,
    Sj,
    Sk,
    Sl,
    Sm,
    Sn,
    So,
    Sr,
    Ss,
    St,
    Sv,
    Sx,
    Sz,
    Ta,
    Tc,
    Td,
    Tf,
    Tg,
    Th,
    Tj,
    Tk,
    Tl,
    Tm,
    Tn,
    To,
    Tr,
    Tt,
    Tv,
    Tw,
    Tz,
    Ua,
    Ug,
    Us,
    Uy,
    Uz,
    Va,
    Vc,
    Ve,
    Vg,
    Vn,
    Vu,
    Wf,
    Ws,
    Xk,
    Ye,
    Yt,
    Za,
    Zm,
    Zw,
    Zz,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkShippingAddressCollectionAllowedCountries::*;
        match self {
            Ac => "AC",
            Ad => "AD",
            Ae => "AE",
            Af => "AF",
            Ag => "AG",
            Ai => "AI",
            Al => "AL",
            Am => "AM",
            Ao => "AO",
            Aq => "AQ",
            Ar => "AR",
            At => "AT",
            Au => "AU",
            Aw => "AW",
            Ax => "AX",
            Az => "AZ",
            Ba => "BA",
            Bb => "BB",
            Bd => "BD",
            Be => "BE",
            Bf => "BF",
            Bg => "BG",
            Bh => "BH",
            Bi => "BI",
            Bj => "BJ",
            Bl => "BL",
            Bm => "BM",
            Bn => "BN",
            Bo => "BO",
            Bq => "BQ",
            Br => "BR",
            Bs => "BS",
            Bt => "BT",
            Bv => "BV",
            Bw => "BW",
            By => "BY",
            Bz => "BZ",
            Ca => "CA",
            Cd => "CD",
            Cf => "CF",
            Cg => "CG",
            Ch => "CH",
            Ci => "CI",
            Ck => "CK",
            Cl => "CL",
            Cm => "CM",
            Cn => "CN",
            Co => "CO",
            Cr => "CR",
            Cv => "CV",
            Cw => "CW",
            Cy => "CY",
            Cz => "CZ",
            De => "DE",
            Dj => "DJ",
            Dk => "DK",
            Dm => "DM",
            Do => "DO",
            Dz => "DZ",
            Ec => "EC",
            Ee => "EE",
            Eg => "EG",
            Eh => "EH",
            Er => "ER",
            Es => "ES",
            Et => "ET",
            Fi => "FI",
            Fj => "FJ",
            Fk => "FK",
            Fo => "FO",
            Fr => "FR",
            Ga => "GA",
            Gb => "GB",
            Gd => "GD",
            Ge => "GE",
            Gf => "GF",
            Gg => "GG",
            Gh => "GH",
            Gi => "GI",
            Gl => "GL",
            Gm => "GM",
            Gn => "GN",
            Gp => "GP",
            Gq => "GQ",
            Gr => "GR",
            Gs => "GS",
            Gt => "GT",
            Gu => "GU",
            Gw => "GW",
            Gy => "GY",
            Hk => "HK",
            Hn => "HN",
            Hr => "HR",
            Ht => "HT",
            Hu => "HU",
            Id => "ID",
            Ie => "IE",
            Il => "IL",
            Im => "IM",
            In => "IN",
            Io => "IO",
            Iq => "IQ",
            Is => "IS",
            It => "IT",
            Je => "JE",
            Jm => "JM",
            Jo => "JO",
            Jp => "JP",
            Ke => "KE",
            Kg => "KG",
            Kh => "KH",
            Ki => "KI",
            Km => "KM",
            Kn => "KN",
            Kr => "KR",
            Kw => "KW",
            Ky => "KY",
            Kz => "KZ",
            La => "LA",
            Lb => "LB",
            Lc => "LC",
            Li => "LI",
            Lk => "LK",
            Lr => "LR",
            Ls => "LS",
            Lt => "LT",
            Lu => "LU",
            Lv => "LV",
            Ly => "LY",
            Ma => "MA",
            Mc => "MC",
            Md => "MD",
            Me => "ME",
            Mf => "MF",
            Mg => "MG",
            Mk => "MK",
            Ml => "ML",
            Mm => "MM",
            Mn => "MN",
            Mo => "MO",
            Mq => "MQ",
            Mr => "MR",
            Ms => "MS",
            Mt => "MT",
            Mu => "MU",
            Mv => "MV",
            Mw => "MW",
            Mx => "MX",
            My => "MY",
            Mz => "MZ",
            Na => "NA",
            Nc => "NC",
            Ne => "NE",
            Ng => "NG",
            Ni => "NI",
            Nl => "NL",
            No => "NO",
            Np => "NP",
            Nr => "NR",
            Nu => "NU",
            Nz => "NZ",
            Om => "OM",
            Pa => "PA",
            Pe => "PE",
            Pf => "PF",
            Pg => "PG",
            Ph => "PH",
            Pk => "PK",
            Pl => "PL",
            Pm => "PM",
            Pn => "PN",
            Pr => "PR",
            Ps => "PS",
            Pt => "PT",
            Py => "PY",
            Qa => "QA",
            Re => "RE",
            Ro => "RO",
            Rs => "RS",
            Ru => "RU",
            Rw => "RW",
            Sa => "SA",
            Sb => "SB",
            Sc => "SC",
            Se => "SE",
            Sg => "SG",
            Sh => "SH",
            Si => "SI",
            Sj => "SJ",
            Sk => "SK",
            Sl => "SL",
            Sm => "SM",
            Sn => "SN",
            So => "SO",
            Sr => "SR",
            Ss => "SS",
            St => "ST",
            Sv => "SV",
            Sx => "SX",
            Sz => "SZ",
            Ta => "TA",
            Tc => "TC",
            Td => "TD",
            Tf => "TF",
            Tg => "TG",
            Th => "TH",
            Tj => "TJ",
            Tk => "TK",
            Tl => "TL",
            Tm => "TM",
            Tn => "TN",
            To => "TO",
            Tr => "TR",
            Tt => "TT",
            Tv => "TV",
            Tw => "TW",
            Tz => "TZ",
            Ua => "UA",
            Ug => "UG",
            Us => "US",
            Uy => "UY",
            Uz => "UZ",
            Va => "VA",
            Vc => "VC",
            Ve => "VE",
            Vg => "VG",
            Vn => "VN",
            Vu => "VU",
            Wf => "WF",
            Ws => "WS",
            Xk => "XK",
            Ye => "YE",
            Yt => "YT",
            Za => "ZA",
            Zm => "ZM",
            Zw => "ZW",
            Zz => "ZZ",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkShippingAddressCollectionAllowedCountries::*;
        match s {
            "AC" => Ok(Ac),
            "AD" => Ok(Ad),
            "AE" => Ok(Ae),
            "AF" => Ok(Af),
            "AG" => Ok(Ag),
            "AI" => Ok(Ai),
            "AL" => Ok(Al),
            "AM" => Ok(Am),
            "AO" => Ok(Ao),
            "AQ" => Ok(Aq),
            "AR" => Ok(Ar),
            "AT" => Ok(At),
            "AU" => Ok(Au),
            "AW" => Ok(Aw),
            "AX" => Ok(Ax),
            "AZ" => Ok(Az),
            "BA" => Ok(Ba),
            "BB" => Ok(Bb),
            "BD" => Ok(Bd),
            "BE" => Ok(Be),
            "BF" => Ok(Bf),
            "BG" => Ok(Bg),
            "BH" => Ok(Bh),
            "BI" => Ok(Bi),
            "BJ" => Ok(Bj),
            "BL" => Ok(Bl),
            "BM" => Ok(Bm),
            "BN" => Ok(Bn),
            "BO" => Ok(Bo),
            "BQ" => Ok(Bq),
            "BR" => Ok(Br),
            "BS" => Ok(Bs),
            "BT" => Ok(Bt),
            "BV" => Ok(Bv),
            "BW" => Ok(Bw),
            "BY" => Ok(By),
            "BZ" => Ok(Bz),
            "CA" => Ok(Ca),
            "CD" => Ok(Cd),
            "CF" => Ok(Cf),
            "CG" => Ok(Cg),
            "CH" => Ok(Ch),
            "CI" => Ok(Ci),
            "CK" => Ok(Ck),
            "CL" => Ok(Cl),
            "CM" => Ok(Cm),
            "CN" => Ok(Cn),
            "CO" => Ok(Co),
            "CR" => Ok(Cr),
            "CV" => Ok(Cv),
            "CW" => Ok(Cw),
            "CY" => Ok(Cy),
            "CZ" => Ok(Cz),
            "DE" => Ok(De),
            "DJ" => Ok(Dj),
            "DK" => Ok(Dk),
            "DM" => Ok(Dm),
            "DO" => Ok(Do),
            "DZ" => Ok(Dz),
            "EC" => Ok(Ec),
            "EE" => Ok(Ee),
            "EG" => Ok(Eg),
            "EH" => Ok(Eh),
            "ER" => Ok(Er),
            "ES" => Ok(Es),
            "ET" => Ok(Et),
            "FI" => Ok(Fi),
            "FJ" => Ok(Fj),
            "FK" => Ok(Fk),
            "FO" => Ok(Fo),
            "FR" => Ok(Fr),
            "GA" => Ok(Ga),
            "GB" => Ok(Gb),
            "GD" => Ok(Gd),
            "GE" => Ok(Ge),
            "GF" => Ok(Gf),
            "GG" => Ok(Gg),
            "GH" => Ok(Gh),
            "GI" => Ok(Gi),
            "GL" => Ok(Gl),
            "GM" => Ok(Gm),
            "GN" => Ok(Gn),
            "GP" => Ok(Gp),
            "GQ" => Ok(Gq),
            "GR" => Ok(Gr),
            "GS" => Ok(Gs),
            "GT" => Ok(Gt),
            "GU" => Ok(Gu),
            "GW" => Ok(Gw),
            "GY" => Ok(Gy),
            "HK" => Ok(Hk),
            "HN" => Ok(Hn),
            "HR" => Ok(Hr),
            "HT" => Ok(Ht),
            "HU" => Ok(Hu),
            "ID" => Ok(Id),
            "IE" => Ok(Ie),
            "IL" => Ok(Il),
            "IM" => Ok(Im),
            "IN" => Ok(In),
            "IO" => Ok(Io),
            "IQ" => Ok(Iq),
            "IS" => Ok(Is),
            "IT" => Ok(It),
            "JE" => Ok(Je),
            "JM" => Ok(Jm),
            "JO" => Ok(Jo),
            "JP" => Ok(Jp),
            "KE" => Ok(Ke),
            "KG" => Ok(Kg),
            "KH" => Ok(Kh),
            "KI" => Ok(Ki),
            "KM" => Ok(Km),
            "KN" => Ok(Kn),
            "KR" => Ok(Kr),
            "KW" => Ok(Kw),
            "KY" => Ok(Ky),
            "KZ" => Ok(Kz),
            "LA" => Ok(La),
            "LB" => Ok(Lb),
            "LC" => Ok(Lc),
            "LI" => Ok(Li),
            "LK" => Ok(Lk),
            "LR" => Ok(Lr),
            "LS" => Ok(Ls),
            "LT" => Ok(Lt),
            "LU" => Ok(Lu),
            "LV" => Ok(Lv),
            "LY" => Ok(Ly),
            "MA" => Ok(Ma),
            "MC" => Ok(Mc),
            "MD" => Ok(Md),
            "ME" => Ok(Me),
            "MF" => Ok(Mf),
            "MG" => Ok(Mg),
            "MK" => Ok(Mk),
            "ML" => Ok(Ml),
            "MM" => Ok(Mm),
            "MN" => Ok(Mn),
            "MO" => Ok(Mo),
            "MQ" => Ok(Mq),
            "MR" => Ok(Mr),
            "MS" => Ok(Ms),
            "MT" => Ok(Mt),
            "MU" => Ok(Mu),
            "MV" => Ok(Mv),
            "MW" => Ok(Mw),
            "MX" => Ok(Mx),
            "MY" => Ok(My),
            "MZ" => Ok(Mz),
            "NA" => Ok(Na),
            "NC" => Ok(Nc),
            "NE" => Ok(Ne),
            "NG" => Ok(Ng),
            "NI" => Ok(Ni),
            "NL" => Ok(Nl),
            "NO" => Ok(No),
            "NP" => Ok(Np),
            "NR" => Ok(Nr),
            "NU" => Ok(Nu),
            "NZ" => Ok(Nz),
            "OM" => Ok(Om),
            "PA" => Ok(Pa),
            "PE" => Ok(Pe),
            "PF" => Ok(Pf),
            "PG" => Ok(Pg),
            "PH" => Ok(Ph),
            "PK" => Ok(Pk),
            "PL" => Ok(Pl),
            "PM" => Ok(Pm),
            "PN" => Ok(Pn),
            "PR" => Ok(Pr),
            "PS" => Ok(Ps),
            "PT" => Ok(Pt),
            "PY" => Ok(Py),
            "QA" => Ok(Qa),
            "RE" => Ok(Re),
            "RO" => Ok(Ro),
            "RS" => Ok(Rs),
            "RU" => Ok(Ru),
            "RW" => Ok(Rw),
            "SA" => Ok(Sa),
            "SB" => Ok(Sb),
            "SC" => Ok(Sc),
            "SE" => Ok(Se),
            "SG" => Ok(Sg),
            "SH" => Ok(Sh),
            "SI" => Ok(Si),
            "SJ" => Ok(Sj),
            "SK" => Ok(Sk),
            "SL" => Ok(Sl),
            "SM" => Ok(Sm),
            "SN" => Ok(Sn),
            "SO" => Ok(So),
            "SR" => Ok(Sr),
            "SS" => Ok(Ss),
            "ST" => Ok(St),
            "SV" => Ok(Sv),
            "SX" => Ok(Sx),
            "SZ" => Ok(Sz),
            "TA" => Ok(Ta),
            "TC" => Ok(Tc),
            "TD" => Ok(Td),
            "TF" => Ok(Tf),
            "TG" => Ok(Tg),
            "TH" => Ok(Th),
            "TJ" => Ok(Tj),
            "TK" => Ok(Tk),
            "TL" => Ok(Tl),
            "TM" => Ok(Tm),
            "TN" => Ok(Tn),
            "TO" => Ok(To),
            "TR" => Ok(Tr),
            "TT" => Ok(Tt),
            "TV" => Ok(Tv),
            "TW" => Ok(Tw),
            "TZ" => Ok(Tz),
            "UA" => Ok(Ua),
            "UG" => Ok(Ug),
            "US" => Ok(Us),
            "UY" => Ok(Uy),
            "UZ" => Ok(Uz),
            "VA" => Ok(Va),
            "VC" => Ok(Vc),
            "VE" => Ok(Ve),
            "VG" => Ok(Vg),
            "VN" => Ok(Vn),
            "VU" => Ok(Vu),
            "WF" => Ok(Wf),
            "WS" => Ok(Ws),
            "XK" => Ok(Xk),
            "YE" => Ok(Ye),
            "YT" => Ok(Yt),
            "ZA" => Ok(Za),
            "ZM" => Ok(Zm),
            "ZW" => Ok(Zw),
            "ZZ" => Ok(Zz),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The shipping rate options to apply to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkShippingOptions<'a> {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
}
impl<'a> CreatePaymentLinkShippingOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
///
/// Changing this value will also affect the hostname in the [url](https://stripe.com/docs/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

impl CreatePaymentLinkSubmitType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkSubmitType::*;
        match self {
            Auto => "auto",
            Book => "book",
            Donate => "donate",
            Pay => "pay",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkSubmitType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkSubmitType::*;
        match s {
            "auto" => Ok(Auto),
            "book" => Ok(Book),
            "donate" => Ok(Donate),
            "pay" => Ok(Pay),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePaymentLinkSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkSubmitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// When creating a subscription, the specified configuration data will be used.
///
/// There must be at least one line item with a recurring price to use `subscription_data`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionData<'a> {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    ///
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    ///
    /// Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}
impl<'a> CreatePaymentLinkSubscriptionData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls tax ID collection during checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkTaxIdCollection {
    /// Set to `true` to enable tax ID collection.
    pub enabled: bool,
}
impl CreatePaymentLinkTaxIdCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkTransferData<'a> {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If specified, successful charges will be attributed to the destination
    /// account for tax reporting, and the funds from charges will be transferred
    /// to the destination account.
    ///
    /// The ID of the resulting transfer will be returned on the successful charge's `transfer` field.
    pub destination: &'a str,
}
impl<'a> CreatePaymentLinkTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), destination }
    }
}
impl<'a> CreatePaymentLink<'a> {
    /// Creates a payment link.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::PaymentLink> {
        client.send_form("/payment_links", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLink<'a> {
    /// Whether the payment link's `url` is active.
    ///
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Behavior after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<UpdatePaymentLinkAfterCompletion<'a>>,
    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// Configuration for automatic tax collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdatePaymentLinkAutomaticTax>,
    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<UpdatePaymentLinkBillingAddressCollection>,
    /// Collect additional information from your customer using custom fields.
    ///
    /// Up to 2 fields are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [UpdatePaymentLinkCustomFields<'a>]>,
    /// Display additional text for your customers using custom text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<UpdatePaymentLinkCustomText<'a>>,
    /// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<UpdatePaymentLinkCustomerCreation>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Generate a post-purchase Invoice for one-time payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<UpdatePaymentLinkInvoiceCreation<'a>>,
    /// The line items representing what is being sold.
    ///
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<&'a [UpdatePaymentLinkLineItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<UpdatePaymentLinkPaymentIntentData<'a>>,
    /// Specify whether Checkout should collect a payment method.
    ///
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<UpdatePaymentLinkPaymentMethodCollection>,
    /// The list of payment method types that customers can use.
    ///
    /// Pass an empty string to enable dynamic payment methods that use your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [UpdatePaymentLinkPaymentMethodTypes]>,
    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<UpdatePaymentLinkShippingAddressCollection<'a>>,
    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<UpdatePaymentLinkSubscriptionData<'a>>,
}
impl<'a> UpdatePaymentLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Behavior after the purchase is complete.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkAfterCompletion<'a> {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<UpdatePaymentLinkAfterCompletionHostedConfirmation<'a>>,
    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<UpdatePaymentLinkAfterCompletionRedirect<'a>>,
    /// The specified behavior after the purchase is complete.
    ///
    /// Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkAfterCompletionType,
}
impl<'a> UpdatePaymentLinkAfterCompletion<'a> {
    pub fn new(type_: UpdatePaymentLinkAfterCompletionType) -> Self {
        Self { hosted_confirmation: Default::default(), redirect: Default::default(), type_ }
    }
}
/// Configuration when `type=hosted_confirmation`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLinkAfterCompletionHostedConfirmation<'a> {
    /// A custom message to display to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<&'a str>,
}
impl<'a> UpdatePaymentLinkAfterCompletionHostedConfirmation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration when `type=redirect`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkAfterCompletionRedirect<'a> {
    /// The URL the customer will be redirected to after the purchase is complete.
    ///
    /// You can embed `{CHECKOUT_SESSION_ID}` into the URL to have the `id` of the completed [checkout session](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-id) included.
    pub url: &'a str,
}
impl<'a> UpdatePaymentLinkAfterCompletionRedirect<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
}
/// The specified behavior after the purchase is complete.
///
/// Either `redirect` or `hosted_confirmation`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl UpdatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkAfterCompletionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "redirect" => Ok(Redirect),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configuration for automatic tax collection.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkAutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}
impl UpdatePaymentLinkAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Configuration for collecting the customer's billing address.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkBillingAddressCollection {
    Auto,
    Required,
}

impl UpdatePaymentLinkBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkBillingAddressCollection::*;
        match self {
            Auto => "auto",
            Required => "required",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkBillingAddressCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkBillingAddressCollection::*;
        match s {
            "auto" => Ok(Auto),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkBillingAddressCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Collect additional information from your customer using custom fields.
///
/// Up to 2 fields are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFields<'a> {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<UpdatePaymentLinkCustomFieldsDropdown<'a>>,
    /// String of your choice that your integration can use to reconcile this field.
    ///
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: &'a str,
    /// The label for the field, displayed to the customer.
    pub label: UpdatePaymentLinkCustomFieldsLabel<'a>,
    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<UpdatePaymentLinkCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<UpdatePaymentLinkCustomFieldsText>,
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkCustomFieldsType,
}
impl<'a> UpdatePaymentLinkCustomFields<'a> {
    pub fn new(
        key: &'a str,
        label: UpdatePaymentLinkCustomFieldsLabel<'a>,
        type_: UpdatePaymentLinkCustomFieldsType,
    ) -> Self {
        Self {
            dropdown: Default::default(),
            key,
            label,
            numeric: Default::default(),
            optional: Default::default(),
            text: Default::default(),
            type_,
        }
    }
}
/// Configuration for `type=dropdown` fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsDropdown<'a> {
    /// The options available for the customer to select.
    ///
    /// Up to 200 options allowed.
    pub options: &'a [UpdatePaymentLinkCustomFieldsDropdownOptions<'a>],
}
impl<'a> UpdatePaymentLinkCustomFieldsDropdown<'a> {
    pub fn new(options: &'a [UpdatePaymentLinkCustomFieldsDropdownOptions<'a>]) -> Self {
        Self { options }
    }
}
/// The options available for the customer to select.
///
/// Up to 200 options allowed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsDropdownOptions<'a> {
    /// The label for the option, displayed to the customer.
    ///
    /// Up to 100 characters.
    pub label: &'a str,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    ///
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: &'a str,
}
impl<'a> UpdatePaymentLinkCustomFieldsDropdownOptions<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self { label, value }
    }
}
/// The label for the field, displayed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsLabel<'a> {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: &'a str,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkCustomFieldsLabelType,
}
impl<'a> UpdatePaymentLinkCustomFieldsLabel<'a> {
    pub fn new(custom: &'a str, type_: UpdatePaymentLinkCustomFieldsLabelType) -> Self {
        Self { custom, type_ }
    }
}
/// The type of the label.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkCustomFieldsLabelType {
    Custom,
}

impl UpdatePaymentLinkCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkCustomFieldsLabelType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkCustomFieldsLabelType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkCustomFieldsLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkCustomFieldsLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configuration for `type=numeric` fields.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl UpdatePaymentLinkCustomFieldsNumeric {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for `type=text` fields.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl UpdatePaymentLinkCustomFieldsText {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of the field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

impl UpdatePaymentLinkCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkCustomFieldsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkCustomFieldsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Display additional text for your customers using custom text.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLinkCustomText<'a> {
    /// Custom text that should be displayed alongside shipping address collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<UpdatePaymentLinkCustomTextShippingAddress<'a>>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<UpdatePaymentLinkCustomTextSubmit<'a>>,
    /// Custom text that should be displayed in place of the default terms of service agreement text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance:
        Option<UpdatePaymentLinkCustomTextTermsOfServiceAcceptance<'a>>,
}
impl<'a> UpdatePaymentLinkCustomText<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Custom text that should be displayed alongside shipping address collection.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomTextShippingAddress<'a> {
    /// Text may be up to 1200 characters in length.
    pub message: &'a str,
}
impl<'a> UpdatePaymentLinkCustomTextShippingAddress<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
/// Custom text that should be displayed alongside the payment confirmation button.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomTextSubmit<'a> {
    /// Text may be up to 1200 characters in length.
    pub message: &'a str,
}
impl<'a> UpdatePaymentLinkCustomTextSubmit<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
/// Custom text that should be displayed in place of the default terms of service agreement text.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomTextTermsOfServiceAcceptance<'a> {
    /// Text may be up to 1200 characters in length.
    pub message: &'a str,
}
impl<'a> UpdatePaymentLinkCustomTextTermsOfServiceAcceptance<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
/// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkCustomerCreation {
    Always,
    IfRequired,
}

impl UpdatePaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Generate a post-purchase Invoice for one-time payments.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreation<'a> {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Invoice PDF configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<UpdatePaymentLinkInvoiceCreationInvoiceData<'a>>,
}
impl<'a> UpdatePaymentLinkInvoiceCreation<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, invoice_data: Default::default() }
    }
}
/// Invoice PDF configuration.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceData<'a> {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<&'a [&'a str]>,
    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [UpdatePaymentLinkInvoiceCreationInvoiceDataCustomFields<'a>]>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions>,
}
impl<'a> UpdatePaymentLinkInvoiceCreationInvoiceData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default custom fields to be displayed on invoices for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceDataCustomFields<'a> {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: &'a str,
    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: &'a str,
}
impl<'a> UpdatePaymentLinkInvoiceCreationInvoiceDataCustomFields<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
}
impl UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
///
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The line items representing what is being sold.
///
/// Each line item represents an item being sold.
/// Up to 20 line items are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkLineItems<'a> {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<UpdatePaymentLinkLineItemsAdjustableQuantity>,
    /// The ID of an existing line item on the payment link.
    pub id: &'a str,
    /// The quantity of the line item being purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}
impl<'a> UpdatePaymentLinkLineItems<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { adjustable_quantity: Default::default(), id, quantity: Default::default() }
    }
}
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative Integer.
    pub enabled: bool,
    /// The maximum quantity the customer can purchase.
    ///
    /// By default this value is 99.
    /// You can specify a value up to 999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity the customer can purchase.
    ///
    /// By default this value is 0.
    /// If there is only one item in the cart then that item's quantity cannot go down to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
impl UpdatePaymentLinkLineItemsAdjustableQuantity {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: Default::default(), minimum: Default::default() }
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLinkPaymentIntentData<'a> {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    ///
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Extra information about the payment.
    ///
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that's set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
}
impl<'a> UpdatePaymentLinkPaymentIntentData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Specify whether Checkout should collect a payment method.
///
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}

impl UpdatePaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The list of payment method types that customers can use.
///
/// Pass an empty string to enable dynamic payment methods that use your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentLinkPaymentMethodTypes {
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

impl UpdatePaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkPaymentMethodTypes::*;
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

impl std::str::FromStr for UpdatePaymentLinkPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkPaymentMethodTypes::*;
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

impl AsRef<str> for UpdatePaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configuration for collecting the customer's shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkShippingAddressCollection<'a> {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: &'a [UpdatePaymentLinkShippingAddressCollectionAllowedCountries],
}
impl<'a> UpdatePaymentLinkShippingAddressCollection<'a> {
    pub fn new(
        allowed_countries: &'a [UpdatePaymentLinkShippingAddressCollectionAllowedCountries],
    ) -> Self {
        Self { allowed_countries }
    }
}
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
/// shipping locations.
///
/// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    Ac,
    Ad,
    Ae,
    Af,
    Ag,
    Ai,
    Al,
    Am,
    Ao,
    Aq,
    Ar,
    At,
    Au,
    Aw,
    Ax,
    Az,
    Ba,
    Bb,
    Bd,
    Be,
    Bf,
    Bg,
    Bh,
    Bi,
    Bj,
    Bl,
    Bm,
    Bn,
    Bo,
    Bq,
    Br,
    Bs,
    Bt,
    Bv,
    Bw,
    By,
    Bz,
    Ca,
    Cd,
    Cf,
    Cg,
    Ch,
    Ci,
    Ck,
    Cl,
    Cm,
    Cn,
    Co,
    Cr,
    Cv,
    Cw,
    Cy,
    Cz,
    De,
    Dj,
    Dk,
    Dm,
    Do,
    Dz,
    Ec,
    Ee,
    Eg,
    Eh,
    Er,
    Es,
    Et,
    Fi,
    Fj,
    Fk,
    Fo,
    Fr,
    Ga,
    Gb,
    Gd,
    Ge,
    Gf,
    Gg,
    Gh,
    Gi,
    Gl,
    Gm,
    Gn,
    Gp,
    Gq,
    Gr,
    Gs,
    Gt,
    Gu,
    Gw,
    Gy,
    Hk,
    Hn,
    Hr,
    Ht,
    Hu,
    Id,
    Ie,
    Il,
    Im,
    In,
    Io,
    Iq,
    Is,
    It,
    Je,
    Jm,
    Jo,
    Jp,
    Ke,
    Kg,
    Kh,
    Ki,
    Km,
    Kn,
    Kr,
    Kw,
    Ky,
    Kz,
    La,
    Lb,
    Lc,
    Li,
    Lk,
    Lr,
    Ls,
    Lt,
    Lu,
    Lv,
    Ly,
    Ma,
    Mc,
    Md,
    Me,
    Mf,
    Mg,
    Mk,
    Ml,
    Mm,
    Mn,
    Mo,
    Mq,
    Mr,
    Ms,
    Mt,
    Mu,
    Mv,
    Mw,
    Mx,
    My,
    Mz,
    Na,
    Nc,
    Ne,
    Ng,
    Ni,
    Nl,
    No,
    Np,
    Nr,
    Nu,
    Nz,
    Om,
    Pa,
    Pe,
    Pf,
    Pg,
    Ph,
    Pk,
    Pl,
    Pm,
    Pn,
    Pr,
    Ps,
    Pt,
    Py,
    Qa,
    Re,
    Ro,
    Rs,
    Ru,
    Rw,
    Sa,
    Sb,
    Sc,
    Se,
    Sg,
    Sh,
    Si,
    Sj,
    Sk,
    Sl,
    Sm,
    Sn,
    So,
    Sr,
    Ss,
    St,
    Sv,
    Sx,
    Sz,
    Ta,
    Tc,
    Td,
    Tf,
    Tg,
    Th,
    Tj,
    Tk,
    Tl,
    Tm,
    Tn,
    To,
    Tr,
    Tt,
    Tv,
    Tw,
    Tz,
    Ua,
    Ug,
    Us,
    Uy,
    Uz,
    Va,
    Vc,
    Ve,
    Vg,
    Vn,
    Vu,
    Wf,
    Ws,
    Xk,
    Ye,
    Yt,
    Za,
    Zm,
    Zw,
    Zz,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkShippingAddressCollectionAllowedCountries::*;
        match self {
            Ac => "AC",
            Ad => "AD",
            Ae => "AE",
            Af => "AF",
            Ag => "AG",
            Ai => "AI",
            Al => "AL",
            Am => "AM",
            Ao => "AO",
            Aq => "AQ",
            Ar => "AR",
            At => "AT",
            Au => "AU",
            Aw => "AW",
            Ax => "AX",
            Az => "AZ",
            Ba => "BA",
            Bb => "BB",
            Bd => "BD",
            Be => "BE",
            Bf => "BF",
            Bg => "BG",
            Bh => "BH",
            Bi => "BI",
            Bj => "BJ",
            Bl => "BL",
            Bm => "BM",
            Bn => "BN",
            Bo => "BO",
            Bq => "BQ",
            Br => "BR",
            Bs => "BS",
            Bt => "BT",
            Bv => "BV",
            Bw => "BW",
            By => "BY",
            Bz => "BZ",
            Ca => "CA",
            Cd => "CD",
            Cf => "CF",
            Cg => "CG",
            Ch => "CH",
            Ci => "CI",
            Ck => "CK",
            Cl => "CL",
            Cm => "CM",
            Cn => "CN",
            Co => "CO",
            Cr => "CR",
            Cv => "CV",
            Cw => "CW",
            Cy => "CY",
            Cz => "CZ",
            De => "DE",
            Dj => "DJ",
            Dk => "DK",
            Dm => "DM",
            Do => "DO",
            Dz => "DZ",
            Ec => "EC",
            Ee => "EE",
            Eg => "EG",
            Eh => "EH",
            Er => "ER",
            Es => "ES",
            Et => "ET",
            Fi => "FI",
            Fj => "FJ",
            Fk => "FK",
            Fo => "FO",
            Fr => "FR",
            Ga => "GA",
            Gb => "GB",
            Gd => "GD",
            Ge => "GE",
            Gf => "GF",
            Gg => "GG",
            Gh => "GH",
            Gi => "GI",
            Gl => "GL",
            Gm => "GM",
            Gn => "GN",
            Gp => "GP",
            Gq => "GQ",
            Gr => "GR",
            Gs => "GS",
            Gt => "GT",
            Gu => "GU",
            Gw => "GW",
            Gy => "GY",
            Hk => "HK",
            Hn => "HN",
            Hr => "HR",
            Ht => "HT",
            Hu => "HU",
            Id => "ID",
            Ie => "IE",
            Il => "IL",
            Im => "IM",
            In => "IN",
            Io => "IO",
            Iq => "IQ",
            Is => "IS",
            It => "IT",
            Je => "JE",
            Jm => "JM",
            Jo => "JO",
            Jp => "JP",
            Ke => "KE",
            Kg => "KG",
            Kh => "KH",
            Ki => "KI",
            Km => "KM",
            Kn => "KN",
            Kr => "KR",
            Kw => "KW",
            Ky => "KY",
            Kz => "KZ",
            La => "LA",
            Lb => "LB",
            Lc => "LC",
            Li => "LI",
            Lk => "LK",
            Lr => "LR",
            Ls => "LS",
            Lt => "LT",
            Lu => "LU",
            Lv => "LV",
            Ly => "LY",
            Ma => "MA",
            Mc => "MC",
            Md => "MD",
            Me => "ME",
            Mf => "MF",
            Mg => "MG",
            Mk => "MK",
            Ml => "ML",
            Mm => "MM",
            Mn => "MN",
            Mo => "MO",
            Mq => "MQ",
            Mr => "MR",
            Ms => "MS",
            Mt => "MT",
            Mu => "MU",
            Mv => "MV",
            Mw => "MW",
            Mx => "MX",
            My => "MY",
            Mz => "MZ",
            Na => "NA",
            Nc => "NC",
            Ne => "NE",
            Ng => "NG",
            Ni => "NI",
            Nl => "NL",
            No => "NO",
            Np => "NP",
            Nr => "NR",
            Nu => "NU",
            Nz => "NZ",
            Om => "OM",
            Pa => "PA",
            Pe => "PE",
            Pf => "PF",
            Pg => "PG",
            Ph => "PH",
            Pk => "PK",
            Pl => "PL",
            Pm => "PM",
            Pn => "PN",
            Pr => "PR",
            Ps => "PS",
            Pt => "PT",
            Py => "PY",
            Qa => "QA",
            Re => "RE",
            Ro => "RO",
            Rs => "RS",
            Ru => "RU",
            Rw => "RW",
            Sa => "SA",
            Sb => "SB",
            Sc => "SC",
            Se => "SE",
            Sg => "SG",
            Sh => "SH",
            Si => "SI",
            Sj => "SJ",
            Sk => "SK",
            Sl => "SL",
            Sm => "SM",
            Sn => "SN",
            So => "SO",
            Sr => "SR",
            Ss => "SS",
            St => "ST",
            Sv => "SV",
            Sx => "SX",
            Sz => "SZ",
            Ta => "TA",
            Tc => "TC",
            Td => "TD",
            Tf => "TF",
            Tg => "TG",
            Th => "TH",
            Tj => "TJ",
            Tk => "TK",
            Tl => "TL",
            Tm => "TM",
            Tn => "TN",
            To => "TO",
            Tr => "TR",
            Tt => "TT",
            Tv => "TV",
            Tw => "TW",
            Tz => "TZ",
            Ua => "UA",
            Ug => "UG",
            Us => "US",
            Uy => "UY",
            Uz => "UZ",
            Va => "VA",
            Vc => "VC",
            Ve => "VE",
            Vg => "VG",
            Vn => "VN",
            Vu => "VU",
            Wf => "WF",
            Ws => "WS",
            Xk => "XK",
            Ye => "YE",
            Yt => "YT",
            Za => "ZA",
            Zm => "ZM",
            Zw => "ZW",
            Zz => "ZZ",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkShippingAddressCollectionAllowedCountries::*;
        match s {
            "AC" => Ok(Ac),
            "AD" => Ok(Ad),
            "AE" => Ok(Ae),
            "AF" => Ok(Af),
            "AG" => Ok(Ag),
            "AI" => Ok(Ai),
            "AL" => Ok(Al),
            "AM" => Ok(Am),
            "AO" => Ok(Ao),
            "AQ" => Ok(Aq),
            "AR" => Ok(Ar),
            "AT" => Ok(At),
            "AU" => Ok(Au),
            "AW" => Ok(Aw),
            "AX" => Ok(Ax),
            "AZ" => Ok(Az),
            "BA" => Ok(Ba),
            "BB" => Ok(Bb),
            "BD" => Ok(Bd),
            "BE" => Ok(Be),
            "BF" => Ok(Bf),
            "BG" => Ok(Bg),
            "BH" => Ok(Bh),
            "BI" => Ok(Bi),
            "BJ" => Ok(Bj),
            "BL" => Ok(Bl),
            "BM" => Ok(Bm),
            "BN" => Ok(Bn),
            "BO" => Ok(Bo),
            "BQ" => Ok(Bq),
            "BR" => Ok(Br),
            "BS" => Ok(Bs),
            "BT" => Ok(Bt),
            "BV" => Ok(Bv),
            "BW" => Ok(Bw),
            "BY" => Ok(By),
            "BZ" => Ok(Bz),
            "CA" => Ok(Ca),
            "CD" => Ok(Cd),
            "CF" => Ok(Cf),
            "CG" => Ok(Cg),
            "CH" => Ok(Ch),
            "CI" => Ok(Ci),
            "CK" => Ok(Ck),
            "CL" => Ok(Cl),
            "CM" => Ok(Cm),
            "CN" => Ok(Cn),
            "CO" => Ok(Co),
            "CR" => Ok(Cr),
            "CV" => Ok(Cv),
            "CW" => Ok(Cw),
            "CY" => Ok(Cy),
            "CZ" => Ok(Cz),
            "DE" => Ok(De),
            "DJ" => Ok(Dj),
            "DK" => Ok(Dk),
            "DM" => Ok(Dm),
            "DO" => Ok(Do),
            "DZ" => Ok(Dz),
            "EC" => Ok(Ec),
            "EE" => Ok(Ee),
            "EG" => Ok(Eg),
            "EH" => Ok(Eh),
            "ER" => Ok(Er),
            "ES" => Ok(Es),
            "ET" => Ok(Et),
            "FI" => Ok(Fi),
            "FJ" => Ok(Fj),
            "FK" => Ok(Fk),
            "FO" => Ok(Fo),
            "FR" => Ok(Fr),
            "GA" => Ok(Ga),
            "GB" => Ok(Gb),
            "GD" => Ok(Gd),
            "GE" => Ok(Ge),
            "GF" => Ok(Gf),
            "GG" => Ok(Gg),
            "GH" => Ok(Gh),
            "GI" => Ok(Gi),
            "GL" => Ok(Gl),
            "GM" => Ok(Gm),
            "GN" => Ok(Gn),
            "GP" => Ok(Gp),
            "GQ" => Ok(Gq),
            "GR" => Ok(Gr),
            "GS" => Ok(Gs),
            "GT" => Ok(Gt),
            "GU" => Ok(Gu),
            "GW" => Ok(Gw),
            "GY" => Ok(Gy),
            "HK" => Ok(Hk),
            "HN" => Ok(Hn),
            "HR" => Ok(Hr),
            "HT" => Ok(Ht),
            "HU" => Ok(Hu),
            "ID" => Ok(Id),
            "IE" => Ok(Ie),
            "IL" => Ok(Il),
            "IM" => Ok(Im),
            "IN" => Ok(In),
            "IO" => Ok(Io),
            "IQ" => Ok(Iq),
            "IS" => Ok(Is),
            "IT" => Ok(It),
            "JE" => Ok(Je),
            "JM" => Ok(Jm),
            "JO" => Ok(Jo),
            "JP" => Ok(Jp),
            "KE" => Ok(Ke),
            "KG" => Ok(Kg),
            "KH" => Ok(Kh),
            "KI" => Ok(Ki),
            "KM" => Ok(Km),
            "KN" => Ok(Kn),
            "KR" => Ok(Kr),
            "KW" => Ok(Kw),
            "KY" => Ok(Ky),
            "KZ" => Ok(Kz),
            "LA" => Ok(La),
            "LB" => Ok(Lb),
            "LC" => Ok(Lc),
            "LI" => Ok(Li),
            "LK" => Ok(Lk),
            "LR" => Ok(Lr),
            "LS" => Ok(Ls),
            "LT" => Ok(Lt),
            "LU" => Ok(Lu),
            "LV" => Ok(Lv),
            "LY" => Ok(Ly),
            "MA" => Ok(Ma),
            "MC" => Ok(Mc),
            "MD" => Ok(Md),
            "ME" => Ok(Me),
            "MF" => Ok(Mf),
            "MG" => Ok(Mg),
            "MK" => Ok(Mk),
            "ML" => Ok(Ml),
            "MM" => Ok(Mm),
            "MN" => Ok(Mn),
            "MO" => Ok(Mo),
            "MQ" => Ok(Mq),
            "MR" => Ok(Mr),
            "MS" => Ok(Ms),
            "MT" => Ok(Mt),
            "MU" => Ok(Mu),
            "MV" => Ok(Mv),
            "MW" => Ok(Mw),
            "MX" => Ok(Mx),
            "MY" => Ok(My),
            "MZ" => Ok(Mz),
            "NA" => Ok(Na),
            "NC" => Ok(Nc),
            "NE" => Ok(Ne),
            "NG" => Ok(Ng),
            "NI" => Ok(Ni),
            "NL" => Ok(Nl),
            "NO" => Ok(No),
            "NP" => Ok(Np),
            "NR" => Ok(Nr),
            "NU" => Ok(Nu),
            "NZ" => Ok(Nz),
            "OM" => Ok(Om),
            "PA" => Ok(Pa),
            "PE" => Ok(Pe),
            "PF" => Ok(Pf),
            "PG" => Ok(Pg),
            "PH" => Ok(Ph),
            "PK" => Ok(Pk),
            "PL" => Ok(Pl),
            "PM" => Ok(Pm),
            "PN" => Ok(Pn),
            "PR" => Ok(Pr),
            "PS" => Ok(Ps),
            "PT" => Ok(Pt),
            "PY" => Ok(Py),
            "QA" => Ok(Qa),
            "RE" => Ok(Re),
            "RO" => Ok(Ro),
            "RS" => Ok(Rs),
            "RU" => Ok(Ru),
            "RW" => Ok(Rw),
            "SA" => Ok(Sa),
            "SB" => Ok(Sb),
            "SC" => Ok(Sc),
            "SE" => Ok(Se),
            "SG" => Ok(Sg),
            "SH" => Ok(Sh),
            "SI" => Ok(Si),
            "SJ" => Ok(Sj),
            "SK" => Ok(Sk),
            "SL" => Ok(Sl),
            "SM" => Ok(Sm),
            "SN" => Ok(Sn),
            "SO" => Ok(So),
            "SR" => Ok(Sr),
            "SS" => Ok(Ss),
            "ST" => Ok(St),
            "SV" => Ok(Sv),
            "SX" => Ok(Sx),
            "SZ" => Ok(Sz),
            "TA" => Ok(Ta),
            "TC" => Ok(Tc),
            "TD" => Ok(Td),
            "TF" => Ok(Tf),
            "TG" => Ok(Tg),
            "TH" => Ok(Th),
            "TJ" => Ok(Tj),
            "TK" => Ok(Tk),
            "TL" => Ok(Tl),
            "TM" => Ok(Tm),
            "TN" => Ok(Tn),
            "TO" => Ok(To),
            "TR" => Ok(Tr),
            "TT" => Ok(Tt),
            "TV" => Ok(Tv),
            "TW" => Ok(Tw),
            "TZ" => Ok(Tz),
            "UA" => Ok(Ua),
            "UG" => Ok(Ug),
            "US" => Ok(Us),
            "UY" => Ok(Uy),
            "UZ" => Ok(Uz),
            "VA" => Ok(Va),
            "VC" => Ok(Vc),
            "VE" => Ok(Ve),
            "VG" => Ok(Vg),
            "VN" => Ok(Vn),
            "VU" => Ok(Vu),
            "WF" => Ok(Wf),
            "WS" => Ok(Ws),
            "XK" => Ok(Xk),
            "YE" => Ok(Ye),
            "YT" => Ok(Yt),
            "ZA" => Ok(Za),
            "ZM" => Ok(Zm),
            "ZW" => Ok(Zw),
            "ZZ" => Ok(Zz),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// When creating a subscription, the specified configuration data will be used.
///
/// There must be at least one line item with a recurring price to use `subscription_data`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLinkSubscriptionData<'a> {
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    ///
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdatePaymentLinkSubscriptionData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdatePaymentLink<'a> {
    /// Updates a payment link.
    pub fn send(
        &self,
        client: &stripe::Client,
        payment_link: &stripe_types::payment_link::PaymentLinkId,
    ) -> stripe::Response<stripe_types::PaymentLink> {
        client.send_form(
            &format!("/payment_links/{payment_link}", payment_link = payment_link),
            self,
            http_types::Method::Post,
        )
    }
}
