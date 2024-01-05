#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DisputeEvidenceDetails {
    /// Date by which evidence must be submitted in order to successfully challenge dispute.
    /// Will be 0 if the customer's bank or credit card company doesn't allow a response for this particular dispute.
    pub due_by: Option<stripe_types::Timestamp>,
    /// Whether evidence has been staged for this dispute.
    pub has_evidence: bool,
    /// Whether the last evidence submission was submitted past the due date.
    /// Defaults to `false` if no evidence submissions have occurred.
    /// If `true`, then delivery of the latest evidence is *not* guaranteed.
    pub past_due: bool,
    /// The number of times evidence has been submitted. Typically, you may only submit evidence once.
    pub submission_count: u64,
}
#[cfg(feature = "min-ser")]
pub struct DisputeEvidenceDetailsBuilder {
    due_by: Option<Option<stripe_types::Timestamp>>,
    has_evidence: Option<bool>,
    past_due: Option<bool>,
    submission_count: Option<u64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DisputeEvidenceDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeEvidenceDetails>,
        builder: DisputeEvidenceDetailsBuilder,
    }

    impl Visitor for Place<DisputeEvidenceDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DisputeEvidenceDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DisputeEvidenceDetailsBuilder {
        type Out = DisputeEvidenceDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "due_by" => Ok(Deserialize::begin(&mut self.due_by)),
                "has_evidence" => Ok(Deserialize::begin(&mut self.has_evidence)),
                "past_due" => Ok(Deserialize::begin(&mut self.past_due)),
                "submission_count" => Ok(Deserialize::begin(&mut self.submission_count)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { due_by: Deserialize::default(), has_evidence: Deserialize::default(), past_due: Deserialize::default(), submission_count: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let due_by = self.due_by.take()?;
            let has_evidence = self.has_evidence.take()?;
            let past_due = self.past_due.take()?;
            let submission_count = self.submission_count.take()?;

            Some(Self::Out { due_by, has_evidence, past_due, submission_count })
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

    impl ObjectDeser for DisputeEvidenceDetails {
        type Builder = DisputeEvidenceDetailsBuilder;
    }
};
