#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Level3LineItems {
    pub discount_amount: Option<i64>,
    pub product_code: String,
    pub product_description: String,
    pub quantity: Option<u64>,
    pub tax_amount: Option<i64>,
    pub unit_cost: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct Level3LineItemsBuilder {
    discount_amount: Option<Option<i64>>,
    product_code: Option<String>,
    product_description: Option<String>,
    quantity: Option<Option<u64>>,
    tax_amount: Option<Option<i64>>,
    unit_cost: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Level3LineItems {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Level3LineItems>,
        builder: Level3LineItemsBuilder,
    }

    impl Visitor for Place<Level3LineItems> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Level3LineItemsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for Level3LineItemsBuilder {
        type Out = Level3LineItems;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "discount_amount" => Ok(Deserialize::begin(&mut self.discount_amount)),
                "product_code" => Ok(Deserialize::begin(&mut self.product_code)),
                "product_description" => Ok(Deserialize::begin(&mut self.product_description)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),
                "tax_amount" => Ok(Deserialize::begin(&mut self.tax_amount)),
                "unit_cost" => Ok(Deserialize::begin(&mut self.unit_cost)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                discount_amount: Deserialize::default(),
                product_code: Deserialize::default(),
                product_description: Deserialize::default(),
                quantity: Deserialize::default(),
                tax_amount: Deserialize::default(),
                unit_cost: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let discount_amount = self.discount_amount.take()?;
            let product_code = self.product_code.take()?;
            let product_description = self.product_description.take()?;
            let quantity = self.quantity.take()?;
            let tax_amount = self.tax_amount.take()?;
            let unit_cost = self.unit_cost.take()?;

            Some(Self::Out { discount_amount, product_code, product_description, quantity, tax_amount, unit_cost })
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

    impl ObjectDeser for Level3LineItems {
        type Builder = Level3LineItemsBuilder;
    }
};
