#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKonbini {
    /// If the payment succeeded, this contains the details of the convenience store where the payment was completed.
    pub store: Option<stripe_shared::PaymentMethodDetailsKonbiniStore>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsKonbiniBuilder {
    store: Option<Option<stripe_shared::PaymentMethodDetailsKonbiniStore>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsKonbini {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsKonbini>,
        builder: PaymentMethodDetailsKonbiniBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsKonbini> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsKonbiniBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsKonbiniBuilder {
        type Out = PaymentMethodDetailsKonbini;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "store" => Ok(Deserialize::begin(&mut self.store)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { store: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let store = self.store.take()?;

            Some(Self::Out { store })
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

    impl ObjectDeser for PaymentMethodDetailsKonbini {
        type Builder = PaymentMethodDetailsKonbiniBuilder;
    }
};
