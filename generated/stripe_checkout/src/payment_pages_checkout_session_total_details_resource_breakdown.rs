#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    /// The aggregated discounts.
    pub discounts: Vec<stripe_shared::LineItemsDiscountAmount>,
    /// The aggregated tax amounts by rate.
    pub taxes: Vec<stripe_shared::LineItemsTaxAmount>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder {
    discounts: Option<Vec<stripe_shared::LineItemsDiscountAmount>>,
    taxes: Option<Vec<stripe_shared::LineItemsTaxAmount>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
        builder: PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder {
        type Out = PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "discounts" => Ok(Deserialize::begin(&mut self.discounts)),
                "taxes" => Ok(Deserialize::begin(&mut self.taxes)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { discounts: Deserialize::default(), taxes: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let discounts = self.discounts.take()?;
            let taxes = self.taxes.take()?;

            Some(Self::Out { discounts, taxes })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
        type Builder = PaymentPagesCheckoutSessionTotalDetailsResourceBreakdownBuilder;
    }
};
