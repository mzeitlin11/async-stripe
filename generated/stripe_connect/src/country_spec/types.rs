/// Stripe needs to collect certain pieces of information about each account
/// created. These requirements can differ depending on the account's country. The
/// Country Specs API makes these rules available to your integration.
///
/// You can also view the information from this API call as [an online
/// guide](/docs/connect/required-verification-information).
///
/// For more details see <<https://stripe.com/docs/api/country_specs/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CountrySpec {
    /// The default currency for this country. This applies to both payment methods and bank accounts.
    pub default_currency: stripe_types::Currency,
    /// Unique identifier for the object. Represented as the ISO country code for this country.
    pub id: stripe_connect::CountrySpecId,
    /// Currencies that can be accepted in the specific country (for transfers).
    pub supported_bank_account_currencies: std::collections::HashMap<String, Vec<String>>,
    /// Currencies that can be accepted in the specified country (for payments).
    pub supported_payment_currencies: Vec<String>,
    /// Payment methods available in the specified country.
    /// You may need to enable some payment methods (e.g., [ACH](https://stripe.com/docs/ach)) on your account before they appear in this list.
    /// The `stripe` payment method refers to [charging through your platform](https://stripe.com/docs/connect/destination-charges).
    pub supported_payment_methods: Vec<String>,
    /// Countries that can accept transfers from the specified country.
    pub supported_transfer_countries: Vec<String>,
    pub verification_fields: stripe_connect::CountrySpecVerificationFields,
}
#[cfg(feature = "min-ser")]
pub struct CountrySpecBuilder {
    default_currency: Option<stripe_types::Currency>,
    id: Option<stripe_connect::CountrySpecId>,
    supported_bank_account_currencies: Option<std::collections::HashMap<String, Vec<String>>>,
    supported_payment_currencies: Option<Vec<String>>,
    supported_payment_methods: Option<Vec<String>>,
    supported_transfer_countries: Option<Vec<String>>,
    verification_fields: Option<stripe_connect::CountrySpecVerificationFields>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CountrySpec {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CountrySpec>,
        builder: CountrySpecBuilder,
    }

    impl Visitor for Place<CountrySpec> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CountrySpecBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CountrySpecBuilder {
        type Out = CountrySpec;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "default_currency" => Ok(Deserialize::begin(&mut self.default_currency)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "supported_bank_account_currencies" => Ok(Deserialize::begin(&mut self.supported_bank_account_currencies)),
                "supported_payment_currencies" => Ok(Deserialize::begin(&mut self.supported_payment_currencies)),
                "supported_payment_methods" => Ok(Deserialize::begin(&mut self.supported_payment_methods)),
                "supported_transfer_countries" => Ok(Deserialize::begin(&mut self.supported_transfer_countries)),
                "verification_fields" => Ok(Deserialize::begin(&mut self.verification_fields)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                default_currency: Deserialize::default(),
                id: Deserialize::default(),
                supported_bank_account_currencies: Deserialize::default(),
                supported_payment_currencies: Deserialize::default(),
                supported_payment_methods: Deserialize::default(),
                supported_transfer_countries: Deserialize::default(),
                verification_fields: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let default_currency = self.default_currency.take()?;
            let id = self.id.take()?;
            let supported_bank_account_currencies = self.supported_bank_account_currencies.take()?;
            let supported_payment_currencies = self.supported_payment_currencies.take()?;
            let supported_payment_methods = self.supported_payment_methods.take()?;
            let supported_transfer_countries = self.supported_transfer_countries.take()?;
            let verification_fields = self.verification_fields.take()?;

            Some(Self::Out { default_currency, id, supported_bank_account_currencies, supported_payment_currencies, supported_payment_methods, supported_transfer_countries, verification_fields })
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

    impl ObjectDeser for CountrySpec {
        type Builder = CountrySpecBuilder;
    }
};
impl stripe_types::Object for CountrySpec {
    type Id = stripe_connect::CountrySpecId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CountrySpecId);
