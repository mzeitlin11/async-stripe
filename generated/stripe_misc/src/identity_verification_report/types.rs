/// A VerificationReport is the result of an attempt to collect and verify data from a user.
/// The collection of verification checks performed is determined from the `type` and `options`
/// parameters used. You can find the result of each verification check performed in the
/// appropriate sub-resource: `document`, `id_number`, `selfie`.
///
/// Each VerificationReport contains a copy of any data collected by the user as well as
/// reference IDs which can be used to access collected images through the [FileUpload](https://stripe.com/docs/api/files).
/// API. To configure and create VerificationReports, use the
/// [VerificationSession](https://stripe.com/docs/api/identity/verification_sessions) API.
///
/// Related guides: [Accessing verification results](https://stripe.com/docs/identity/verification-sessions#results).
///
/// For more details see <<https://stripe.com/docs/api/identity/verification_reports/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IdentityVerificationReport {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub document: Option<stripe_misc::GelatoDocumentReport>,
    /// Unique identifier for the object.
    pub id: stripe_misc::IdentityVerificationReportId,
    pub id_number: Option<stripe_misc::GelatoIdNumberReport>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub options: Option<stripe_misc::GelatoVerificationReportOptions>,
    pub selfie: Option<stripe_misc::GelatoSelfieReport>,
    /// Type of report.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: Option<stripe_misc::IdentityVerificationReportType>,
    /// ID of the VerificationSession that created this report.
    pub verification_session: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IdentityVerificationReportBuilder {
    created: Option<stripe_types::Timestamp>,
    document: Option<Option<stripe_misc::GelatoDocumentReport>>,
    id: Option<stripe_misc::IdentityVerificationReportId>,
    id_number: Option<Option<stripe_misc::GelatoIdNumberReport>>,
    livemode: Option<bool>,
    options: Option<Option<stripe_misc::GelatoVerificationReportOptions>>,
    selfie: Option<Option<stripe_misc::GelatoSelfieReport>>,
    type_: Option<Option<stripe_misc::IdentityVerificationReportType>>,
    verification_session: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IdentityVerificationReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IdentityVerificationReport>,
        builder: IdentityVerificationReportBuilder,
    }

    impl Visitor for Place<IdentityVerificationReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IdentityVerificationReportBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IdentityVerificationReportBuilder {
        type Out = IdentityVerificationReport;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "document" => Ok(Deserialize::begin(&mut self.document)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "id_number" => Ok(Deserialize::begin(&mut self.id_number)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "options" => Ok(Deserialize::begin(&mut self.options)),
                "selfie" => Ok(Deserialize::begin(&mut self.selfie)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "verification_session" => Ok(Deserialize::begin(&mut self.verification_session)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                document: Deserialize::default(),
                id: Deserialize::default(),
                id_number: Deserialize::default(),
                livemode: Deserialize::default(),
                options: Deserialize::default(),
                selfie: Deserialize::default(),
                type_: Deserialize::default(),
                verification_session: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let document = self.document.take()?;
            let id = self.id.take()?;
            let id_number = self.id_number.take()?;
            let livemode = self.livemode.take()?;
            let options = self.options.take()?;
            let selfie = self.selfie.take()?;
            let type_ = self.type_.take()?;
            let verification_session = self.verification_session.take()?;

            Some(Self::Out { created, document, id, id_number, livemode, options, selfie, type_, verification_session })
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

    impl ObjectDeser for IdentityVerificationReport {
        type Builder = IdentityVerificationReportBuilder;
    }
};
impl stripe_types::Object for IdentityVerificationReport {
    type Id = stripe_misc::IdentityVerificationReportId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IdentityVerificationReportId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IdentityVerificationReportType {
    Document,
    IdNumber,
}
impl IdentityVerificationReportType {
    pub fn as_str(self) -> &'static str {
        use IdentityVerificationReportType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for IdentityVerificationReportType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdentityVerificationReportType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IdentityVerificationReportType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IdentityVerificationReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdentityVerificationReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IdentityVerificationReportType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdentityVerificationReportType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IdentityVerificationReportType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdentityVerificationReportType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
