#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SigmaScheduledQueryRunError {
    /// Information about the run failure.
    pub message: String,
}
#[cfg(feature = "min-ser")]
pub struct SigmaScheduledQueryRunErrorBuilder {
    message: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SigmaScheduledQueryRunError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SigmaScheduledQueryRunError>,
        builder: SigmaScheduledQueryRunErrorBuilder,
    }

    impl Visitor for Place<SigmaScheduledQueryRunError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SigmaScheduledQueryRunErrorBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SigmaScheduledQueryRunErrorBuilder {
        type Out = SigmaScheduledQueryRunError;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
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

    impl ObjectDeser for SigmaScheduledQueryRunError {
        type Builder = SigmaScheduledQueryRunErrorBuilder;
    }
};
