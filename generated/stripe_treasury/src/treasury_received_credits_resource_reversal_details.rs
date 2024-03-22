#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryReceivedCreditsResourceReversalDetails {
    /// Time before which a ReceivedCredit can be reversed.
    pub deadline: Option<stripe_types::Timestamp>,
    /// Set if a ReceivedCredit cannot be reversed.
    pub restricted_reason: Option<TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryReceivedCreditsResourceReversalDetailsBuilder {
    deadline: Option<Option<stripe_types::Timestamp>>,
    restricted_reason: Option<Option<TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryReceivedCreditsResourceReversalDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCreditsResourceReversalDetails>,
        builder: TreasuryReceivedCreditsResourceReversalDetailsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCreditsResourceReversalDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryReceivedCreditsResourceReversalDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryReceivedCreditsResourceReversalDetailsBuilder {
        type Out = TreasuryReceivedCreditsResourceReversalDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "deadline" => Ok(Deserialize::begin(&mut self.deadline)),
                "restricted_reason" => Ok(Deserialize::begin(&mut self.restricted_reason)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { deadline: Deserialize::default(), restricted_reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let deadline = self.deadline.take()?;
            let restricted_reason = self.restricted_reason.take()?;

            Some(Self::Out { deadline, restricted_reason })
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

    impl ObjectDeser for TreasuryReceivedCreditsResourceReversalDetails {
        type Builder = TreasuryReceivedCreditsResourceReversalDetailsBuilder;
    }
};
/// Set if a ReceivedCredit cannot be reversed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}
impl TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::*;
        match self {
            AlreadyReversed => "already_reversed",
            DeadlinePassed => "deadline_passed",
            NetworkRestricted => "network_restricted",
            Other => "other",
            SourceFlowRestricted => "source_flow_restricted",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::*;
        match s {
            "already_reversed" => Ok(AlreadyReversed),
            "deadline_passed" => Ok(DeadlinePassed),
            "network_restricted" => Ok(NetworkRestricted),
            "other" => Ok(Other),
            "source_flow_restricted" => Ok(SourceFlowRestricted),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
