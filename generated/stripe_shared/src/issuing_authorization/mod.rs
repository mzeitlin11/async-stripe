/// When an [issued card](https://stripe.com/docs/issuing) is used to make a purchase, an Issuing `Authorization`
/// object is created.
///
/// [Authorizations](https://stripe.com/docs/issuing/purchases/authorizations) must be approved for the purchase to be completed successfully.  Related guide: [Issued card authorizations](https://stripe.com/docs/issuing/purchases/authorizations).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingAuthorization {
    /// The total amount that was authorized or rejected.
    ///
    /// This amount is in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// `amount` should be the same as `merchant_amount`, unless `currency` and `merchant_currency` are different.
    pub amount: i64,
    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_shared::IssuingAuthorizationAmountDetails>,
    /// Whether the authorization has been approved.
    pub approved: bool,
    /// How the card details were provided.
    pub authorization_method: IssuingAuthorizationAuthorizationMethod,
    /// List of balance transactions associated with this authorization.
    pub balance_transactions: Vec<stripe_shared::BalanceTransaction>,
    pub card: stripe_shared::IssuingCard,
    /// The cardholder to whom this authorization belongs.
    pub cardholder: Option<stripe_types::Expandable<stripe_shared::IssuingCardholder>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The currency of the cardholder.
    ///
    /// This currency can be different from the currency presented at authorization and the `merchant_currency` field on this authorization.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_shared::issuing_authorization::IssuingAuthorizationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The total amount that was authorized or rejected.
    ///
    /// This amount is in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// `merchant_amount` should be the same as `amount`, unless `merchant_currency` and `currency` are different.
    pub merchant_amount: i64,
    /// The local currency that was presented to the cardholder for the authorization.
    ///
    /// This currency can be different from the cardholder currency and the `currency` field on this authorization.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: stripe_types::Currency,
    pub merchant_data: stripe_shared::IssuingAuthorizationMerchantData,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about the authorization, such as identifiers, set by the card network.
    pub network_data: Option<stripe_shared::IssuingAuthorizationNetworkData>,
    /// The pending authorization request.
    ///
    /// This field will only be non-null during an `issuing_authorization.request` webhook.
    pub pending_request: Option<stripe_shared::IssuingAuthorizationPendingRequest>,
    /// History of every time a `pending_request` authorization was approved/declined, either by you directly or by Stripe (e.g.
    ///
    /// based on your spending_controls).
    /// If the merchant changes the authorization by performing an incremental authorization, you can look at this field to see the previous requests for the authorization.
    /// This field can be helpful in determining why a given authorization was approved/declined.
    pub request_history: Vec<stripe_shared::IssuingAuthorizationRequest>,
    /// The current status of the authorization in its lifecycle.
    pub status: IssuingAuthorizationStatus,
    /// [Token](https://stripe.com/docs/api/issuing/tokens/object) object used for this authorization.
    ///
    /// If a network token was not used for this authorization, this field will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<stripe_types::Expandable<stripe_shared::IssuingToken>>,
    /// List of [transactions](https://stripe.com/docs/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<stripe_shared::IssuingTransaction>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://stripe.com/docs/api/treasury/financial_accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_shared::IssuingAuthorizationTreasury>,
    pub verification_data: stripe_shared::IssuingAuthorizationVerificationData,
    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    /// Will populate as `null` when no digital wallet was utilized.
    pub wallet: Option<String>,
}
/// How the card details were provided.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationAuthorizationMethod {
    Chip,
    Contactless,
    KeyedIn,
    Online,
    Swipe,
}
impl IssuingAuthorizationAuthorizationMethod {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationAuthorizationMethod::*;
        match self {
            Chip => "chip",
            Contactless => "contactless",
            KeyedIn => "keyed_in",
            Online => "online",
            Swipe => "swipe",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthorizationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthorizationMethod::*;
        match s {
            "chip" => Ok(Chip),
            "contactless" => Ok(Contactless),
            "keyed_in" => Ok(KeyedIn),
            "online" => Ok(Online),
            "swipe" => Ok(Swipe),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingAuthorizationAuthorizationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationAuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationAuthorizationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthorizationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingAuthorizationAuthorizationMethod")
        })
    }
}
/// The current status of the authorization in its lifecycle.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationStatus {
    Closed,
    Pending,
    Reversed,
}
impl IssuingAuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationStatus::*;
        match self {
            Closed => "closed",
            Pending => "pending",
            Reversed => "reversed",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationStatus::*;
        match s {
            "closed" => Ok(Closed),
            "pending" => Ok(Pending),
            "reversed" => Ok(Reversed),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingAuthorizationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationStatus"))
    }
}
impl stripe_types::Object for IssuingAuthorization {
    type Id = stripe_shared::issuing_authorization::IssuingAuthorizationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IssuingAuthorizationId, "iauth_");
