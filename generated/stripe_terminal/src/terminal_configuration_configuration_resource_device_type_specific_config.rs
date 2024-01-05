#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    /// A File ID representing an image you would like displayed on the reader.
    pub splashscreen: Option<stripe_types::Expandable<stripe_shared::File>>,
}
#[cfg(feature = "min-ser")]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder {
    splashscreen: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
        builder: TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder {
        type Out = TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "splashscreen" => Ok(Deserialize::begin(&mut self.splashscreen)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { splashscreen: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let splashscreen = self.splashscreen.take()?;

            Some(Self::Out { splashscreen })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
        type Builder = TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfigBuilder;
    }
};
