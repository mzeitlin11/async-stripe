#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomTextPosition {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionCustomTextPositionBuilder {
    message: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCustomTextPosition {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomTextPosition>,
        builder: PaymentPagesCheckoutSessionCustomTextPositionBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomTextPosition> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionCustomTextPositionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomTextPositionBuilder {
        type Out = PaymentPagesCheckoutSessionCustomTextPosition;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "message" => Ok(Deserialize::begin(&mut self.message)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { message: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let message = self.message.take()?;

            Some(Self::Out { message })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomTextPosition {
        type Builder = PaymentPagesCheckoutSessionCustomTextPositionBuilder;
    }
};
