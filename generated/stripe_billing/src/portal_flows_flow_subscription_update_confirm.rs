#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsFlowSubscriptionUpdateConfirm {
    /// The coupon or promotion code to apply to this subscription update.
    /// Currently, only up to one may be specified.
    pub discounts: Option<Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmDiscount>>,
    /// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
    /// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
    pub items: Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItem>,
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsFlowSubscriptionUpdateConfirmBuilder {
    discounts: Option<Option<Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmDiscount>>>,
    items: Option<Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItem>>,
    subscription: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsFlowSubscriptionUpdateConfirm {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlowSubscriptionUpdateConfirm>,
        builder: PortalFlowsFlowSubscriptionUpdateConfirmBuilder,
    }

    impl Visitor for Place<PortalFlowsFlowSubscriptionUpdateConfirm> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsFlowSubscriptionUpdateConfirmBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsFlowSubscriptionUpdateConfirmBuilder {
        type Out = PortalFlowsFlowSubscriptionUpdateConfirm;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "discounts" => Ok(Deserialize::begin(&mut self.discounts)),
                "items" => Ok(Deserialize::begin(&mut self.items)),
                "subscription" => Ok(Deserialize::begin(&mut self.subscription)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { discounts: Deserialize::default(), items: Deserialize::default(), subscription: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let discounts = self.discounts.take()?;
            let items = self.items.take()?;
            let subscription = self.subscription.take()?;

            Some(Self::Out { discounts, items, subscription })
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

    impl ObjectDeser for PortalFlowsFlowSubscriptionUpdateConfirm {
        type Builder = PortalFlowsFlowSubscriptionUpdateConfirmBuilder;
    }
};
