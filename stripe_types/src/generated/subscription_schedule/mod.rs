/// A subscription schedule allows you to create and manage the lifecycle of a subscription by predefining expected changes.
///
/// Related guide: [Subscription schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionSchedule {
    /// ID of the Connect Application that created the schedule.
    pub application: Option<stripe_types::Expandable<stripe_types::application::Application>>,
    /// Time at which the subscription schedule was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Time at which the subscription schedule was completed.
    ///
    /// Measured in seconds since the Unix epoch.
    pub completed_at: Option<stripe_types::Timestamp>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`.
    pub current_phase: Option<stripe_types::current_phase::CurrentPhase>,
    /// ID of the customer who owns the subscription schedule.
    pub customer: stripe_types::Expandable<stripe_types::customer::Customer>,
    pub default_settings: stripe_types::default_settings::DefaultSettings,
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    ///
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    pub end_behavior: SubscriptionScheduleEndBehavior,
    /// Unique identifier for the object.
    pub id: stripe_types::subscription_schedule::SubscriptionScheduleId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SubscriptionScheduleObject,
    /// Configuration for the subscription schedule's phases.
    pub phases: Vec<stripe_types::phase::Phase>,
    /// Time at which the subscription schedule was released.
    ///
    /// Measured in seconds since the Unix epoch.
    pub released_at: Option<stripe_types::Timestamp>,
    /// ID of the subscription once managed by the subscription schedule (if it is released).
    pub released_subscription: Option<String>,
    /// The present status of the subscription schedule.
    ///
    /// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
    /// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
    pub status: SubscriptionScheduleStatus,
    /// ID of the subscription managed by the subscription schedule.
    pub subscription: Option<stripe_types::Expandable<stripe_types::subscription::Subscription>>,
    /// ID of the test clock this subscription schedule belongs to.
    pub test_clock:
        Option<stripe_types::Expandable<stripe_types::test_helpers::test_clock::TestClock>>,
}
/// Behavior of the subscription schedule and underlying subscription when it ends.
///
/// Possible values are `release` or `cancel` with the default being `release`.
/// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}

impl SubscriptionScheduleEndBehavior {
    pub fn as_str(self) -> &'static str {
        use SubscriptionScheduleEndBehavior::*;
        match self {
            Cancel => "cancel",
            None => "none",
            Release => "release",
            Renew => "renew",
        }
    }
}

impl std::str::FromStr for SubscriptionScheduleEndBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionScheduleEndBehavior::*;
        match s {
            "cancel" => Ok(Cancel),
            "none" => Ok(None),
            "release" => Ok(Release),
            "renew" => Ok(Renew),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionScheduleEndBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SubscriptionScheduleEndBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionScheduleEndBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SubscriptionScheduleEndBehavior")
        })
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionScheduleObject {
    SubscriptionSchedule,
}

impl SubscriptionScheduleObject {
    pub fn as_str(self) -> &'static str {
        use SubscriptionScheduleObject::*;
        match self {
            SubscriptionSchedule => "subscription_schedule",
        }
    }
}

impl std::str::FromStr for SubscriptionScheduleObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionScheduleObject::*;
        match s {
            "subscription_schedule" => Ok(SubscriptionSchedule),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionScheduleObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SubscriptionScheduleObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionScheduleObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionScheduleObject"))
    }
}
/// The present status of the subscription schedule.
///
/// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
/// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionScheduleStatus {
    Active,
    Canceled,
    Completed,
    NotStarted,
    Released,
}

impl SubscriptionScheduleStatus {
    pub fn as_str(self) -> &'static str {
        use SubscriptionScheduleStatus::*;
        match self {
            Active => "active",
            Canceled => "canceled",
            Completed => "completed",
            NotStarted => "not_started",
            Released => "released",
        }
    }
}

impl std::str::FromStr for SubscriptionScheduleStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionScheduleStatus::*;
        match s {
            "active" => Ok(Active),
            "canceled" => Ok(Canceled),
            "completed" => Ok(Completed),
            "not_started" => Ok(NotStarted),
            "released" => Ok(Released),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionScheduleStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SubscriptionScheduleStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionScheduleStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionScheduleStatus"))
    }
}
impl stripe_types::Object for SubscriptionSchedule {
    type Id = stripe_types::subscription_schedule::SubscriptionScheduleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(SubscriptionScheduleId, "sub_sched_");
