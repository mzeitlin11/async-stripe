#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoicesFromInvoice {
    /// The relation between this invoice and the cloned invoice
    pub action: String,
    /// The invoice that was cloned.
    pub invoice: stripe_types::Expandable<stripe_shared::Invoice>,
}
#[cfg(feature = "min-ser")]
pub struct InvoicesFromInvoiceBuilder {
    action: Option<String>,
    invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesFromInvoice {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesFromInvoice>,
        builder: InvoicesFromInvoiceBuilder,
    }

    impl Visitor for Place<InvoicesFromInvoice> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoicesFromInvoiceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoicesFromInvoiceBuilder {
        type Out = InvoicesFromInvoice;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "action" => Ok(Deserialize::begin(&mut self.action)),
                "invoice" => Ok(Deserialize::begin(&mut self.invoice)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { action: Deserialize::default(), invoice: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let action = self.action.take()?;
            let invoice = self.invoice.take()?;

            Some(Self::Out { action, invoice })
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

    impl ObjectDeser for InvoicesFromInvoice {
        type Builder = InvoicesFromInvoiceBuilder;
    }
};
