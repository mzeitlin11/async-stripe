/// Some payment methods have no required amount that a customer must send.
/// Customers can be instructed to send any amount, and it can be made up of
/// multiple transactions. As such, sources can have multiple associated
/// transactions.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTransaction {
    pub ach_credit_transfer: Option<stripe_shared::SourceTransactionAchCreditTransferData>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,
    pub chf_credit_transfer: Option<stripe_shared::SourceTransactionChfCreditTransferData>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    pub gbp_credit_transfer: Option<stripe_shared::SourceTransactionGbpCreditTransferData>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SourceTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub paper_check: Option<stripe_shared::SourceTransactionPaperCheckData>,
    pub sepa_credit_transfer: Option<stripe_shared::SourceTransactionSepaCreditTransferData>,
    /// The ID of the source this transaction is attached to.
    pub source: String,
    /// The status of the transaction, one of `succeeded`, `pending`, or `failed`.
    pub status: String,
    /// The type of source this transaction is attached to.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: SourceTransactionType,
}
#[cfg(feature = "min-ser")]
pub struct SourceTransactionBuilder {
    ach_credit_transfer: Option<Option<stripe_shared::SourceTransactionAchCreditTransferData>>,
    amount: Option<i64>,
    chf_credit_transfer: Option<Option<stripe_shared::SourceTransactionChfCreditTransferData>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    gbp_credit_transfer: Option<Option<stripe_shared::SourceTransactionGbpCreditTransferData>>,
    id: Option<stripe_shared::SourceTransactionId>,
    livemode: Option<bool>,
    paper_check: Option<Option<stripe_shared::SourceTransactionPaperCheckData>>,
    sepa_credit_transfer: Option<Option<stripe_shared::SourceTransactionSepaCreditTransferData>>,
    source: Option<String>,
    status: Option<String>,
    type_: Option<SourceTransactionType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransaction>,
        builder: SourceTransactionBuilder,
    }

    impl Visitor for Place<SourceTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTransactionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTransactionBuilder {
        type Out = SourceTransaction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "ach_credit_transfer" => Ok(Deserialize::begin(&mut self.ach_credit_transfer)),
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "chf_credit_transfer" => Ok(Deserialize::begin(&mut self.chf_credit_transfer)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "gbp_credit_transfer" => Ok(Deserialize::begin(&mut self.gbp_credit_transfer)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "paper_check" => Ok(Deserialize::begin(&mut self.paper_check)),
                "sepa_credit_transfer" => Ok(Deserialize::begin(&mut self.sepa_credit_transfer)),
                "source" => Ok(Deserialize::begin(&mut self.source)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                ach_credit_transfer: Deserialize::default(),
                amount: Deserialize::default(),
                chf_credit_transfer: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                gbp_credit_transfer: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                paper_check: Deserialize::default(),
                sepa_credit_transfer: Deserialize::default(),
                source: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let ach_credit_transfer = self.ach_credit_transfer.take()?;
            let amount = self.amount.take()?;
            let chf_credit_transfer = self.chf_credit_transfer.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let gbp_credit_transfer = self.gbp_credit_transfer.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let paper_check = self.paper_check.take()?;
            let sepa_credit_transfer = self.sepa_credit_transfer.take()?;
            let source = self.source.take()?;
            let status = self.status.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { ach_credit_transfer, amount, chf_credit_transfer, created, currency, gbp_credit_transfer, id, livemode, paper_check, sepa_credit_transfer, source, status, type_ })
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

    impl ObjectDeser for SourceTransaction {
        type Builder = SourceTransactionBuilder;
    }
};
/// The type of source this transaction is attached to.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SourceTransactionType {
    AchCreditTransfer,
    AchDebit,
    Alipay,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl SourceTransactionType {
    pub fn as_str(self) -> &'static str {
        use SourceTransactionType::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            Alipay => "alipay",
            Bancontact => "bancontact",
            Card => "card",
            CardPresent => "card_present",
            Eps => "eps",
            Giropay => "giropay",
            Ideal => "ideal",
            Klarna => "klarna",
            Multibanco => "multibanco",
            P24 => "p24",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            ThreeDSecure => "three_d_secure",
            Wechat => "wechat",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for SourceTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SourceTransactionType::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "alipay" => Ok(Alipay),
            "bancontact" => Ok(Bancontact),
            "card" => Ok(Card),
            "card_present" => Ok(CardPresent),
            "eps" => Ok(Eps),
            "giropay" => Ok(Giropay),
            "ideal" => Ok(Ideal),
            "klarna" => Ok(Klarna),
            "multibanco" => Ok(Multibanco),
            "p24" => Ok(P24),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "three_d_secure" => Ok(ThreeDSecure),
            "wechat" => Ok(Wechat),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SourceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SourceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SourceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SourceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SourceTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SourceTransactionType::from_str(s).unwrap_or(SourceTransactionType::Unknown));
        Ok(())
    }
}
impl stripe_types::Object for SourceTransaction {
    type Id = stripe_shared::SourceTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SourceTransactionId);
