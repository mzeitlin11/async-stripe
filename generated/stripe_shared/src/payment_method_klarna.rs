#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodKlarna {
    /// The customer's date of birth, if provided.
    pub dob: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaDob>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodKlarnaBuilder {
    dob: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaDob>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodKlarna>,
        builder: PaymentMethodKlarnaBuilder,
    }

    impl Visitor for Place<PaymentMethodKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodKlarnaBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodKlarnaBuilder {
        type Out = PaymentMethodKlarna;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "dob" => Ok(Deserialize::begin(&mut self.dob)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { dob: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let dob = self.dob.take()?;

            Some(Self::Out { dob })
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

    impl ObjectDeser for PaymentMethodKlarna {
        type Builder = PaymentMethodKlarnaBuilder;
    }
};
