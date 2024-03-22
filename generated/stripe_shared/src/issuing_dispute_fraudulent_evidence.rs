#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingDisputeFraudulentEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingDisputeFraudulentEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    explanation: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingDisputeFraudulentEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeFraudulentEvidence>,
        builder: IssuingDisputeFraudulentEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeFraudulentEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingDisputeFraudulentEvidenceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingDisputeFraudulentEvidenceBuilder {
        type Out = IssuingDisputeFraudulentEvidence;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "additional_documentation" => Ok(Deserialize::begin(&mut self.additional_documentation)),
                "explanation" => Ok(Deserialize::begin(&mut self.explanation)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { additional_documentation: Deserialize::default(), explanation: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let additional_documentation = self.additional_documentation.take()?;
            let explanation = self.explanation.take()?;

            Some(Self::Out { additional_documentation, explanation })
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

    impl ObjectDeser for IssuingDisputeFraudulentEvidence {
        type Builder = IssuingDisputeFraudulentEvidenceBuilder;
    }
};
