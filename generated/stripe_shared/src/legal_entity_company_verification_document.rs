#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LegalEntityCompanyVerificationDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub back: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// A user-displayable string describing the verification state of this document.
    pub details: Option<String>,
    /// One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`.
    /// A machine-readable code specifying the verification state for this document.
    pub details_code: Option<String>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub front: Option<stripe_types::Expandable<stripe_shared::File>>,
}
#[cfg(feature = "min-ser")]
pub struct LegalEntityCompanyVerificationDocumentBuilder {
    back: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    details: Option<Option<String>>,
    details_code: Option<Option<String>>,
    front: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for LegalEntityCompanyVerificationDocument {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityCompanyVerificationDocument>,
        builder: LegalEntityCompanyVerificationDocumentBuilder,
    }

    impl Visitor for Place<LegalEntityCompanyVerificationDocument> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: LegalEntityCompanyVerificationDocumentBuilder::deser_default() }))
        }
    }

    impl MapBuilder for LegalEntityCompanyVerificationDocumentBuilder {
        type Out = LegalEntityCompanyVerificationDocument;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "back" => Ok(Deserialize::begin(&mut self.back)),
                "details" => Ok(Deserialize::begin(&mut self.details)),
                "details_code" => Ok(Deserialize::begin(&mut self.details_code)),
                "front" => Ok(Deserialize::begin(&mut self.front)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { back: Deserialize::default(), details: Deserialize::default(), details_code: Deserialize::default(), front: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let back = self.back.take()?;
            let details = self.details.take()?;
            let details_code = self.details_code.take()?;
            let front = self.front.take()?;

            Some(Self::Out { back, details, details_code, front })
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

    impl ObjectDeser for LegalEntityCompanyVerificationDocument {
        type Builder = LegalEntityCompanyVerificationDocumentBuilder;
    }
};
