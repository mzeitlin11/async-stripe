/// `Source` objects allow you to accept a variety of payment methods.
///
/// They represent a customer's payment instrument, and can be used with the Stripe API just like a `Card` object: once chargeable, they can be charged, or can be attached to customers.  Stripe doesn't recommend using the deprecated [Sources API](https://stripe.com/docs/api/sources). We recommend that you adopt the [PaymentMethods API](https://stripe.com/docs/api/payment_methods). This newer API provides access to our latest features and payment method types.  Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).  For more details see <<https://stripe.com/docs/api/sources/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<stripe_shared::SourceTypeAchCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<stripe_shared::SourceTypeAchDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_shared::SourceTypeAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_shared::SourceTypeAlipay>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_shared::SourceTypeAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_shared::SourceTypeBancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_shared::SourceTypeCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_shared::SourceTypeCardPresent>,
    /// The client secret of the source.
    ///
    /// Used for client-side retrieval using a publishable key.
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verification: Option<stripe_shared::SourceCodeVerificationFlow>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    ///
    /// This is the currency for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub currency: Option<stripe_types::Currency>,
    /// The ID of the customer to which this source is attached.
    ///
    /// This will not be present when the source has not been attached to a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_shared::SourceTypeEps>,
    /// The authentication `flow` of the source.
    ///
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_shared::SourceTypeGiropay>,
    /// Unique identifier for the object.
    pub id: stripe_shared::source::SourceId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_shared::SourceTypeIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_shared::SourceTypeKlarna>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_shared::SourceTypeMultibanco>,
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub owner: Option<stripe_shared::SourceOwner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_shared::SourceTypeP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<stripe_shared::SourceReceiverFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<stripe_shared::SourceRedirectFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<stripe_shared::SourceTypeSepaCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_shared::SourceTypeSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_shared::SourceTypeSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<stripe_shared::SourceOrder>,
    /// Extra information about a source.
    ///
    /// This will appear on your customer's statement every time you charge the source.
    pub statement_descriptor: Option<String>,
    /// The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`.
    ///
    /// Only `chargeable` sources can be used to create a charge.
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<stripe_shared::SourceTypeThreeDSecure>,
    /// The `type` of the source.
    ///
    /// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
    /// An additional hash is included on the source with a name matching this value.
    /// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
    #[serde(rename = "type")]
    pub type_: SourceType,
    /// Either `reusable` or `single_use`.
    ///
    /// Whether this source should be reusable or not.
    /// Some source types may or may not be reusable by construction, while others may leave the option at creation.
    /// If an incompatible value is passed, an error will be returned.
    pub usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<stripe_shared::SourceTypeWechat>,
}
/// The `type` of the source.
///
/// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
/// An additional hash is included on the source with a name matching this value.
/// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SourceType {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    Alipay,
    AuBecsDebit,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl SourceType {
    pub fn as_str(self) -> &'static str {
        use SourceType::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            AcssDebit => "acss_debit",
            Alipay => "alipay",
            AuBecsDebit => "au_becs_debit",
            Bancontact => "bancontact",
            Card => "card",
            CardPresent => "card_present",
            Eps => "eps",
            Giropay => "giropay",
            Ideal => "ideal",
            Klarna => "klarna",
            Multibanco => "multibanco",
            P24 => "p24",
            SepaCreditTransfer => "sepa_credit_transfer",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            ThreeDSecure => "three_d_secure",
            Wechat => "wechat",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for SourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SourceType::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "acss_debit" => Ok(AcssDebit),
            "alipay" => Ok(Alipay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bancontact" => Ok(Bancontact),
            "card" => Ok(Card),
            "card_present" => Ok(CardPresent),
            "eps" => Ok(Eps),
            "giropay" => Ok(Giropay),
            "ideal" => Ok(Ideal),
            "klarna" => Ok(Klarna),
            "multibanco" => Ok(Multibanco),
            "p24" => Ok(P24),
            "sepa_credit_transfer" => Ok(SepaCreditTransfer),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "three_d_secure" => Ok(ThreeDSecure),
            "wechat" => Ok(Wechat),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(SourceType::Unknown))
    }
}
impl stripe_types::Object for Source {
    type Id = stripe_shared::source::SourceId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SourceId, "src_");
