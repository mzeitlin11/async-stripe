#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionBillingThresholds {
    /// Monetary threshold that triggers the subscription to create an invoice
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    /// This value may not be `true` if the subscription contains items with plans that have `aggregate_usage=last_ever`.
    pub reset_billing_cycle_anchor: Option<bool>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionBillingThresholdsBuilder {
    amount_gte: Option<Option<i64>>,
    reset_billing_cycle_anchor: Option<Option<bool>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionBillingThresholds {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionBillingThresholds>,
        builder: SubscriptionBillingThresholdsBuilder,
    }

    impl Visitor for Place<SubscriptionBillingThresholds> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionBillingThresholdsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionBillingThresholdsBuilder {
        type Out = SubscriptionBillingThresholds;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_gte" => Ok(Deserialize::begin(&mut self.amount_gte)),
                "reset_billing_cycle_anchor" => Ok(Deserialize::begin(&mut self.reset_billing_cycle_anchor)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount_gte: Deserialize::default(), reset_billing_cycle_anchor: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_gte = self.amount_gte.take()?;
            let reset_billing_cycle_anchor = self.reset_billing_cycle_anchor.take()?;

            Some(Self::Out { amount_gte, reset_billing_cycle_anchor })
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

    impl ObjectDeser for SubscriptionBillingThresholds {
        type Builder = SubscriptionBillingThresholdsBuilder;
    }
};
