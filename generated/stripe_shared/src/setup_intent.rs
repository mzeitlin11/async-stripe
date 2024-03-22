/// A SetupIntent guides you through the process of setting up and saving a customer's payment credentials for future payments.
/// For example, you can use a SetupIntent to set up and save your customer's card without immediately collecting a payment.
/// Later, you can use [PaymentIntents](https://stripe.com/docs/api#payment_intents) to drive the payment flow.
///
/// Create a SetupIntent when you're ready to collect your customer's payment credentials.
/// Don't maintain long-lived, unconfirmed SetupIntents because they might not be valid.
/// The SetupIntent transitions through multiple [statuses](https://stripe.com/docs/payments/intents#intent-statuses) as it guides.
/// you through the setup process.
///
/// Successful SetupIntents result in payment credentials that are optimized for future payments.
/// For example, cardholders in [certain regions](/guides/strong-customer-authentication) might need to be run through.
/// [Strong Customer Authentication](https://stripe.com/docs/strong-customer-authentication) during payment method collection.
/// to streamline later [off-session payments](https://stripe.com/docs/payments/setup-intents).
/// If you use the SetupIntent with a [Customer](https://stripe.com/docs/api#setup_intent_object-customer),.
/// it automatically attaches the resulting payment method to that Customer after successful setup.
/// We recommend using SetupIntents or [setup_future_usage](https://stripe.com/docs/api#payment_intent_object-setup_future_usage) on.
/// PaymentIntents to save payment methods to prevent saving invalid or unoptimized payment methods.
///
/// By using SetupIntents, you can reduce friction for your customers, even as regulations change over time.
///
/// Related guide: [Setup Intents API](https://stripe.com/docs/payments/setup-intents)
///
/// For more details see <<https://stripe.com/docs/api/setup_intents/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntent {
    /// ID of the Connect application that created the SetupIntent.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    pub attach_to_self: Option<bool>,
    /// Settings for dynamic payment methods compatible with this Setup Intent
    pub automatic_payment_methods: Option<stripe_shared::PaymentFlowsAutomaticPaymentMethodsSetupIntent>,
    /// Reason for cancellation of this SetupIntent, one of `abandoned`, `requested_by_customer`, or `duplicate`.
    pub cancellation_reason: Option<stripe_shared::SetupIntentCancellationReason>,
    /// The client secret of this SetupIntent. Used for client-side retrieval using a publishable key.
    ///
    /// The client secret can be used to complete payment setup from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    pub client_secret: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub flow_directions: Option<Vec<stripe_shared::SetupIntentFlowDirections>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SetupIntentId,
    /// The error encountered in the previous SetupIntent confirmation.
    pub last_setup_error: Option<Box<stripe_shared::ApiErrors>>,
    /// The most recent SetupAttempt for this SetupIntent.
    pub latest_attempt: Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the multi use Mandate generated by the SetupIntent.
    pub mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// If present, this property tells you what actions you need to take in order for your customer to continue payment setup.
    pub next_action: Option<stripe_shared::SetupIntentNextAction>,
    /// The account (if any) for which the setup is intended.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// ID of the payment method used with this SetupIntent.
    pub payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// Information about the payment method configuration used for this Setup Intent.
    pub payment_method_configuration_details: Option<stripe_shared::PaymentMethodConfigBizPaymentMethodConfigurationDetails>,
    /// Payment method-specific configuration for this SetupIntent.
    pub payment_method_options: Option<stripe_shared::SetupIntentPaymentMethodOptions>,
    /// The list of payment method types (e.g. card) that this SetupIntent is allowed to set up.
    pub payment_method_types: Vec<String>,
    /// ID of the single_use Mandate generated by the SetupIntent.
    pub single_use_mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// [Status](https://stripe.com/docs/payments/intents#intent-statuses) of this SetupIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `canceled`, or `succeeded`.
    pub status: SetupIntentStatus,
    /// Indicates how the payment method is intended to be used in the future.
    ///
    /// Use `on_session` if you intend to only reuse the payment method when the customer is in your checkout flow.
    /// Use `off_session` if your customer may or may not be in your checkout flow.
    /// If not provided, this value defaults to `off_session`.
    pub usage: String,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentBuilder {
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    attach_to_self: Option<Option<bool>>,
    automatic_payment_methods: Option<Option<stripe_shared::PaymentFlowsAutomaticPaymentMethodsSetupIntent>>,
    cancellation_reason: Option<Option<stripe_shared::SetupIntentCancellationReason>>,
    client_secret: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    description: Option<Option<String>>,
    flow_directions: Option<Option<Vec<stripe_shared::SetupIntentFlowDirections>>>,
    id: Option<stripe_shared::SetupIntentId>,
    last_setup_error: Option<Option<Box<stripe_shared::ApiErrors>>>,
    latest_attempt: Option<Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>>,
    livemode: Option<bool>,
    mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    next_action: Option<Option<stripe_shared::SetupIntentNextAction>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    payment_method_configuration_details: Option<Option<stripe_shared::PaymentMethodConfigBizPaymentMethodConfigurationDetails>>,
    payment_method_options: Option<Option<stripe_shared::SetupIntentPaymentMethodOptions>>,
    payment_method_types: Option<Vec<String>>,
    single_use_mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    status: Option<SetupIntentStatus>,
    usage: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntent>,
        builder: SetupIntentBuilder,
    }

    impl Visitor for Place<SetupIntent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentBuilder {
        type Out = SetupIntent;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "attach_to_self" => Ok(Deserialize::begin(&mut self.attach_to_self)),
                "automatic_payment_methods" => Ok(Deserialize::begin(&mut self.automatic_payment_methods)),
                "cancellation_reason" => Ok(Deserialize::begin(&mut self.cancellation_reason)),
                "client_secret" => Ok(Deserialize::begin(&mut self.client_secret)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "flow_directions" => Ok(Deserialize::begin(&mut self.flow_directions)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "last_setup_error" => Ok(Deserialize::begin(&mut self.last_setup_error)),
                "latest_attempt" => Ok(Deserialize::begin(&mut self.latest_attempt)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "mandate" => Ok(Deserialize::begin(&mut self.mandate)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "next_action" => Ok(Deserialize::begin(&mut self.next_action)),
                "on_behalf_of" => Ok(Deserialize::begin(&mut self.on_behalf_of)),
                "payment_method" => Ok(Deserialize::begin(&mut self.payment_method)),
                "payment_method_configuration_details" => Ok(Deserialize::begin(&mut self.payment_method_configuration_details)),
                "payment_method_options" => Ok(Deserialize::begin(&mut self.payment_method_options)),
                "payment_method_types" => Ok(Deserialize::begin(&mut self.payment_method_types)),
                "single_use_mandate" => Ok(Deserialize::begin(&mut self.single_use_mandate)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "usage" => Ok(Deserialize::begin(&mut self.usage)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                application: Deserialize::default(),
                attach_to_self: Deserialize::default(),
                automatic_payment_methods: Deserialize::default(),
                cancellation_reason: Deserialize::default(),
                client_secret: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
                description: Deserialize::default(),
                flow_directions: Deserialize::default(),
                id: Deserialize::default(),
                last_setup_error: Deserialize::default(),
                latest_attempt: Deserialize::default(),
                livemode: Deserialize::default(),
                mandate: Deserialize::default(),
                metadata: Deserialize::default(),
                next_action: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                payment_method: Deserialize::default(),
                payment_method_configuration_details: Deserialize::default(),
                payment_method_options: Deserialize::default(),
                payment_method_types: Deserialize::default(),
                single_use_mandate: Deserialize::default(),
                status: Deserialize::default(),
                usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let application = self.application.take()?;
            let attach_to_self = self.attach_to_self.take()?;
            let automatic_payment_methods = self.automatic_payment_methods.take()?;
            let cancellation_reason = self.cancellation_reason.take()?;
            let client_secret = self.client_secret.take()?;
            let created = self.created.take()?;
            let customer = self.customer.take()?;
            let description = self.description.take()?;
            let flow_directions = self.flow_directions.take()?;
            let id = self.id.take()?;
            let last_setup_error = self.last_setup_error.take()?;
            let latest_attempt = self.latest_attempt.take()?;
            let livemode = self.livemode.take()?;
            let mandate = self.mandate.take()?;
            let metadata = self.metadata.take()?;
            let next_action = self.next_action.take()?;
            let on_behalf_of = self.on_behalf_of.take()?;
            let payment_method = self.payment_method.take()?;
            let payment_method_configuration_details = self.payment_method_configuration_details.take()?;
            let payment_method_options = self.payment_method_options.take()?;
            let payment_method_types = self.payment_method_types.take()?;
            let single_use_mandate = self.single_use_mandate.take()?;
            let status = self.status.take()?;
            let usage = self.usage.take()?;

            Some(Self::Out {
                application,
                attach_to_self,
                automatic_payment_methods,
                cancellation_reason,
                client_secret,
                created,
                customer,
                description,
                flow_directions,
                id,
                last_setup_error,
                latest_attempt,
                livemode,
                mandate,
                metadata,
                next_action,
                on_behalf_of,
                payment_method,
                payment_method_configuration_details,
                payment_method_options,
                payment_method_types,
                single_use_mandate,
                status,
                usage,
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

    impl ObjectDeser for SetupIntent {
        type Builder = SetupIntentBuilder;
    }
};
/// [Status](https://stripe.com/docs/payments/intents#intent-statuses) of this SetupIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `canceled`, or `succeeded`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}
impl SetupIntentStatus {
    pub fn as_str(self) -> &'static str {
        use SetupIntentStatus::*;
        match self {
            Canceled => "canceled",
            Processing => "processing",
            RequiresAction => "requires_action",
            RequiresConfirmation => "requires_confirmation",
            RequiresPaymentMethod => "requires_payment_method",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for SetupIntentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "processing" => Ok(Processing),
            "requires_action" => Ok(RequiresAction),
            "requires_confirmation" => Ok(RequiresConfirmation),
            "requires_payment_method" => Ok(RequiresPaymentMethod),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupIntentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupIntentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupIntentStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for SetupIntent {
    type Id = stripe_shared::SetupIntentId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SetupIntentId, "seti_");
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentCancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
}
impl SetupIntentCancellationReason {
    pub fn as_str(self) -> &'static str {
        use SetupIntentCancellationReason::*;
        match self {
            Abandoned => "abandoned",
            Duplicate => "duplicate",
            RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl std::str::FromStr for SetupIntentCancellationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentCancellationReason::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "duplicate" => Ok(Duplicate),
            "requested_by_customer" => Ok(RequestedByCustomer),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupIntentCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentCancellationReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentCancellationReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupIntentCancellationReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentCancellationReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentFlowDirections {
    Inbound,
    Outbound,
}
impl SetupIntentFlowDirections {
    pub fn as_str(self) -> &'static str {
        use SetupIntentFlowDirections::*;
        match self {
            Inbound => "inbound",
            Outbound => "outbound",
        }
    }
}

impl std::str::FromStr for SetupIntentFlowDirections {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentFlowDirections::*;
        match s {
            "inbound" => Ok(Inbound),
            "outbound" => Ok(Outbound),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupIntentFlowDirections {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentFlowDirections {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentFlowDirections {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentFlowDirections"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentFlowDirections {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupIntentFlowDirections> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentFlowDirections::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
