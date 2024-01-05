#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct MandateSepaDebit {
    /// The unique reference of the mandate.
    pub reference: String,
    /// The URL of the mandate.
    /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}
#[cfg(feature = "min-ser")]
pub struct MandateSepaDebitBuilder {
    reference: Option<String>,
    url: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for MandateSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandateSepaDebit>,
        builder: MandateSepaDebitBuilder,
    }

    impl Visitor for Place<MandateSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: MandateSepaDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for MandateSepaDebitBuilder {
        type Out = MandateSepaDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { reference: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let reference = self.reference.take()?;
            let url = self.url.take()?;

            Some(Self::Out { reference, url })
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

    impl ObjectDeser for MandateSepaDebit {
        type Builder = MandateSepaDebitBuilder;
    }
};
