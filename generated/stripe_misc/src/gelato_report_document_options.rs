#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct GelatoReportDocumentOptions {
    /// Array of strings of allowed identity document types.
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    pub allowed_types: Option<Vec<GelatoReportDocumentOptionsAllowedTypes>>,
    /// Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    pub require_id_number: Option<bool>,
    /// Disable image uploads, identity document images have to be captured using the device’s camera.
    pub require_live_capture: Option<bool>,
    /// Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face.
    /// [Learn more](https://stripe.com/docs/identity/selfie).
    pub require_matching_selfie: Option<bool>,
}
#[cfg(feature = "min-ser")]
pub struct GelatoReportDocumentOptionsBuilder {
    allowed_types: Option<Option<Vec<GelatoReportDocumentOptionsAllowedTypes>>>,
    require_id_number: Option<Option<bool>>,
    require_live_capture: Option<Option<bool>>,
    require_matching_selfie: Option<Option<bool>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoReportDocumentOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoReportDocumentOptions>,
        builder: GelatoReportDocumentOptionsBuilder,
    }

    impl Visitor for Place<GelatoReportDocumentOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: GelatoReportDocumentOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for GelatoReportDocumentOptionsBuilder {
        type Out = GelatoReportDocumentOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "allowed_types" => Ok(Deserialize::begin(&mut self.allowed_types)),
                "require_id_number" => Ok(Deserialize::begin(&mut self.require_id_number)),
                "require_live_capture" => Ok(Deserialize::begin(&mut self.require_live_capture)),
                "require_matching_selfie" => Ok(Deserialize::begin(&mut self.require_matching_selfie)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { allowed_types: Deserialize::default(), require_id_number: Deserialize::default(), require_live_capture: Deserialize::default(), require_matching_selfie: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let allowed_types = self.allowed_types.take()?;
            let require_id_number = self.require_id_number.take()?;
            let require_live_capture = self.require_live_capture.take()?;
            let require_matching_selfie = self.require_matching_selfie.take()?;

            Some(Self::Out { allowed_types, require_id_number, require_live_capture, require_matching_selfie })
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

    impl ObjectDeser for GelatoReportDocumentOptions {
        type Builder = GelatoReportDocumentOptionsBuilder;
    }
};
/// Array of strings of allowed identity document types.
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoReportDocumentOptionsAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}
impl GelatoReportDocumentOptionsAllowedTypes {
    pub fn as_str(self) -> &'static str {
        use GelatoReportDocumentOptionsAllowedTypes::*;
        match self {
            DrivingLicense => "driving_license",
            IdCard => "id_card",
            Passport => "passport",
        }
    }
}

impl std::str::FromStr for GelatoReportDocumentOptionsAllowedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoReportDocumentOptionsAllowedTypes::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for GelatoReportDocumentOptionsAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for GelatoReportDocumentOptionsAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoReportDocumentOptionsAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoReportDocumentOptionsAllowedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoReportDocumentOptionsAllowedTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for GelatoReportDocumentOptionsAllowedTypes"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for GelatoReportDocumentOptionsAllowedTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<GelatoReportDocumentOptionsAllowedTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoReportDocumentOptionsAllowedTypes::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
