#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct MandateSingleUse {
    /// The amount of the payment on a single use mandate.
    pub amount: i64,
    /// The currency of the payment on a single use mandate.
    pub currency: stripe_types::Currency,
}
#[cfg(feature = "min-ser")]
pub struct MandateSingleUseBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for MandateSingleUse {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandateSingleUse>,
        builder: MandateSingleUseBuilder,
    }

    impl Visitor for Place<MandateSingleUse> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: MandateSingleUseBuilder::deser_default() }))
        }
    }

    impl MapBuilder for MandateSingleUseBuilder {
        type Out = MandateSingleUse;
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

    impl ObjectDeser for MandateSingleUse {
        type Builder = MandateSingleUseBuilder;
    }
};
