#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxIdVerification {
    /// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
    pub status: TaxIdVerificationStatus,
    /// Verified address.
    pub verified_address: Option<String>,
    /// Verified name.
    pub verified_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct TaxIdVerificationBuilder {
    status: Option<TaxIdVerificationStatus>,
    verified_address: Option<Option<String>>,
    verified_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxIdVerification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxIdVerification>,
        builder: TaxIdVerificationBuilder,
    }

    impl Visitor for Place<TaxIdVerification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxIdVerificationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxIdVerificationBuilder {
        type Out = TaxIdVerification;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "verified_address" => Ok(Deserialize::begin(&mut self.verified_address)),
                "verified_name" => Ok(Deserialize::begin(&mut self.verified_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { status: Deserialize::default(), verified_address: Deserialize::default(), verified_name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let status = self.status.take()?;
            let verified_address = self.verified_address.take()?;
            let verified_name = self.verified_name.take()?;

            Some(Self::Out { status, verified_address, verified_name })
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

    impl ObjectDeser for TaxIdVerification {
        type Builder = TaxIdVerificationBuilder;
    }
};
/// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxIdVerificationStatus {
    Pending,
    Unavailable,
    Unverified,
    Verified,
}
impl TaxIdVerificationStatus {
    pub fn as_str(self) -> &'static str {
        use TaxIdVerificationStatus::*;
        match self {
            Pending => "pending",
            Unavailable => "unavailable",
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for TaxIdVerificationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxIdVerificationStatus::*;
        match s {
            "pending" => Ok(Pending),
            "unavailable" => Ok(Unavailable),
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxIdVerificationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxIdVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxIdVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxIdVerificationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxIdVerificationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxIdVerificationStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxIdVerificationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxIdVerificationStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxIdVerificationStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
