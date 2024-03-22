#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct QuotesResourceTotalDetails {
    /// This is the sum of all the discounts.
    pub amount_discount: i64,
    /// This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,
    /// This is the sum of all the tax amounts.
    pub amount_tax: i64,
    pub breakdown: Option<stripe_shared::QuotesResourceTotalDetailsResourceBreakdown>,
}
#[cfg(feature = "min-ser")]
pub struct QuotesResourceTotalDetailsBuilder {
    amount_discount: Option<i64>,
    amount_shipping: Option<Option<i64>>,
    amount_tax: Option<i64>,
    breakdown: Option<Option<stripe_shared::QuotesResourceTotalDetailsResourceBreakdown>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceTotalDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceTotalDetails>,
        builder: QuotesResourceTotalDetailsBuilder,
    }

    impl Visitor for Place<QuotesResourceTotalDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: QuotesResourceTotalDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for QuotesResourceTotalDetailsBuilder {
        type Out = QuotesResourceTotalDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_discount" => Ok(Deserialize::begin(&mut self.amount_discount)),
                "amount_shipping" => Ok(Deserialize::begin(&mut self.amount_shipping)),
                "amount_tax" => Ok(Deserialize::begin(&mut self.amount_tax)),
                "breakdown" => Ok(Deserialize::begin(&mut self.breakdown)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount_discount: Deserialize::default(), amount_shipping: Deserialize::default(), amount_tax: Deserialize::default(), breakdown: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_discount = self.amount_discount.take()?;
            let amount_shipping = self.amount_shipping.take()?;
            let amount_tax = self.amount_tax.take()?;
            let breakdown = self.breakdown.take()?;

            Some(Self::Out { amount_discount, amount_shipping, amount_tax, breakdown })
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

    impl ObjectDeser for QuotesResourceTotalDetails {
        type Builder = QuotesResourceTotalDetailsBuilder;
    }
};
