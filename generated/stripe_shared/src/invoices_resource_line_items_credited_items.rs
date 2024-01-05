#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoicesResourceLineItemsCreditedItems {
    /// Invoice containing the credited invoice line items
    pub invoice: String,
    /// Credited invoice line items
    pub invoice_line_items: Vec<String>,
}
#[cfg(feature = "min-ser")]
pub struct InvoicesResourceLineItemsCreditedItemsBuilder {
    invoice: Option<String>,
    invoice_line_items: Option<Vec<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesResourceLineItemsCreditedItems {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesResourceLineItemsCreditedItems>,
        builder: InvoicesResourceLineItemsCreditedItemsBuilder,
    }

    impl Visitor for Place<InvoicesResourceLineItemsCreditedItems> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoicesResourceLineItemsCreditedItemsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoicesResourceLineItemsCreditedItemsBuilder {
        type Out = InvoicesResourceLineItemsCreditedItems;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "invoice" => Ok(Deserialize::begin(&mut self.invoice)),
                "invoice_line_items" => Ok(Deserialize::begin(&mut self.invoice_line_items)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { invoice: Deserialize::default(), invoice_line_items: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let invoice = self.invoice.take()?;
            let invoice_line_items = self.invoice_line_items.take()?;

            Some(Self::Out { invoice, invoice_line_items })
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

    impl ObjectDeser for InvoicesResourceLineItemsCreditedItems {
        type Builder = InvoicesResourceLineItemsCreditedItemsBuilder;
    }
};
