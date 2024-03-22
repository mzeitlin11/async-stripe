#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PromotionCodeCurrencyOption {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: i64,
}
#[cfg(feature = "min-ser")]
pub struct PromotionCodeCurrencyOptionBuilder {
    minimum_amount: Option<i64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PromotionCodeCurrencyOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PromotionCodeCurrencyOption>,
        builder: PromotionCodeCurrencyOptionBuilder,
    }

    impl Visitor for Place<PromotionCodeCurrencyOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PromotionCodeCurrencyOptionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PromotionCodeCurrencyOptionBuilder {
        type Out = PromotionCodeCurrencyOption;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "minimum_amount" => Ok(Deserialize::begin(&mut self.minimum_amount)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { minimum_amount: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let minimum_amount = self.minimum_amount.take()?;

            Some(Self::Out { minimum_amount })
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

    impl ObjectDeser for PromotionCodeCurrencyOption {
        type Builder = PromotionCodeCurrencyOptionBuilder;
    }
};
