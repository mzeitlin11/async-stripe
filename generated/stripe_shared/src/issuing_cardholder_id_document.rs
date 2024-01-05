#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardholderIdDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub back: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub front: Option<stripe_types::Expandable<stripe_shared::File>>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardholderIdDocumentBuilder {
    back: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    front: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardholderIdDocument {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderIdDocument>,
        builder: IssuingCardholderIdDocumentBuilder,
    }

    impl Visitor for Place<IssuingCardholderIdDocument> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardholderIdDocumentBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardholderIdDocumentBuilder {
        type Out = IssuingCardholderIdDocument;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "back" => Ok(Deserialize::begin(&mut self.back)),
                "front" => Ok(Deserialize::begin(&mut self.front)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { back: Deserialize::default(), front: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let back = self.back.take()?;
            let front = self.front.take()?;

            Some(Self::Out { back, front })
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

    impl ObjectDeser for IssuingCardholderIdDocument {
        type Builder = IssuingCardholderIdDocumentBuilder;
    }
};
