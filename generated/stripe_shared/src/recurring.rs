#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Recurring {
    /// Specifies a usage aggregation strategy for prices of `usage_type=metered`.
    /// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period.
    /// Defaults to `sum`.
    pub aggregate_usage: Option<RecurringAggregateUsage>,
    /// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: RecurringInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    /// Default number of trial days when subscribing a customer to this price using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined.
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub usage_type: RecurringUsageType,
}
#[cfg(feature = "min-ser")]
pub struct RecurringBuilder {
    aggregate_usage: Option<Option<RecurringAggregateUsage>>,
    interval: Option<RecurringInterval>,
    interval_count: Option<u64>,
    trial_period_days: Option<Option<u32>>,
    usage_type: Option<RecurringUsageType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Recurring {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Recurring>,
        builder: RecurringBuilder,
    }

    impl Visitor for Place<Recurring> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RecurringBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RecurringBuilder {
        type Out = Recurring;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "aggregate_usage" => Ok(Deserialize::begin(&mut self.aggregate_usage)),
                "interval" => Ok(Deserialize::begin(&mut self.interval)),
                "interval_count" => Ok(Deserialize::begin(&mut self.interval_count)),
                "trial_period_days" => Ok(Deserialize::begin(&mut self.trial_period_days)),
                "usage_type" => Ok(Deserialize::begin(&mut self.usage_type)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                aggregate_usage: Deserialize::default(),
                interval: Deserialize::default(),
                interval_count: Deserialize::default(),
                trial_period_days: Deserialize::default(),
                usage_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let aggregate_usage = self.aggregate_usage.take()?;
            let interval = self.interval.take()?;
            let interval_count = self.interval_count.take()?;
            let trial_period_days = self.trial_period_days.take()?;
            let usage_type = self.usage_type.take()?;

            Some(Self::Out { aggregate_usage, interval, interval_count, trial_period_days, usage_type })
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

    impl ObjectDeser for Recurring {
        type Builder = RecurringBuilder;
    }
};
/// Specifies a usage aggregation strategy for prices of `usage_type=metered`.
/// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period.
/// Defaults to `sum`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RecurringAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}
impl RecurringAggregateUsage {
    pub fn as_str(self) -> &'static str {
        use RecurringAggregateUsage::*;
        match self {
            LastDuringPeriod => "last_during_period",
            LastEver => "last_ever",
            Max => "max",
            Sum => "sum",
        }
    }
}

impl std::str::FromStr for RecurringAggregateUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RecurringAggregateUsage::*;
        match s {
            "last_during_period" => Ok(LastDuringPeriod),
            "last_ever" => Ok(LastEver),
            "max" => Ok(Max),
            "sum" => Ok(Sum),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for RecurringAggregateUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for RecurringAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RecurringAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RecurringAggregateUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RecurringAggregateUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for RecurringAggregateUsage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RecurringAggregateUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<RecurringAggregateUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RecurringAggregateUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl RecurringInterval {
    pub fn as_str(self) -> &'static str {
        use RecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for RecurringInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for RecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for RecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for RecurringInterval"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RecurringInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<RecurringInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RecurringInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Configures how the quantity per period should be determined.
/// Can be either `metered` or `licensed`.
/// `licensed` automatically bills the `quantity` set when adding it to a subscription.
/// `metered` aggregates the total usage based on usage records.
/// Defaults to `licensed`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RecurringUsageType {
    Licensed,
    Metered,
}
impl RecurringUsageType {
    pub fn as_str(self) -> &'static str {
        use RecurringUsageType::*;
        match self {
            Licensed => "licensed",
            Metered => "metered",
        }
    }
}

impl std::str::FromStr for RecurringUsageType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RecurringUsageType::*;
        match s {
            "licensed" => Ok(Licensed),
            "metered" => Ok(Metered),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for RecurringUsageType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for RecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RecurringUsageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RecurringUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for RecurringUsageType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RecurringUsageType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<RecurringUsageType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RecurringUsageType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
