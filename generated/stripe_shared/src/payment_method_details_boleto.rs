#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsBoleto {
    /// The tax ID of the customer (CPF for individuals consumers or CNPJ for businesses consumers)
    pub tax_id: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsBoletoBuilder {
    tax_id: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsBoleto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsBoleto>,
        builder: PaymentMethodDetailsBoletoBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsBoleto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsBoletoBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsBoletoBuilder {
        type Out = PaymentMethodDetailsBoleto;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "tax_id" => Ok(Deserialize::begin(&mut self.tax_id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { tax_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let tax_id = self.tax_id.take()?;

            Some(Self::Out { tax_id })
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

    impl ObjectDeser for PaymentMethodDetailsBoleto {
        type Builder = PaymentMethodDetailsBoletoBuilder;
    }
};
