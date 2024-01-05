#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsSubscriptionUpdateConfirmItem {
    /// The ID of the [subscription item](https://stripe.com/docs/api/subscriptions/object#subscription_object-items-data-id) to be updated.
    pub id: Option<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItemId>,
    /// The price the customer should subscribe to through this flow.
    /// The price must also be included in the configuration's [`features.subscription_update.products`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
    pub price: Option<String>,
    /// [Quantity](https://stripe.com/docs/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
    pub quantity: Option<u64>,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsSubscriptionUpdateConfirmItemBuilder {
    id: Option<Option<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItemId>>,
    price: Option<Option<String>>,
    quantity: Option<Option<u64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsSubscriptionUpdateConfirmItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsSubscriptionUpdateConfirmItem>,
        builder: PortalFlowsSubscriptionUpdateConfirmItemBuilder,
    }

    impl Visitor for Place<PortalFlowsSubscriptionUpdateConfirmItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsSubscriptionUpdateConfirmItemBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsSubscriptionUpdateConfirmItemBuilder {
        type Out = PortalFlowsSubscriptionUpdateConfirmItem;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "price" => Ok(Deserialize::begin(&mut self.price)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), price: Deserialize::default(), quantity: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let id = self.id.take()?;
            let price = self.price.take()?;
            let quantity = self.quantity.take()?;

            Some(Self::Out { id, price, quantity })
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

    impl ObjectDeser for PortalFlowsSubscriptionUpdateConfirmItem {
        type Builder = PortalFlowsSubscriptionUpdateConfirmItemBuilder;
    }
};
impl stripe_types::Object for PortalFlowsSubscriptionUpdateConfirmItem {
    type Id = Option<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItemId>;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PortalFlowsSubscriptionUpdateConfirmItemId);
