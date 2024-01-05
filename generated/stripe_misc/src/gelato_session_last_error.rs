/// Shows last VerificationSession error
#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct GelatoSessionLastError {
    /// A short machine-readable string giving the reason for the verification or user-session failure.
    pub code: Option<GelatoSessionLastErrorCode>,
    /// A message that explains the reason for verification or user-session failure.
    pub reason: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct GelatoSessionLastErrorBuilder {
    code: Option<Option<GelatoSessionLastErrorCode>>,
    reason: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoSessionLastError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSessionLastError>,
        builder: GelatoSessionLastErrorBuilder,
    }

    impl Visitor for Place<GelatoSessionLastError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: GelatoSessionLastErrorBuilder::deser_default() }))
        }
    }

    impl MapBuilder for GelatoSessionLastErrorBuilder {
        type Out = GelatoSessionLastError;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
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

    impl ObjectDeser for GelatoSessionLastError {
        type Builder = GelatoSessionLastErrorBuilder;
    }
};
/// A short machine-readable string giving the reason for the verification or user-session failure.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoSessionLastErrorCode {
    Abandoned,
    ConsentDeclined,
    CountryNotSupported,
    DeviceNotSupported,
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
    UnderSupportedAge,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl GelatoSessionLastErrorCode {
    pub fn as_str(self) -> &'static str {
        use GelatoSessionLastErrorCode::*;
        match self {
            Abandoned => "abandoned",
            ConsentDeclined => "consent_declined",
            CountryNotSupported => "country_not_supported",
            DeviceNotSupported => "device_not_supported",
            DocumentExpired => "document_expired",
            DocumentTypeNotSupported => "document_type_not_supported",
            DocumentUnverifiedOther => "document_unverified_other",
            IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            IdNumberMismatch => "id_number_mismatch",
            IdNumberUnverifiedOther => "id_number_unverified_other",
            SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            SelfieFaceMismatch => "selfie_face_mismatch",
            SelfieManipulated => "selfie_manipulated",
            SelfieUnverifiedOther => "selfie_unverified_other",
            UnderSupportedAge => "under_supported_age",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for GelatoSessionLastErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSessionLastErrorCode::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "consent_declined" => Ok(ConsentDeclined),
            "country_not_supported" => Ok(CountryNotSupported),
            "device_not_supported" => Ok(DeviceNotSupported),
            "document_expired" => Ok(DocumentExpired),
            "document_type_not_supported" => Ok(DocumentTypeNotSupported),
            "document_unverified_other" => Ok(DocumentUnverifiedOther),
            "id_number_insufficient_document_data" => Ok(IdNumberInsufficientDocumentData),
            "id_number_mismatch" => Ok(IdNumberMismatch),
            "id_number_unverified_other" => Ok(IdNumberUnverifiedOther),
            "selfie_document_missing_photo" => Ok(SelfieDocumentMissingPhoto),
            "selfie_face_mismatch" => Ok(SelfieFaceMismatch),
            "selfie_manipulated" => Ok(SelfieManipulated),
            "selfie_unverified_other" => Ok(SelfieUnverifiedOther),
            "under_supported_age" => Ok(UnderSupportedAge),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for GelatoSessionLastErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for GelatoSessionLastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSessionLastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoSessionLastErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoSessionLastErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for GelatoSessionLastErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<GelatoSessionLastErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoSessionLastErrorCode::from_str(s).unwrap_or(GelatoSessionLastErrorCode::Unknown));
        Ok(())
    }
}
