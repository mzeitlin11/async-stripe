#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsDefault {
    /// Type of registration in `country`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsDefaultType,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductRegistrationsResourceCountryOptionsDefaultBuilder {
    type_: Option<TaxProductRegistrationsResourceCountryOptionsDefaultType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsDefault {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
        builder: TaxProductRegistrationsResourceCountryOptionsDefaultBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsDefault> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductRegistrationsResourceCountryOptionsDefaultBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsDefaultBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsDefault;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let type_ = self.type_.take()?;

            Some(Self::Out { type_ })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsDefault {
        type Builder = TaxProductRegistrationsResourceCountryOptionsDefaultBuilder;
    }
};
/// Type of registration in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsDefaultType {
    Standard,
}
impl TaxProductRegistrationsResourceCountryOptionsDefaultType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsDefaultType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsDefaultType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductRegistrationsResourceCountryOptionsDefaultType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductRegistrationsResourceCountryOptionsDefaultType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductRegistrationsResourceCountryOptionsDefaultType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
