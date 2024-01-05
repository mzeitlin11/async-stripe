#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingTransactionReceiptData {
    /// The description of the item. The maximum length of this field is 26 characters.
    pub description: Option<String>,
    /// The quantity of the item.
    pub quantity: Option<f64>,
    /// The total for this line item in cents.
    pub total: Option<i64>,
    /// The unit cost of the item in cents.
    pub unit_cost: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTransactionReceiptDataBuilder {
    description: Option<Option<String>>,
    quantity: Option<Option<f64>>,
    total: Option<Option<i64>>,
    unit_cost: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionReceiptData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionReceiptData>,
        builder: IssuingTransactionReceiptDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionReceiptData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTransactionReceiptDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTransactionReceiptDataBuilder {
        type Out = IssuingTransactionReceiptData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),
                "total" => Ok(Deserialize::begin(&mut self.total)),
                "unit_cost" => Ok(Deserialize::begin(&mut self.unit_cost)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { description: Deserialize::default(), quantity: Deserialize::default(), total: Deserialize::default(), unit_cost: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let description = self.description.take()?;
            let quantity = self.quantity.take()?;
            let total = self.total.take()?;
            let unit_cost = self.unit_cost.take()?;

            Some(Self::Out { description, quantity, total, unit_cost })
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

    impl ObjectDeser for IssuingTransactionReceiptData {
        type Builder = IssuingTransactionReceiptDataBuilder;
    }
};
