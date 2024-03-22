#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct GelatoIdNumberReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoIdNumberReportErrorCode>,
    /// A human-readable message giving the reason for the failure.
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct GelatoIdNumberReportErrorBuilder {
    code: Option<Option<GelatoIdNumberReportErrorCode>>,
    reason: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoIdNumberReportError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoIdNumberReportError>,
        builder: GelatoIdNumberReportErrorBuilder,
    }

    impl Visitor for Place<GelatoIdNumberReportError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: GelatoIdNumberReportErrorBuilder::deser_default() }))
        }
    }

    impl MapBuilder for GelatoIdNumberReportErrorBuilder {
        type Out = GelatoIdNumberReportError;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "code" => Ok(Deserialize::begin(&mut self.code)),
                "reason" => Ok(Deserialize::begin(&mut self.reason)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { code: Deserialize::default(), reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let code = self.code.take()?;
            let reason = self.reason.take()?;

            Some(Self::Out { code, reason })
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

    impl ObjectDeser for GelatoIdNumberReportError {
        type Builder = GelatoIdNumberReportErrorBuilder;
    }
};
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoIdNumberReportErrorCode {
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
}
impl GelatoIdNumberReportErrorCode {
    pub fn as_str(self) -> &'static str {
        use GelatoIdNumberReportErrorCode::*;
        match self {
            IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            IdNumberMismatch => "id_number_mismatch",
            IdNumberUnverifiedOther => "id_number_unverified_other",
        }
    }
}

impl std::str::FromStr for GelatoIdNumberReportErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoIdNumberReportErrorCode::*;
        match s {
            "id_number_insufficient_document_data" => Ok(IdNumberInsufficientDocumentData),
            "id_number_mismatch" => Ok(IdNumberMismatch),
            "id_number_unverified_other" => Ok(IdNumberUnverifiedOther),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for GelatoIdNumberReportErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for GelatoIdNumberReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoIdNumberReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoIdNumberReportErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoIdNumberReportErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for GelatoIdNumberReportErrorCode"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for GelatoIdNumberReportErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<GelatoIdNumberReportErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoIdNumberReportErrorCode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
