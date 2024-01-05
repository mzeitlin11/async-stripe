#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CouponAppliesTo {
    /// A list of product IDs this coupon applies to
    pub products: Vec<String>,
}
#[cfg(feature = "min-ser")]
pub struct CouponAppliesToBuilder {
    products: Option<Vec<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CouponAppliesTo {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CouponAppliesTo>,
        builder: CouponAppliesToBuilder,
    }

    impl Visitor for Place<CouponAppliesTo> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CouponAppliesToBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CouponAppliesToBuilder {
        type Out = CouponAppliesTo;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "products" => Ok(Deserialize::begin(&mut self.products)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { products: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let products = self.products.take()?;

            Some(Self::Out { products })
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

    impl ObjectDeser for CouponAppliesTo {
        type Builder = CouponAppliesToBuilder;
    }
};
