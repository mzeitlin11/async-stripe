#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFeatures {
    pub customer_update: stripe_billing::PortalCustomerUpdate,
    pub invoice_history: stripe_billing::PortalInvoiceList,
    pub payment_method_update: stripe_billing::PortalPaymentMethodUpdate,
    pub subscription_cancel: stripe_billing::PortalSubscriptionCancel,
    pub subscription_pause: stripe_billing::PortalSubscriptionPause,
    pub subscription_update: stripe_billing::PortalSubscriptionUpdate,
}
#[cfg(feature = "min-ser")]
pub struct PortalFeaturesBuilder {
    customer_update: Option<stripe_billing::PortalCustomerUpdate>,
    invoice_history: Option<stripe_billing::PortalInvoiceList>,
    payment_method_update: Option<stripe_billing::PortalPaymentMethodUpdate>,
    subscription_cancel: Option<stripe_billing::PortalSubscriptionCancel>,
    subscription_pause: Option<stripe_billing::PortalSubscriptionPause>,
    subscription_update: Option<stripe_billing::PortalSubscriptionUpdate>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFeatures>,
        builder: PortalFeaturesBuilder,
    }

    impl Visitor for Place<PortalFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFeaturesBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFeaturesBuilder {
        type Out = PortalFeatures;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "customer_update" => Ok(Deserialize::begin(&mut self.customer_update)),
                "invoice_history" => Ok(Deserialize::begin(&mut self.invoice_history)),
                "payment_method_update" => Ok(Deserialize::begin(&mut self.payment_method_update)),
                "subscription_cancel" => Ok(Deserialize::begin(&mut self.subscription_cancel)),
                "subscription_pause" => Ok(Deserialize::begin(&mut self.subscription_pause)),
                "subscription_update" => Ok(Deserialize::begin(&mut self.subscription_update)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                customer_update: Deserialize::default(),
                invoice_history: Deserialize::default(),
                payment_method_update: Deserialize::default(),
                subscription_cancel: Deserialize::default(),
                subscription_pause: Deserialize::default(),
                subscription_update: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let customer_update = self.customer_update.take()?;
            let invoice_history = self.invoice_history.take()?;
            let payment_method_update = self.payment_method_update.take()?;
            let subscription_cancel = self.subscription_cancel.take()?;
            let subscription_pause = self.subscription_pause.take()?;
            let subscription_update = self.subscription_update.take()?;

            Some(Self::Out { customer_update, invoice_history, payment_method_update, subscription_cancel, subscription_pause, subscription_update })
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

    impl ObjectDeser for PortalFeatures {
        type Builder = PortalFeaturesBuilder;
    }
};
