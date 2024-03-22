#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceInvoiceSettings {
    /// The account tax IDs associated with the invoice.
    pub account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>,
    /// A list of up to 4 custom fields to be displayed on the invoice.
    pub custom_fields: Option<Vec<stripe_shared::InvoiceSettingCustomField>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Footer to be displayed on the invoice.
    pub footer: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Options for invoice PDF rendering.
    pub rendering_options: Option<stripe_shared::InvoiceSettingRenderingOptions>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceInvoiceSettingsBuilder {
    account_tax_ids: Option<Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>>,
    custom_fields: Option<Option<Vec<stripe_shared::InvoiceSettingCustomField>>>,
    description: Option<Option<String>>,
    footer: Option<Option<String>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    rendering_options: Option<Option<stripe_shared::InvoiceSettingRenderingOptions>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceInvoiceSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceInvoiceSettings>,
        builder: PaymentLinksResourceInvoiceSettingsBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceInvoiceSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceInvoiceSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceInvoiceSettingsBuilder {
        type Out = PaymentLinksResourceInvoiceSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account_tax_ids" => Ok(Deserialize::begin(&mut self.account_tax_ids)),
                "custom_fields" => Ok(Deserialize::begin(&mut self.custom_fields)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "footer" => Ok(Deserialize::begin(&mut self.footer)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "rendering_options" => Ok(Deserialize::begin(&mut self.rendering_options)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account_tax_ids: Deserialize::default(),
                custom_fields: Deserialize::default(),
                description: Deserialize::default(),
                footer: Deserialize::default(),
                metadata: Deserialize::default(),
                rendering_options: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_tax_ids = self.account_tax_ids.take()?;
            let custom_fields = self.custom_fields.take()?;
            let description = self.description.take()?;
            let footer = self.footer.take()?;
            let metadata = self.metadata.take()?;
            let rendering_options = self.rendering_options.take()?;

            Some(Self::Out { account_tax_ids, custom_fields, description, footer, metadata, rendering_options })
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

    impl ObjectDeser for PaymentLinksResourceInvoiceSettings {
        type Builder = PaymentLinksResourceInvoiceSettingsBuilder;
    }
};
