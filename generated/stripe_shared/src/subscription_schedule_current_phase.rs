#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionScheduleCurrentPhase {
    /// The end of this phase of the subscription schedule.
    pub end_date: stripe_types::Timestamp,
    /// The start of this phase of the subscription schedule.
    pub start_date: stripe_types::Timestamp,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionScheduleCurrentPhaseBuilder {
    end_date: Option<stripe_types::Timestamp>,
    start_date: Option<stripe_types::Timestamp>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionScheduleCurrentPhase {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionScheduleCurrentPhase>,
        builder: SubscriptionScheduleCurrentPhaseBuilder,
    }

    impl Visitor for Place<SubscriptionScheduleCurrentPhase> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionScheduleCurrentPhaseBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionScheduleCurrentPhaseBuilder {
        type Out = SubscriptionScheduleCurrentPhase;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "end_date" => Ok(Deserialize::begin(&mut self.end_date)),
                "start_date" => Ok(Deserialize::begin(&mut self.start_date)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { end_date: Deserialize::default(), start_date: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let end_date = self.end_date.take()?;
            let start_date = self.start_date.take()?;

            Some(Self::Out { end_date, start_date })
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

    impl ObjectDeser for SubscriptionScheduleCurrentPhase {
        type Builder = SubscriptionScheduleCurrentPhaseBuilder;
    }
};
