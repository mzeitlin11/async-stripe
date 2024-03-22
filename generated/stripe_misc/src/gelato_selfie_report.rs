/// Result from a selfie check
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct GelatoSelfieReport {
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the identity document used in this check.
    pub document: Option<String>,
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoSelfieReportError>,
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the selfie used in this check.
    pub selfie: Option<String>,
    /// Status of this `selfie` check.
    pub status: GelatoSelfieReportStatus,
}
#[cfg(feature = "min-ser")]
pub struct GelatoSelfieReportBuilder {
    document: Option<Option<String>>,
    error: Option<Option<stripe_misc::GelatoSelfieReportError>>,
    selfie: Option<Option<String>>,
    status: Option<GelatoSelfieReportStatus>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoSelfieReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSelfieReport>,
        builder: GelatoSelfieReportBuilder,
    }

    impl Visitor for Place<GelatoSelfieReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: GelatoSelfieReportBuilder::deser_default() }))
        }
    }

    impl MapBuilder for GelatoSelfieReportBuilder {
        type Out = GelatoSelfieReport;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "document" => Ok(Deserialize::begin(&mut self.document)),
                "error" => Ok(Deserialize::begin(&mut self.error)),
                "selfie" => Ok(Deserialize::begin(&mut self.selfie)),
                "status" => Ok(Deserialize::begin(&mut self.status)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { document: Deserialize::default(), error: Deserialize::default(), selfie: Deserialize::default(), status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let document = self.document.take()?;
            let error = self.error.take()?;
            let selfie = self.selfie.take()?;
            let status = self.status.take()?;

            Some(Self::Out { document, error, selfie, status })
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

    impl ObjectDeser for GelatoSelfieReport {
        type Builder = GelatoSelfieReportBuilder;
    }
};
/// Status of this `selfie` check.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoSelfieReportStatus {
    Unverified,
    Verified,
}
impl GelatoSelfieReportStatus {
    pub fn as_str(self) -> &'static str {
        use GelatoSelfieReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for GelatoSelfieReportStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSelfieReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for GelatoSelfieReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for GelatoSelfieReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSelfieReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoSelfieReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoSelfieReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for GelatoSelfieReportStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for GelatoSelfieReportStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<GelatoSelfieReportStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoSelfieReportStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
