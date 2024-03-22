/// Pending Updates store the changes pending from a previous update that will be applied
/// to the Subscription upon successful payment.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionsResourcePendingUpdate {
    /// If the update is applied, determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// The timestamp is in UTC format.
    pub billing_cycle_anchor: Option<stripe_types::Timestamp>,
    /// The point after which the changes reflected by this update will be discarded and no longer applied.
    pub expires_at: stripe_types::Timestamp,
    /// List of subscription items, each with an attached plan, that will be set if the update is applied.
    pub subscription_items: Option<Vec<stripe_shared::SubscriptionItem>>,
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time, if the update is applied.
    pub trial_end: Option<stripe_types::Timestamp>,
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    pub trial_from_plan: Option<bool>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionsResourcePendingUpdateBuilder {
    billing_cycle_anchor: Option<Option<stripe_types::Timestamp>>,
    expires_at: Option<stripe_types::Timestamp>,
    subscription_items: Option<Option<Vec<stripe_shared::SubscriptionItem>>>,
    trial_end: Option<Option<stripe_types::Timestamp>>,
    trial_from_plan: Option<Option<bool>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionsResourcePendingUpdate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourcePendingUpdate>,
        builder: SubscriptionsResourcePendingUpdateBuilder,
    }

    impl Visitor for Place<SubscriptionsResourcePendingUpdate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionsResourcePendingUpdateBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionsResourcePendingUpdateBuilder {
        type Out = SubscriptionsResourcePendingUpdate;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "billing_cycle_anchor" => Ok(Deserialize::begin(&mut self.billing_cycle_anchor)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "subscription_items" => Ok(Deserialize::begin(&mut self.subscription_items)),
                "trial_end" => Ok(Deserialize::begin(&mut self.trial_end)),
                "trial_from_plan" => Ok(Deserialize::begin(&mut self.trial_from_plan)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                billing_cycle_anchor: Deserialize::default(),
                expires_at: Deserialize::default(),
                subscription_items: Deserialize::default(),
                trial_end: Deserialize::default(),
                trial_from_plan: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let billing_cycle_anchor = self.billing_cycle_anchor.take()?;
            let expires_at = self.expires_at.take()?;
            let subscription_items = self.subscription_items.take()?;
            let trial_end = self.trial_end.take()?;
            let trial_from_plan = self.trial_from_plan.take()?;

            Some(Self::Out { billing_cycle_anchor, expires_at, subscription_items, trial_end, trial_from_plan })
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

    impl ObjectDeser for SubscriptionsResourcePendingUpdate {
        type Builder = SubscriptionsResourcePendingUpdateBuilder;
    }
};
