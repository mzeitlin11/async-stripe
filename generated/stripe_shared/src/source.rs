/// `Source` objects allow you to accept a variety of payment methods. They
/// represent a customer's payment instrument, and can be used with the Stripe API
/// just like a `Card` object: once chargeable, they can be charged, or can be
/// attached to customers.
///
/// Stripe doesn't recommend using the deprecated [Sources API](https://stripe.com/docs/api/sources).
/// We recommend that you adopt the [PaymentMethods API](https://stripe.com/docs/api/payment_methods).
/// This newer API provides access to our latest features and payment method types.
///
/// Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).
///
/// For more details see <<https://stripe.com/docs/api/sources/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Source {
    pub ach_credit_transfer: Option<stripe_shared::SourceTypeAchCreditTransfer>,
    pub ach_debit: Option<stripe_shared::SourceTypeAchDebit>,
    pub acss_debit: Option<stripe_shared::SourceTypeAcssDebit>,
    pub alipay: Option<stripe_shared::SourceTypeAlipay>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source.
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub amount: Option<i64>,
    pub au_becs_debit: Option<stripe_shared::SourceTypeAuBecsDebit>,
    pub bancontact: Option<stripe_shared::SourceTypeBancontact>,
    pub card: Option<stripe_shared::SourceTypeCard>,
    pub card_present: Option<stripe_shared::SourceTypeCardPresent>,
    /// The client secret of the source. Used for client-side retrieval using a publishable key.
    pub client_secret: String,
    pub code_verification: Option<stripe_shared::SourceCodeVerificationFlow>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    /// This is the currency for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub currency: Option<stripe_types::Currency>,
    /// The ID of the customer to which this source is attached.
    /// This will not be present when the source has not been attached to a customer.
    pub customer: Option<String>,
    pub eps: Option<stripe_shared::SourceTypeEps>,
    /// The authentication `flow` of the source.
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: String,
    pub giropay: Option<stripe_shared::SourceTypeGiropay>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SourceId,
    pub ideal: Option<stripe_shared::SourceTypeIdeal>,
    pub klarna: Option<stripe_shared::SourceTypeKlarna>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub multibanco: Option<stripe_shared::SourceTypeMultibanco>,
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub owner: Option<stripe_shared::SourceOwner>,
    pub p24: Option<stripe_shared::SourceTypeP24>,
    pub receiver: Option<stripe_shared::SourceReceiverFlow>,
    pub redirect: Option<stripe_shared::SourceRedirectFlow>,
    pub sepa_credit_transfer: Option<stripe_shared::SourceTypeSepaCreditTransfer>,
    pub sepa_debit: Option<stripe_shared::SourceTypeSepaDebit>,
    pub sofort: Option<stripe_shared::SourceTypeSofort>,
    pub source_order: Option<stripe_shared::SourceOrder>,
    /// Extra information about a source.
    /// This will appear on your customer's statement every time you charge the source.
    pub statement_descriptor: Option<String>,
    /// The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`.
    /// Only `chargeable` sources can be used to create a charge.
    pub status: String,
    pub three_d_secure: Option<stripe_shared::SourceTypeThreeDSecure>,
    /// The `type` of the source.
    /// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
    /// An additional hash is included on the source with a name matching this value.
    /// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: SourceType,
    /// Either `reusable` or `single_use`.
    /// Whether this source should be reusable or not.
    /// Some source types may or may not be reusable by construction, while others may leave the option at creation.
    /// If an incompatible value is passed, an error will be returned.
    pub usage: Option<String>,
    pub wechat: Option<stripe_shared::SourceTypeWechat>,
}
#[cfg(feature = "min-ser")]
pub struct SourceBuilder {
    ach_credit_transfer: Option<Option<stripe_shared::SourceTypeAchCreditTransfer>>,
    ach_debit: Option<Option<stripe_shared::SourceTypeAchDebit>>,
    acss_debit: Option<Option<stripe_shared::SourceTypeAcssDebit>>,
    alipay: Option<Option<stripe_shared::SourceTypeAlipay>>,
    amount: Option<Option<i64>>,
    au_becs_debit: Option<Option<stripe_shared::SourceTypeAuBecsDebit>>,
    bancontact: Option<Option<stripe_shared::SourceTypeBancontact>>,
    card: Option<Option<stripe_shared::SourceTypeCard>>,
    card_present: Option<Option<stripe_shared::SourceTypeCardPresent>>,
    client_secret: Option<String>,
    code_verification: Option<Option<stripe_shared::SourceCodeVerificationFlow>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<Option<stripe_types::Currency>>,
    customer: Option<Option<String>>,
    eps: Option<Option<stripe_shared::SourceTypeEps>>,
    flow: Option<String>,
    giropay: Option<Option<stripe_shared::SourceTypeGiropay>>,
    id: Option<stripe_shared::SourceId>,
    ideal: Option<Option<stripe_shared::SourceTypeIdeal>>,
    klarna: Option<Option<stripe_shared::SourceTypeKlarna>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    multibanco: Option<Option<stripe_shared::SourceTypeMultibanco>>,
    owner: Option<Option<stripe_shared::SourceOwner>>,
    p24: Option<Option<stripe_shared::SourceTypeP24>>,
    receiver: Option<Option<stripe_shared::SourceReceiverFlow>>,
    redirect: Option<Option<stripe_shared::SourceRedirectFlow>>,
    sepa_credit_transfer: Option<Option<stripe_shared::SourceTypeSepaCreditTransfer>>,
    sepa_debit: Option<Option<stripe_shared::SourceTypeSepaDebit>>,
    sofort: Option<Option<stripe_shared::SourceTypeSofort>>,
    source_order: Option<Option<stripe_shared::SourceOrder>>,
    statement_descriptor: Option<Option<String>>,
    status: Option<String>,
    three_d_secure: Option<Option<stripe_shared::SourceTypeThreeDSecure>>,
    type_: Option<SourceType>,
    usage: Option<Option<String>>,
    wechat: Option<Option<stripe_shared::SourceTypeWechat>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Source {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Source>,
        builder: SourceBuilder,
    }

    impl Visitor for Place<Source> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceBuilder {
        type Out = Source;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "ach_credit_transfer" => Ok(Deserialize::begin(&mut self.ach_credit_transfer)),
                "ach_debit" => Ok(Deserialize::begin(&mut self.ach_debit)),
                "acss_debit" => Ok(Deserialize::begin(&mut self.acss_debit)),
                "alipay" => Ok(Deserialize::begin(&mut self.alipay)),
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "au_becs_debit" => Ok(Deserialize::begin(&mut self.au_becs_debit)),
                "bancontact" => Ok(Deserialize::begin(&mut self.bancontact)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "card_present" => Ok(Deserialize::begin(&mut self.card_present)),
                "client_secret" => Ok(Deserialize::begin(&mut self.client_secret)),
                "code_verification" => Ok(Deserialize::begin(&mut self.code_verification)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "eps" => Ok(Deserialize::begin(&mut self.eps)),
                "flow" => Ok(Deserialize::begin(&mut self.flow)),
                "giropay" => Ok(Deserialize::begin(&mut self.giropay)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "ideal" => Ok(Deserialize::begin(&mut self.ideal)),
                "klarna" => Ok(Deserialize::begin(&mut self.klarna)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "multibanco" => Ok(Deserialize::begin(&mut self.multibanco)),
                "owner" => Ok(Deserialize::begin(&mut self.owner)),
                "p24" => Ok(Deserialize::begin(&mut self.p24)),
                "receiver" => Ok(Deserialize::begin(&mut self.receiver)),
                "redirect" => Ok(Deserialize::begin(&mut self.redirect)),
                "sepa_credit_transfer" => Ok(Deserialize::begin(&mut self.sepa_credit_transfer)),
                "sepa_debit" => Ok(Deserialize::begin(&mut self.sepa_debit)),
                "sofort" => Ok(Deserialize::begin(&mut self.sofort)),
                "source_order" => Ok(Deserialize::begin(&mut self.source_order)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "three_d_secure" => Ok(Deserialize::begin(&mut self.three_d_secure)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "usage" => Ok(Deserialize::begin(&mut self.usage)),
                "wechat" => Ok(Deserialize::begin(&mut self.wechat)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                ach_credit_transfer: Deserialize::default(),
                ach_debit: Deserialize::default(),
                acss_debit: Deserialize::default(),
                alipay: Deserialize::default(),
                amount: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                client_secret: Deserialize::default(),
                code_verification: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                eps: Deserialize::default(),
                flow: Deserialize::default(),
                giropay: Deserialize::default(),
                id: Deserialize::default(),
                ideal: Deserialize::default(),
                klarna: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                multibanco: Deserialize::default(),
                owner: Deserialize::default(),
                p24: Deserialize::default(),
                receiver: Deserialize::default(),
                redirect: Deserialize::default(),
                sepa_credit_transfer: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                source_order: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                three_d_secure: Deserialize::default(),
                type_: Deserialize::default(),
                usage: Deserialize::default(),
                wechat: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let ach_credit_transfer = self.ach_credit_transfer.take()?;
            let ach_debit = self.ach_debit.take()?;
            let acss_debit = self.acss_debit.take()?;
            let alipay = self.alipay.take()?;
            let amount = self.amount.take()?;
            let au_becs_debit = self.au_becs_debit.take()?;
            let bancontact = self.bancontact.take()?;
            let card = self.card.take()?;
            let card_present = self.card_present.take()?;
            let client_secret = self.client_secret.take()?;
            let code_verification = self.code_verification.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let customer = self.customer.take()?;
            let eps = self.eps.take()?;
            let flow = self.flow.take()?;
            let giropay = self.giropay.take()?;
            let id = self.id.take()?;
            let ideal = self.ideal.take()?;
            let klarna = self.klarna.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let multibanco = self.multibanco.take()?;
            let owner = self.owner.take()?;
            let p24 = self.p24.take()?;
            let receiver = self.receiver.take()?;
            let redirect = self.redirect.take()?;
            let sepa_credit_transfer = self.sepa_credit_transfer.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let sofort = self.sofort.take()?;
            let source_order = self.source_order.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;
            let status = self.status.take()?;
            let three_d_secure = self.three_d_secure.take()?;
            let type_ = self.type_.take()?;
            let usage = self.usage.take()?;
            let wechat = self.wechat.take()?;

            Some(Self::Out {
                ach_credit_transfer,
                ach_debit,
                acss_debit,
                alipay,
                amount,
                au_becs_debit,
                bancontact,
                card,
                card_present,
                client_secret,
                code_verification,
                created,
                currency,
                customer,
                eps,
                flow,
                giropay,
                id,
                ideal,
                klarna,
                livemode,
                metadata,
                multibanco,
                owner,
                p24,
                receiver,
                redirect,
                sepa_credit_transfer,
                sepa_debit,
                sofort,
                source_order,
                statement_descriptor,
                status,
                three_d_secure,
                type_,
                usage,
                wechat,
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

    impl ObjectDeser for Source {
        type Builder = SourceBuilder;
    }
};
/// The `type` of the source.
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
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SourceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SourceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SourceType::from_str(s).unwrap_or(SourceType::Unknown));
        Ok(())
    }
}
impl stripe_types::Object for Source {
    type Id = stripe_shared::SourceId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SourceId, "src_");
