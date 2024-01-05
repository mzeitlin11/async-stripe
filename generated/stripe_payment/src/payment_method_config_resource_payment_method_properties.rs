#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodConfigResourcePaymentMethodProperties {
    /// Whether this payment method may be offered at checkout.
    /// True if `display_preference` is `on` and the payment method's capability is active.
    pub available: bool,
    pub display_preference: stripe_payment::PaymentMethodConfigResourceDisplayPreference,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodConfigResourcePaymentMethodPropertiesBuilder {
    available: Option<bool>,
    display_preference: Option<stripe_payment::PaymentMethodConfigResourceDisplayPreference>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodConfigResourcePaymentMethodProperties {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodConfigResourcePaymentMethodProperties>,
        builder: PaymentMethodConfigResourcePaymentMethodPropertiesBuilder,
    }

    impl Visitor for Place<PaymentMethodConfigResourcePaymentMethodProperties> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodConfigResourcePaymentMethodPropertiesBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodConfigResourcePaymentMethodPropertiesBuilder {
        type Out = PaymentMethodConfigResourcePaymentMethodProperties;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "available" => Ok(Deserialize::begin(&mut self.available)),
                "display_preference" => Ok(Deserialize::begin(&mut self.display_preference)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { available: Deserialize::default(), display_preference: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let available = self.available.take()?;
            let display_preference = self.display_preference.take()?;

            Some(Self::Out { available, display_preference })
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

    impl ObjectDeser for PaymentMethodConfigResourcePaymentMethodProperties {
        type Builder = PaymentMethodConfigResourcePaymentMethodPropertiesBuilder;
    }
};
