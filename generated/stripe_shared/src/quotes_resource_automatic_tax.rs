#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct QuotesResourceAutomaticTax {
    /// Automatically calculate taxes
    pub enabled: bool,
    /// The status of the most recent automated tax calculation for this quote.
    pub status: Option<QuotesResourceAutomaticTaxStatus>,
}
#[cfg(feature = "min-ser")]
pub struct QuotesResourceAutomaticTaxBuilder {
    enabled: Option<bool>,
    status: Option<Option<QuotesResourceAutomaticTaxStatus>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceAutomaticTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceAutomaticTax>,
        builder: QuotesResourceAutomaticTaxBuilder,
    }

    impl Visitor for Place<QuotesResourceAutomaticTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: QuotesResourceAutomaticTaxBuilder::deser_default() }))
        }
    }

    impl MapBuilder for QuotesResourceAutomaticTaxBuilder {
        type Out = QuotesResourceAutomaticTax;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),
                "status" => Ok(Deserialize::begin(&mut self.status)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default(), status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let enabled = self.enabled.take()?;
            let status = self.status.take()?;

            Some(Self::Out { enabled, status })
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

    impl ObjectDeser for QuotesResourceAutomaticTax {
        type Builder = QuotesResourceAutomaticTaxBuilder;
    }
};
/// The status of the most recent automated tax calculation for this quote.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QuotesResourceAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}
impl QuotesResourceAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        use QuotesResourceAutomaticTaxStatus::*;
        match self {
            Complete => "complete",
            Failed => "failed",
            RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl std::str::FromStr for QuotesResourceAutomaticTaxStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuotesResourceAutomaticTaxStatus::*;
        match s {
            "complete" => Ok(Complete),
            "failed" => Ok(Failed),
            "requires_location_inputs" => Ok(RequiresLocationInputs),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for QuotesResourceAutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for QuotesResourceAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for QuotesResourceAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for QuotesResourceAutomaticTaxStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotesResourceAutomaticTaxStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for QuotesResourceAutomaticTaxStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for QuotesResourceAutomaticTaxStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<QuotesResourceAutomaticTaxStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(QuotesResourceAutomaticTaxStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
