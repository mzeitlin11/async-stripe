#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoicesInvoiceRendering {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
    /// Invoice pdf rendering options
    pub pdf: Option<stripe_shared::InvoiceRenderingPdf>,
}
#[cfg(feature = "min-ser")]
pub struct InvoicesInvoiceRenderingBuilder {
    amount_tax_display: Option<Option<String>>,
    pdf: Option<Option<stripe_shared::InvoiceRenderingPdf>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesInvoiceRendering {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesInvoiceRendering>,
        builder: InvoicesInvoiceRenderingBuilder,
    }

    impl Visitor for Place<InvoicesInvoiceRendering> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoicesInvoiceRenderingBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoicesInvoiceRenderingBuilder {
        type Out = InvoicesInvoiceRendering;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount_tax_display" => Ok(Deserialize::begin(&mut self.amount_tax_display)),
                "pdf" => Ok(Deserialize::begin(&mut self.pdf)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount_tax_display: Deserialize::default(), pdf: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_tax_display = self.amount_tax_display.take()?;
            let pdf = self.pdf.take()?;

            Some(Self::Out { amount_tax_display, pdf })
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

    impl ObjectDeser for InvoicesInvoiceRendering {
        type Builder = InvoicesInvoiceRenderingBuilder;
    }
};
