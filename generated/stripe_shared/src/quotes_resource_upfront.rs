#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct QuotesResourceUpfront {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// The line items that will appear on the next invoice after this quote is accepted.
    /// This does not include pending invoice items that exist on the customer but may still be included in the next invoice.
    pub line_items: Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>,
    pub total_details: stripe_shared::QuotesResourceTotalDetails,
}
#[cfg(feature = "min-ser")]
pub struct QuotesResourceUpfrontBuilder {
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    line_items: Option<Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>>,
    total_details: Option<stripe_shared::QuotesResourceTotalDetails>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceUpfront {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceUpfront>,
        builder: QuotesResourceUpfrontBuilder,
    }

    impl Visitor for Place<QuotesResourceUpfront> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: QuotesResourceUpfrontBuilder::deser_default() }))
        }
    }

    impl MapBuilder for QuotesResourceUpfrontBuilder {
        type Out = QuotesResourceUpfront;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_subtotal" => Ok(Deserialize::begin(&mut self.amount_subtotal)),
                "amount_total" => Ok(Deserialize::begin(&mut self.amount_total)),
                "line_items" => Ok(Deserialize::begin(&mut self.line_items)),
                "total_details" => Ok(Deserialize::begin(&mut self.total_details)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount_subtotal: Deserialize::default(), amount_total: Deserialize::default(), line_items: Deserialize::default(), total_details: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_subtotal = self.amount_subtotal.take()?;
            let amount_total = self.amount_total.take()?;
            let line_items = self.line_items.take()?;
            let total_details = self.total_details.take()?;

            Some(Self::Out { amount_subtotal, amount_total, line_items, total_details })
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

    impl ObjectDeser for QuotesResourceUpfront {
        type Builder = QuotesResourceUpfrontBuilder;
    }
};
