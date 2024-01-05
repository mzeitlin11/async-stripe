#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomText {
    /// Custom text that should be displayed alongside shipping address collection.
    pub shipping_address: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    pub submit: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
    /// Custom text that should be displayed in place of the default terms of service agreement text.
    pub terms_of_service_acceptance: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionCustomTextBuilder {
    shipping_address: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>>,
    submit: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>>,
    terms_of_service_acceptance: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCustomText {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomText>,
        builder: PaymentPagesCheckoutSessionCustomTextBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomText> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionCustomTextBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomTextBuilder {
        type Out = PaymentPagesCheckoutSessionCustomText;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "shipping_address" => Ok(Deserialize::begin(&mut self.shipping_address)),
                "submit" => Ok(Deserialize::begin(&mut self.submit)),
                "terms_of_service_acceptance" => Ok(Deserialize::begin(&mut self.terms_of_service_acceptance)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { shipping_address: Deserialize::default(), submit: Deserialize::default(), terms_of_service_acceptance: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let shipping_address = self.shipping_address.take()?;
            let submit = self.submit.take()?;
            let terms_of_service_acceptance = self.terms_of_service_acceptance.take()?;

            Some(Self::Out { shipping_address, submit, terms_of_service_acceptance })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomText {
        type Builder = PaymentPagesCheckoutSessionCustomTextBuilder;
    }
};
