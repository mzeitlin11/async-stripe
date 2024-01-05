#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsStatusDetails {
    pub active: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourceActive>,
    pub pending: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceTaxSettingsStatusDetailsBuilder {
    active: Option<Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourceActive>>,
    pending: Option<Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourcePending>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxSettingsStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxSettingsStatusDetails>,
        builder: TaxProductResourceTaxSettingsStatusDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxSettingsStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceTaxSettingsStatusDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxSettingsStatusDetailsBuilder {
        type Out = TaxProductResourceTaxSettingsStatusDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "active" => Ok(Deserialize::begin(&mut self.active)),
                "pending" => Ok(Deserialize::begin(&mut self.pending)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { active: Deserialize::default(), pending: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let active = self.active.take()?;
            let pending = self.pending.take()?;

            Some(Self::Out { active, pending })
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

    impl ObjectDeser for TaxProductResourceTaxSettingsStatusDetails {
        type Builder = TaxProductResourceTaxSettingsStatusDetailsBuilder;
    }
};
