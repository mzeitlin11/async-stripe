#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaynow {
    /// Reference number associated with this PayNow payment
    pub reference: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsPaynowBuilder {
    reference: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsPaynow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaynow>,
        builder: PaymentMethodDetailsPaynowBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaynow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsPaynowBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsPaynowBuilder {
        type Out = PaymentMethodDetailsPaynow;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "reference" => Ok(Deserialize::begin(&mut self.reference)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { reference: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let reference = self.reference.take()?;

            Some(Self::Out { reference })
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

    impl ObjectDeser for PaymentMethodDetailsPaynow {
        type Builder = PaymentMethodDetailsPaynowBuilder;
    }
};
