#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsPaypal {
    /// The PayPal Billing Agreement ID (BAID).
    /// This is an ID generated by PayPal which represents the mandate between the merchant and the customer.
    pub billing_agreement_id: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentPaymentMethodOptionsPaypalBuilder {
    billing_agreement_id: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentPaymentMethodOptionsPaypal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsPaypal>,
        builder: SetupIntentPaymentMethodOptionsPaypalBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsPaypal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentPaymentMethodOptionsPaypalBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsPaypalBuilder {
        type Out = SetupIntentPaymentMethodOptionsPaypal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "billing_agreement_id" => Ok(Deserialize::begin(&mut self.billing_agreement_id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { billing_agreement_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let billing_agreement_id = self.billing_agreement_id.take()?;

            Some(Self::Out { billing_agreement_id })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsPaypal {
        type Builder = SetupIntentPaymentMethodOptionsPaypalBuilder;
    }
};
