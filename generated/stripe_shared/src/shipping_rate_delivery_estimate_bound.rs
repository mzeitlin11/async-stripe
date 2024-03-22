#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ShippingRateDeliveryEstimateBound {
    /// A unit of time.
    pub unit: ShippingRateDeliveryEstimateBoundUnit,
    /// Must be greater than 0.
    pub value: i64,
}
#[cfg(feature = "min-ser")]
pub struct ShippingRateDeliveryEstimateBoundBuilder {
    unit: Option<ShippingRateDeliveryEstimateBoundUnit>,
    value: Option<i64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ShippingRateDeliveryEstimateBound {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ShippingRateDeliveryEstimateBound>,
        builder: ShippingRateDeliveryEstimateBoundBuilder,
    }

    impl Visitor for Place<ShippingRateDeliveryEstimateBound> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ShippingRateDeliveryEstimateBoundBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ShippingRateDeliveryEstimateBoundBuilder {
        type Out = ShippingRateDeliveryEstimateBound;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "unit" => Ok(Deserialize::begin(&mut self.unit)),
                "value" => Ok(Deserialize::begin(&mut self.value)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { unit: Deserialize::default(), value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let unit = self.unit.take()?;
            let value = self.value.take()?;

            Some(Self::Out { unit, value })
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

    impl ObjectDeser for ShippingRateDeliveryEstimateBound {
        type Builder = ShippingRateDeliveryEstimateBoundBuilder;
    }
};
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ShippingRateDeliveryEstimateBoundUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl ShippingRateDeliveryEstimateBoundUnit {
    pub fn as_str(self) -> &'static str {
        use ShippingRateDeliveryEstimateBoundUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for ShippingRateDeliveryEstimateBoundUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShippingRateDeliveryEstimateBoundUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ShippingRateDeliveryEstimateBoundUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ShippingRateDeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ShippingRateDeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ShippingRateDeliveryEstimateBoundUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRateDeliveryEstimateBoundUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ShippingRateDeliveryEstimateBoundUnit"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingRateDeliveryEstimateBoundUnit {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ShippingRateDeliveryEstimateBoundUnit> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingRateDeliveryEstimateBoundUnit::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
