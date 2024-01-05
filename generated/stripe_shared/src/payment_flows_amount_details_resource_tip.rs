#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetailsResourceTip {
    /// Portion of the amount that corresponds to a tip.
    pub amount: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentFlowsAmountDetailsResourceTipBuilder {
    amount: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentFlowsAmountDetailsResourceTip {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAmountDetailsResourceTip>,
        builder: PaymentFlowsAmountDetailsResourceTipBuilder,
    }

    impl Visitor for Place<PaymentFlowsAmountDetailsResourceTip> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentFlowsAmountDetailsResourceTipBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentFlowsAmountDetailsResourceTipBuilder {
        type Out = PaymentFlowsAmountDetailsResourceTip;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;

            Some(Self::Out { amount })
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

    impl ObjectDeser for PaymentFlowsAmountDetailsResourceTip {
        type Builder = PaymentFlowsAmountDetailsResourceTipBuilder;
    }
};
