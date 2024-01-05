#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankConnectionsResourceOwnershipRefresh {
    /// The time at which the last refresh attempt was initiated. Measured in seconds since the Unix epoch.
    pub last_attempted_at: stripe_types::Timestamp,
    /// The status of the last refresh attempt.
    pub status: BankConnectionsResourceOwnershipRefreshStatus,
}
#[cfg(feature = "min-ser")]
pub struct BankConnectionsResourceOwnershipRefreshBuilder {
    last_attempted_at: Option<stripe_types::Timestamp>,
    status: Option<BankConnectionsResourceOwnershipRefreshStatus>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BankConnectionsResourceOwnershipRefresh {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceOwnershipRefresh>,
        builder: BankConnectionsResourceOwnershipRefreshBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceOwnershipRefresh> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: BankConnectionsResourceOwnershipRefreshBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BankConnectionsResourceOwnershipRefreshBuilder {
        type Out = BankConnectionsResourceOwnershipRefresh;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "last_attempted_at" => Ok(Deserialize::begin(&mut self.last_attempted_at)),
                "status" => Ok(Deserialize::begin(&mut self.status)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { last_attempted_at: Deserialize::default(), status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let last_attempted_at = self.last_attempted_at.take()?;
            let status = self.status.take()?;

            Some(Self::Out { last_attempted_at, status })
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

    impl ObjectDeser for BankConnectionsResourceOwnershipRefresh {
        type Builder = BankConnectionsResourceOwnershipRefreshBuilder;
    }
};
/// The status of the last refresh attempt.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceOwnershipRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}
impl BankConnectionsResourceOwnershipRefreshStatus {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceOwnershipRefreshStatus::*;
        match self {
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceOwnershipRefreshStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceOwnershipRefreshStatus::*;
        match s {
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for BankConnectionsResourceOwnershipRefreshStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for BankConnectionsResourceOwnershipRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceOwnershipRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceOwnershipRefreshStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceOwnershipRefreshStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceOwnershipRefreshStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankConnectionsResourceOwnershipRefreshStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<BankConnectionsResourceOwnershipRefreshStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankConnectionsResourceOwnershipRefreshStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
