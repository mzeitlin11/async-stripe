/// `ExchangeRate` objects allow you to determine the rates that Stripe is currently
/// using to convert from one currency to another. Since this number is variable
/// throughout the day, there are various reasons why you might want to know the current
/// rate (for example, to dynamically price an item for a user with a default
/// payment in a foreign currency).
///
/// Please refer to our [Exchange Rates API](https://stripe.com/docs/fx-rates) guide for more details.
///
/// *[Note: this integration path is supported but no longer recommended]* Additionally,
/// you can guarantee that a charge is made with an exchange rate that you expect is
/// current. To do so, you must pass in the exchange_rate to charges endpoints. If the
/// value is no longer up to date, the charge won't go through. Please refer to our
/// [Using with charges](https://stripe.com/docs/exchange-rates) guide for more details.
///
/// -----
///
/// &nbsp;
///
/// *This Exchange Rates API is a Beta Service and is subject to Stripe's terms of service.
/// You may use the API solely for the purpose of transacting on Stripe.
/// For example, the API may be queried in order to:*.
///
/// - *localize prices for processing payments on Stripe*
/// - *reconcile Stripe transactions*
/// - *determine how much money to send to a connected account*
/// - *determine app fees to charge a connected account*
///
/// *Using this Exchange Rates API beta for any purpose other than to transact on Stripe is strictly prohibited and constitutes a violation of Stripe's terms of service.*.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ExchangeRate {
    /// Unique identifier for the object.
    /// Represented as the three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) in lowercase.
    pub id: stripe_misc::ExchangeRateId,
    /// Hash where the keys are supported currencies and the values are the exchange rate at which the base id currency converts to the key currency.
    pub rates: std::collections::HashMap<String, f64>,
}
#[cfg(feature = "min-ser")]
pub struct ExchangeRateBuilder {
    id: Option<stripe_misc::ExchangeRateId>,
    rates: Option<std::collections::HashMap<String, f64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ExchangeRate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ExchangeRate>,
        builder: ExchangeRateBuilder,
    }

    impl Visitor for Place<ExchangeRate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ExchangeRateBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ExchangeRateBuilder {
        type Out = ExchangeRate;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "rates" => Ok(Deserialize::begin(&mut self.rates)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), rates: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let id = self.id.take()?;
            let rates = self.rates.take()?;

            Some(Self::Out { id, rates })
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

    impl ObjectDeser for ExchangeRate {
        type Builder = ExchangeRateBuilder;
    }
};
impl stripe_types::Object for ExchangeRate {
    type Id = stripe_misc::ExchangeRateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ExchangeRateId);
