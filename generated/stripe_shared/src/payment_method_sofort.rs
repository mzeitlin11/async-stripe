#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodSofortBuilder {
    country: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodSofort {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodSofort>,
        builder: PaymentMethodSofortBuilder,
    }

    impl Visitor for Place<PaymentMethodSofort> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodSofortBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodSofortBuilder {
        type Out = PaymentMethodSofort;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "country" => Ok(Deserialize::begin(&mut self.country)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { country: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let country = self.country.take()?;

            Some(Self::Out { country })
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

    impl ObjectDeser for PaymentMethodSofort {
        type Builder = PaymentMethodSofortBuilder;
    }
};
