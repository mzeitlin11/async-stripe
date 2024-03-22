/// When an [issued card](https://stripe.com/docs/issuing) is used to make a purchase, an Issuing `Authorization`.
/// object is created.
/// [Authorizations](https://stripe.com/docs/issuing/purchases/authorizations) must be approved for the.
/// purchase to be completed successfully.
///
/// Related guide: [Issued card authorizations](https://stripe.com/docs/issuing/purchases/authorizations).
///
/// For more details see <<https://stripe.com/docs/api/issuing/authorizations/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingAuthorization {
    /// The total amount that was authorized or rejected.
    /// This amount is in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// `amount` should be the same as `merchant_amount`, unless `currency` and `merchant_currency` are different.
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_shared::IssuingAuthorizationAmountDetails>,
    /// Whether the authorization has been approved.
    pub approved: bool,
    /// How the card details were provided.
    pub authorization_method: stripe_shared::IssuingAuthorizationAuthorizationMethod,
    /// List of balance transactions associated with this authorization.
    pub balance_transactions: Vec<stripe_shared::BalanceTransaction>,
    pub card: stripe_shared::IssuingCard,
    /// The cardholder to whom this authorization belongs.
    pub cardholder: Option<stripe_types::Expandable<stripe_shared::IssuingCardholder>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The currency of the cardholder.
    /// This currency can be different from the currency presented at authorization and the `merchant_currency` field on this authorization.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingAuthorizationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The total amount that was authorized or rejected.
    /// This amount is in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// `merchant_amount` should be the same as `amount`, unless `merchant_currency` and `currency` are different.
    pub merchant_amount: i64,
    /// The local currency that was presented to the cardholder for the authorization.
    /// This currency can be different from the cardholder currency and the `currency` field on this authorization.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: stripe_types::Currency,
    pub merchant_data: stripe_shared::IssuingAuthorizationMerchantData,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about the authorization, such as identifiers, set by the card network.
    pub network_data: Option<stripe_shared::IssuingAuthorizationNetworkData>,
    /// The pending authorization request.
    /// This field will only be non-null during an `issuing_authorization.request` webhook.
    pub pending_request: Option<stripe_shared::IssuingAuthorizationPendingRequest>,
    /// History of every time a `pending_request` authorization was approved/declined, either by you directly or by Stripe (e.g.
    /// based on your spending_controls).
    /// If the merchant changes the authorization by performing an incremental authorization, you can look at this field to see the previous requests for the authorization.
    /// This field can be helpful in determining why a given authorization was approved/declined.
    pub request_history: Vec<stripe_shared::IssuingAuthorizationRequest>,
    /// The current status of the authorization in its lifecycle.
    pub status: stripe_shared::IssuingAuthorizationStatus,
    /// [Token](https://stripe.com/docs/api/issuing/tokens/object) object used for this authorization.
    /// If a network token was not used for this authorization, this field will be null.
    pub token: Option<stripe_types::Expandable<stripe_shared::IssuingToken>>,
    /// List of [transactions](https://stripe.com/docs/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<stripe_shared::IssuingTransaction>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://stripe.com/docs/api/treasury/financial_accounts).
    pub treasury: Option<stripe_shared::IssuingAuthorizationTreasury>,
    pub verification_data: stripe_shared::IssuingAuthorizationVerificationData,
    /// The digital wallet used for this transaction.
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    /// Will populate as `null` when no digital wallet was utilized.
    pub wallet: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingAuthorizationBuilder {
    amount: Option<i64>,
    amount_details: Option<Option<stripe_shared::IssuingAuthorizationAmountDetails>>,
    approved: Option<bool>,
    authorization_method: Option<stripe_shared::IssuingAuthorizationAuthorizationMethod>,
    balance_transactions: Option<Vec<stripe_shared::BalanceTransaction>>,
    card: Option<stripe_shared::IssuingCard>,
    cardholder: Option<Option<stripe_types::Expandable<stripe_shared::IssuingCardholder>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    id: Option<stripe_shared::IssuingAuthorizationId>,
    livemode: Option<bool>,
    merchant_amount: Option<i64>,
    merchant_currency: Option<stripe_types::Currency>,
    merchant_data: Option<stripe_shared::IssuingAuthorizationMerchantData>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network_data: Option<Option<stripe_shared::IssuingAuthorizationNetworkData>>,
    pending_request: Option<Option<stripe_shared::IssuingAuthorizationPendingRequest>>,
    request_history: Option<Vec<stripe_shared::IssuingAuthorizationRequest>>,
    status: Option<stripe_shared::IssuingAuthorizationStatus>,
    token: Option<Option<stripe_types::Expandable<stripe_shared::IssuingToken>>>,
    transactions: Option<Vec<stripe_shared::IssuingTransaction>>,
    treasury: Option<Option<stripe_shared::IssuingAuthorizationTreasury>>,
    verification_data: Option<stripe_shared::IssuingAuthorizationVerificationData>,
    wallet: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingAuthorization {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorization>,
        builder: IssuingAuthorizationBuilder,
    }

    impl Visitor for Place<IssuingAuthorization> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingAuthorizationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingAuthorizationBuilder {
        type Out = IssuingAuthorization;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_details" => Ok(Deserialize::begin(&mut self.amount_details)),
                "approved" => Ok(Deserialize::begin(&mut self.approved)),
                "authorization_method" => Ok(Deserialize::begin(&mut self.authorization_method)),
                "balance_transactions" => Ok(Deserialize::begin(&mut self.balance_transactions)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "cardholder" => Ok(Deserialize::begin(&mut self.cardholder)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "merchant_amount" => Ok(Deserialize::begin(&mut self.merchant_amount)),
                "merchant_currency" => Ok(Deserialize::begin(&mut self.merchant_currency)),
                "merchant_data" => Ok(Deserialize::begin(&mut self.merchant_data)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "network_data" => Ok(Deserialize::begin(&mut self.network_data)),
                "pending_request" => Ok(Deserialize::begin(&mut self.pending_request)),
                "request_history" => Ok(Deserialize::begin(&mut self.request_history)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "token" => Ok(Deserialize::begin(&mut self.token)),
                "transactions" => Ok(Deserialize::begin(&mut self.transactions)),
                "treasury" => Ok(Deserialize::begin(&mut self.treasury)),
                "verification_data" => Ok(Deserialize::begin(&mut self.verification_data)),
                "wallet" => Ok(Deserialize::begin(&mut self.wallet)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_details: Deserialize::default(),
                approved: Deserialize::default(),
                authorization_method: Deserialize::default(),
                balance_transactions: Deserialize::default(),
                card: Deserialize::default(),
                cardholder: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                merchant_amount: Deserialize::default(),
                merchant_currency: Deserialize::default(),
                merchant_data: Deserialize::default(),
                metadata: Deserialize::default(),
                network_data: Deserialize::default(),
                pending_request: Deserialize::default(),
                request_history: Deserialize::default(),
                status: Deserialize::default(),
                token: Deserialize::default(),
                transactions: Deserialize::default(),
                treasury: Deserialize::default(),
                verification_data: Deserialize::default(),
                wallet: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_details = self.amount_details.take()?;
            let approved = self.approved.take()?;
            let authorization_method = self.authorization_method.take()?;
            let balance_transactions = self.balance_transactions.take()?;
            let card = self.card.take()?;
            let cardholder = self.cardholder.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let merchant_amount = self.merchant_amount.take()?;
            let merchant_currency = self.merchant_currency.take()?;
            let merchant_data = self.merchant_data.take()?;
            let metadata = self.metadata.take()?;
            let network_data = self.network_data.take()?;
            let pending_request = self.pending_request.take()?;
            let request_history = self.request_history.take()?;
            let status = self.status.take()?;
            let token = self.token.take()?;
            let transactions = self.transactions.take()?;
            let treasury = self.treasury.take()?;
            let verification_data = self.verification_data.take()?;
            let wallet = self.wallet.take()?;

            Some(Self::Out {
                amount,
                amount_details,
                approved,
                authorization_method,
                balance_transactions,
                card,
                cardholder,
                created,
                currency,
                id,
                livemode,
                merchant_amount,
                merchant_currency,
                merchant_data,
                metadata,
                network_data,
                pending_request,
                request_history,
                status,
                token,
                transactions,
                treasury,
                verification_data,
                wallet,
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

    impl ObjectDeser for IssuingAuthorization {
        type Builder = IssuingAuthorizationBuilder;
    }
};
impl stripe_types::Object for IssuingAuthorization {
    type Id = stripe_shared::IssuingAuthorizationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IssuingAuthorizationId, "iauth_");
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
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationAuthorizationMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingAuthorizationAuthorizationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationAuthorizationMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationAuthorizationMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
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
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingAuthorizationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
