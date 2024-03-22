#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ProductFeature {
    /// The feature's name. Up to 80 characters long.
    pub name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct ProductFeatureBuilder {
    name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ProductFeature {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ProductFeature>,
        builder: ProductFeatureBuilder,
    }

    impl Visitor for Place<ProductFeature> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ProductFeatureBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ProductFeatureBuilder {
        type Out = ProductFeature;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "name" => Ok(Deserialize::begin(&mut self.name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let name = self.name.take()?;

            Some(Self::Out { name })
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

    impl ObjectDeser for ProductFeature {
        type Builder = ProductFeatureBuilder;
    }
};
