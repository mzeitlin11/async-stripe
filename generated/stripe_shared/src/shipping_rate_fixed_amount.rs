#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ShippingRateFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options: Option<std::collections::HashMap<stripe_types::Currency, stripe_shared::ShippingRateCurrencyOption>>,
}
#[cfg(feature = "min-ser")]
pub struct ShippingRateFixedAmountBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    currency_options: Option<Option<std::collections::HashMap<stripe_types::Currency, stripe_shared::ShippingRateCurrencyOption>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ShippingRateFixedAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ShippingRateFixedAmount>,
        builder: ShippingRateFixedAmountBuilder,
    }

    impl Visitor for Place<ShippingRateFixedAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ShippingRateFixedAmountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ShippingRateFixedAmountBuilder {
        type Out = ShippingRateFixedAmount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "currency_options" => Ok(Deserialize::begin(&mut self.currency_options)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), currency: Deserialize::default(), currency_options: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let currency = self.currency.take()?;
            let currency_options = self.currency_options.take()?;

            Some(Self::Out { amount, currency, currency_options })
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

    impl ObjectDeser for ShippingRateFixedAmount {
        type Builder = ShippingRateFixedAmountBuilder;
    }
};
