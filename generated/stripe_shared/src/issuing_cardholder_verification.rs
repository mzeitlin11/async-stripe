#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardholderVerification {
    /// An identifying document, either a passport or local ID card.
    pub document: Option<stripe_shared::IssuingCardholderIdDocument>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardholderVerificationBuilder {
    document: Option<Option<stripe_shared::IssuingCardholderIdDocument>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardholderVerification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderVerification>,
        builder: IssuingCardholderVerificationBuilder,
    }

    impl Visitor for Place<IssuingCardholderVerification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardholderVerificationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardholderVerificationBuilder {
        type Out = IssuingCardholderVerification;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "document" => Ok(Deserialize::begin(&mut self.document)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { document: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let document = self.document.take()?;

            Some(Self::Out { document })
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

    impl ObjectDeser for IssuingCardholderVerification {
        type Builder = IssuingCardholderVerificationBuilder;
    }
};
