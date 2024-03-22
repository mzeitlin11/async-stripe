#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingAuthorizationAuthenticationExemption {
    /// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
    pub claimed_by: IssuingAuthorizationAuthenticationExemptionClaimedBy,
    /// The specific exemption claimed for this authorization.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: IssuingAuthorizationAuthenticationExemptionType,
}
#[cfg(feature = "min-ser")]
pub struct IssuingAuthorizationAuthenticationExemptionBuilder {
    claimed_by: Option<IssuingAuthorizationAuthenticationExemptionClaimedBy>,
    type_: Option<IssuingAuthorizationAuthenticationExemptionType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingAuthorizationAuthenticationExemption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationAuthenticationExemption>,
        builder: IssuingAuthorizationAuthenticationExemptionBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationAuthenticationExemption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingAuthorizationAuthenticationExemptionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingAuthorizationAuthenticationExemptionBuilder {
        type Out = IssuingAuthorizationAuthenticationExemption;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "claimed_by" => Ok(Deserialize::begin(&mut self.claimed_by)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { claimed_by: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let claimed_by = self.claimed_by.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { claimed_by, type_ })
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

    impl ObjectDeser for IssuingAuthorizationAuthenticationExemption {
        type Builder = IssuingAuthorizationAuthenticationExemptionBuilder;
    }
};
/// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationAuthenticationExemptionClaimedBy {
    Acquirer,
    Issuer,
}
impl IssuingAuthorizationAuthenticationExemptionClaimedBy {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationAuthenticationExemptionClaimedBy::*;
        match self {
            Acquirer => "acquirer",
            Issuer => "issuer",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthenticationExemptionClaimedBy::*;
        match s {
            "acquirer" => Ok(Acquirer),
            "issuer" => Ok(Issuer),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationAuthenticationExemptionClaimedBy"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationAuthenticationExemptionClaimedBy> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationAuthenticationExemptionClaimedBy::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The specific exemption claimed for this authorization.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationAuthenticationExemptionType {
    LowValueTransaction,
    TransactionRiskAnalysis,
}
impl IssuingAuthorizationAuthenticationExemptionType {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationAuthenticationExemptionType::*;
        match self {
            LowValueTransaction => "low_value_transaction",
            TransactionRiskAnalysis => "transaction_risk_analysis",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthenticationExemptionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthenticationExemptionType::*;
        match s {
            "low_value_transaction" => Ok(LowValueTransaction),
            "transaction_risk_analysis" => Ok(TransactionRiskAnalysis),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingAuthorizationAuthenticationExemptionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationAuthenticationExemptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthenticationExemptionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationAuthenticationExemptionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingAuthorizationAuthenticationExemptionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationAuthenticationExemptionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationAuthenticationExemptionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
