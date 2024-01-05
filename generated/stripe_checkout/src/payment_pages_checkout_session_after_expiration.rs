#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionAfterExpiration {
    /// When set, configuration used to recover the Checkout Session on expiry.
    pub recovery: Option<stripe_checkout::PaymentPagesCheckoutSessionAfterExpirationRecovery>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionAfterExpirationBuilder {
    recovery: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionAfterExpirationRecovery>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionAfterExpiration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionAfterExpiration>,
        builder: PaymentPagesCheckoutSessionAfterExpirationBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionAfterExpiration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionAfterExpirationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionAfterExpirationBuilder {
        type Out = PaymentPagesCheckoutSessionAfterExpiration;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "recovery" => Ok(Deserialize::begin(&mut self.recovery)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { recovery: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let recovery = self.recovery.take()?;

            Some(Self::Out { recovery })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionAfterExpiration {
        type Builder = PaymentPagesCheckoutSessionAfterExpirationBuilder;
    }
};
