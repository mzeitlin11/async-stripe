#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardNetworkToken {
    /// Indicates if Stripe used a network token, either user provided or Stripe managed when processing the transaction.
    pub used: bool,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsCardNetworkTokenBuilder {
    used: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsCardNetworkToken {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardNetworkToken>,
        builder: PaymentMethodDetailsCardNetworkTokenBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardNetworkToken> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsCardNetworkTokenBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardNetworkTokenBuilder {
        type Out = PaymentMethodDetailsCardNetworkToken;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "used" => Ok(Deserialize::begin(&mut self.used)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { used: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let used = self.used.take()?;

            Some(Self::Out { used })
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

    impl ObjectDeser for PaymentMethodDetailsCardNetworkToken {
        type Builder = PaymentMethodDetailsCardNetworkTokenBuilder;
    }
};
