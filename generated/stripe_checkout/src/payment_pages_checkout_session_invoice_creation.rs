#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionInvoiceCreation {
    /// Indicates whether invoice creation is enabled for the Checkout Session.
    pub enabled: bool,
    pub invoice_data: stripe_checkout::PaymentPagesCheckoutSessionInvoiceSettings,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionInvoiceCreationBuilder {
    enabled: Option<bool>,
    invoice_data: Option<stripe_checkout::PaymentPagesCheckoutSessionInvoiceSettings>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionInvoiceCreation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionInvoiceCreation>,
        builder: PaymentPagesCheckoutSessionInvoiceCreationBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionInvoiceCreation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionInvoiceCreationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionInvoiceCreationBuilder {
        type Out = PaymentPagesCheckoutSessionInvoiceCreation;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
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

    impl ObjectDeser for PaymentPagesCheckoutSessionInvoiceCreation {
        type Builder = PaymentPagesCheckoutSessionInvoiceCreationBuilder;
    }
};
