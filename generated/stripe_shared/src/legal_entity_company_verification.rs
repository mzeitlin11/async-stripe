#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LegalEntityCompanyVerification {
    pub document: stripe_shared::LegalEntityCompanyVerificationDocument,
}
#[cfg(feature = "min-ser")]
pub struct LegalEntityCompanyVerificationBuilder {
    document: Option<stripe_shared::LegalEntityCompanyVerificationDocument>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for LegalEntityCompanyVerification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityCompanyVerification>,
        builder: LegalEntityCompanyVerificationBuilder,
    }

    impl Visitor for Place<LegalEntityCompanyVerification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: LegalEntityCompanyVerificationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for LegalEntityCompanyVerificationBuilder {
        type Out = LegalEntityCompanyVerification;
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

    impl ObjectDeser for LegalEntityCompanyVerification {
        type Builder = LegalEntityCompanyVerificationBuilder;
    }
};
