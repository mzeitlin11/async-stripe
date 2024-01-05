/// Any use of an [issued card](https://stripe.com/docs/issuing) that results in funds entering or leaving.
/// your Stripe account, such as a completed purchase or refund, is represented by an Issuing
/// `Transaction` object.
///
/// Related guide: [Issued card transactions](https://stripe.com/docs/issuing/purchases/transactions)
///
/// For more details see <<https://stripe.com/docs/api/issuing/transactions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingTransaction {
    /// The transaction amount, which will be reflected in your balance.
    /// This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_shared::IssuingTransactionAmountDetails>,
    /// The `Authorization` object that led to this transaction.
    pub authorization: Option<stripe_types::Expandable<stripe_shared::IssuingAuthorization>>,
    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// The card used to make this transaction.
    pub card: stripe_types::Expandable<stripe_shared::IssuingCard>,
    /// The cardholder to whom this transaction belongs.
    pub cardholder: Option<stripe_types::Expandable<stripe_shared::IssuingCardholder>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// If you've disputed the transaction, the ID of the dispute.
    pub dispute: Option<stripe_types::Expandable<stripe_shared::IssuingDispute>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,
    /// The currency with which the merchant is taking payment.
    pub merchant_currency: stripe_types::Currency,
    pub merchant_data: stripe_shared::IssuingAuthorizationMerchantData,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about the transaction, such as processing dates, set by the card network.
    pub network_data: Option<stripe_shared::IssuingTransactionNetworkData>,
    /// Additional purchase information that is optionally provided by the merchant.
    pub purchase_details: Option<stripe_shared::IssuingTransactionPurchaseDetails>,
    /// [Token](https://stripe.com/docs/api/issuing/tokens/object) object used for this transaction.
    /// If a network token was not used for this transaction, this field will be null.
    pub token: Option<stripe_types::Expandable<stripe_shared::IssuingToken>>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this transaction if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    pub treasury: Option<stripe_shared::IssuingTransactionTreasury>,
    /// The nature of the transaction.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: stripe_shared::IssuingTransactionType,
    /// The digital wallet used for this transaction. One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<IssuingTransactionWallet>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTransactionBuilder {
    amount: Option<i64>,
    amount_details: Option<Option<stripe_shared::IssuingTransactionAmountDetails>>,
    authorization: Option<Option<stripe_types::Expandable<stripe_shared::IssuingAuthorization>>>,
    balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    card: Option<stripe_types::Expandable<stripe_shared::IssuingCard>>,
    cardholder: Option<Option<stripe_types::Expandable<stripe_shared::IssuingCardholder>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    dispute: Option<Option<stripe_types::Expandable<stripe_shared::IssuingDispute>>>,
    id: Option<stripe_shared::IssuingTransactionId>,
    livemode: Option<bool>,
    merchant_amount: Option<i64>,
    merchant_currency: Option<stripe_types::Currency>,
    merchant_data: Option<stripe_shared::IssuingAuthorizationMerchantData>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network_data: Option<Option<stripe_shared::IssuingTransactionNetworkData>>,
    purchase_details: Option<Option<stripe_shared::IssuingTransactionPurchaseDetails>>,
    token: Option<Option<stripe_types::Expandable<stripe_shared::IssuingToken>>>,
    treasury: Option<Option<stripe_shared::IssuingTransactionTreasury>>,
    type_: Option<stripe_shared::IssuingTransactionType>,
    wallet: Option<Option<IssuingTransactionWallet>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransaction>,
        builder: IssuingTransactionBuilder,
    }

    impl Visitor for Place<IssuingTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTransactionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTransactionBuilder {
        type Out = IssuingTransaction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_details" => Ok(Deserialize::begin(&mut self.amount_details)),
                "authorization" => Ok(Deserialize::begin(&mut self.authorization)),
                "balance_transaction" => Ok(Deserialize::begin(&mut self.balance_transaction)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "cardholder" => Ok(Deserialize::begin(&mut self.cardholder)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "dispute" => Ok(Deserialize::begin(&mut self.dispute)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "merchant_amount" => Ok(Deserialize::begin(&mut self.merchant_amount)),
                "merchant_currency" => Ok(Deserialize::begin(&mut self.merchant_currency)),
                "merchant_data" => Ok(Deserialize::begin(&mut self.merchant_data)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "network_data" => Ok(Deserialize::begin(&mut self.network_data)),
                "purchase_details" => Ok(Deserialize::begin(&mut self.purchase_details)),
                "token" => Ok(Deserialize::begin(&mut self.token)),
                "treasury" => Ok(Deserialize::begin(&mut self.treasury)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "wallet" => Ok(Deserialize::begin(&mut self.wallet)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_details: Deserialize::default(),
                authorization: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                card: Deserialize::default(),
                cardholder: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                dispute: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                merchant_amount: Deserialize::default(),
                merchant_currency: Deserialize::default(),
                merchant_data: Deserialize::default(),
                metadata: Deserialize::default(),
                network_data: Deserialize::default(),
                purchase_details: Deserialize::default(),
                token: Deserialize::default(),
                treasury: Deserialize::default(),
                type_: Deserialize::default(),
                wallet: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_details = self.amount_details.take()?;
            let authorization = self.authorization.take()?;
            let balance_transaction = self.balance_transaction.take()?;
            let card = self.card.take()?;
            let cardholder = self.cardholder.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let dispute = self.dispute.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let merchant_amount = self.merchant_amount.take()?;
            let merchant_currency = self.merchant_currency.take()?;
            let merchant_data = self.merchant_data.take()?;
            let metadata = self.metadata.take()?;
            let network_data = self.network_data.take()?;
            let purchase_details = self.purchase_details.take()?;
            let token = self.token.take()?;
            let treasury = self.treasury.take()?;
            let type_ = self.type_.take()?;
            let wallet = self.wallet.take()?;

            Some(Self::Out {
                amount,
                amount_details,
                authorization,
                balance_transaction,
                card,
                cardholder,
                created,
                currency,
                dispute,
                id,
                livemode,
                merchant_amount,
                merchant_currency,
                merchant_data,
                metadata,
                network_data,
                purchase_details,
                token,
                treasury,
                type_,
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

    impl ObjectDeser for IssuingTransaction {
        type Builder = IssuingTransactionBuilder;
    }
};
/// The digital wallet used for this transaction. One of `apple_pay`, `google_pay`, or `samsung_pay`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTransactionWallet {
    ApplePay,
    GooglePay,
    SamsungPay,
}
impl IssuingTransactionWallet {
    pub fn as_str(self) -> &'static str {
        use IssuingTransactionWallet::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            SamsungPay => "samsung_pay",
        }
    }
}

impl std::str::FromStr for IssuingTransactionWallet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTransactionWallet::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "samsung_pay" => Ok(SamsungPay),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingTransactionWallet {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingTransactionWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTransactionWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTransactionWallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTransactionWallet {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingTransactionWallet"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingTransactionWallet {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingTransactionWallet> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTransactionWallet::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for IssuingTransaction {
    type Id = stripe_shared::IssuingTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IssuingTransactionId, "ipi_");
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTransactionType {
    Capture,
    Refund,
}
impl IssuingTransactionType {
    pub fn as_str(self) -> &'static str {
        use IssuingTransactionType::*;
        match self {
            Capture => "capture",
            Refund => "refund",
        }
    }
}

impl std::str::FromStr for IssuingTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTransactionType::*;
        match s {
            "capture" => Ok(Capture),
            "refund" => Ok(Refund),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingTransactionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
