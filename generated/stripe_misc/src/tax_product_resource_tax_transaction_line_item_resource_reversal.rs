#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceTaxTransactionLineItemResourceReversal {
    /// The `id` of the line item to reverse in the original transaction.
    pub original_line_item: String,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceTaxTransactionLineItemResourceReversalBuilder {
    original_line_item: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxTransactionLineItemResourceReversal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxTransactionLineItemResourceReversal>,
        builder: TaxProductResourceTaxTransactionLineItemResourceReversalBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxTransactionLineItemResourceReversal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceTaxTransactionLineItemResourceReversalBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxTransactionLineItemResourceReversalBuilder {
        type Out = TaxProductResourceTaxTransactionLineItemResourceReversal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "original_line_item" => Ok(Deserialize::begin(&mut self.original_line_item)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { original_line_item: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let original_line_item = self.original_line_item.take()?;

            Some(Self::Out { original_line_item })
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

    impl ObjectDeser for TaxProductResourceTaxTransactionLineItemResourceReversal {
        type Builder = TaxProductResourceTaxTransactionLineItemResourceReversalBuilder;
    }
};
