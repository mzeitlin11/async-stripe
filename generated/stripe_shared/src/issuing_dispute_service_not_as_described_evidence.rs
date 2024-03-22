#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingDisputeServiceNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Date when order was canceled.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Reason for canceling the order.
    pub cancellation_reason: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Date when the product was received.
    pub received_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingDisputeServiceNotAsDescribedEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    cancellation_reason: Option<Option<String>>,
    explanation: Option<Option<String>>,
    received_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingDisputeServiceNotAsDescribedEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeServiceNotAsDescribedEvidence>,
        builder: IssuingDisputeServiceNotAsDescribedEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeServiceNotAsDescribedEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingDisputeServiceNotAsDescribedEvidenceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingDisputeServiceNotAsDescribedEvidenceBuilder {
        type Out = IssuingDisputeServiceNotAsDescribedEvidence;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "additional_documentation" => Ok(Deserialize::begin(&mut self.additional_documentation)),
                "canceled_at" => Ok(Deserialize::begin(&mut self.canceled_at)),
                "cancellation_reason" => Ok(Deserialize::begin(&mut self.cancellation_reason)),
                "explanation" => Ok(Deserialize::begin(&mut self.explanation)),
                "received_at" => Ok(Deserialize::begin(&mut self.received_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                additional_documentation: Deserialize::default(),
                canceled_at: Deserialize::default(),
                cancellation_reason: Deserialize::default(),
                explanation: Deserialize::default(),
                received_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let additional_documentation = self.additional_documentation.take()?;
            let canceled_at = self.canceled_at.take()?;
            let cancellation_reason = self.cancellation_reason.take()?;
            let explanation = self.explanation.take()?;
            let received_at = self.received_at.take()?;

            Some(Self::Out { additional_documentation, canceled_at, cancellation_reason, explanation, received_at })
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

    impl ObjectDeser for IssuingDisputeServiceNotAsDescribedEvidence {
        type Builder = IssuingDisputeServiceNotAsDescribedEvidenceBuilder;
    }
};
