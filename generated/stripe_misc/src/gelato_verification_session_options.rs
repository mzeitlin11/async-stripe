#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct GelatoVerificationSessionOptions {
    pub document: Option<stripe_misc::GelatoSessionDocumentOptions>,
    pub id_number: Option<stripe_misc::GelatoSessionIdNumberOptions>,
}
#[cfg(feature = "min-ser")]
pub struct GelatoVerificationSessionOptionsBuilder {
    document: Option<Option<stripe_misc::GelatoSessionDocumentOptions>>,
    id_number: Option<Option<stripe_misc::GelatoSessionIdNumberOptions>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoVerificationSessionOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoVerificationSessionOptions>,
        builder: GelatoVerificationSessionOptionsBuilder,
    }

    impl Visitor for Place<GelatoVerificationSessionOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: GelatoVerificationSessionOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for GelatoVerificationSessionOptionsBuilder {
        type Out = GelatoVerificationSessionOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "document" => Ok(Deserialize::begin(&mut self.document)),
                "id_number" => Ok(Deserialize::begin(&mut self.id_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { document: Deserialize::default(), id_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let document = self.document.take()?;
            let id_number = self.id_number.take()?;

            Some(Self::Out { document, id_number })
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

    impl ObjectDeser for GelatoVerificationSessionOptions {
        type Builder = GelatoVerificationSessionOptionsBuilder;
    }
};
