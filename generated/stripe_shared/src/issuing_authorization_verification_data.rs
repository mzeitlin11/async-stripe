#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingAuthorizationVerificationData {
    /// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
    pub address_line1_check: IssuingAuthorizationVerificationDataAddressLine1Check,
    /// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
    pub address_postal_code_check: IssuingAuthorizationVerificationDataAddressPostalCodeCheck,
    /// The exemption applied to this authorization.
    pub authentication_exemption: Option<stripe_shared::IssuingAuthorizationAuthenticationExemption>,
    /// Whether the cardholder provided a CVC and if it matched Stripe’s record.
    pub cvc_check: IssuingAuthorizationVerificationDataCvcCheck,
    /// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
    pub expiry_check: IssuingAuthorizationVerificationDataExpiryCheck,
    /// The postal code submitted as part of the authorization used for postal code verification.
    pub postal_code: Option<String>,
    /// 3D Secure details.
    pub three_d_secure: Option<stripe_shared::IssuingAuthorizationThreeDSecure>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingAuthorizationVerificationDataBuilder {
    address_line1_check: Option<IssuingAuthorizationVerificationDataAddressLine1Check>,
    address_postal_code_check: Option<IssuingAuthorizationVerificationDataAddressPostalCodeCheck>,
    authentication_exemption: Option<Option<stripe_shared::IssuingAuthorizationAuthenticationExemption>>,
    cvc_check: Option<IssuingAuthorizationVerificationDataCvcCheck>,
    expiry_check: Option<IssuingAuthorizationVerificationDataExpiryCheck>,
    postal_code: Option<Option<String>>,
    three_d_secure: Option<Option<stripe_shared::IssuingAuthorizationThreeDSecure>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingAuthorizationVerificationData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationVerificationData>,
        builder: IssuingAuthorizationVerificationDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationVerificationData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingAuthorizationVerificationDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingAuthorizationVerificationDataBuilder {
        type Out = IssuingAuthorizationVerificationData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "address_line1_check" => Ok(Deserialize::begin(&mut self.address_line1_check)),
                "address_postal_code_check" => Ok(Deserialize::begin(&mut self.address_postal_code_check)),
                "authentication_exemption" => Ok(Deserialize::begin(&mut self.authentication_exemption)),
                "cvc_check" => Ok(Deserialize::begin(&mut self.cvc_check)),
                "expiry_check" => Ok(Deserialize::begin(&mut self.expiry_check)),
                "postal_code" => Ok(Deserialize::begin(&mut self.postal_code)),
                "three_d_secure" => Ok(Deserialize::begin(&mut self.three_d_secure)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                address_line1_check: Deserialize::default(),
                address_postal_code_check: Deserialize::default(),
                authentication_exemption: Deserialize::default(),
                cvc_check: Deserialize::default(),
                expiry_check: Deserialize::default(),
                postal_code: Deserialize::default(),
                three_d_secure: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address_line1_check = self.address_line1_check.take()?;
            let address_postal_code_check = self.address_postal_code_check.take()?;
            let authentication_exemption = self.authentication_exemption.take()?;
            let cvc_check = self.cvc_check.take()?;
            let expiry_check = self.expiry_check.take()?;
            let postal_code = self.postal_code.take()?;
            let three_d_secure = self.three_d_secure.take()?;

            Some(Self::Out { address_line1_check, address_postal_code_check, authentication_exemption, cvc_check, expiry_check, postal_code, three_d_secure })
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

    impl ObjectDeser for IssuingAuthorizationVerificationData {
        type Builder = IssuingAuthorizationVerificationDataBuilder;
    }
};
/// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationVerificationDataAddressLine1Check {
    Match,
    Mismatch,
    NotProvided,
}
impl IssuingAuthorizationVerificationDataAddressLine1Check {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationVerificationDataAddressLine1Check::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationVerificationDataAddressLine1Check {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationVerificationDataAddressLine1Check::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationVerificationDataAddressLine1Check"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationVerificationDataAddressLine1Check> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationVerificationDataAddressLine1Check::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    Match,
    Mismatch,
    NotProvided,
}
impl IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationVerificationDataAddressPostalCodeCheck::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationVerificationDataAddressPostalCodeCheck::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationVerificationDataAddressPostalCodeCheck"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationVerificationDataAddressPostalCodeCheck> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationVerificationDataAddressPostalCodeCheck::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Whether the cardholder provided a CVC and if it matched Stripe’s record.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationVerificationDataCvcCheck {
    Match,
    Mismatch,
    NotProvided,
}
impl IssuingAuthorizationVerificationDataCvcCheck {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationVerificationDataCvcCheck::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationVerificationDataCvcCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationVerificationDataCvcCheck::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingAuthorizationVerificationDataCvcCheck {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingAuthorizationVerificationDataCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationVerificationDataCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationVerificationDataCvcCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationVerificationDataCvcCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationVerificationDataCvcCheck"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingAuthorizationVerificationDataCvcCheck {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationVerificationDataCvcCheck> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationVerificationDataCvcCheck::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationVerificationDataExpiryCheck {
    Match,
    Mismatch,
    NotProvided,
}
impl IssuingAuthorizationVerificationDataExpiryCheck {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationVerificationDataExpiryCheck::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationVerificationDataExpiryCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationVerificationDataExpiryCheck::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingAuthorizationVerificationDataExpiryCheck {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingAuthorizationVerificationDataExpiryCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationVerificationDataExpiryCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationVerificationDataExpiryCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationVerificationDataExpiryCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationVerificationDataExpiryCheck"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingAuthorizationVerificationDataExpiryCheck {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationVerificationDataExpiryCheck> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationVerificationDataExpiryCheck::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
