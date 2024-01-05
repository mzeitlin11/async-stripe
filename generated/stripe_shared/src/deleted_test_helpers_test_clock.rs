#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedTestHelpersTestClock {
    /// Always true for a deleted object
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::TestHelpersTestClockId,
}
#[cfg(feature = "min-ser")]
pub struct DeletedTestHelpersTestClockBuilder {
    deleted: Option<stripe_types::AlwaysTrue>,
    id: Option<stripe_shared::TestHelpersTestClockId>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DeletedTestHelpersTestClock {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DeletedTestHelpersTestClock>,
        builder: DeletedTestHelpersTestClockBuilder,
    }

    impl Visitor for Place<DeletedTestHelpersTestClock> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DeletedTestHelpersTestClockBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DeletedTestHelpersTestClockBuilder {
        type Out = DeletedTestHelpersTestClock;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
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

    impl ObjectDeser for DeletedTestHelpersTestClock {
        type Builder = DeletedTestHelpersTestClockBuilder;
    }
};
impl stripe_types::Object for DeletedTestHelpersTestClock {
    type Id = stripe_shared::TestHelpersTestClockId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
