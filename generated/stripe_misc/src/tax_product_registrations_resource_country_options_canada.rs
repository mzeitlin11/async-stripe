#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsCanada {
    pub province_standard: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard>,
    /// Type of registration in Canada.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsCanadaType,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductRegistrationsResourceCountryOptionsCanadaBuilder {
    province_standard: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard>>,
    type_: Option<TaxProductRegistrationsResourceCountryOptionsCanadaType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsCanada {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsCanada>,
        builder: TaxProductRegistrationsResourceCountryOptionsCanadaBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsCanada> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductRegistrationsResourceCountryOptionsCanadaBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsCanadaBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsCanada;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "province_standard" => Ok(Deserialize::begin(&mut self.province_standard)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { province_standard: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let province_standard = self.province_standard.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { province_standard, type_ })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsCanada {
        type Builder = TaxProductRegistrationsResourceCountryOptionsCanadaBuilder;
    }
};
/// Type of registration in Canada.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsCanadaType {
    ProvinceStandard,
    Simplified,
    Standard,
}
impl TaxProductRegistrationsResourceCountryOptionsCanadaType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsCanadaType::*;
        match self {
            ProvinceStandard => "province_standard",
            Simplified => "simplified",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsCanadaType::*;
        match s {
            "province_standard" => Ok(ProvinceStandard),
            "simplified" => Ok(Simplified),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductRegistrationsResourceCountryOptionsCanadaType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductRegistrationsResourceCountryOptionsCanadaType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductRegistrationsResourceCountryOptionsCanadaType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
