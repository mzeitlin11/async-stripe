#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsSepaDebit {
    pub mandate_options: Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentPaymentMethodOptionsSepaDebitBuilder {
    mandate_options: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentPaymentMethodOptionsSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsSepaDebit>,
        builder: SetupIntentPaymentMethodOptionsSepaDebitBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentPaymentMethodOptionsSepaDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsSepaDebitBuilder {
        type Out = SetupIntentPaymentMethodOptionsSepaDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "mandate_options" => Ok(Deserialize::begin(&mut self.mandate_options)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { mandate_options: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let mandate_options = self.mandate_options.take()?;

            Some(Self::Out { mandate_options })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsSepaDebit {
        type Builder = SetupIntentPaymentMethodOptionsSepaDebitBuilder;
    }
};
