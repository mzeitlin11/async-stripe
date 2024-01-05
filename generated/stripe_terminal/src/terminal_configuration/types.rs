/// A Configurations object represents how features should be configured for terminal readers.
///
/// For more details see <<https://stripe.com/docs/api/terminal/configuration/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TerminalConfiguration {
    pub bbpos_wisepos_e: Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalConfigurationId,
    /// Whether this Configuration is the default for your account
    pub is_account_default: Option<bool>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub offline: Option<stripe_terminal::TerminalConfigurationConfigurationResourceOfflineConfig>,
    pub tipping: Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>,
    pub verifone_p400: Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
}
#[cfg(feature = "min-ser")]
pub struct TerminalConfigurationBuilder {
    bbpos_wisepos_e: Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>>,
    id: Option<stripe_terminal::TerminalConfigurationId>,
    is_account_default: Option<Option<bool>>,
    livemode: Option<bool>,
    offline: Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceOfflineConfig>>,
    tipping: Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>>,
    verifone_p400: Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalConfiguration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfiguration>,
        builder: TerminalConfigurationBuilder,
    }

    impl Visitor for Place<TerminalConfiguration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TerminalConfigurationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TerminalConfigurationBuilder {
        type Out = TerminalConfiguration;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bbpos_wisepos_e" => Ok(Deserialize::begin(&mut self.bbpos_wisepos_e)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "is_account_default" => Ok(Deserialize::begin(&mut self.is_account_default)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "offline" => Ok(Deserialize::begin(&mut self.offline)),
                "tipping" => Ok(Deserialize::begin(&mut self.tipping)),
                "verifone_p400" => Ok(Deserialize::begin(&mut self.verifone_p400)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                bbpos_wisepos_e: Deserialize::default(),
                id: Deserialize::default(),
                is_account_default: Deserialize::default(),
                livemode: Deserialize::default(),
                offline: Deserialize::default(),
                tipping: Deserialize::default(),
                verifone_p400: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bbpos_wisepos_e = self.bbpos_wisepos_e.take()?;
            let id = self.id.take()?;
            let is_account_default = self.is_account_default.take()?;
            let livemode = self.livemode.take()?;
            let offline = self.offline.take()?;
            let tipping = self.tipping.take()?;
            let verifone_p400 = self.verifone_p400.take()?;

            Some(Self::Out { bbpos_wisepos_e, id, is_account_default, livemode, offline, tipping, verifone_p400 })
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

    impl ObjectDeser for TerminalConfiguration {
        type Builder = TerminalConfigurationBuilder;
    }
};
impl stripe_types::Object for TerminalConfiguration {
    type Id = stripe_terminal::TerminalConfigurationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TerminalConfigurationId, "tmc_");
