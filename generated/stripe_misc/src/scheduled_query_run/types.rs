/// If you have [scheduled a Sigma query](https://stripe.com/docs/sigma/scheduled-queries), you'll
/// receive a `sigma.scheduled_query_run.created` webhook each time the query
/// runs. The webhook contains a `ScheduledQueryRun` object, which you can use to
/// retrieve the query results.
///
/// For more details see <<https://stripe.com/docs/api/sigma/scheduled_queries/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ScheduledQueryRun {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// When the query was run, Sigma contained a snapshot of your Stripe data at this time.
    pub data_load_time: stripe_types::Timestamp,
    pub error: Option<stripe_misc::SigmaScheduledQueryRunError>,
    /// The file object representing the results of the query.
    pub file: Option<stripe_shared::File>,
    /// Unique identifier for the object.
    pub id: stripe_misc::ScheduledQueryRunId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Time at which the result expires and is no longer available for download.
    pub result_available_until: stripe_types::Timestamp,
    /// SQL for the query.
    pub sql: String,
    /// The query's execution status, which will be `completed` for successful runs, and `canceled`, `failed`, or `timed_out` otherwise.
    pub status: String,
    /// Title of the query.
    pub title: String,
}
#[cfg(feature = "min-ser")]
pub struct ScheduledQueryRunBuilder {
    created: Option<stripe_types::Timestamp>,
    data_load_time: Option<stripe_types::Timestamp>,
    error: Option<Option<stripe_misc::SigmaScheduledQueryRunError>>,
    file: Option<Option<stripe_shared::File>>,
    id: Option<stripe_misc::ScheduledQueryRunId>,
    livemode: Option<bool>,
    result_available_until: Option<stripe_types::Timestamp>,
    sql: Option<String>,
    status: Option<String>,
    title: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ScheduledQueryRun {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ScheduledQueryRun>,
        builder: ScheduledQueryRunBuilder,
    }

    impl Visitor for Place<ScheduledQueryRun> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ScheduledQueryRunBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ScheduledQueryRunBuilder {
        type Out = ScheduledQueryRun;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "data_load_time" => Ok(Deserialize::begin(&mut self.data_load_time)),
                "error" => Ok(Deserialize::begin(&mut self.error)),
                "file" => Ok(Deserialize::begin(&mut self.file)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "result_available_until" => Ok(Deserialize::begin(&mut self.result_available_until)),
                "sql" => Ok(Deserialize::begin(&mut self.sql)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "title" => Ok(Deserialize::begin(&mut self.title)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                data_load_time: Deserialize::default(),
                error: Deserialize::default(),
                file: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                result_available_until: Deserialize::default(),
                sql: Deserialize::default(),
                status: Deserialize::default(),
                title: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let data_load_time = self.data_load_time.take()?;
            let error = self.error.take()?;
            let file = self.file.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let result_available_until = self.result_available_until.take()?;
            let sql = self.sql.take()?;
            let status = self.status.take()?;
            let title = self.title.take()?;

            Some(Self::Out { created, data_load_time, error, file, id, livemode, result_available_until, sql, status, title })
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

    impl ObjectDeser for ScheduledQueryRun {
        type Builder = ScheduledQueryRunBuilder;
    }
};
impl stripe_types::Object for ScheduledQueryRun {
    type Id = stripe_misc::ScheduledQueryRunId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ScheduledQueryRunId, "sqr_");
