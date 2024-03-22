#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionKonbiniSeicomart {
    /// The confirmation number.
    pub confirmation_number: Option<String>,
    /// The payment code.
    pub payment_code: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionKonbiniSeicomartBuilder {
    confirmation_number: Option<Option<String>>,
    payment_code: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionKonbiniSeicomart {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionKonbiniSeicomart>,
        builder: PaymentIntentNextActionKonbiniSeicomartBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionKonbiniSeicomart> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionKonbiniSeicomartBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionKonbiniSeicomartBuilder {
        type Out = PaymentIntentNextActionKonbiniSeicomart;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "confirmation_number" => Ok(Deserialize::begin(&mut self.confirmation_number)),
                "payment_code" => Ok(Deserialize::begin(&mut self.payment_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { confirmation_number: Deserialize::default(), payment_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let confirmation_number = self.confirmation_number.take()?;
            let payment_code = self.payment_code.take()?;

            Some(Self::Out { confirmation_number, payment_code })
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

    impl ObjectDeser for PaymentIntentNextActionKonbiniSeicomart {
        type Builder = PaymentIntentNextActionKonbiniSeicomartBuilder;
    }
};
