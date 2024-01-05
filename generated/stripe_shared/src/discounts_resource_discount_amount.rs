#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DiscountsResourceDiscountAmount {
    /// The amount, in cents (or local equivalent), of the discount.
    pub amount: i64,
    /// The discount that was applied to get this discount amount.
    pub discount: stripe_types::Expandable<stripe_shared::Discount>,
}
#[cfg(feature = "min-ser")]
pub struct DiscountsResourceDiscountAmountBuilder {
    amount: Option<i64>,
    discount: Option<stripe_types::Expandable<stripe_shared::Discount>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DiscountsResourceDiscountAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DiscountsResourceDiscountAmount>,
        builder: DiscountsResourceDiscountAmountBuilder,
    }

    impl Visitor for Place<DiscountsResourceDiscountAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DiscountsResourceDiscountAmountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DiscountsResourceDiscountAmountBuilder {
        type Out = DiscountsResourceDiscountAmount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "discount" => Ok(Deserialize::begin(&mut self.discount)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), discount: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let discount = self.discount.take()?;

            Some(Self::Out { amount, discount })
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

    impl ObjectDeser for DiscountsResourceDiscountAmount {
        type Builder = DiscountsResourceDiscountAmountBuilder;
    }
};
