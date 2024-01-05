/// A subscription schedule allows you to create and manage the lifecycle of a subscription by predefining expected changes.
///
/// Related guide: [Subscription schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
///
/// For more details see <<https://stripe.com/docs/api/subscription_schedules/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionSchedule {
    /// ID of the Connect Application that created the schedule.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// Time at which the subscription schedule was canceled. Measured in seconds since the Unix epoch.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Time at which the subscription schedule was completed. Measured in seconds since the Unix epoch.
    pub completed_at: Option<stripe_types::Timestamp>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`.
    pub current_phase: Option<stripe_shared::SubscriptionScheduleCurrentPhase>,
    /// ID of the customer who owns the subscription schedule.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    pub default_settings: stripe_shared::SubscriptionSchedulesResourceDefaultSettings,
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    pub end_behavior: stripe_shared::SubscriptionScheduleEndBehavior,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionScheduleId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Configuration for the subscription schedule's phases.
    pub phases: Vec<stripe_shared::SubscriptionSchedulePhaseConfiguration>,
    /// Time at which the subscription schedule was released. Measured in seconds since the Unix epoch.
    pub released_at: Option<stripe_types::Timestamp>,
    /// ID of the subscription once managed by the subscription schedule (if it is released).
    pub released_subscription: Option<String>,
    /// The present status of the subscription schedule.
    /// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
    /// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
    pub status: SubscriptionScheduleStatus,
    /// ID of the subscription managed by the subscription schedule.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// ID of the test clock this subscription schedule belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionScheduleBuilder {
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    completed_at: Option<Option<stripe_types::Timestamp>>,
    created: Option<stripe_types::Timestamp>,
    current_phase: Option<Option<stripe_shared::SubscriptionScheduleCurrentPhase>>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    default_settings: Option<stripe_shared::SubscriptionSchedulesResourceDefaultSettings>,
    end_behavior: Option<stripe_shared::SubscriptionScheduleEndBehavior>,
    id: Option<stripe_shared::SubscriptionScheduleId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    phases: Option<Vec<stripe_shared::SubscriptionSchedulePhaseConfiguration>>,
    released_at: Option<Option<stripe_types::Timestamp>>,
    released_subscription: Option<Option<String>>,
    status: Option<SubscriptionScheduleStatus>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionSchedule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedule>,
        builder: SubscriptionScheduleBuilder,
    }

    impl Visitor for Place<SubscriptionSchedule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionScheduleBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionScheduleBuilder {
        type Out = SubscriptionSchedule;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "canceled_at" => Ok(Deserialize::begin(&mut self.canceled_at)),
                "completed_at" => Ok(Deserialize::begin(&mut self.completed_at)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "current_phase" => Ok(Deserialize::begin(&mut self.current_phase)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "default_settings" => Ok(Deserialize::begin(&mut self.default_settings)),
                "end_behavior" => Ok(Deserialize::begin(&mut self.end_behavior)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "phases" => Ok(Deserialize::begin(&mut self.phases)),
                "released_at" => Ok(Deserialize::begin(&mut self.released_at)),
                "released_subscription" => Ok(Deserialize::begin(&mut self.released_subscription)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "subscription" => Ok(Deserialize::begin(&mut self.subscription)),
                "test_clock" => Ok(Deserialize::begin(&mut self.test_clock)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                application: Deserialize::default(),
                canceled_at: Deserialize::default(),
                completed_at: Deserialize::default(),
                created: Deserialize::default(),
                current_phase: Deserialize::default(),
                customer: Deserialize::default(),
                default_settings: Deserialize::default(),
                end_behavior: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                phases: Deserialize::default(),
                released_at: Deserialize::default(),
                released_subscription: Deserialize::default(),
                status: Deserialize::default(),
                subscription: Deserialize::default(),
                test_clock: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let application = self.application.take()?;
            let canceled_at = self.canceled_at.take()?;
            let completed_at = self.completed_at.take()?;
            let created = self.created.take()?;
            let current_phase = self.current_phase.take()?;
            let customer = self.customer.take()?;
            let default_settings = self.default_settings.take()?;
            let end_behavior = self.end_behavior.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let phases = self.phases.take()?;
            let released_at = self.released_at.take()?;
            let released_subscription = self.released_subscription.take()?;
            let status = self.status.take()?;
            let subscription = self.subscription.take()?;
            let test_clock = self.test_clock.take()?;

            Some(Self::Out {
                application,
                canceled_at,
                completed_at,
                created,
                current_phase,
                customer,
                default_settings,
                end_behavior,
                id,
                livemode,
                metadata,
                phases,
                released_at,
                released_subscription,
                status,
                subscription,
                test_clock,
            })
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

    impl ObjectDeser for SubscriptionSchedule {
        type Builder = SubscriptionScheduleBuilder;
    }
};
/// The present status of the subscription schedule.
/// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
/// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
#[derive(Copy, Clone, Eq, PartialEq)]
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionScheduleStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionScheduleStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SubscriptionScheduleStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionScheduleStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for SubscriptionSchedule {
    type Id = stripe_shared::SubscriptionScheduleId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SubscriptionScheduleId, "sub_sched_");
#[derive(Copy, Clone, Eq, PartialEq)]
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionScheduleEndBehavior"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionScheduleEndBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SubscriptionScheduleEndBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionScheduleEndBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
