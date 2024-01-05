#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct QuotesResourceSubscriptionDataSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    /// This date is ignored if it is in the past when the quote is accepted.
    /// Measured in seconds since the Unix epoch.
    pub effective_date: Option<stripe_types::Timestamp>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on the subscription or subscription schedule when the quote is accepted.
    /// If a recurring price is included in `line_items`, this field will be passed to the resulting subscription's `metadata` field.
    /// If `subscription_data.effective_date` is used, this field will be passed to the resulting subscription schedule's `phases.metadata` field.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,
}
#[cfg(feature = "min-ser")]
pub struct QuotesResourceSubscriptionDataSubscriptionDataBuilder {
    description: Option<Option<String>>,
    effective_date: Option<Option<stripe_types::Timestamp>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    trial_period_days: Option<Option<u32>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceSubscriptionDataSubscriptionData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceSubscriptionDataSubscriptionData>,
        builder: QuotesResourceSubscriptionDataSubscriptionDataBuilder,
    }

    impl Visitor for Place<QuotesResourceSubscriptionDataSubscriptionData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: QuotesResourceSubscriptionDataSubscriptionDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for QuotesResourceSubscriptionDataSubscriptionDataBuilder {
        type Out = QuotesResourceSubscriptionDataSubscriptionData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "effective_date" => Ok(Deserialize::begin(&mut self.effective_date)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "trial_period_days" => Ok(Deserialize::begin(&mut self.trial_period_days)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { description: Deserialize::default(), effective_date: Deserialize::default(), metadata: Deserialize::default(), trial_period_days: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let description = self.description.take()?;
            let effective_date = self.effective_date.take()?;
            let metadata = self.metadata.take()?;
            let trial_period_days = self.trial_period_days.take()?;

            Some(Self::Out { description, effective_date, metadata, trial_period_days })
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

    impl ObjectDeser for QuotesResourceSubscriptionDataSubscriptionData {
        type Builder = QuotesResourceSubscriptionDataSubscriptionDataBuilder;
    }
};
