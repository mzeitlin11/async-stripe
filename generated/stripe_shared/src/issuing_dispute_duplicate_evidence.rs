#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingDisputeDuplicateEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    pub card_statement: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    pub cash_receipt: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    pub check_image: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of.
    /// Of the two or more transactions that are copies of each other, this is original undisputed one.
    pub original_transaction: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingDisputeDuplicateEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    card_statement: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    cash_receipt: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    check_image: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    explanation: Option<Option<String>>,
    original_transaction: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingDisputeDuplicateEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeDuplicateEvidence>,
        builder: IssuingDisputeDuplicateEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeDuplicateEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingDisputeDuplicateEvidenceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingDisputeDuplicateEvidenceBuilder {
        type Out = IssuingDisputeDuplicateEvidence;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "additional_documentation" => Ok(Deserialize::begin(&mut self.additional_documentation)),
                "card_statement" => Ok(Deserialize::begin(&mut self.card_statement)),
                "cash_receipt" => Ok(Deserialize::begin(&mut self.cash_receipt)),
                "check_image" => Ok(Deserialize::begin(&mut self.check_image)),
                "explanation" => Ok(Deserialize::begin(&mut self.explanation)),
                "original_transaction" => Ok(Deserialize::begin(&mut self.original_transaction)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                additional_documentation: Deserialize::default(),
                card_statement: Deserialize::default(),
                cash_receipt: Deserialize::default(),
                check_image: Deserialize::default(),
                explanation: Deserialize::default(),
                original_transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let additional_documentation = self.additional_documentation.take()?;
            let card_statement = self.card_statement.take()?;
            let cash_receipt = self.cash_receipt.take()?;
            let check_image = self.check_image.take()?;
            let explanation = self.explanation.take()?;
            let original_transaction = self.original_transaction.take()?;

            Some(Self::Out { additional_documentation, card_statement, cash_receipt, check_image, explanation, original_transaction })
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

    impl ObjectDeser for IssuingDisputeDuplicateEvidence {
        type Builder = IssuingDisputeDuplicateEvidenceBuilder;
    }
};
