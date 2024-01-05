#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceSettingCustomField {
    /// The name of the custom field.
    pub name: String,
    /// The value of the custom field.
    pub value: String,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceSettingCustomFieldBuilder {
    name: Option<String>,
    value: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceSettingCustomField {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceSettingCustomField>,
        builder: InvoiceSettingCustomFieldBuilder,
    }

    impl Visitor for Place<InvoiceSettingCustomField> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceSettingCustomFieldBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceSettingCustomFieldBuilder {
        type Out = InvoiceSettingCustomField;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "value" => Ok(Deserialize::begin(&mut self.value)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { name: Deserialize::default(), value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let name = self.name.take()?;
            let value = self.value.take()?;

            Some(Self::Out { name, value })
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

    impl ObjectDeser for InvoiceSettingCustomField {
        type Builder = InvoiceSettingCustomFieldBuilder;
    }
};
