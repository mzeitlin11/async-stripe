/// The Report Type resource corresponds to a particular type of report, such as
/// the "Activity summary" or "Itemized payouts" reports. These objects are
/// identified by an ID belonging to a set of enumerated values. See
/// [API Access to Reports documentation](https://stripe.com/docs/reporting/statements/api)
/// for those Report Type IDs, along with required and optional parameters.
///
/// Note that certain report types can only be run based on your live-mode data (not test-mode
/// data), and will error when queried without a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).
///
/// For more details see <<https://stripe.com/docs/api/reporting/report_type/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReportingReportType {
    /// Most recent time for which this Report Type is available. Measured in seconds since the Unix epoch.
    pub data_available_end: stripe_types::Timestamp,
    /// Earliest time for which this Report Type is available. Measured in seconds since the Unix epoch.
    pub data_available_start: stripe_types::Timestamp,
    /// List of column names that are included by default when this Report Type gets run.
    /// (If the Report Type doesn't support the `columns` parameter, this will be null.).
    pub default_columns: Option<Vec<String>>,
    /// The [ID of the Report Type](https://stripe.com/docs/reporting/statements/api#available-report-types), such as `balance.summary.1`.
    pub id: stripe_misc::ReportingReportTypeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Human-readable name of the Report Type
    pub name: String,
    /// When this Report Type was latest updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
    /// Version of the Report Type.
    /// Different versions report with the same ID will have the same purpose, but may take different run parameters or have different result schemas.
    pub version: i64,
}
#[cfg(feature = "min-ser")]
pub struct ReportingReportTypeBuilder {
    data_available_end: Option<stripe_types::Timestamp>,
    data_available_start: Option<stripe_types::Timestamp>,
    default_columns: Option<Option<Vec<String>>>,
    id: Option<stripe_misc::ReportingReportTypeId>,
    livemode: Option<bool>,
    name: Option<String>,
    updated: Option<stripe_types::Timestamp>,
    version: Option<i64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ReportingReportType {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReportingReportType>,
        builder: ReportingReportTypeBuilder,
    }

    impl Visitor for Place<ReportingReportType> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ReportingReportTypeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ReportingReportTypeBuilder {
        type Out = ReportingReportType;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "data_available_end" => Ok(Deserialize::begin(&mut self.data_available_end)),
                "data_available_start" => Ok(Deserialize::begin(&mut self.data_available_start)),
                "default_columns" => Ok(Deserialize::begin(&mut self.default_columns)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "updated" => Ok(Deserialize::begin(&mut self.updated)),
                "version" => Ok(Deserialize::begin(&mut self.version)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                data_available_end: Deserialize::default(),
                data_available_start: Deserialize::default(),
                default_columns: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                name: Deserialize::default(),
                updated: Deserialize::default(),
                version: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let data_available_end = self.data_available_end.take()?;
            let data_available_start = self.data_available_start.take()?;
            let default_columns = self.default_columns.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let name = self.name.take()?;
            let updated = self.updated.take()?;
            let version = self.version.take()?;

            Some(Self::Out { data_available_end, data_available_start, default_columns, id, livemode, name, updated, version })
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

    impl ObjectDeser for ReportingReportType {
        type Builder = ReportingReportTypeBuilder;
    }
};
impl stripe_types::Object for ReportingReportType {
    type Id = stripe_misc::ReportingReportTypeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ReportingReportTypeId);
