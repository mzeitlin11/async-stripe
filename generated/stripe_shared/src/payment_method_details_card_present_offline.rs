#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardPresentOffline {
    /// Time at which the payment was collected while offline
    pub stored_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsCardPresentOfflineBuilder {
    stored_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsCardPresentOffline {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardPresentOffline>,
        builder: PaymentMethodDetailsCardPresentOfflineBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardPresentOffline> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsCardPresentOfflineBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardPresentOfflineBuilder {
        type Out = PaymentMethodDetailsCardPresentOffline;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "stored_at" => Ok(Deserialize::begin(&mut self.stored_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { stored_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let stored_at = self.stored_at.take()?;

            Some(Self::Out { stored_at })
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

    impl ObjectDeser for PaymentMethodDetailsCardPresentOffline {
        type Builder = PaymentMethodDetailsCardPresentOfflineBuilder;
    }
};
