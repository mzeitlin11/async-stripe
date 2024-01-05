/// A test clock enables deterministic control over objects in testmode.
/// With a test clock, you can create.
/// objects at a frozen time in the past or future, and advance to a specific future time to observe webhooks and state changes.
/// After the clock advances,.
/// you can either validate the current state of your scenario (and test your assumptions), change the current state of your scenario (and test more complex scenarios), or keep advancing forward in time.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TestHelpersTestClock {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which this clock is scheduled to auto delete.
    pub deletes_after: stripe_types::Timestamp,
    /// Time at which all objects belonging to this clock are frozen.
    pub frozen_time: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_shared::TestHelpersTestClockId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The custom name supplied at creation.
    pub name: Option<String>,
    /// The status of the Test Clock.
    pub status: TestHelpersTestClockStatus,
}
#[cfg(feature = "min-ser")]
pub struct TestHelpersTestClockBuilder {
    created: Option<stripe_types::Timestamp>,
    deletes_after: Option<stripe_types::Timestamp>,
    frozen_time: Option<stripe_types::Timestamp>,
    id: Option<stripe_shared::TestHelpersTestClockId>,
    livemode: Option<bool>,
    name: Option<Option<String>>,
    status: Option<TestHelpersTestClockStatus>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TestHelpersTestClock {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TestHelpersTestClock>,
        builder: TestHelpersTestClockBuilder,
    }

    impl Visitor for Place<TestHelpersTestClock> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TestHelpersTestClockBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TestHelpersTestClockBuilder {
        type Out = TestHelpersTestClock;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "deletes_after" => Ok(Deserialize::begin(&mut self.deletes_after)),
                "frozen_time" => Ok(Deserialize::begin(&mut self.frozen_time)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "status" => Ok(Deserialize::begin(&mut self.status)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                deletes_after: Deserialize::default(),
                frozen_time: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                name: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let deletes_after = self.deletes_after.take()?;
            let frozen_time = self.frozen_time.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let name = self.name.take()?;
            let status = self.status.take()?;

            Some(Self::Out { created, deletes_after, frozen_time, id, livemode, name, status })
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

    impl ObjectDeser for TestHelpersTestClock {
        type Builder = TestHelpersTestClockBuilder;
    }
};
/// The status of the Test Clock.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TestHelpersTestClockStatus {
    Advancing,
    InternalFailure,
    Ready,
}
impl TestHelpersTestClockStatus {
    pub fn as_str(self) -> &'static str {
        use TestHelpersTestClockStatus::*;
        match self {
            Advancing => "advancing",
            InternalFailure => "internal_failure",
            Ready => "ready",
        }
    }
}

impl std::str::FromStr for TestHelpersTestClockStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TestHelpersTestClockStatus::*;
        match s {
            "advancing" => Ok(Advancing),
            "internal_failure" => Ok(InternalFailure),
            "ready" => Ok(Ready),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TestHelpersTestClockStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TestHelpersTestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TestHelpersTestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TestHelpersTestClockStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TestHelpersTestClockStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TestHelpersTestClockStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TestHelpersTestClockStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TestHelpersTestClockStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TestHelpersTestClockStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TestHelpersTestClock {
    type Id = stripe_shared::TestHelpersTestClockId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TestHelpersTestClockId, "clock_");
