#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PriceTier {
    /// Price for the entire tier.
    pub flat_amount: Option<i64>,
    /// Same as `flat_amount`, but contains a decimal value with at most 12 decimal places.
    pub flat_amount_decimal: Option<String>,
    /// Per unit price for units relevant to the tier.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
    /// Up to and including to this quantity will be contained in the tier.
    pub up_to: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct PriceTierBuilder {
    flat_amount: Option<Option<i64>>,
    flat_amount_decimal: Option<Option<String>>,
    unit_amount: Option<Option<i64>>,
    unit_amount_decimal: Option<Option<String>>,
    up_to: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PriceTier {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PriceTier>,
        builder: PriceTierBuilder,
    }

    impl Visitor for Place<PriceTier> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PriceTierBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PriceTierBuilder {
        type Out = PriceTier;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "flat_amount" => Ok(Deserialize::begin(&mut self.flat_amount)),
                "flat_amount_decimal" => Ok(Deserialize::begin(&mut self.flat_amount_decimal)),
                "unit_amount" => Ok(Deserialize::begin(&mut self.unit_amount)),
                "unit_amount_decimal" => Ok(Deserialize::begin(&mut self.unit_amount_decimal)),
                "up_to" => Ok(Deserialize::begin(&mut self.up_to)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                flat_amount: Deserialize::default(),
                flat_amount_decimal: Deserialize::default(),
                unit_amount: Deserialize::default(),
                unit_amount_decimal: Deserialize::default(),
                up_to: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let flat_amount = self.flat_amount.take()?;
            let flat_amount_decimal = self.flat_amount_decimal.take()?;
            let unit_amount = self.unit_amount.take()?;
            let unit_amount_decimal = self.unit_amount_decimal.take()?;
            let up_to = self.up_to.take()?;

            Some(Self::Out { flat_amount, flat_amount_decimal, unit_amount, unit_amount_decimal, up_to })
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

    impl ObjectDeser for PriceTier {
        type Builder = PriceTierBuilder;
    }
};
