/// Subscription items allow you to create customer subscriptions with more than
/// one plan, making it easy to represent complex billing relationships.
///
/// For more details see <<https://stripe.com/docs/api/subscription_items/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionItemBillingThresholds>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionItemId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    pub plan: stripe_shared::Plan,
    pub price: stripe_shared::Price,
    /// The [quantity](https://stripe.com/docs/subscriptions/quantities) of the plan to which the customer should be subscribed.
    pub quantity: Option<u64>,
    /// The `subscription` this `subscription_item` belongs to.
    pub subscription: String,
    /// The tax rates which apply to this `subscription_item`.
    /// When set, the `default_tax_rates` on the subscription do not apply to this `subscription_item`.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionItemBuilder {
    billing_thresholds: Option<Option<stripe_shared::SubscriptionItemBillingThresholds>>,
    created: Option<i64>,
    id: Option<stripe_shared::SubscriptionItemId>,
    metadata: Option<std::collections::HashMap<String, String>>,
    plan: Option<stripe_shared::Plan>,
    price: Option<stripe_shared::Price>,
    quantity: Option<Option<u64>>,
    subscription: Option<String>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionItem>,
        builder: SubscriptionItemBuilder,
    }

    impl Visitor for Place<SubscriptionItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionItemBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionItemBuilder {
        type Out = SubscriptionItem;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "billing_thresholds" => Ok(Deserialize::begin(&mut self.billing_thresholds)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "plan" => Ok(Deserialize::begin(&mut self.plan)),
                "price" => Ok(Deserialize::begin(&mut self.price)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),
                "subscription" => Ok(Deserialize::begin(&mut self.subscription)),
                "tax_rates" => Ok(Deserialize::begin(&mut self.tax_rates)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                billing_thresholds: Deserialize::default(),
                created: Deserialize::default(),
                id: Deserialize::default(),
                metadata: Deserialize::default(),
                plan: Deserialize::default(),
                price: Deserialize::default(),
                quantity: Deserialize::default(),
                subscription: Deserialize::default(),
                tax_rates: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let billing_thresholds = self.billing_thresholds.take()?;
            let created = self.created.take()?;
            let id = self.id.take()?;
            let metadata = self.metadata.take()?;
            let plan = self.plan.take()?;
            let price = self.price.take()?;
            let quantity = self.quantity.take()?;
            let subscription = self.subscription.take()?;
            let tax_rates = self.tax_rates.take()?;

            Some(Self::Out { billing_thresholds, created, id, metadata, plan, price, quantity, subscription, tax_rates })
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

    impl ObjectDeser for SubscriptionItem {
        type Builder = SubscriptionItemBuilder;
    }
};
impl stripe_types::Object for SubscriptionItem {
    type Id = stripe_shared::SubscriptionItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SubscriptionItemId, "si_");
