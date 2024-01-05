#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalSubscriptionUpdateProduct {
    /// The list of price IDs which, when subscribed to, a subscription can be updated.
    pub prices: Vec<String>,
    /// The product ID.
    pub product: String,
}
#[cfg(feature = "min-ser")]
pub struct PortalSubscriptionUpdateProductBuilder {
    prices: Option<Vec<String>>,
    product: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalSubscriptionUpdateProduct {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionUpdateProduct>,
        builder: PortalSubscriptionUpdateProductBuilder,
    }

    impl Visitor for Place<PortalSubscriptionUpdateProduct> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalSubscriptionUpdateProductBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalSubscriptionUpdateProductBuilder {
        type Out = PortalSubscriptionUpdateProduct;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "prices" => Ok(Deserialize::begin(&mut self.prices)),
                "product" => Ok(Deserialize::begin(&mut self.product)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { prices: Deserialize::default(), product: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let prices = self.prices.take()?;
            let product = self.product.take()?;

            Some(Self::Out { prices, product })
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

    impl ObjectDeser for PortalSubscriptionUpdateProduct {
        type Builder = PortalSubscriptionUpdateProductBuilder;
    }
};
