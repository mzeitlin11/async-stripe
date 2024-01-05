#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTransactionPaperCheckData {
    /// Time at which the deposited funds will be available for use.
    /// Measured in seconds since the Unix epoch.
    pub available_at: Option<String>,
    /// Comma-separated list of invoice IDs associated with the paper check.
    pub invoices: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTransactionPaperCheckDataBuilder {
    available_at: Option<Option<String>>,
    invoices: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTransactionPaperCheckData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionPaperCheckData>,
        builder: SourceTransactionPaperCheckDataBuilder,
    }

    impl Visitor for Place<SourceTransactionPaperCheckData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTransactionPaperCheckDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTransactionPaperCheckDataBuilder {
        type Out = SourceTransactionPaperCheckData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "available_at" => Ok(Deserialize::begin(&mut self.available_at)),
                "invoices" => Ok(Deserialize::begin(&mut self.invoices)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { available_at: Deserialize::default(), invoices: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let available_at = self.available_at.take()?;
            let invoices = self.invoices.take()?;

            Some(Self::Out { available_at, invoices })
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

    impl ObjectDeser for SourceTransactionPaperCheckData {
        type Builder = SourceTransactionPaperCheckDataBuilder;
    }
};
