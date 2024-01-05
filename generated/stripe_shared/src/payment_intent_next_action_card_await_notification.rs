#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionCardAwaitNotification {
    /// The time that payment will be attempted.
    /// If customer approval is required, they need to provide approval before this time.
    pub charge_attempt_at: Option<stripe_types::Timestamp>,
    /// For payments greater than INR 15000, the customer must provide explicit approval of the payment with their bank.
    /// For payments of lower amount, no customer action is required.
    pub customer_approval_required: Option<bool>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionCardAwaitNotificationBuilder {
    charge_attempt_at: Option<Option<stripe_types::Timestamp>>,
    customer_approval_required: Option<Option<bool>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionCardAwaitNotification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionCardAwaitNotification>,
        builder: PaymentIntentNextActionCardAwaitNotificationBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionCardAwaitNotification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionCardAwaitNotificationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionCardAwaitNotificationBuilder {
        type Out = PaymentIntentNextActionCardAwaitNotification;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "charge_attempt_at" => Ok(Deserialize::begin(&mut self.charge_attempt_at)),
                "customer_approval_required" => Ok(Deserialize::begin(&mut self.customer_approval_required)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { charge_attempt_at: Deserialize::default(), customer_approval_required: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let charge_attempt_at = self.charge_attempt_at.take()?;
            let customer_approval_required = self.customer_approval_required.take()?;

            Some(Self::Out { charge_attempt_at, customer_approval_required })
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

    impl ObjectDeser for PaymentIntentNextActionCardAwaitNotification {
        type Builder = PaymentIntentNextActionCardAwaitNotificationBuilder;
    }
};
