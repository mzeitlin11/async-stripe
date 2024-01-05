#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingDisputeEvidence {
    pub canceled: Option<stripe_shared::IssuingDisputeCanceledEvidence>,
    pub duplicate: Option<stripe_shared::IssuingDisputeDuplicateEvidence>,
    pub fraudulent: Option<stripe_shared::IssuingDisputeFraudulentEvidence>,
    pub merchandise_not_as_described: Option<stripe_shared::IssuingDisputeMerchandiseNotAsDescribedEvidence>,
    pub not_received: Option<stripe_shared::IssuingDisputeNotReceivedEvidence>,
    pub other: Option<stripe_shared::IssuingDisputeOtherEvidence>,
    /// The reason for filing the dispute. Its value will match the field containing the evidence.
    pub reason: IssuingDisputeEvidenceReason,
    pub service_not_as_described: Option<stripe_shared::IssuingDisputeServiceNotAsDescribedEvidence>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingDisputeEvidenceBuilder {
    canceled: Option<Option<stripe_shared::IssuingDisputeCanceledEvidence>>,
    duplicate: Option<Option<stripe_shared::IssuingDisputeDuplicateEvidence>>,
    fraudulent: Option<Option<stripe_shared::IssuingDisputeFraudulentEvidence>>,
    merchandise_not_as_described: Option<Option<stripe_shared::IssuingDisputeMerchandiseNotAsDescribedEvidence>>,
    not_received: Option<Option<stripe_shared::IssuingDisputeNotReceivedEvidence>>,
    other: Option<Option<stripe_shared::IssuingDisputeOtherEvidence>>,
    reason: Option<IssuingDisputeEvidenceReason>,
    service_not_as_described: Option<Option<stripe_shared::IssuingDisputeServiceNotAsDescribedEvidence>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingDisputeEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeEvidence>,
        builder: IssuingDisputeEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingDisputeEvidenceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingDisputeEvidenceBuilder {
        type Out = IssuingDisputeEvidence;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "canceled" => Ok(Deserialize::begin(&mut self.canceled)),
                "duplicate" => Ok(Deserialize::begin(&mut self.duplicate)),
                "fraudulent" => Ok(Deserialize::begin(&mut self.fraudulent)),
                "merchandise_not_as_described" => Ok(Deserialize::begin(&mut self.merchandise_not_as_described)),
                "not_received" => Ok(Deserialize::begin(&mut self.not_received)),
                "other" => Ok(Deserialize::begin(&mut self.other)),
                "reason" => Ok(Deserialize::begin(&mut self.reason)),
                "service_not_as_described" => Ok(Deserialize::begin(&mut self.service_not_as_described)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                canceled: Deserialize::default(),
                duplicate: Deserialize::default(),
                fraudulent: Deserialize::default(),
                merchandise_not_as_described: Deserialize::default(),
                not_received: Deserialize::default(),
                other: Deserialize::default(),
                reason: Deserialize::default(),
                service_not_as_described: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let canceled = self.canceled.take()?;
            let duplicate = self.duplicate.take()?;
            let fraudulent = self.fraudulent.take()?;
            let merchandise_not_as_described = self.merchandise_not_as_described.take()?;
            let not_received = self.not_received.take()?;
            let other = self.other.take()?;
            let reason = self.reason.take()?;
            let service_not_as_described = self.service_not_as_described.take()?;

            Some(Self::Out { canceled, duplicate, fraudulent, merchandise_not_as_described, not_received, other, reason, service_not_as_described })
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

    impl ObjectDeser for IssuingDisputeEvidence {
        type Builder = IssuingDisputeEvidenceBuilder;
    }
};
/// The reason for filing the dispute. Its value will match the field containing the evidence.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}
impl IssuingDisputeEvidenceReason {
    pub fn as_str(self) -> &'static str {
        use IssuingDisputeEvidenceReason::*;
        match self {
            Canceled => "canceled",
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            MerchandiseNotAsDescribed => "merchandise_not_as_described",
            NotReceived => "not_received",
            Other => "other",
            ServiceNotAsDescribed => "service_not_as_described",
        }
    }
}

impl std::str::FromStr for IssuingDisputeEvidenceReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeEvidenceReason::*;
        match s {
            "canceled" => Ok(Canceled),
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "merchandise_not_as_described" => Ok(MerchandiseNotAsDescribed),
            "not_received" => Ok(NotReceived),
            "other" => Ok(Other),
            "service_not_as_described" => Ok(ServiceNotAsDescribed),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingDisputeEvidenceReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingDisputeEvidenceReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingDisputeEvidenceReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingDisputeEvidenceReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingDisputeEvidenceReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingDisputeEvidenceReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingDisputeEvidenceReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
