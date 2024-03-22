/// Usage records allow you to report customer usage and metrics to Stripe for
/// metered billing of subscription prices.
///
/// Related guide: [Metered billing](https://stripe.com/docs/billing/subscriptions/metered-billing)
///
/// For more details see <<https://stripe.com/docs/api/usage_records/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: stripe_billing::UsageRecordId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The usage quantity for the specified date.
    pub quantity: u64,
    /// The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,
    /// The timestamp when this usage occurred.
    pub timestamp: stripe_types::Timestamp,
}
#[cfg(feature = "min-ser")]
pub struct UsageRecordBuilder {
    id: Option<stripe_billing::UsageRecordId>,
    livemode: Option<bool>,
    quantity: Option<u64>,
    subscription_item: Option<String>,
    timestamp: Option<stripe_types::Timestamp>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for UsageRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<UsageRecord>,
        builder: UsageRecordBuilder,
    }

    impl Visitor for Place<UsageRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: UsageRecordBuilder::deser_default() }))
        }
    }

    impl MapBuilder for UsageRecordBuilder {
        type Out = UsageRecord;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),
                "subscription_item" => Ok(Deserialize::begin(&mut self.subscription_item)),
                "timestamp" => Ok(Deserialize::begin(&mut self.timestamp)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), livemode: Deserialize::default(), quantity: Deserialize::default(), subscription_item: Deserialize::default(), timestamp: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let quantity = self.quantity.take()?;
            let subscription_item = self.subscription_item.take()?;
            let timestamp = self.timestamp.take()?;

            Some(Self::Out { id, livemode, quantity, subscription_item, timestamp })
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

    impl ObjectDeser for UsageRecord {
        type Builder = UsageRecordBuilder;
    }
};
impl stripe_types::Object for UsageRecord {
    type Id = stripe_billing::UsageRecordId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(UsageRecordId, "mbur_");
