#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BalanceAmountNet {
    /// Balance amount.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    pub source_types: Option<stripe_core::BalanceAmountBySourceType>,
}
#[cfg(feature = "min-ser")]
pub struct BalanceAmountNetBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    source_types: Option<Option<stripe_core::BalanceAmountBySourceType>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BalanceAmountNet {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceAmountNet>,
        builder: BalanceAmountNetBuilder,
    }

    impl Visitor for Place<BalanceAmountNet> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: BalanceAmountNetBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BalanceAmountNetBuilder {
        type Out = BalanceAmountNet;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "source_types" => Ok(Deserialize::begin(&mut self.source_types)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), currency: Deserialize::default(), source_types: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let currency = self.currency.take()?;
            let source_types = self.source_types.take()?;

            Some(Self::Out { amount, currency, source_types })
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

    impl ObjectDeser for BalanceAmountNet {
        type Builder = BalanceAmountNetBuilder;
    }
};
