#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsSubscriptionUpdateConfirmDiscount {
    /// The ID of the coupon to apply to this subscription update.
    pub coupon: Option<String>,
    /// The ID of a promotion code to apply to this subscription update.
    pub promotion_code: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsSubscriptionUpdateConfirmDiscountBuilder {
    coupon: Option<Option<String>>,
    promotion_code: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsSubscriptionUpdateConfirmDiscount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsSubscriptionUpdateConfirmDiscount>,
        builder: PortalFlowsSubscriptionUpdateConfirmDiscountBuilder,
    }

    impl Visitor for Place<PortalFlowsSubscriptionUpdateConfirmDiscount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsSubscriptionUpdateConfirmDiscountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsSubscriptionUpdateConfirmDiscountBuilder {
        type Out = PortalFlowsSubscriptionUpdateConfirmDiscount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "coupon" => Ok(Deserialize::begin(&mut self.coupon)),
                "promotion_code" => Ok(Deserialize::begin(&mut self.promotion_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { coupon: Deserialize::default(), promotion_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let coupon = self.coupon.take()?;
            let promotion_code = self.promotion_code.take()?;

            Some(Self::Out { coupon, promotion_code })
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

    impl ObjectDeser for PortalFlowsSubscriptionUpdateConfirmDiscount {
        type Builder = PortalFlowsSubscriptionUpdateConfirmDiscountBuilder;
    }
};
