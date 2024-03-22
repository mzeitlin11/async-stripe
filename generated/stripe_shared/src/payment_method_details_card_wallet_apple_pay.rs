#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardWalletApplePay {}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsCardWalletApplePayBuilder {}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsCardWalletApplePay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardWalletApplePay>,
        builder: PaymentMethodDetailsCardWalletApplePayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardWalletApplePay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsCardWalletApplePayBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardWalletApplePayBuilder {
        type Out = PaymentMethodDetailsCardWalletApplePay;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
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

    impl ObjectDeser for PaymentMethodDetailsCardWalletApplePay {
        type Builder = PaymentMethodDetailsCardWalletApplePayBuilder;
    }
};
