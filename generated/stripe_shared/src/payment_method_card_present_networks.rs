#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodCardPresentNetworks {
    /// All available networks for the card.
    pub available: Vec<String>,
    /// The preferred network for the card.
    pub preferred: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodCardPresentNetworksBuilder {
    available: Option<Vec<String>>,
    preferred: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodCardPresentNetworks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCardPresentNetworks>,
        builder: PaymentMethodCardPresentNetworksBuilder,
    }

    impl Visitor for Place<PaymentMethodCardPresentNetworks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodCardPresentNetworksBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodCardPresentNetworksBuilder {
        type Out = PaymentMethodCardPresentNetworks;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "available" => Ok(Deserialize::begin(&mut self.available)),
                "preferred" => Ok(Deserialize::begin(&mut self.preferred)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { available: Deserialize::default(), preferred: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let available = self.available.take()?;
            let preferred = self.preferred.take()?;

            Some(Self::Out { available, preferred })
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

    impl ObjectDeser for PaymentMethodCardPresentNetworks {
        type Builder = PaymentMethodCardPresentNetworksBuilder;
    }
};
