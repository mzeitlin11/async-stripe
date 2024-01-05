#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct UsageRecordSummary {
    /// Unique identifier for the object.
    pub id: stripe_shared::UsageRecordSummaryId,
    /// The invoice in which this usage period has been billed for.
    pub invoice: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub period: stripe_shared::Period,
    /// The ID of the subscription item this summary is describing.
    pub subscription_item: String,
    /// The total usage within this usage period.
    pub total_usage: i64,
}
#[cfg(feature = "min-ser")]
pub struct UsageRecordSummaryBuilder {
    id: Option<stripe_shared::UsageRecordSummaryId>,
    invoice: Option<Option<String>>,
    livemode: Option<bool>,
    period: Option<stripe_shared::Period>,
    subscription_item: Option<String>,
    total_usage: Option<i64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for UsageRecordSummary {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<UsageRecordSummary>,
        builder: UsageRecordSummaryBuilder,
    }

    impl Visitor for Place<UsageRecordSummary> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: UsageRecordSummaryBuilder::deser_default() }))
        }
    }

    impl MapBuilder for UsageRecordSummaryBuilder {
        type Out = UsageRecordSummary;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "invoice" => Ok(Deserialize::begin(&mut self.invoice)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "period" => Ok(Deserialize::begin(&mut self.period)),
                "subscription_item" => Ok(Deserialize::begin(&mut self.subscription_item)),
                "total_usage" => Ok(Deserialize::begin(&mut self.total_usage)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                livemode: Deserialize::default(),
                period: Deserialize::default(),
                subscription_item: Deserialize::default(),
                total_usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let id = self.id.take()?;
            let invoice = self.invoice.take()?;
            let livemode = self.livemode.take()?;
            let period = self.period.take()?;
            let subscription_item = self.subscription_item.take()?;
            let total_usage = self.total_usage.take()?;

            Some(Self::Out { id, invoice, livemode, period, subscription_item, total_usage })
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

    impl ObjectDeser for UsageRecordSummary {
        type Builder = UsageRecordSummaryBuilder;
    }
};
impl stripe_types::Object for UsageRecordSummary {
    type Id = stripe_shared::UsageRecordSummaryId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(UsageRecordSummaryId, "urs_" | "sis_");
