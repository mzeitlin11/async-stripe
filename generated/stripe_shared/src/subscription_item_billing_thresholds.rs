#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionItemBillingThresholds {
    /// Usage threshold that triggers the subscription to create an invoice
    pub usage_gte: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionItemBillingThresholdsBuilder {
    usage_gte: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionItemBillingThresholds {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionItemBillingThresholds>,
        builder: SubscriptionItemBillingThresholdsBuilder,
    }

    impl Visitor for Place<SubscriptionItemBillingThresholds> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionItemBillingThresholdsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionItemBillingThresholdsBuilder {
        type Out = SubscriptionItemBillingThresholds;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "usage_gte" => Ok(Deserialize::begin(&mut self.usage_gte)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { usage_gte: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let usage_gte = self.usage_gte.take()?;

            Some(Self::Out { usage_gte })
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

    impl ObjectDeser for SubscriptionItemBillingThresholds {
        type Builder = SubscriptionItemBillingThresholdsBuilder;
    }
};
