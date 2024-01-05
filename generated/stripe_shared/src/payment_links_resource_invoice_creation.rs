#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceInvoiceCreation {
    /// Enable creating an invoice on successful payment.
    pub enabled: bool,
    /// Configuration for the invoice. Default invoice values will be used if unspecified.
    pub invoice_data: Option<stripe_shared::PaymentLinksResourceInvoiceSettings>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceInvoiceCreationBuilder {
    enabled: Option<bool>,
    invoice_data: Option<Option<stripe_shared::PaymentLinksResourceInvoiceSettings>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceInvoiceCreation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceInvoiceCreation>,
        builder: PaymentLinksResourceInvoiceCreationBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceInvoiceCreation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceInvoiceCreationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceInvoiceCreationBuilder {
        type Out = PaymentLinksResourceInvoiceCreation;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),
                "invoice_data" => Ok(Deserialize::begin(&mut self.invoice_data)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default(), invoice_data: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let enabled = self.enabled.take()?;
            let invoice_data = self.invoice_data.take()?;

            Some(Self::Out { enabled, invoice_data })
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

    impl ObjectDeser for PaymentLinksResourceInvoiceCreation {
        type Builder = PaymentLinksResourceInvoiceCreationBuilder;
    }
};
