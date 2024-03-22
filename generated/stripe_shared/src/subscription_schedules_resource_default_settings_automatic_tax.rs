#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    /// Whether Stripe automatically computes tax on invoices created during this phase.
    pub enabled: bool,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder {
    enabled: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourceDefaultSettingsAutomaticTax>,
        builder: SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourceDefaultSettingsAutomaticTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder {
        type Out = SubscriptionSchedulesResourceDefaultSettingsAutomaticTax;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let enabled = self.enabled.take()?;

            Some(Self::Out { enabled })
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

    impl ObjectDeser for SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
        type Builder = SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder;
    }
};
