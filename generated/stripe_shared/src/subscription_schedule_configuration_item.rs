/// A phase item describes the price and quantity of a phase.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionScheduleConfigurationItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionItemBillingThresholds>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an item.
    /// Metadata on this item will update the underlying subscription item's `metadata` when the phase is entered.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// ID of the plan to which the customer should be subscribed.
    pub plan: stripe_types::Expandable<stripe_shared::Plan>,
    /// ID of the price to which the customer should be subscribed.
    pub price: stripe_types::Expandable<stripe_shared::Price>,
    /// Quantity of the plan to which the customer should be subscribed.
    pub quantity: Option<u64>,
    /// The tax rates which apply to this `phase_item`.
    /// When set, the `default_tax_rates` on the phase do not apply to this `phase_item`.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionScheduleConfigurationItemBuilder {
    billing_thresholds: Option<Option<stripe_shared::SubscriptionItemBillingThresholds>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    plan: Option<stripe_types::Expandable<stripe_shared::Plan>>,
    price: Option<stripe_types::Expandable<stripe_shared::Price>>,
    quantity: Option<Option<u64>>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionScheduleConfigurationItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionScheduleConfigurationItem>,
        builder: SubscriptionScheduleConfigurationItemBuilder,
    }

    impl Visitor for Place<SubscriptionScheduleConfigurationItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionScheduleConfigurationItemBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionScheduleConfigurationItemBuilder {
        type Out = SubscriptionScheduleConfigurationItem;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "billing_thresholds" => Ok(Deserialize::begin(&mut self.billing_thresholds)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "plan" => Ok(Deserialize::begin(&mut self.plan)),
                "price" => Ok(Deserialize::begin(&mut self.price)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),
                "tax_rates" => Ok(Deserialize::begin(&mut self.tax_rates)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                billing_thresholds: Deserialize::default(),
                metadata: Deserialize::default(),
                plan: Deserialize::default(),
                price: Deserialize::default(),
                quantity: Deserialize::default(),
                tax_rates: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let billing_thresholds = self.billing_thresholds.take()?;
            let metadata = self.metadata.take()?;
            let plan = self.plan.take()?;
            let price = self.price.take()?;
            let quantity = self.quantity.take()?;
            let tax_rates = self.tax_rates.take()?;

            Some(Self::Out { billing_thresholds, metadata, plan, price, quantity, tax_rates })
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

    impl ObjectDeser for SubscriptionScheduleConfigurationItem {
        type Builder = SubscriptionScheduleConfigurationItemBuilder;
    }
};
