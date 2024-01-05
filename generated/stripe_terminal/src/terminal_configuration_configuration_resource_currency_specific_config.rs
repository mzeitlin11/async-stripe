#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    /// Fixed amounts displayed when collecting a tip
    pub fixed_amounts: Option<Vec<i64>>,
    /// Percentages displayed when collecting a tip
    pub percentages: Option<Vec<i64>>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    pub smart_tip_threshold: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder {
    fixed_amounts: Option<Option<Vec<i64>>>,
    percentages: Option<Option<Vec<i64>>>,
    smart_tip_threshold: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
        builder: TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceCurrencySpecificConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder {
        type Out = TerminalConfigurationConfigurationResourceCurrencySpecificConfig;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "fixed_amounts" => Ok(Deserialize::begin(&mut self.fixed_amounts)),
                "percentages" => Ok(Deserialize::begin(&mut self.percentages)),
                "smart_tip_threshold" => Ok(Deserialize::begin(&mut self.smart_tip_threshold)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { fixed_amounts: Deserialize::default(), percentages: Deserialize::default(), smart_tip_threshold: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let fixed_amounts = self.fixed_amounts.take()?;
            let percentages = self.percentages.take()?;
            let smart_tip_threshold = self.smart_tip_threshold.take()?;

            Some(Self::Out { fixed_amounts, percentages, smart_tip_threshold })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
        type Builder = TerminalConfigurationConfigurationResourceCurrencySpecificConfigBuilder;
    }
};
