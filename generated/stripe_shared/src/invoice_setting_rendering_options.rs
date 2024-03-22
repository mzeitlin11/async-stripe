#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceSettingRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceSettingRenderingOptionsBuilder {
    amount_tax_display: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceSettingRenderingOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceSettingRenderingOptions>,
        builder: InvoiceSettingRenderingOptionsBuilder,
    }

    impl Visitor for Place<InvoiceSettingRenderingOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceSettingRenderingOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceSettingRenderingOptionsBuilder {
        type Out = InvoiceSettingRenderingOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_tax_display" => Ok(Deserialize::begin(&mut self.amount_tax_display)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount_tax_display: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_tax_display = self.amount_tax_display.take()?;

            Some(Self::Out { amount_tax_display })
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

    impl ObjectDeser for InvoiceSettingRenderingOptions {
        type Builder = InvoiceSettingRenderingOptionsBuilder;
    }
};
