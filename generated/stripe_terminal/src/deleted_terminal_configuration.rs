#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedTerminalConfiguration {
    /// Always true for a deleted object
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalConfigurationId,
}
#[cfg(feature = "min-ser")]
pub struct DeletedTerminalConfigurationBuilder {
    deleted: Option<stripe_types::AlwaysTrue>,
    id: Option<stripe_terminal::TerminalConfigurationId>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DeletedTerminalConfiguration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DeletedTerminalConfiguration>,
        builder: DeletedTerminalConfigurationBuilder,
    }

    impl Visitor for Place<DeletedTerminalConfiguration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DeletedTerminalConfigurationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DeletedTerminalConfigurationBuilder {
        type Out = DeletedTerminalConfiguration;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "deleted" => Ok(Deserialize::begin(&mut self.deleted)),
                "id" => Ok(Deserialize::begin(&mut self.id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { deleted: Deserialize::default(), id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let deleted = self.deleted.take()?;
            let id = self.id.take()?;

            Some(Self::Out { deleted, id })
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

    impl ObjectDeser for DeletedTerminalConfiguration {
        type Builder = DeletedTerminalConfigurationBuilder;
    }
};
impl stripe_types::Object for DeletedTerminalConfiguration {
    type Id = stripe_terminal::TerminalConfigurationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
