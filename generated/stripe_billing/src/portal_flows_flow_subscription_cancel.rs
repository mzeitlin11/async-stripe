#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsFlowSubscriptionCancel {
    /// Specify a retention strategy to be used in the cancellation flow.
    pub retention: Option<stripe_billing::PortalFlowsRetention>,
    /// The ID of the subscription to be canceled.
    pub subscription: String,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsFlowSubscriptionCancelBuilder {
    retention: Option<Option<stripe_billing::PortalFlowsRetention>>,
    subscription: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsFlowSubscriptionCancel {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlowSubscriptionCancel>,
        builder: PortalFlowsFlowSubscriptionCancelBuilder,
    }

    impl Visitor for Place<PortalFlowsFlowSubscriptionCancel> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsFlowSubscriptionCancelBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsFlowSubscriptionCancelBuilder {
        type Out = PortalFlowsFlowSubscriptionCancel;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "retention" => Ok(Deserialize::begin(&mut self.retention)),
                "subscription" => Ok(Deserialize::begin(&mut self.subscription)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { retention: Deserialize::default(), subscription: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let retention = self.retention.take()?;
            let subscription = self.subscription.take()?;

            Some(Self::Out { retention, subscription })
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

    impl ObjectDeser for PortalFlowsFlowSubscriptionCancel {
        type Builder = PortalFlowsFlowSubscriptionCancelBuilder;
    }
};
