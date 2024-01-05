#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoicesShippingCost {
    /// Total shipping cost before any taxes are applied.
    pub amount_subtotal: i64,
    /// Total tax amount applied due to shipping costs. If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    /// Total shipping cost after taxes are applied.
    pub amount_total: i64,
    /// The ID of the ShippingRate for this invoice.
    pub shipping_rate: Option<stripe_types::Expandable<stripe_shared::ShippingRate>>,
    /// The taxes applied to the shipping rate.
    pub taxes: Option<Vec<stripe_shared::LineItemsTaxAmount>>,
}
#[cfg(feature = "min-ser")]
pub struct InvoicesShippingCostBuilder {
    amount_subtotal: Option<i64>,
    amount_tax: Option<i64>,
    amount_total: Option<i64>,
    shipping_rate: Option<Option<stripe_types::Expandable<stripe_shared::ShippingRate>>>,
    taxes: Option<Option<Vec<stripe_shared::LineItemsTaxAmount>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesShippingCost {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesShippingCost>,
        builder: InvoicesShippingCostBuilder,
    }

    impl Visitor for Place<InvoicesShippingCost> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoicesShippingCostBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoicesShippingCostBuilder {
        type Out = InvoicesShippingCost;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount_subtotal" => Ok(Deserialize::begin(&mut self.amount_subtotal)),
                "amount_tax" => Ok(Deserialize::begin(&mut self.amount_tax)),
                "amount_total" => Ok(Deserialize::begin(&mut self.amount_total)),
                "shipping_rate" => Ok(Deserialize::begin(&mut self.shipping_rate)),
                "taxes" => Ok(Deserialize::begin(&mut self.taxes)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount_subtotal: Deserialize::default(),
                amount_tax: Deserialize::default(),
                amount_total: Deserialize::default(),
                shipping_rate: Deserialize::default(),
                taxes: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_subtotal = self.amount_subtotal.take()?;
            let amount_tax = self.amount_tax.take()?;
            let amount_total = self.amount_total.take()?;
            let shipping_rate = self.shipping_rate.take()?;
            let taxes = self.taxes.take()?;

            Some(Self::Out { amount_subtotal, amount_tax, amount_total, shipping_rate, taxes })
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

    impl ObjectDeser for InvoicesShippingCost {
        type Builder = InvoicesShippingCostBuilder;
    }
};
