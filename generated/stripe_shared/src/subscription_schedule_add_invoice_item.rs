/// An Add Invoice Item describes the prices and quantities that will be added as pending invoice items when entering a phase.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionScheduleAddInvoiceItem {
    /// ID of the price used to generate the invoice item.
    pub price: stripe_types::Expandable<stripe_shared::Price>,
    /// The quantity of the invoice item.
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionScheduleAddInvoiceItemBuilder {
    price: Option<stripe_types::Expandable<stripe_shared::Price>>,
    quantity: Option<Option<u64>>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionScheduleAddInvoiceItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionScheduleAddInvoiceItem>,
        builder: SubscriptionScheduleAddInvoiceItemBuilder,
    }

    impl Visitor for Place<SubscriptionScheduleAddInvoiceItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionScheduleAddInvoiceItemBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionScheduleAddInvoiceItemBuilder {
        type Out = SubscriptionScheduleAddInvoiceItem;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "price" => Ok(Deserialize::begin(&mut self.price)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),
                "tax_rates" => Ok(Deserialize::begin(&mut self.tax_rates)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { price: Deserialize::default(), quantity: Deserialize::default(), tax_rates: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let price = self.price.take()?;
            let quantity = self.quantity.take()?;
            let tax_rates = self.tax_rates.take()?;

            Some(Self::Out { price, quantity, tax_rates })
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

    impl ObjectDeser for SubscriptionScheduleAddInvoiceItem {
        type Builder = SubscriptionScheduleAddInvoiceItemBuilder;
    }
};
