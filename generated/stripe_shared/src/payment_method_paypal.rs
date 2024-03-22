#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodPaypal {
    /// Owner's email. Values are provided by PayPal directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub payer_email: Option<String>,
    /// PayPal account PayerID. This identifier uniquely identifies the PayPal customer.
    pub payer_id: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodPaypalBuilder {
    payer_email: Option<Option<String>>,
    payer_id: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodPaypal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodPaypal>,
        builder: PaymentMethodPaypalBuilder,
    }

    impl Visitor for Place<PaymentMethodPaypal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodPaypalBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodPaypalBuilder {
        type Out = PaymentMethodPaypal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "payer_email" => Ok(Deserialize::begin(&mut self.payer_email)),
                "payer_id" => Ok(Deserialize::begin(&mut self.payer_id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { payer_email: Deserialize::default(), payer_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let payer_email = self.payer_email.take()?;
            let payer_id = self.payer_id.take()?;

            Some(Self::Out { payer_email, payer_id })
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

    impl ObjectDeser for PaymentMethodPaypal {
        type Builder = PaymentMethodPaypalBuilder;
    }
};
