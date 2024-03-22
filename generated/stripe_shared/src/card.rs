/// You can store multiple cards on a customer in order to charge the customer
/// later. You can also store multiple debit cards on a recipient in order to
/// transfer to those cards later.
///
/// Related guide: [Card payments with Sources](https://stripe.com/docs/sources/cards)
///
/// For more details see <<https://stripe.com/docs/api/cards/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Card {
    /// The account this card belongs to.
    /// This attribute will not be in the card object if the card belongs to a customer or recipient instead.
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// City/District/Suburb/Town/Village.
    pub address_city: Option<String>,
    /// Billing address country, if provided when creating card.
    pub address_country: Option<String>,
    /// Address line 1 (Street address/PO Box/Company name).
    pub address_line1: Option<String>,
    /// If `address_line1` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,
    /// Address line 2 (Apartment/Suite/Unit/Building).
    pub address_line2: Option<String>,
    /// State/County/Province/Region.
    pub address_state: Option<String>,
    /// ZIP or postal code.
    pub address_zip: Option<String>,
    /// If `address_zip` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_zip_check: Option<String>,
    /// A set of available payout methods for this card.
    /// Only values from this set should be passed as the `method` when creating a payout.
    pub available_payout_methods: Option<Vec<CardAvailablePayoutMethods>>,
    /// Card brand.
    /// Can be `American Express`, `Diners Club`, `Discover`, `Eftpos Australia`, `JCB`, `MasterCard`, `UnionPay`, `Visa`, or `Unknown`.
    pub brand: String,
    /// Two-letter ISO code representing the country of the card.
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    /// Three-letter [ISO code for currency](https://stripe.com/docs/payouts).
    /// Only applicable on accounts (not customers or recipients).
    /// The card can be used as a transfer destination for funds in this currency.
    pub currency: Option<stripe_types::Currency>,
    /// The customer that this card belongs to.
    /// This attribute will not be in the card object if the card belongs to an account or recipient instead.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// If a CVC was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    /// A result of unchecked indicates that CVC was provided but hasn't been checked yet.
    /// Checks are typically performed when attaching a card to a Customer object, or when creating a charge.
    /// For more details, see [Check if a card is valid without a charge](https://support.stripe.com/questions/check-if-a-card-is-valid-without-a-charge).
    pub cvc_check: Option<String>,
    /// Whether this card is the default external account for its currency.
    pub default_for_currency: Option<bool>,
    /// A high-level description of the type of cards issued in this range.
    /// (For internal use only and not typically available in standard API requests.).
    pub description: Option<String>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /// Uniquely identifies this particular card number.
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.
    ///
    /// *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    pub fingerprint: Option<String>,
    /// Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::CardId,
    /// Issuer identification number of the card.
    /// (For internal use only and not typically available in standard API requests.).
    pub iin: Option<String>,
    /// The name of the card's issuing bank.
    /// (For internal use only and not typically available in standard API requests.).
    pub issuer: Option<String>,
    /// The last four digits of the card.
    pub last4: String,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Cardholder name.
    pub name: Option<String>,
    /// For external accounts that are cards, possible values are `new` and `errored`.
    /// If a payout fails, the status is set to `errored` and [scheduled payouts](https://stripe.com/docs/payouts#payout-schedule) are stopped until account details are updated.
    pub status: Option<String>,
    /// If the card number is tokenized, this is the method that was used.
    /// Can be `android_pay` (includes Google Pay), `apple_pay`, `masterpass`, `visa_checkout`, or null.
    pub tokenization_method: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct CardBuilder {
    account: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    address_city: Option<Option<String>>,
    address_country: Option<Option<String>>,
    address_line1: Option<Option<String>>,
    address_line1_check: Option<Option<String>>,
    address_line2: Option<Option<String>>,
    address_state: Option<Option<String>>,
    address_zip: Option<Option<String>>,
    address_zip_check: Option<Option<String>>,
    available_payout_methods: Option<Option<Vec<CardAvailablePayoutMethods>>>,
    brand: Option<String>,
    country: Option<Option<String>>,
    currency: Option<Option<stripe_types::Currency>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    cvc_check: Option<Option<String>>,
    default_for_currency: Option<Option<bool>>,
    description: Option<Option<String>>,
    dynamic_last4: Option<Option<String>>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    fingerprint: Option<Option<String>>,
    funding: Option<String>,
    id: Option<stripe_shared::CardId>,
    iin: Option<Option<String>>,
    issuer: Option<Option<String>>,
    last4: Option<String>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    name: Option<Option<String>>,
    status: Option<Option<String>>,
    tokenization_method: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Card {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Card>,
        builder: CardBuilder,
    }

    impl Visitor for Place<Card> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CardBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CardBuilder {
        type Out = Card;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account" => Ok(Deserialize::begin(&mut self.account)),
                "address_city" => Ok(Deserialize::begin(&mut self.address_city)),
                "address_country" => Ok(Deserialize::begin(&mut self.address_country)),
                "address_line1" => Ok(Deserialize::begin(&mut self.address_line1)),
                "address_line1_check" => Ok(Deserialize::begin(&mut self.address_line1_check)),
                "address_line2" => Ok(Deserialize::begin(&mut self.address_line2)),
                "address_state" => Ok(Deserialize::begin(&mut self.address_state)),
                "address_zip" => Ok(Deserialize::begin(&mut self.address_zip)),
                "address_zip_check" => Ok(Deserialize::begin(&mut self.address_zip_check)),
                "available_payout_methods" => Ok(Deserialize::begin(&mut self.available_payout_methods)),
                "brand" => Ok(Deserialize::begin(&mut self.brand)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "cvc_check" => Ok(Deserialize::begin(&mut self.cvc_check)),
                "default_for_currency" => Ok(Deserialize::begin(&mut self.default_for_currency)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "dynamic_last4" => Ok(Deserialize::begin(&mut self.dynamic_last4)),
                "exp_month" => Ok(Deserialize::begin(&mut self.exp_month)),
                "exp_year" => Ok(Deserialize::begin(&mut self.exp_year)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "funding" => Ok(Deserialize::begin(&mut self.funding)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "iin" => Ok(Deserialize::begin(&mut self.iin)),
                "issuer" => Ok(Deserialize::begin(&mut self.issuer)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "tokenization_method" => Ok(Deserialize::begin(&mut self.tokenization_method)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                address_city: Deserialize::default(),
                address_country: Deserialize::default(),
                address_line1: Deserialize::default(),
                address_line1_check: Deserialize::default(),
                address_line2: Deserialize::default(),
                address_state: Deserialize::default(),
                address_zip: Deserialize::default(),
                address_zip_check: Deserialize::default(),
                available_payout_methods: Deserialize::default(),
                brand: Deserialize::default(),
                country: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                cvc_check: Deserialize::default(),
                default_for_currency: Deserialize::default(),
                description: Deserialize::default(),
                dynamic_last4: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                fingerprint: Deserialize::default(),
                funding: Deserialize::default(),
                id: Deserialize::default(),
                iin: Deserialize::default(),
                issuer: Deserialize::default(),
                last4: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
                status: Deserialize::default(),
                tokenization_method: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account = self.account.take()?;
            let address_city = self.address_city.take()?;
            let address_country = self.address_country.take()?;
            let address_line1 = self.address_line1.take()?;
            let address_line1_check = self.address_line1_check.take()?;
            let address_line2 = self.address_line2.take()?;
            let address_state = self.address_state.take()?;
            let address_zip = self.address_zip.take()?;
            let address_zip_check = self.address_zip_check.take()?;
            let available_payout_methods = self.available_payout_methods.take()?;
            let brand = self.brand.take()?;
            let country = self.country.take()?;
            let currency = self.currency.take()?;
            let customer = self.customer.take()?;
            let cvc_check = self.cvc_check.take()?;
            let default_for_currency = self.default_for_currency.take()?;
            let description = self.description.take()?;
            let dynamic_last4 = self.dynamic_last4.take()?;
            let exp_month = self.exp_month.take()?;
            let exp_year = self.exp_year.take()?;
            let fingerprint = self.fingerprint.take()?;
            let funding = self.funding.take()?;
            let id = self.id.take()?;
            let iin = self.iin.take()?;
            let issuer = self.issuer.take()?;
            let last4 = self.last4.take()?;
            let metadata = self.metadata.take()?;
            let name = self.name.take()?;
            let status = self.status.take()?;
            let tokenization_method = self.tokenization_method.take()?;

            Some(Self::Out {
                account,
                address_city,
                address_country,
                address_line1,
                address_line1_check,
                address_line2,
                address_state,
                address_zip,
                address_zip_check,
                available_payout_methods,
                brand,
                country,
                currency,
                customer,
                cvc_check,
                default_for_currency,
                description,
                dynamic_last4,
                exp_month,
                exp_year,
                fingerprint,
                funding,
                id,
                iin,
                issuer,
                last4,
                metadata,
                name,
                status,
                tokenization_method,
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

    impl ObjectDeser for Card {
        type Builder = CardBuilder;
    }
};
/// A set of available payout methods for this card.
/// Only values from this set should be passed as the `method` when creating a payout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CardAvailablePayoutMethods {
    Instant,
    Standard,
}
impl CardAvailablePayoutMethods {
    pub fn as_str(self) -> &'static str {
        use CardAvailablePayoutMethods::*;
        match self {
            Instant => "instant",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CardAvailablePayoutMethods {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardAvailablePayoutMethods::*;
        match s {
            "instant" => Ok(Instant),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CardAvailablePayoutMethods {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CardAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CardAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CardAvailablePayoutMethods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardAvailablePayoutMethods {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CardAvailablePayoutMethods"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CardAvailablePayoutMethods {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CardAvailablePayoutMethods> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardAvailablePayoutMethods::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Card {
    type Id = stripe_shared::CardId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CardId, "card_");
