#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductResourceJurisdiction {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// A human-readable name for the jurisdiction imposing the tax.
    pub display_name: String,
    /// Indicates the level of the jurisdiction imposing the tax.
    pub level: TaxProductResourceJurisdictionLevel,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductResourceJurisdictionBuilder {
    country: Option<String>,
    display_name: Option<String>,
    level: Option<TaxProductResourceJurisdictionLevel>,
    state: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceJurisdiction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceJurisdiction>,
        builder: TaxProductResourceJurisdictionBuilder,
    }

    impl Visitor for Place<TaxProductResourceJurisdiction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductResourceJurisdictionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductResourceJurisdictionBuilder {
        type Out = TaxProductResourceJurisdiction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "display_name" => Ok(Deserialize::begin(&mut self.display_name)),
                "level" => Ok(Deserialize::begin(&mut self.level)),
                "state" => Ok(Deserialize::begin(&mut self.state)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { country: Deserialize::default(), display_name: Deserialize::default(), level: Deserialize::default(), state: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let country = self.country.take()?;
            let display_name = self.display_name.take()?;
            let level = self.level.take()?;
            let state = self.state.take()?;

            Some(Self::Out { country, display_name, level, state })
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

    impl ObjectDeser for TaxProductResourceJurisdiction {
        type Builder = TaxProductResourceJurisdictionBuilder;
    }
};
/// Indicates the level of the jurisdiction imposing the tax.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceJurisdictionLevel {
    City,
    Country,
    County,
    District,
    State,
}
impl TaxProductResourceJurisdictionLevel {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceJurisdictionLevel::*;
        match self {
            City => "city",
            Country => "country",
            County => "county",
            District => "district",
            State => "state",
        }
    }
}

impl std::str::FromStr for TaxProductResourceJurisdictionLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceJurisdictionLevel::*;
        match s {
            "city" => Ok(City),
            "country" => Ok(Country),
            "county" => Ok(County),
            "district" => Ok(District),
            "state" => Ok(State),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductResourceJurisdictionLevel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductResourceJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceJurisdictionLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceJurisdictionLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductResourceJurisdictionLevel"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductResourceJurisdictionLevel {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductResourceJurisdictionLevel> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductResourceJurisdictionLevel::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
