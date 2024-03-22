#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoicesResourceLineItemsProrationDetails {
    /// For a credit proration `line_item`, the original debit line_items to which the credit proration applies.
    pub credited_items: Option<stripe_shared::InvoicesResourceLineItemsCreditedItems>,
}
#[cfg(feature = "min-ser")]
pub struct InvoicesResourceLineItemsProrationDetailsBuilder {
    credited_items: Option<Option<stripe_shared::InvoicesResourceLineItemsCreditedItems>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesResourceLineItemsProrationDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesResourceLineItemsProrationDetails>,
        builder: InvoicesResourceLineItemsProrationDetailsBuilder,
    }

    impl Visitor for Place<InvoicesResourceLineItemsProrationDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoicesResourceLineItemsProrationDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoicesResourceLineItemsProrationDetailsBuilder {
        type Out = InvoicesResourceLineItemsProrationDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "credited_items" => Ok(Deserialize::begin(&mut self.credited_items)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { credited_items: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let credited_items = self.credited_items.take()?;

            Some(Self::Out { credited_items })
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

    impl ObjectDeser for InvoicesResourceLineItemsProrationDetails {
        type Builder = InvoicesResourceLineItemsProrationDetailsBuilder;
    }
};
