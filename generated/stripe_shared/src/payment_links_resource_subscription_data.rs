#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    pub metadata: std::collections::HashMap<String, String>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceSubscriptionDataBuilder {
    description: Option<Option<String>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    trial_period_days: Option<Option<u32>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceSubscriptionData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceSubscriptionData>,
        builder: PaymentLinksResourceSubscriptionDataBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceSubscriptionData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceSubscriptionDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceSubscriptionDataBuilder {
        type Out = PaymentLinksResourceSubscriptionData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "trial_period_days" => Ok(Deserialize::begin(&mut self.trial_period_days)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { description: Deserialize::default(), metadata: Deserialize::default(), trial_period_days: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let description = self.description.take()?;
            let metadata = self.metadata.take()?;
            let trial_period_days = self.trial_period_days.take()?;

            Some(Self::Out { description, metadata, trial_period_days })
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

    impl ObjectDeser for PaymentLinksResourceSubscriptionData {
        type Builder = PaymentLinksResourceSubscriptionDataBuilder;
    }
};
