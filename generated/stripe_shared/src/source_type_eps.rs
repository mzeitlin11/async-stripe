#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeEps {
    pub reference: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeEpsBuilder {
    reference: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeEps {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeEps>,
        builder: SourceTypeEpsBuilder,
    }

    impl Visitor for Place<SourceTypeEps> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeEpsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeEpsBuilder {
        type Out = SourceTypeEps;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { reference: Deserialize::default(), statement_descriptor: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let reference = self.reference.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;

            Some(Self::Out { reference, statement_descriptor })
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

    impl ObjectDeser for SourceTypeEps {
        type Builder = SourceTypeEpsBuilder;
    }
};
