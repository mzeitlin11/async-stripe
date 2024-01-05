#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Level3 {
    pub customer_reference: Option<String>,
    pub line_items: Vec<stripe_shared::Level3LineItems>,
    pub merchant_reference: String,
    pub shipping_address_zip: Option<String>,
    pub shipping_amount: Option<i64>,
    pub shipping_from_zip: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct Level3Builder {
    customer_reference: Option<Option<String>>,
    line_items: Option<Vec<stripe_shared::Level3LineItems>>,
    merchant_reference: Option<String>,
    shipping_address_zip: Option<Option<String>>,
    shipping_amount: Option<Option<i64>>,
    shipping_from_zip: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Level3 {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Level3>,
        builder: Level3Builder,
    }

    impl Visitor for Place<Level3> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Level3Builder::deser_default() }))
        }
    }

    impl MapBuilder for Level3Builder {
        type Out = Level3;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "customer_reference" => Ok(Deserialize::begin(&mut self.customer_reference)),
                "line_items" => Ok(Deserialize::begin(&mut self.line_items)),
                "merchant_reference" => Ok(Deserialize::begin(&mut self.merchant_reference)),
                "shipping_address_zip" => Ok(Deserialize::begin(&mut self.shipping_address_zip)),
                "shipping_amount" => Ok(Deserialize::begin(&mut self.shipping_amount)),
                "shipping_from_zip" => Ok(Deserialize::begin(&mut self.shipping_from_zip)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                customer_reference: Deserialize::default(),
                line_items: Deserialize::default(),
                merchant_reference: Deserialize::default(),
                shipping_address_zip: Deserialize::default(),
                shipping_amount: Deserialize::default(),
                shipping_from_zip: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let customer_reference = self.customer_reference.take()?;
            let line_items = self.line_items.take()?;
            let merchant_reference = self.merchant_reference.take()?;
            let shipping_address_zip = self.shipping_address_zip.take()?;
            let shipping_amount = self.shipping_amount.take()?;
            let shipping_from_zip = self.shipping_from_zip.take()?;

            Some(Self::Out { customer_reference, line_items, merchant_reference, shipping_address_zip, shipping_amount, shipping_from_zip })
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

    impl ObjectDeser for Level3 {
        type Builder = Level3Builder;
    }
};
