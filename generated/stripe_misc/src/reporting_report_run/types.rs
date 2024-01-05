/// The Report Run object represents an instance of a report type generated with
/// specific run parameters. Once the object is created, Stripe begins processing the report.
/// When the report has finished running, it will give you a reference to a file
/// where you can retrieve your results. For an overview, see
/// [API Access to Reports](https://stripe.com/docs/reporting/statements/api).
///
/// Note that certain report types can only be run based on your live-mode data (not test-mode
/// data), and will error when queried without a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).
///
/// For more details see <<https://stripe.com/docs/api/reporting/report_run/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReportingReportRun {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If something should go wrong during the run, a message about the failure (populated when
    ///  `status=failed`).
    pub error: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::ReportingReportRunId,
    /// `true` if the report is run on live mode data and `false` if it is run on test mode data.
    pub livemode: bool,
    pub parameters: stripe_misc::FinancialReportingFinanceReportRunRunParameters,
    /// The ID of the [report type](https://stripe.com/docs/reports/report-types) to run, such as `"balance.summary.1"`.
    pub report_type: String,
    /// The file object representing the result of the report run (populated when
    ///  `status=succeeded`).
    pub result: Option<stripe_shared::File>,
    /// Status of this report run. This will be `pending` when the run is initially created.
    ///  When the run finishes, this will be set to `succeeded` and the `result` field will be populated.
    /// Rarely, we may encounter an error, at which point this will be set to `failed` and the `error` field will be populated.
    pub status: String,
    /// Timestamp at which this run successfully finished (populated when
    ///  `status=succeeded`). Measured in seconds since the Unix epoch.
    pub succeeded_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct ReportingReportRunBuilder {
    created: Option<stripe_types::Timestamp>,
    error: Option<Option<String>>,
    id: Option<stripe_misc::ReportingReportRunId>,
    livemode: Option<bool>,
    parameters: Option<stripe_misc::FinancialReportingFinanceReportRunRunParameters>,
    report_type: Option<String>,
    result: Option<Option<stripe_shared::File>>,
    status: Option<String>,
    succeeded_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ReportingReportRun {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReportingReportRun>,
        builder: ReportingReportRunBuilder,
    }

    impl Visitor for Place<ReportingReportRun> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ReportingReportRunBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ReportingReportRunBuilder {
        type Out = ReportingReportRun;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "error" => Ok(Deserialize::begin(&mut self.error)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "parameters" => Ok(Deserialize::begin(&mut self.parameters)),
                "report_type" => Ok(Deserialize::begin(&mut self.report_type)),
                "result" => Ok(Deserialize::begin(&mut self.result)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "succeeded_at" => Ok(Deserialize::begin(&mut self.succeeded_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                error: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                parameters: Deserialize::default(),
                report_type: Deserialize::default(),
                result: Deserialize::default(),
                status: Deserialize::default(),
                succeeded_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let error = self.error.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let parameters = self.parameters.take()?;
            let report_type = self.report_type.take()?;
            let result = self.result.take()?;
            let status = self.status.take()?;
            let succeeded_at = self.succeeded_at.take()?;

            Some(Self::Out { created, error, id, livemode, parameters, report_type, result, status, succeeded_at })
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

    impl ObjectDeser for ReportingReportRun {
        type Builder = ReportingReportRunBuilder;
    }
};
impl stripe_types::Object for ReportingReportRun {
    type Id = stripe_misc::ReportingReportRunId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ReportingReportRunId);
