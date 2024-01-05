#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct QuotesResourceRecurring {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: QuotesResourceRecurringInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    pub total_details: stripe_shared::QuotesResourceTotalDetails,
}
#[cfg(feature = "min-ser")]
pub struct QuotesResourceRecurringBuilder {
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    interval: Option<QuotesResourceRecurringInterval>,
    interval_count: Option<u64>,
    total_details: Option<stripe_shared::QuotesResourceTotalDetails>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceRecurring {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceRecurring>,
        builder: QuotesResourceRecurringBuilder,
    }

    impl Visitor for Place<QuotesResourceRecurring> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: QuotesResourceRecurringBuilder::deser_default() }))
        }
    }

    impl MapBuilder for QuotesResourceRecurringBuilder {
        type Out = QuotesResourceRecurring;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount_subtotal" => Ok(Deserialize::begin(&mut self.amount_subtotal)),
                "amount_total" => Ok(Deserialize::begin(&mut self.amount_total)),
                "interval" => Ok(Deserialize::begin(&mut self.interval)),
                "interval_count" => Ok(Deserialize::begin(&mut self.interval_count)),
                "total_details" => Ok(Deserialize::begin(&mut self.total_details)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount_subtotal: Deserialize::default(),
                amount_total: Deserialize::default(),
                interval: Deserialize::default(),
                interval_count: Deserialize::default(),
                total_details: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_subtotal = self.amount_subtotal.take()?;
            let amount_total = self.amount_total.take()?;
            let interval = self.interval.take()?;
            let interval_count = self.interval_count.take()?;
            let total_details = self.total_details.take()?;

            Some(Self::Out { amount_subtotal, amount_total, interval, interval_count, total_details })
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

    impl ObjectDeser for QuotesResourceRecurring {
        type Builder = QuotesResourceRecurringBuilder;
    }
};
/// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QuotesResourceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl QuotesResourceRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use QuotesResourceRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for QuotesResourceRecurringInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuotesResourceRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for QuotesResourceRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for QuotesResourceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for QuotesResourceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for QuotesResourceRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotesResourceRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for QuotesResourceRecurringInterval"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for QuotesResourceRecurringInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<QuotesResourceRecurringInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(QuotesResourceRecurringInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
