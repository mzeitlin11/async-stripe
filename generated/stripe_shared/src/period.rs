#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Period {
    /// The end date of this usage period. All usage up to and including this point in time is included.
    pub end: Option<stripe_types::Timestamp>,
    /// The start date of this usage period. All usage after this point in time is included.
    pub start: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct PeriodBuilder {
    end: Option<Option<stripe_types::Timestamp>>,
    start: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Period {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Period>,
        builder: PeriodBuilder,
    }

    impl Visitor for Place<Period> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PeriodBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PeriodBuilder {
        type Out = Period;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "end" => Ok(Deserialize::begin(&mut self.end)),
                "start" => Ok(Deserialize::begin(&mut self.start)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { end: Deserialize::default(), start: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let end = self.end.take()?;
            let start = self.start.take()?;

            Some(Self::Out { end, start })
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

    impl ObjectDeser for Period {
        type Builder = PeriodBuilder;
    }
};
