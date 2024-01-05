#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetails {
    /// The array that contains reasons for a FinancialAccount closure.
    pub reasons: Vec<TreasuryFinancialAccountsResourceClosedStatusDetailsReasons>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder {
    reasons: Option<Vec<TreasuryFinancialAccountsResourceClosedStatusDetailsReasons>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountsResourceClosedStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceClosedStatusDetails>,
        builder: TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceClosedStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder {
        type Out = TreasuryFinancialAccountsResourceClosedStatusDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "reasons" => Ok(Deserialize::begin(&mut self.reasons)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { reasons: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let reasons = self.reasons.take()?;

            Some(Self::Out { reasons })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceClosedStatusDetails {
        type Builder = TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder;
    }
};
/// The array that contains reasons for a FinancialAccount closure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    AccountRejected,
    ClosedByPlatform,
    Other,
}
impl TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::*;
        match self {
            AccountRejected => "account_rejected",
            ClosedByPlatform => "closed_by_platform",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::*;
        match s {
            "account_rejected" => Ok(AccountRejected),
            "closed_by_platform" => Ok(ClosedByPlatform),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryFinancialAccountsResourceClosedStatusDetailsReasons> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
