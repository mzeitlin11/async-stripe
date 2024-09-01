/// Point in Time
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoDataDocumentReportExpirationDate {
    /// Numerical day between 1 and 31.
    pub day: Option<i64>,
    /// Numerical month between 1 and 12.
    pub month: Option<i64>,
    /// The four-digit year.
    pub year: Option<i64>,
}
#[doc(hidden)]
pub struct GelatoDataDocumentReportExpirationDateBuilder {
    day: Option<Option<i64>>,
    month: Option<Option<i64>>,
    year: Option<Option<i64>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoDataDocumentReportExpirationDate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoDataDocumentReportExpirationDate>,
        builder: GelatoDataDocumentReportExpirationDateBuilder,
    }

    impl Visitor for Place<GelatoDataDocumentReportExpirationDate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoDataDocumentReportExpirationDateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoDataDocumentReportExpirationDateBuilder {
        type Out = GelatoDataDocumentReportExpirationDate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "day" => Deserialize::begin(&mut self.day),
                "month" => Deserialize::begin(&mut self.month),
                "year" => Deserialize::begin(&mut self.year),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                day: Deserialize::default(),
                month: Deserialize::default(),
                year: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(day), Some(month), Some(year)) = (self.day, self.month, self.year) else {
                return None;
            };
            Some(Self::Out { day, month, year })
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

    impl ObjectDeser for GelatoDataDocumentReportExpirationDate {
        type Builder = GelatoDataDocumentReportExpirationDateBuilder;
    }

    impl FromValueOpt for GelatoDataDocumentReportExpirationDate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoDataDocumentReportExpirationDateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "day" => b.day = FromValueOpt::from_value(v),
                    "month" => b.month = FromValueOpt::from_value(v),
                    "year" => b.year = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
