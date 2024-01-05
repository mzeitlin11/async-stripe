#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ShippingRateDeliveryEstimate {
    /// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    pub maximum: Option<stripe_shared::ShippingRateDeliveryEstimateBound>,
    /// The lower bound of the estimated range. If empty, represents no lower bound.
    pub minimum: Option<stripe_shared::ShippingRateDeliveryEstimateBound>,
}
#[cfg(feature = "min-ser")]
pub struct ShippingRateDeliveryEstimateBuilder {
    maximum: Option<Option<stripe_shared::ShippingRateDeliveryEstimateBound>>,
    minimum: Option<Option<stripe_shared::ShippingRateDeliveryEstimateBound>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ShippingRateDeliveryEstimate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ShippingRateDeliveryEstimate>,
        builder: ShippingRateDeliveryEstimateBuilder,
    }

    impl Visitor for Place<ShippingRateDeliveryEstimate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ShippingRateDeliveryEstimateBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ShippingRateDeliveryEstimateBuilder {
        type Out = ShippingRateDeliveryEstimate;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "maximum" => Ok(Deserialize::begin(&mut self.maximum)),
                "minimum" => Ok(Deserialize::begin(&mut self.minimum)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { maximum: Deserialize::default(), minimum: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let maximum = self.maximum.take()?;
            let minimum = self.minimum.take()?;

            Some(Self::Out { maximum, minimum })
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

    impl ObjectDeser for ShippingRateDeliveryEstimate {
        type Builder = ShippingRateDeliveryEstimateBuilder;
    }
};
