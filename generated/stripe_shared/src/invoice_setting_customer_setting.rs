#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceSettingCustomerSetting {
    /// Default custom fields to be displayed on invoices for this customer.
    pub custom_fields: Option<Vec<stripe_shared::InvoiceSettingCustomField>>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    pub default_payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// Default footer to be displayed on invoices for this customer.
    pub footer: Option<String>,
    /// Default options for invoice PDF rendering for this customer.
    pub rendering_options: Option<stripe_shared::InvoiceSettingRenderingOptions>,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceSettingCustomerSettingBuilder {
    custom_fields: Option<Option<Vec<stripe_shared::InvoiceSettingCustomField>>>,
    default_payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    footer: Option<Option<String>>,
    rendering_options: Option<Option<stripe_shared::InvoiceSettingRenderingOptions>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceSettingCustomerSetting {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceSettingCustomerSetting>,
        builder: InvoiceSettingCustomerSettingBuilder,
    }

    impl Visitor for Place<InvoiceSettingCustomerSetting> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceSettingCustomerSettingBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceSettingCustomerSettingBuilder {
        type Out = InvoiceSettingCustomerSetting;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "custom_fields" => Ok(Deserialize::begin(&mut self.custom_fields)),
                "default_payment_method" => Ok(Deserialize::begin(&mut self.default_payment_method)),
                "footer" => Ok(Deserialize::begin(&mut self.footer)),
                "rendering_options" => Ok(Deserialize::begin(&mut self.rendering_options)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { custom_fields: Deserialize::default(), default_payment_method: Deserialize::default(), footer: Deserialize::default(), rendering_options: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let custom_fields = self.custom_fields.take()?;
            let default_payment_method = self.default_payment_method.take()?;
            let footer = self.footer.take()?;
            let rendering_options = self.rendering_options.take()?;

            Some(Self::Out { custom_fields, default_payment_method, footer, rendering_options })
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

    impl ObjectDeser for InvoiceSettingCustomerSetting {
        type Builder = InvoiceSettingCustomerSettingBuilder;
    }
};
