/// Result from a document check
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct GelatoDocumentReport {
    /// Address as it appears in the document.
    pub address: Option<stripe_shared::Address>,
    /// Date of birth as it appears in the document.
    pub dob: Option<stripe_misc::GelatoDataDocumentReportDateOfBirth>,
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoDocumentReportError>,
    /// Expiration date of the document.
    pub expiration_date: Option<stripe_misc::GelatoDataDocumentReportExpirationDate>,
    /// Array of [File](https://stripe.com/docs/api/files) ids containing images for this document.
    pub files: Option<Vec<String>>,
    /// First name as it appears in the document.
    pub first_name: Option<String>,
    /// Issued date of the document.
    pub issued_date: Option<stripe_misc::GelatoDataDocumentReportIssuedDate>,
    /// Issuing country of the document.
    pub issuing_country: Option<String>,
    /// Last name as it appears in the document.
    pub last_name: Option<String>,
    /// Document ID number.
    pub number: Option<String>,
    /// Status of this `document` check.
    pub status: GelatoDocumentReportStatus,
    /// Type of the document.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: Option<GelatoDocumentReportType>,
}
#[cfg(feature = "min-ser")]
pub struct GelatoDocumentReportBuilder {
    address: Option<Option<stripe_shared::Address>>,
    dob: Option<Option<stripe_misc::GelatoDataDocumentReportDateOfBirth>>,
    error: Option<Option<stripe_misc::GelatoDocumentReportError>>,
    expiration_date: Option<Option<stripe_misc::GelatoDataDocumentReportExpirationDate>>,
    files: Option<Option<Vec<String>>>,
    first_name: Option<Option<String>>,
    issued_date: Option<Option<stripe_misc::GelatoDataDocumentReportIssuedDate>>,
    issuing_country: Option<Option<String>>,
    last_name: Option<Option<String>>,
    number: Option<Option<String>>,
    status: Option<GelatoDocumentReportStatus>,
    type_: Option<Option<GelatoDocumentReportType>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoDocumentReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoDocumentReport>,
        builder: GelatoDocumentReportBuilder,
    }

    impl Visitor for Place<GelatoDocumentReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: GelatoDocumentReportBuilder::deser_default() }))
        }
    }

    impl MapBuilder for GelatoDocumentReportBuilder {
        type Out = GelatoDocumentReport;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),
                "dob" => Ok(Deserialize::begin(&mut self.dob)),
                "error" => Ok(Deserialize::begin(&mut self.error)),
                "expiration_date" => Ok(Deserialize::begin(&mut self.expiration_date)),
                "files" => Ok(Deserialize::begin(&mut self.files)),
                "first_name" => Ok(Deserialize::begin(&mut self.first_name)),
                "issued_date" => Ok(Deserialize::begin(&mut self.issued_date)),
                "issuing_country" => Ok(Deserialize::begin(&mut self.issuing_country)),
                "last_name" => Ok(Deserialize::begin(&mut self.last_name)),
                "number" => Ok(Deserialize::begin(&mut self.number)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                dob: Deserialize::default(),
                error: Deserialize::default(),
                expiration_date: Deserialize::default(),
                files: Deserialize::default(),
                first_name: Deserialize::default(),
                issued_date: Deserialize::default(),
                issuing_country: Deserialize::default(),
                last_name: Deserialize::default(),
                number: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;
            let dob = self.dob.take()?;
            let error = self.error.take()?;
            let expiration_date = self.expiration_date.take()?;
            let files = self.files.take()?;
            let first_name = self.first_name.take()?;
            let issued_date = self.issued_date.take()?;
            let issuing_country = self.issuing_country.take()?;
            let last_name = self.last_name.take()?;
            let number = self.number.take()?;
            let status = self.status.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { address, dob, error, expiration_date, files, first_name, issued_date, issuing_country, last_name, number, status, type_ })
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

    impl ObjectDeser for GelatoDocumentReport {
        type Builder = GelatoDocumentReportBuilder;
    }
};
/// Status of this `document` check.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoDocumentReportStatus {
    Unverified,
    Verified,
}
impl GelatoDocumentReportStatus {
    pub fn as_str(self) -> &'static str {
        use GelatoDocumentReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for GelatoDocumentReportStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoDocumentReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for GelatoDocumentReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for GelatoDocumentReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoDocumentReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoDocumentReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for GelatoDocumentReportStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for GelatoDocumentReportStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<GelatoDocumentReportStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoDocumentReportStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Type of the document.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoDocumentReportType {
    DrivingLicense,
    IdCard,
    Passport,
}
impl GelatoDocumentReportType {
    pub fn as_str(self) -> &'static str {
        use GelatoDocumentReportType::*;
        match self {
            DrivingLicense => "driving_license",
            IdCard => "id_card",
            Passport => "passport",
        }
    }
}

impl std::str::FromStr for GelatoDocumentReportType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoDocumentReportType::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for GelatoDocumentReportType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for GelatoDocumentReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoDocumentReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoDocumentReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for GelatoDocumentReportType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for GelatoDocumentReportType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<GelatoDocumentReportType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoDocumentReportType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
