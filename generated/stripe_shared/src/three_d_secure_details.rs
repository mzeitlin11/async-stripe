#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ThreeDSecureDetails {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    pub authentication_flow: Option<ThreeDSecureDetailsAuthenticationFlow>,
    /// Indicates the outcome of 3D Secure authentication.
    pub result: Option<ThreeDSecureDetailsResult>,
    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    pub result_reason: Option<ThreeDSecureDetailsResultReason>,
    /// The version of 3D Secure that was used.
    pub version: Option<ThreeDSecureDetailsVersion>,
}
#[cfg(feature = "min-ser")]
pub struct ThreeDSecureDetailsBuilder {
    authentication_flow: Option<Option<ThreeDSecureDetailsAuthenticationFlow>>,
    result: Option<Option<ThreeDSecureDetailsResult>>,
    result_reason: Option<Option<ThreeDSecureDetailsResultReason>>,
    version: Option<Option<ThreeDSecureDetailsVersion>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ThreeDSecureDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureDetails>,
        builder: ThreeDSecureDetailsBuilder,
    }

    impl Visitor for Place<ThreeDSecureDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ThreeDSecureDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ThreeDSecureDetailsBuilder {
        type Out = ThreeDSecureDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "authentication_flow" => Ok(Deserialize::begin(&mut self.authentication_flow)),
                "result" => Ok(Deserialize::begin(&mut self.result)),
                "result_reason" => Ok(Deserialize::begin(&mut self.result_reason)),
                "version" => Ok(Deserialize::begin(&mut self.version)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { authentication_flow: Deserialize::default(), result: Deserialize::default(), result_reason: Deserialize::default(), version: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let authentication_flow = self.authentication_flow.take()?;
            let result = self.result.take()?;
            let result_reason = self.result_reason.take()?;
            let version = self.version.take()?;

            Some(Self::Out { authentication_flow, result, result_reason, version })
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

    impl ObjectDeser for ThreeDSecureDetails {
        type Builder = ThreeDSecureDetailsBuilder;
    }
};
/// For authenticated transactions: how the customer was authenticated by
/// the issuing bank.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsAuthenticationFlow {
    Challenge,
    Frictionless,
}
impl ThreeDSecureDetailsAuthenticationFlow {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsAuthenticationFlow::*;
        match self {
            Challenge => "challenge",
            Frictionless => "frictionless",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsAuthenticationFlow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsAuthenticationFlow::*;
        match s {
            "challenge" => Ok(Challenge),
            "frictionless" => Ok(Frictionless),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ThreeDSecureDetailsAuthenticationFlow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsAuthenticationFlow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsAuthenticationFlow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsAuthenticationFlow"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureDetailsAuthenticationFlow {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsAuthenticationFlow> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsAuthenticationFlow::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Indicates the outcome of 3D Secure authentication.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
}
impl ThreeDSecureDetailsResult {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsResult::*;
        match self {
            AttemptAcknowledged => "attempt_acknowledged",
            Authenticated => "authenticated",
            Exempted => "exempted",
            Failed => "failed",
            NotSupported => "not_supported",
            ProcessingError => "processing_error",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsResult {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsResult::*;
        match s {
            "attempt_acknowledged" => Ok(AttemptAcknowledged),
            "authenticated" => Ok(Authenticated),
            "exempted" => Ok(Exempted),
            "failed" => Ok(Failed),
            "not_supported" => Ok(NotSupported),
            "processing_error" => Ok(ProcessingError),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ThreeDSecureDetailsResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsResult"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureDetailsResult {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsResult> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsResult::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Additional information about why 3D Secure succeeded or failed based
/// on the `result`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}
impl ThreeDSecureDetailsResultReason {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsResultReason::*;
        match self {
            Abandoned => "abandoned",
            Bypassed => "bypassed",
            Canceled => "canceled",
            CardNotEnrolled => "card_not_enrolled",
            NetworkNotSupported => "network_not_supported",
            ProtocolError => "protocol_error",
            Rejected => "rejected",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsResultReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsResultReason::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "bypassed" => Ok(Bypassed),
            "canceled" => Ok(Canceled),
            "card_not_enrolled" => Ok(CardNotEnrolled),
            "network_not_supported" => Ok(NetworkNotSupported),
            "protocol_error" => Ok(ProtocolError),
            "rejected" => Ok(Rejected),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ThreeDSecureDetailsResultReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsResultReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsResultReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsResultReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureDetailsResultReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsResultReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsResultReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The version of 3D Secure that was used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsVersion {
    V1_0_2,
    V2_1_0,
    V2_2_0,
}
impl ThreeDSecureDetailsVersion {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsVersion::*;
        match self {
            V1_0_2 => "1.0.2",
            V2_1_0 => "2.1.0",
            V2_2_0 => "2.2.0",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsVersion::*;
        match s {
            "1.0.2" => Ok(V1_0_2),
            "2.1.0" => Ok(V2_1_0),
            "2.2.0" => Ok(V2_2_0),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ThreeDSecureDetailsVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsVersion"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureDetailsVersion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsVersion> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsVersion::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
