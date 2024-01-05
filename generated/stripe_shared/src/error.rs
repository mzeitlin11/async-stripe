/// An error response from the Stripe API
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Error {
    pub error: Box<stripe_shared::ApiErrors>,
}
#[cfg(feature = "min-ser")]
pub struct ErrorBuilder {
    error: Option<Box<stripe_shared::ApiErrors>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Error {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Error>,
        builder: ErrorBuilder,
    }

    impl Visitor for Place<Error> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ErrorBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ErrorBuilder {
        type Out = Error;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "error" => Ok(Deserialize::begin(&mut self.error)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { error: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let error = self.error.take()?;

            Some(Self::Out { error })
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

    impl ObjectDeser for Error {
        type Builder = ErrorBuilder;
    }
};
