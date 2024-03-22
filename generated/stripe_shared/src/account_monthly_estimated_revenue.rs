#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountMonthlyEstimatedRevenue {
    /// A non-negative integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
}
#[cfg(feature = "min-ser")]
pub struct AccountMonthlyEstimatedRevenueBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountMonthlyEstimatedRevenue {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountMonthlyEstimatedRevenue>,
        builder: AccountMonthlyEstimatedRevenueBuilder,
    }

    impl Visitor for Place<AccountMonthlyEstimatedRevenue> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountMonthlyEstimatedRevenueBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountMonthlyEstimatedRevenueBuilder {
        type Out = AccountMonthlyEstimatedRevenue;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), currency: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let currency = self.currency.take()?;

            Some(Self::Out { amount, currency })
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

    impl ObjectDeser for AccountMonthlyEstimatedRevenue {
        type Builder = AccountMonthlyEstimatedRevenueBuilder;
    }
};
