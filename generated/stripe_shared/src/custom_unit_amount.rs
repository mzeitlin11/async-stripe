#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomUnitAmount {
    /// The maximum unit amount the customer can specify for this item.
    pub maximum: Option<i64>,
    /// The minimum unit amount the customer can specify for this item.
    /// Must be at least the minimum charge amount.
    pub minimum: Option<i64>,
    /// The starting unit amount which can be updated by the customer.
    pub preset: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct CustomUnitAmountBuilder {
    maximum: Option<Option<i64>>,
    minimum: Option<Option<i64>>,
    preset: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomUnitAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomUnitAmount>,
        builder: CustomUnitAmountBuilder,
    }

    impl Visitor for Place<CustomUnitAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomUnitAmountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomUnitAmountBuilder {
        type Out = CustomUnitAmount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "maximum" => Ok(Deserialize::begin(&mut self.maximum)),
                "minimum" => Ok(Deserialize::begin(&mut self.minimum)),
                "preset" => Ok(Deserialize::begin(&mut self.preset)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { maximum: Deserialize::default(), minimum: Deserialize::default(), preset: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let maximum = self.maximum.take()?;
            let minimum = self.minimum.take()?;
            let preset = self.preset.take()?;

            Some(Self::Out { maximum, minimum, preset })
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

    impl ObjectDeser for CustomUnitAmount {
        type Builder = CustomUnitAmountBuilder;
    }
};
