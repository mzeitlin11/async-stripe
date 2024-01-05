#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCardInstallments {
    /// Installment plans that may be selected for this PaymentIntent.
    pub available_plans: Option<Vec<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>>,
    /// Whether Installments are enabled for this PaymentIntent.
    pub enabled: bool,
    /// Installment plan selected for this PaymentIntent.
    pub plan: Option<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodOptionsCardInstallmentsBuilder {
    available_plans: Option<Option<Vec<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>>>,
    enabled: Option<bool>,
    plan: Option<Option<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodOptionsCardInstallments {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCardInstallments>,
        builder: PaymentMethodOptionsCardInstallmentsBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCardInstallments> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodOptionsCardInstallmentsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCardInstallmentsBuilder {
        type Out = PaymentMethodOptionsCardInstallments;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "available_plans" => Ok(Deserialize::begin(&mut self.available_plans)),
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),
                "plan" => Ok(Deserialize::begin(&mut self.plan)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { available_plans: Deserialize::default(), enabled: Deserialize::default(), plan: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let available_plans = self.available_plans.take()?;
            let enabled = self.enabled.take()?;
            let plan = self.plan.take()?;

            Some(Self::Out { available_plans, enabled, plan })
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

    impl ObjectDeser for PaymentMethodOptionsCardInstallments {
        type Builder = PaymentMethodOptionsCardInstallmentsBuilder;
    }
};
