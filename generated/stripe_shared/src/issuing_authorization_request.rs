#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingAuthorizationRequest {
    /// The `pending_request.amount` at the time of the request, presented in your card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// Stripe held this amount from your account to fund the authorization if the request was approved.
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_shared::IssuingAuthorizationAmountDetails>,
    /// Whether this request was approved.
    pub approved: bool,
    /// A code created by Stripe which is shared with the merchant to validate the authorization.
    /// This field will be populated if the authorization message was approved.
    /// The code typically starts with the letter "S", followed by a six-digit number.
    /// For example, "S498162".
    /// Please note that the code is not guaranteed to be unique across authorizations.
    pub authorization_code: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The `pending_request.merchant_amount` at the time of the request, presented in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,
    /// The currency that was collected by the merchant and presented to the cardholder for the authorization.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: stripe_types::Currency,
    /// The card network's estimate of the likelihood that an authorization is fraudulent.
    /// Takes on values between 1 and 99.
    pub network_risk_score: Option<i64>,
    /// When an authorization is approved or declined by you or by Stripe, this field provides additional detail on the reason for the outcome.
    pub reason: IssuingAuthorizationRequestReason,
    /// If the `request_history.reason` is `webhook_error` because the direct webhook response is invalid (for example, parsing errors or missing parameters), we surface a more detailed error message via this field.
    pub reason_message: Option<String>,
    /// Time when the card network received an authorization request from the acquirer in UTC.
    /// Referred to by networks as transmission time.
    pub requested_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingAuthorizationRequestBuilder {
    amount: Option<i64>,
    amount_details: Option<Option<stripe_shared::IssuingAuthorizationAmountDetails>>,
    approved: Option<bool>,
    authorization_code: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    merchant_amount: Option<i64>,
    merchant_currency: Option<stripe_types::Currency>,
    network_risk_score: Option<Option<i64>>,
    reason: Option<IssuingAuthorizationRequestReason>,
    reason_message: Option<Option<String>>,
    requested_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingAuthorizationRequest {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationRequest>,
        builder: IssuingAuthorizationRequestBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationRequest> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingAuthorizationRequestBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingAuthorizationRequestBuilder {
        type Out = IssuingAuthorizationRequest;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_details" => Ok(Deserialize::begin(&mut self.amount_details)),
                "approved" => Ok(Deserialize::begin(&mut self.approved)),
                "authorization_code" => Ok(Deserialize::begin(&mut self.authorization_code)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "merchant_amount" => Ok(Deserialize::begin(&mut self.merchant_amount)),
                "merchant_currency" => Ok(Deserialize::begin(&mut self.merchant_currency)),
                "network_risk_score" => Ok(Deserialize::begin(&mut self.network_risk_score)),
                "reason" => Ok(Deserialize::begin(&mut self.reason)),
                "reason_message" => Ok(Deserialize::begin(&mut self.reason_message)),
                "requested_at" => Ok(Deserialize::begin(&mut self.requested_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_details: Deserialize::default(),
                approved: Deserialize::default(),
                authorization_code: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                merchant_amount: Deserialize::default(),
                merchant_currency: Deserialize::default(),
                network_risk_score: Deserialize::default(),
                reason: Deserialize::default(),
                reason_message: Deserialize::default(),
                requested_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_details = self.amount_details.take()?;
            let approved = self.approved.take()?;
            let authorization_code = self.authorization_code.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let merchant_amount = self.merchant_amount.take()?;
            let merchant_currency = self.merchant_currency.take()?;
            let network_risk_score = self.network_risk_score.take()?;
            let reason = self.reason.take()?;
            let reason_message = self.reason_message.take()?;
            let requested_at = self.requested_at.take()?;

            Some(Self::Out { amount, amount_details, approved, authorization_code, created, currency, merchant_amount, merchant_currency, network_risk_score, reason, reason_message, requested_at })
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

    impl ObjectDeser for IssuingAuthorizationRequest {
        type Builder = IssuingAuthorizationRequestBuilder;
    }
};
/// When an authorization is approved or declined by you or by Stripe, this field provides additional detail on the reason for the outcome.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationRequestReason {
    AccountDisabled,
    CardActive,
    CardInactive,
    CardholderInactive,
    CardholderVerificationRequired,
    InsufficientFunds,
    NotAllowed,
    SpendingControls,
    SuspectedFraud,
    VerificationFailed,
    WebhookApproved,
    WebhookDeclined,
    WebhookError,
    WebhookTimeout,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl IssuingAuthorizationRequestReason {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationRequestReason::*;
        match self {
            AccountDisabled => "account_disabled",
            CardActive => "card_active",
            CardInactive => "card_inactive",
            CardholderInactive => "cardholder_inactive",
            CardholderVerificationRequired => "cardholder_verification_required",
            InsufficientFunds => "insufficient_funds",
            NotAllowed => "not_allowed",
            SpendingControls => "spending_controls",
            SuspectedFraud => "suspected_fraud",
            VerificationFailed => "verification_failed",
            WebhookApproved => "webhook_approved",
            WebhookDeclined => "webhook_declined",
            WebhookError => "webhook_error",
            WebhookTimeout => "webhook_timeout",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationRequestReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationRequestReason::*;
        match s {
            "account_disabled" => Ok(AccountDisabled),
            "card_active" => Ok(CardActive),
            "card_inactive" => Ok(CardInactive),
            "cardholder_inactive" => Ok(CardholderInactive),
            "cardholder_verification_required" => Ok(CardholderVerificationRequired),
            "insufficient_funds" => Ok(InsufficientFunds),
            "not_allowed" => Ok(NotAllowed),
            "spending_controls" => Ok(SpendingControls),
            "suspected_fraud" => Ok(SuspectedFraud),
            "verification_failed" => Ok(VerificationFailed),
            "webhook_approved" => Ok(WebhookApproved),
            "webhook_declined" => Ok(WebhookDeclined),
            "webhook_error" => Ok(WebhookError),
            "webhook_timeout" => Ok(WebhookTimeout),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingAuthorizationRequestReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingAuthorizationRequestReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationRequestReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationRequestReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationRequestReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingAuthorizationRequestReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationRequestReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationRequestReason::from_str(s).unwrap_or(IssuingAuthorizationRequestReason::Unknown));
        Ok(())
    }
}
