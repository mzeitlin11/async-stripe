#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CheckoutPixPaymentMethodOptions {
    /// The number of seconds after which Pix payment will expire.
    pub expires_after_seconds: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct CheckoutPixPaymentMethodOptionsBuilder {
    expires_after_seconds: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutPixPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutPixPaymentMethodOptions>,
        builder: CheckoutPixPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutPixPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CheckoutPixPaymentMethodOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CheckoutPixPaymentMethodOptionsBuilder {
        type Out = CheckoutPixPaymentMethodOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "expires_after_seconds" => Ok(Deserialize::begin(&mut self.expires_after_seconds)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { expires_after_seconds: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let expires_after_seconds = self.expires_after_seconds.take()?;

            Some(Self::Out { expires_after_seconds })
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

    impl ObjectDeser for CheckoutPixPaymentMethodOptions {
        type Builder = CheckoutPixPaymentMethodOptionsBuilder;
    }
};
