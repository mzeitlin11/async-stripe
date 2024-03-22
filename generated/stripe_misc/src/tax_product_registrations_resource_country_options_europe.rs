#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsEurope {
    pub standard: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEuStandard>,
    /// Type of registration in an EU country.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsEuropeType,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductRegistrationsResourceCountryOptionsEuropeBuilder {
    standard: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEuStandard>>,
    type_: Option<TaxProductRegistrationsResourceCountryOptionsEuropeType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsEurope {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
        builder: TaxProductRegistrationsResourceCountryOptionsEuropeBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsEurope> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductRegistrationsResourceCountryOptionsEuropeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsEuropeBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsEurope;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "standard" => Ok(Deserialize::begin(&mut self.standard)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { standard: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let standard = self.standard.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { standard, type_ })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsEurope {
        type Builder = TaxProductRegistrationsResourceCountryOptionsEuropeBuilder;
    }
};
/// Type of registration in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsEuropeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl TaxProductRegistrationsResourceCountryOptionsEuropeType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsEuropeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsEuropeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductRegistrationsResourceCountryOptionsEuropeType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductRegistrationsResourceCountryOptionsEuropeType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductRegistrationsResourceCountryOptionsEuropeType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
