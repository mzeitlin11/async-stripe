#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentCardProcessing {
    pub customer_notification: Option<stripe_shared::PaymentIntentProcessingCustomerNotification>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentCardProcessingBuilder {
    customer_notification: Option<Option<stripe_shared::PaymentIntentProcessingCustomerNotification>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentCardProcessing {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentCardProcessing>,
        builder: PaymentIntentCardProcessingBuilder,
    }

    impl Visitor for Place<PaymentIntentCardProcessing> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentCardProcessingBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentCardProcessingBuilder {
        type Out = PaymentIntentCardProcessing;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "customer_notification" => Ok(Deserialize::begin(&mut self.customer_notification)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { customer_notification: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let customer_notification = self.customer_notification.take()?;

            Some(Self::Out { customer_notification })
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

    impl ObjectDeser for PaymentIntentCardProcessing {
        type Builder = PaymentIntentCardProcessingBuilder;
    }
};
