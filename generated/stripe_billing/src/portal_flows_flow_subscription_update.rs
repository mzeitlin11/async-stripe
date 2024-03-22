#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsFlowSubscriptionUpdate {
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsFlowSubscriptionUpdateBuilder {
    subscription: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsFlowSubscriptionUpdate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlowSubscriptionUpdate>,
        builder: PortalFlowsFlowSubscriptionUpdateBuilder,
    }

    impl Visitor for Place<PortalFlowsFlowSubscriptionUpdate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsFlowSubscriptionUpdateBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsFlowSubscriptionUpdateBuilder {
        type Out = PortalFlowsFlowSubscriptionUpdate;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "subscription" => Ok(Deserialize::begin(&mut self.subscription)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { subscription: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let subscription = self.subscription.take()?;

            Some(Self::Out { subscription })
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

    impl ObjectDeser for PortalFlowsFlowSubscriptionUpdate {
        type Builder = PortalFlowsFlowSubscriptionUpdateBuilder;
    }
};
