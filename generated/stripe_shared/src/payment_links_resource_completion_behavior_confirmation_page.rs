#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPage {
    /// The custom message that is displayed to the customer after the purchase is complete.
    pub custom_message: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPageBuilder {
    custom_message: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceCompletionBehaviorConfirmationPage {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCompletionBehaviorConfirmationPage>,
        builder: PaymentLinksResourceCompletionBehaviorConfirmationPageBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCompletionBehaviorConfirmationPage> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceCompletionBehaviorConfirmationPageBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCompletionBehaviorConfirmationPageBuilder {
        type Out = PaymentLinksResourceCompletionBehaviorConfirmationPage;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "custom_message" => Ok(Deserialize::begin(&mut self.custom_message)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { custom_message: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let custom_message = self.custom_message.take()?;

            Some(Self::Out { custom_message })
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

    impl ObjectDeser for PaymentLinksResourceCompletionBehaviorConfirmationPage {
        type Builder = PaymentLinksResourceCompletionBehaviorConfirmationPageBuilder;
    }
};
