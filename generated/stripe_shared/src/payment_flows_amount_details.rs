#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetails {
    pub tip: Option<stripe_shared::PaymentFlowsAmountDetailsResourceTip>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentFlowsAmountDetailsBuilder {
    tip: Option<Option<stripe_shared::PaymentFlowsAmountDetailsResourceTip>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentFlowsAmountDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAmountDetails>,
        builder: PaymentFlowsAmountDetailsBuilder,
    }

    impl Visitor for Place<PaymentFlowsAmountDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentFlowsAmountDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentFlowsAmountDetailsBuilder {
        type Out = PaymentFlowsAmountDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "tip" => Ok(Deserialize::begin(&mut self.tip)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { tip: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let tip = self.tip.take()?;

            Some(Self::Out { tip })
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

    impl ObjectDeser for PaymentFlowsAmountDetails {
        type Builder = PaymentFlowsAmountDetailsBuilder;
    }
};
