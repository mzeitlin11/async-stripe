#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentProcessingCustomerNotification {
    /// Whether customer approval has been requested for this payment.
    /// For payments greater than INR 15000 or mandate amount, the customer must provide explicit approval of the payment with their bank.
    pub approval_requested: Option<bool>,
    /// If customer approval is required, they need to provide approval before this time.
    pub completes_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentProcessingCustomerNotificationBuilder {
    approval_requested: Option<Option<bool>>,
    completes_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentProcessingCustomerNotification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentProcessingCustomerNotification>,
        builder: PaymentIntentProcessingCustomerNotificationBuilder,
    }

    impl Visitor for Place<PaymentIntentProcessingCustomerNotification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentProcessingCustomerNotificationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentProcessingCustomerNotificationBuilder {
        type Out = PaymentIntentProcessingCustomerNotification;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "approval_requested" => Ok(Deserialize::begin(&mut self.approval_requested)),
                "completes_at" => Ok(Deserialize::begin(&mut self.completes_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { approval_requested: Deserialize::default(), completes_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let approval_requested = self.approval_requested.take()?;
            let completes_at = self.completes_at.take()?;

            Some(Self::Out { approval_requested, completes_at })
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

    impl ObjectDeser for PaymentIntentProcessingCustomerNotification {
        type Builder = PaymentIntentProcessingCustomerNotificationBuilder;
    }
};
