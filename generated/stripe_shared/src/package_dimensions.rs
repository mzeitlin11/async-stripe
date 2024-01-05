#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PackageDimensions {
    /// Height, in inches.
    pub height: f64,
    /// Length, in inches.
    pub length: f64,
    /// Weight, in ounces.
    pub weight: f64,
    /// Width, in inches.
    pub width: f64,
}
#[cfg(feature = "min-ser")]
pub struct PackageDimensionsBuilder {
    height: Option<f64>,
    length: Option<f64>,
    weight: Option<f64>,
    width: Option<f64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PackageDimensions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PackageDimensions>,
        builder: PackageDimensionsBuilder,
    }

    impl Visitor for Place<PackageDimensions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PackageDimensionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PackageDimensionsBuilder {
        type Out = PackageDimensions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "height" => Ok(Deserialize::begin(&mut self.height)),
                "length" => Ok(Deserialize::begin(&mut self.length)),
                "weight" => Ok(Deserialize::begin(&mut self.weight)),
                "width" => Ok(Deserialize::begin(&mut self.width)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { height: Deserialize::default(), length: Deserialize::default(), weight: Deserialize::default(), width: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let height = self.height.take()?;
            let length = self.length.take()?;
            let weight = self.weight.take()?;
            let width = self.width.take()?;

            Some(Self::Out { height, length, weight, width })
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

    impl ObjectDeser for PackageDimensions {
        type Builder = PackageDimensionsBuilder;
    }
};
