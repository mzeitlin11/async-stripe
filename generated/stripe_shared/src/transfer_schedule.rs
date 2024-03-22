#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TransferSchedule {
    /// The number of days charges for the account will be held before being paid out.
    pub delay_days: u32,
    /// How frequently funds will be paid out.
    /// One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
    pub interval: String,
    /// The day of the month funds will be paid out.
    /// Only shown if `interval` is monthly.
    /// Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    pub monthly_anchor: Option<u8>,
    /// The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc.
    /// Only shown if `interval` is weekly.
    pub weekly_anchor: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct TransferScheduleBuilder {
    delay_days: Option<u32>,
    interval: Option<String>,
    monthly_anchor: Option<Option<u8>>,
    weekly_anchor: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TransferSchedule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TransferSchedule>,
        builder: TransferScheduleBuilder,
    }

    impl Visitor for Place<TransferSchedule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TransferScheduleBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TransferScheduleBuilder {
        type Out = TransferSchedule;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "delay_days" => Ok(Deserialize::begin(&mut self.delay_days)),
                "interval" => Ok(Deserialize::begin(&mut self.interval)),
                "monthly_anchor" => Ok(Deserialize::begin(&mut self.monthly_anchor)),
                "weekly_anchor" => Ok(Deserialize::begin(&mut self.weekly_anchor)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { delay_days: Deserialize::default(), interval: Deserialize::default(), monthly_anchor: Deserialize::default(), weekly_anchor: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let delay_days = self.delay_days.take()?;
            let interval = self.interval.take()?;
            let monthly_anchor = self.monthly_anchor.take()?;
            let weekly_anchor = self.weekly_anchor.take()?;

            Some(Self::Out { delay_days, interval, monthly_anchor, weekly_anchor })
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

    impl ObjectDeser for TransferSchedule {
        type Builder = TransferScheduleBuilder;
    }
};
