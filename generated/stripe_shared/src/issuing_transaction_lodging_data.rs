#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingTransactionLodgingData {
    /// The time of checking into the lodging.
    pub check_in_at: Option<i64>,
    /// The number of nights stayed at the lodging.
    pub nights: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTransactionLodgingDataBuilder {
    check_in_at: Option<Option<i64>>,
    nights: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionLodgingData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionLodgingData>,
        builder: IssuingTransactionLodgingDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionLodgingData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTransactionLodgingDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTransactionLodgingDataBuilder {
        type Out = IssuingTransactionLodgingData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "check_in_at" => Ok(Deserialize::begin(&mut self.check_in_at)),
                "nights" => Ok(Deserialize::begin(&mut self.nights)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { check_in_at: Deserialize::default(), nights: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let check_in_at = self.check_in_at.take()?;
            let nights = self.nights.take()?;

            Some(Self::Out { check_in_at, nights })
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

    impl ObjectDeser for IssuingTransactionLodgingData {
        type Builder = IssuingTransactionLodgingDataBuilder;
    }
};
