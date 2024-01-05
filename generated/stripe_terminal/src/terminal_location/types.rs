/// A Location represents a grouping of readers.
///
/// Related guide: [Fleet management](https://stripe.com/docs/terminal/fleet/locations)
///
/// For more details see <<https://stripe.com/docs/api/terminal/locations/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TerminalLocation {
    pub address: stripe_shared::Address,
    /// The ID of a configuration that will be used to customize all readers in this location.
    pub configuration_overrides: Option<String>,
    /// The display name of the location.
    pub display_name: String,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalLocationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
}
#[cfg(feature = "min-ser")]
pub struct TerminalLocationBuilder {
    address: Option<stripe_shared::Address>,
    configuration_overrides: Option<Option<String>>,
    display_name: Option<String>,
    id: Option<stripe_terminal::TerminalLocationId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalLocation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalLocation>,
        builder: TerminalLocationBuilder,
    }

    impl Visitor for Place<TerminalLocation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TerminalLocationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TerminalLocationBuilder {
        type Out = TerminalLocation;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),
                "configuration_overrides" => Ok(Deserialize::begin(&mut self.configuration_overrides)),
                "display_name" => Ok(Deserialize::begin(&mut self.display_name)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                configuration_overrides: Deserialize::default(),
                display_name: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;
            let configuration_overrides = self.configuration_overrides.take()?;
            let display_name = self.display_name.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;

            Some(Self::Out { address, configuration_overrides, display_name, id, livemode, metadata })
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

    impl ObjectDeser for TerminalLocation {
        type Builder = TerminalLocationBuilder;
    }
};
impl stripe_types::Object for TerminalLocation {
    type Id = stripe_terminal::TerminalLocationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TerminalLocationId, "tml_");
