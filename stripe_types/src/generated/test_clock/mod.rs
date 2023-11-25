/// A test clock enables deterministic control over objects in testmode.
///
/// With a test clock, you can create objects at a frozen time in the past or future, and advance to a specific future time to observe webhooks and state changes.
/// After the clock advances, you can either validate the current state of your scenario (and test your assumptions), change the current state of your scenario (and test more complex scenarios), or keep advancing forward in time.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TestClock {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which this clock is scheduled to auto delete.
    pub deletes_after: stripe_types::Timestamp,
    /// Time at which all objects belonging to this clock are frozen.
    pub frozen_time: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_types::test_clock::TestHelpersTestClockId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The custom name supplied at creation.
    pub name: Option<String>,
    /// The status of the Test Clock.
    pub status: TestClockStatus,
}
/// The status of the Test Clock.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TestClockStatus {
    Advancing,
    InternalFailure,
    Ready,
}
impl TestClockStatus {
    pub fn as_str(self) -> &'static str {
        use TestClockStatus::*;
        match self {
            Advancing => "advancing",
            InternalFailure => "internal_failure",
            Ready => "ready",
        }
    }
}

impl std::str::FromStr for TestClockStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TestClockStatus::*;
        match s {
            "advancing" => Ok(Advancing),
            "internal_failure" => Ok(InternalFailure),
            "ready" => Ok(Ready),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TestClockStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TestClockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TestClockStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TestClockStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TestClockStatus"))
    }
}
impl stripe_types::Object for TestClock {
    type Id = stripe_types::test_clock::TestHelpersTestClockId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TestHelpersTestClockId, "clock_");
