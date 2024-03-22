/// A VerificationSession guides you through the process of collecting and verifying the identities
/// of your users. It contains details about the type of verification, such as what [verification
/// check](/docs/identity/verification-checks) to perform. Only create one VerificationSession for
/// each verification in your system.
///
/// A VerificationSession transitions through [multiple
/// statuses](/docs/identity/how-sessions-work) throughout its lifetime as it progresses through
/// the verification flow. The VerificationSession contains the user's verified data after
/// verification checks are complete.
///
/// Related guide: [The Verification Sessions API](https://stripe.com/docs/identity/verification-sessions).
///
/// For more details see <<https://stripe.com/docs/api/identity/verification_sessions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IdentityVerificationSession {
    /// The short-lived client secret used by Stripe.js to [show a verification modal](https://stripe.com/docs/js/identity/modal) inside your app.
    /// This client secret expires after 24 hours and can only be used once.
    /// Don’t store it, log it, embed it in a URL, or expose it to anyone other than the user.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    /// Refer to our docs on [passing the client secret to the frontend](https://stripe.com/docs/identity/verification-sessions#client-secret) to learn more.
    pub client_secret: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::IdentityVerificationSessionId,
    /// If present, this property tells you the last error encountered when processing the verification.
    pub last_error: Option<stripe_misc::GelatoSessionLastError>,
    /// ID of the most recent VerificationReport.
    /// [Learn more about accessing detailed verification results.](https://stripe.com/docs/identity/verification-sessions#results).
    pub last_verification_report: Option<stripe_types::Expandable<stripe_misc::IdentityVerificationReport>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// A set of options for the session’s verification checks.
    pub options: Option<stripe_misc::GelatoVerificationSessionOptions>,
    /// Redaction status of this VerificationSession.
    /// If the VerificationSession is not redacted, this field will be null.
    pub redaction: Option<stripe_misc::VerificationSessionRedaction>,
    /// Status of this VerificationSession.
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub status: stripe_misc::IdentityVerificationSessionStatus,
    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: Option<stripe_misc::IdentityVerificationSessionType>,
    /// The short-lived URL that you use to redirect a user to Stripe to submit their identity information.
    /// This URL expires after 48 hours and can only be used once.
    /// Don’t store it, log it, send it in emails or expose it to anyone other than the user.
    /// Refer to our docs on [verifying identity documents](https://stripe.com/docs/identity/verify-identity-documents?platform=web&type=redirect) to learn how to redirect users to Stripe.
    pub url: Option<String>,
    /// The user’s verified data.
    pub verified_outputs: Option<stripe_misc::GelatoVerifiedOutputs>,
}
#[cfg(feature = "min-ser")]
pub struct IdentityVerificationSessionBuilder {
    client_secret: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_misc::IdentityVerificationSessionId>,
    last_error: Option<Option<stripe_misc::GelatoSessionLastError>>,
    last_verification_report: Option<Option<stripe_types::Expandable<stripe_misc::IdentityVerificationReport>>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    options: Option<Option<stripe_misc::GelatoVerificationSessionOptions>>,
    redaction: Option<Option<stripe_misc::VerificationSessionRedaction>>,
    status: Option<stripe_misc::IdentityVerificationSessionStatus>,
    type_: Option<Option<stripe_misc::IdentityVerificationSessionType>>,
    url: Option<Option<String>>,
    verified_outputs: Option<Option<stripe_misc::GelatoVerifiedOutputs>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IdentityVerificationSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IdentityVerificationSession>,
        builder: IdentityVerificationSessionBuilder,
    }

    impl Visitor for Place<IdentityVerificationSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IdentityVerificationSessionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IdentityVerificationSessionBuilder {
        type Out = IdentityVerificationSession;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "client_secret" => Ok(Deserialize::begin(&mut self.client_secret)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "last_error" => Ok(Deserialize::begin(&mut self.last_error)),
                "last_verification_report" => Ok(Deserialize::begin(&mut self.last_verification_report)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "options" => Ok(Deserialize::begin(&mut self.options)),
                "redaction" => Ok(Deserialize::begin(&mut self.redaction)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "url" => Ok(Deserialize::begin(&mut self.url)),
                "verified_outputs" => Ok(Deserialize::begin(&mut self.verified_outputs)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                client_secret: Deserialize::default(),
                created: Deserialize::default(),
                id: Deserialize::default(),
                last_error: Deserialize::default(),
                last_verification_report: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                options: Deserialize::default(),
                redaction: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
                url: Deserialize::default(),
                verified_outputs: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let client_secret = self.client_secret.take()?;
            let created = self.created.take()?;
            let id = self.id.take()?;
            let last_error = self.last_error.take()?;
            let last_verification_report = self.last_verification_report.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let options = self.options.take()?;
            let redaction = self.redaction.take()?;
            let status = self.status.take()?;
            let type_ = self.type_.take()?;
            let url = self.url.take()?;
            let verified_outputs = self.verified_outputs.take()?;

            Some(Self::Out { client_secret, created, id, last_error, last_verification_report, livemode, metadata, options, redaction, status, type_, url, verified_outputs })
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

    impl ObjectDeser for IdentityVerificationSession {
        type Builder = IdentityVerificationSessionBuilder;
    }
};
impl stripe_types::Object for IdentityVerificationSession {
    type Id = stripe_misc::IdentityVerificationSessionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IdentityVerificationSessionId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IdentityVerificationSessionStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
}
impl IdentityVerificationSessionStatus {
    pub fn as_str(self) -> &'static str {
        use IdentityVerificationSessionStatus::*;
        match self {
            Canceled => "canceled",
            Processing => "processing",
            RequiresInput => "requires_input",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for IdentityVerificationSessionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdentityVerificationSessionStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "processing" => Ok(Processing),
            "requires_input" => Ok(RequiresInput),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IdentityVerificationSessionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IdentityVerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IdentityVerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IdentityVerificationSessionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdentityVerificationSessionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IdentityVerificationSessionStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdentityVerificationSessionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IdentityVerificationSessionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdentityVerificationSessionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IdentityVerificationSessionType {
    Document,
    IdNumber,
}
impl IdentityVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        use IdentityVerificationSessionType::*;
        match self {
            Document => "document",
            IdNumber => "id_number",
        }
    }
}

impl std::str::FromStr for IdentityVerificationSessionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdentityVerificationSessionType::*;
        match s {
            "document" => Ok(Document),
            "id_number" => Ok(IdNumber),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IdentityVerificationSessionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IdentityVerificationSessionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdentityVerificationSessionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IdentityVerificationSessionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdentityVerificationSessionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IdentityVerificationSessionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdentityVerificationSessionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
