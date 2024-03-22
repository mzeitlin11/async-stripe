#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialReportingFinanceReportRunRunParameters {
    /// The set of output columns requested for inclusion in the report run.
    pub columns: Option<Vec<String>>,
    /// Connected account ID by which to filter the report run.
    pub connected_account: Option<String>,
    /// Currency of objects to be included in the report run.
    pub currency: Option<stripe_types::Currency>,
    /// Ending timestamp of data to be included in the report run.
    /// Can be any UTC timestamp between 1 second after the user specified `interval_start` and 1 second before this report's last `data_available_end` value.
    pub interval_end: Option<stripe_types::Timestamp>,
    /// Starting timestamp of data to be included in the report run.
    /// Can be any UTC timestamp between 1 second after this report's `data_available_start` and 1 second before the user specified `interval_end` value.
    pub interval_start: Option<stripe_types::Timestamp>,
    /// Payout ID by which to filter the report run.
    pub payout: Option<String>,
    /// Category of balance transactions to be included in the report run.
    pub reporting_category: Option<String>,
    /// Defaults to `Etc/UTC`.
    /// The output timezone for all timestamps in the report.
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    /// Has no effect on `interval_start` or `interval_end`.
    pub timezone: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct FinancialReportingFinanceReportRunRunParametersBuilder {
    columns: Option<Option<Vec<String>>>,
    connected_account: Option<Option<String>>,
    currency: Option<Option<stripe_types::Currency>>,
    interval_end: Option<Option<stripe_types::Timestamp>>,
    interval_start: Option<Option<stripe_types::Timestamp>>,
    payout: Option<Option<String>>,
    reporting_category: Option<Option<String>>,
    timezone: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FinancialReportingFinanceReportRunRunParameters {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FinancialReportingFinanceReportRunRunParameters>,
        builder: FinancialReportingFinanceReportRunRunParametersBuilder,
    }

    impl Visitor for Place<FinancialReportingFinanceReportRunRunParameters> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FinancialReportingFinanceReportRunRunParametersBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FinancialReportingFinanceReportRunRunParametersBuilder {
        type Out = FinancialReportingFinanceReportRunRunParameters;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "columns" => Ok(Deserialize::begin(&mut self.columns)),
                "connected_account" => Ok(Deserialize::begin(&mut self.connected_account)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "interval_end" => Ok(Deserialize::begin(&mut self.interval_end)),
                "interval_start" => Ok(Deserialize::begin(&mut self.interval_start)),
                "payout" => Ok(Deserialize::begin(&mut self.payout)),
                "reporting_category" => Ok(Deserialize::begin(&mut self.reporting_category)),
                "timezone" => Ok(Deserialize::begin(&mut self.timezone)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                columns: Deserialize::default(),
                connected_account: Deserialize::default(),
                currency: Deserialize::default(),
                interval_end: Deserialize::default(),
                interval_start: Deserialize::default(),
                payout: Deserialize::default(),
                reporting_category: Deserialize::default(),
                timezone: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let columns = self.columns.take()?;
            let connected_account = self.connected_account.take()?;
            let currency = self.currency.take()?;
            let interval_end = self.interval_end.take()?;
            let interval_start = self.interval_start.take()?;
            let payout = self.payout.take()?;
            let reporting_category = self.reporting_category.take()?;
            let timezone = self.timezone.take()?;

            Some(Self::Out { columns, connected_account, currency, interval_end, interval_start, payout, reporting_category, timezone })
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

    impl ObjectDeser for FinancialReportingFinanceReportRunRunParameters {
        type Builder = FinancialReportingFinanceReportRunRunParametersBuilder;
    }
};
