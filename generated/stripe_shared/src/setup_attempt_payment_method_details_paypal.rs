#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupAttemptPaymentMethodDetailsPaypal {}
#[cfg(feature = "min-ser")]
pub struct SetupAttemptPaymentMethodDetailsPaypalBuilder {}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupAttemptPaymentMethodDetailsPaypal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupAttemptPaymentMethodDetailsPaypal>,
        builder: SetupAttemptPaymentMethodDetailsPaypalBuilder,
    }

    impl Visitor for Place<SetupAttemptPaymentMethodDetailsPaypal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupAttemptPaymentMethodDetailsPaypalBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupAttemptPaymentMethodDetailsPaypalBuilder {
        type Out = SetupAttemptPaymentMethodDetailsPaypal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {}
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {})
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

    impl ObjectDeser for SetupAttemptPaymentMethodDetailsPaypal {
        type Builder = SetupAttemptPaymentMethodDetailsPaypalBuilder;
    }
};
