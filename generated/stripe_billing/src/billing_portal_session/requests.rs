#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSession<'a> {
    /// The ID of an existing [configuration](https://stripe.com/docs/api/customer_portal/configuration) to use for this session, describing its functionality and features.
    /// If not specified, the session uses the default configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<&'a str>,
    /// The ID of an existing customer.
    pub customer: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Information about a specific flow for the customer to go through.
    /// See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_data: Option<CreateBillingPortalSessionFlowData<'a>>,
    /// The IETF language tag of the locale customer portal is displayed in.
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<stripe_billing::BillingPortalSessionLocale>,
    /// The `on_behalf_of` account to use for this session.
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/separate-charges-and-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> CreateBillingPortalSession<'a> {
    pub fn new(customer: &'a str) -> Self {
        Self {
            configuration: None,
            customer,
            expand: None,
            flow_data: None,
            locale: None,
            on_behalf_of: None,
            return_url: None,
        }
    }
}
/// Information about a specific flow for the customer to go through.
/// See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowData<'a> {
    /// Behavior after the flow is completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<CreateBillingPortalSessionFlowDataAfterCompletion<'a>>,
    /// Configuration when `flow_data.type=subscription_cancel`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<CreateBillingPortalSessionFlowDataSubscriptionCancel<'a>>,
    /// Configuration when `flow_data.type=subscription_update`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<CreateBillingPortalSessionFlowDataSubscriptionUpdate<'a>>,
    /// Configuration when `flow_data.type=subscription_update_confirm`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update_confirm:
        Option<CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirm<'a>>,
    /// Type of flow that the customer will go through.
    #[serde(rename = "type")]
    pub type_: CreateBillingPortalSessionFlowDataType,
}
impl<'a> CreateBillingPortalSessionFlowData<'a> {
    pub fn new(type_: CreateBillingPortalSessionFlowDataType) -> Self {
        Self {
            after_completion: None,
            subscription_cancel: None,
            subscription_update: None,
            subscription_update_confirm: None,
            type_,
        }
    }
}
/// Behavior after the flow is completed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataAfterCompletion<'a> {
    /// Configuration when `after_completion.type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation:
        Option<CreateBillingPortalSessionFlowDataAfterCompletionHostedConfirmation<'a>>,
    /// Configuration when `after_completion.type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<CreateBillingPortalSessionFlowDataAfterCompletionRedirect<'a>>,
    /// The specified behavior after the flow is completed.
    #[serde(rename = "type")]
    pub type_: CreateBillingPortalSessionFlowDataAfterCompletionType,
}
impl<'a> CreateBillingPortalSessionFlowDataAfterCompletion<'a> {
    pub fn new(type_: CreateBillingPortalSessionFlowDataAfterCompletionType) -> Self {
        Self { hosted_confirmation: None, redirect: None, type_ }
    }
}
/// Configuration when `after_completion.type=hosted_confirmation`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataAfterCompletionHostedConfirmation<'a> {
    /// A custom message to display to the customer after the flow is completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<&'a str>,
}
impl<'a> CreateBillingPortalSessionFlowDataAfterCompletionHostedConfirmation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration when `after_completion.type=redirect`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataAfterCompletionRedirect<'a> {
    /// The URL the customer will be redirected to after the flow is completed.
    pub return_url: &'a str,
}
impl<'a> CreateBillingPortalSessionFlowDataAfterCompletionRedirect<'a> {
    pub fn new(return_url: &'a str) -> Self {
        Self { return_url }
    }
}
/// The specified behavior after the flow is completed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalSessionFlowDataAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
}
impl CreateBillingPortalSessionFlowDataAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalSessionFlowDataAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            PortalHomepage => "portal_homepage",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for CreateBillingPortalSessionFlowDataAfterCompletionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalSessionFlowDataAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "portal_homepage" => Ok(PortalHomepage),
            "redirect" => Ok(Redirect),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configuration when `flow_data.type=subscription_cancel`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionCancel<'a> {
    /// Specify a retention strategy to be used in the cancellation flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<CreateBillingPortalSessionFlowDataSubscriptionCancelRetention<'a>>,
    /// The ID of the subscription to be canceled.
    pub subscription: &'a str,
}
impl<'a> CreateBillingPortalSessionFlowDataSubscriptionCancel<'a> {
    pub fn new(subscription: &'a str) -> Self {
        Self { retention: None, subscription }
    }
}
/// Specify a retention strategy to be used in the cancellation flow.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionCancelRetention<'a> {
    /// Configuration when `retention.type=coupon_offer`.
    pub coupon_offer: CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer<'a>,
    /// Type of retention strategy to use with the customer.
    #[serde(rename = "type")]
    pub type_: CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType,
}
impl<'a> CreateBillingPortalSessionFlowDataSubscriptionCancelRetention<'a> {
    pub fn new(
        coupon_offer: CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer<'a>,
        type_: CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType,
    ) -> Self {
        Self { coupon_offer, type_ }
    }
}
/// Configuration when `retention.type=coupon_offer`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer<'a> {
    /// The ID of the coupon to be offered.
    pub coupon: &'a str,
}
impl<'a> CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer<'a> {
    pub fn new(coupon: &'a str) -> Self {
        Self { coupon }
    }
}
/// Type of retention strategy to use with the customer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    CouponOffer,
}
impl CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType::*;
        match self {
            CouponOffer => "coupon_offer",
        }
    }
}

impl std::str::FromStr for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType::*;
        match s {
            "coupon_offer" => Ok(CouponOffer),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configuration when `flow_data.type=subscription_update`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdate<'a> {
    /// The ID of the subscription to be updated.
    pub subscription: &'a str,
}
impl<'a> CreateBillingPortalSessionFlowDataSubscriptionUpdate<'a> {
    pub fn new(subscription: &'a str) -> Self {
        Self { subscription }
    }
}
/// Configuration when `flow_data.type=subscription_update_confirm`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirm<'a> {
    /// The coupon or promotion code to apply to this subscription update.
    /// Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts:
        Option<&'a [CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmDiscounts<'a>]>,
    /// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
    /// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
    pub items: &'a [CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems<'a>],
    /// The ID of the subscription to be updated.
    pub subscription: &'a str,
}
impl<'a> CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirm<'a> {
    pub fn new(
        items: &'a [CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems<'a>],
        subscription: &'a str,
    ) -> Self {
        Self { discounts: None, items, subscription }
    }
}
/// The coupon or promotion code to apply to this subscription update.
/// Currently, only up to one may be specified.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmDiscounts<'a> {
    /// The ID of the coupon to apply to this subscription update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// The ID of a promotion code to apply to this subscription update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
}
impl<'a> CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
/// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems<'a> {
    /// The ID of the [subscription item](https://stripe.com/docs/api/subscriptions/object#subscription_object-items-data-id) to be updated.
    pub id: &'a str,
    /// The price the customer should subscribe to through this flow.
    /// The price must also be included in the configuration's [`features.subscription_update.products`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// [Quantity](https://stripe.com/docs/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}
impl<'a> CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { id, price: None, quantity: None }
    }
}
/// Type of flow that the customer will go through.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalSessionFlowDataType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
}
impl CreateBillingPortalSessionFlowDataType {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalSessionFlowDataType::*;
        match self {
            PaymentMethodUpdate => "payment_method_update",
            SubscriptionCancel => "subscription_cancel",
            SubscriptionUpdate => "subscription_update",
            SubscriptionUpdateConfirm => "subscription_update_confirm",
        }
    }
}

impl std::str::FromStr for CreateBillingPortalSessionFlowDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalSessionFlowDataType::*;
        match s {
            "payment_method_update" => Ok(PaymentMethodUpdate),
            "subscription_cancel" => Ok(SubscriptionCancel),
            "subscription_update" => Ok(SubscriptionUpdate),
            "subscription_update_confirm" => Ok(SubscriptionUpdateConfirm),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateBillingPortalSessionFlowDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateBillingPortalSessionFlowDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingPortalSessionFlowDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingPortalSessionFlowDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateBillingPortalSession<'a> {
    /// Creates a session of the customer portal.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_billing::BillingPortalSession> {
        client.send_form("/billing_portal/sessions", self, http_types::Method::Post)
    }
}
