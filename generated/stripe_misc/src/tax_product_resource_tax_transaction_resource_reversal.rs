#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceTaxTransactionResourceReversal {
    /// The `id` of the reversed `Transaction` object.
    pub original_transaction: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceTaxTransactionResourceReversalBuilder {
    original_transaction: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxTransactionResourceReversal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxTransactionResourceReversal>,
        builder: TaxProductResourceTaxTransactionResourceReversalBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxTransactionResourceReversal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceTaxTransactionResourceReversalBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxTransactionResourceReversalBuilder {
        type Out = TaxProductResourceTaxTransactionResourceReversal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "original_transaction" => Ok(Deserialize::begin(&mut self.original_transaction)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { original_transaction: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let original_transaction = self.original_transaction.take()?;

            Some(Self::Out { original_transaction })
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

    impl ObjectDeser for TaxProductResourceTaxTransactionResourceReversal {
        type Builder = TaxProductResourceTaxTransactionResourceReversalBuilder;
    }
};
