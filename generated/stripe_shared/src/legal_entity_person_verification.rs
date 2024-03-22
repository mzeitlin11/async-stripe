#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LegalEntityPersonVerification {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    pub additional_document: Option<stripe_shared::LegalEntityPersonVerificationDocument>,
    /// A user-displayable string describing the verification state for the person.
    /// For example, this may say "Provided identity information could not be verified".
    pub details: Option<String>,
    /// One of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`, `document_name_mismatch`, `document_nationality_mismatch`, `failed_keyed_identity`, or `failed_other`.
    /// A machine-readable code specifying the verification state for the person.
    pub details_code: Option<String>,
    pub document: Option<stripe_shared::LegalEntityPersonVerificationDocument>,
    /// The state of verification for the person.
    /// Possible values are `unverified`, `pending`, or `verified`.
    pub status: String,
}
#[cfg(feature = "min-ser")]
pub struct LegalEntityPersonVerificationBuilder {
    additional_document: Option<Option<stripe_shared::LegalEntityPersonVerificationDocument>>,
    details: Option<Option<String>>,
    details_code: Option<Option<String>>,
    document: Option<Option<stripe_shared::LegalEntityPersonVerificationDocument>>,
    status: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for LegalEntityPersonVerification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityPersonVerification>,
        builder: LegalEntityPersonVerificationBuilder,
    }

    impl Visitor for Place<LegalEntityPersonVerification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: LegalEntityPersonVerificationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for LegalEntityPersonVerificationBuilder {
        type Out = LegalEntityPersonVerification;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "additional_document" => Ok(Deserialize::begin(&mut self.additional_document)),
                "details" => Ok(Deserialize::begin(&mut self.details)),
                "details_code" => Ok(Deserialize::begin(&mut self.details_code)),
                "document" => Ok(Deserialize::begin(&mut self.document)),
                "status" => Ok(Deserialize::begin(&mut self.status)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                additional_document: Deserialize::default(),
                details: Deserialize::default(),
                details_code: Deserialize::default(),
                document: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let additional_document = self.additional_document.take()?;
            let details = self.details.take()?;
            let details_code = self.details_code.take()?;
            let document = self.document.take()?;
            let status = self.status.take()?;

            Some(Self::Out { additional_document, details, details_code, document, status })
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

    impl ObjectDeser for LegalEntityPersonVerification {
        type Builder = LegalEntityPersonVerificationBuilder;
    }
};
