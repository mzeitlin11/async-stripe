#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourcePending {
    /// The list of missing fields that are required to perform calculations.
    /// It includes the entry `head_office` when the status is `pending`.
    /// It is recommended to set the optional values even if they aren't listed as required for calculating taxes.
    /// Calculations can fail if missing fields aren't explicitly provided on every call.
    pub missing_fields: Option<Vec<String>>,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourcePendingBuilder {
    missing_fields: Option<Option<Vec<String>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxSettingsStatusDetailsResourcePending {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
        builder: TaxProductResourceTaxSettingsStatusDetailsResourcePendingBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxSettingsStatusDetailsResourcePending> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceTaxSettingsStatusDetailsResourcePendingBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxSettingsStatusDetailsResourcePendingBuilder {
        type Out = TaxProductResourceTaxSettingsStatusDetailsResourcePending;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "missing_fields" => Ok(Deserialize::begin(&mut self.missing_fields)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { missing_fields: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let missing_fields = self.missing_fields.take()?;

            Some(Self::Out { missing_fields })
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

    impl ObjectDeser for TaxProductResourceTaxSettingsStatusDetailsResourcePending {
        type Builder = TaxProductResourceTaxSettingsStatusDetailsResourcePendingBuilder;
    }
};
