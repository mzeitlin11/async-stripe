#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingTransactionFuelData {
    /// The type of fuel that was purchased.
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: String,
    /// The units for `volume_decimal`. One of `us_gallon` or `liter`.
    pub unit: String,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    pub unit_cost_decimal: String,
    /// The volume of the fuel that was pumped, represented as a decimal string with at most 12 decimal places.
    pub volume_decimal: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTransactionFuelDataBuilder {
    type_: Option<String>,
    unit: Option<String>,
    unit_cost_decimal: Option<String>,
    volume_decimal: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionFuelData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFuelData>,
        builder: IssuingTransactionFuelDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionFuelData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTransactionFuelDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTransactionFuelDataBuilder {
        type Out = IssuingTransactionFuelData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "unit" => Ok(Deserialize::begin(&mut self.unit)),
                "unit_cost_decimal" => Ok(Deserialize::begin(&mut self.unit_cost_decimal)),
                "volume_decimal" => Ok(Deserialize::begin(&mut self.volume_decimal)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { type_: Deserialize::default(), unit: Deserialize::default(), unit_cost_decimal: Deserialize::default(), volume_decimal: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let type_ = self.type_.take()?;
            let unit = self.unit.take()?;
            let unit_cost_decimal = self.unit_cost_decimal.take()?;
            let volume_decimal = self.volume_decimal.take()?;

            Some(Self::Out { type_, unit, unit_cost_decimal, volume_decimal })
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

    impl ObjectDeser for IssuingTransactionFuelData {
        type Builder = IssuingTransactionFuelDataBuilder;
    }
};
