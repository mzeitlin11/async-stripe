#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Date when the product was received.
    pub received_at: Option<stripe_types::Timestamp>,
    /// Description of the cardholder's attempt to return the product.
    pub return_description: Option<String>,
    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    explanation: Option<Option<String>>,
    received_at: Option<Option<stripe_types::Timestamp>>,
    return_description: Option<Option<String>>,
    return_status: Option<Option<IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus>>,
    returned_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingDisputeMerchandiseNotAsDescribedEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeMerchandiseNotAsDescribedEvidence>,
        builder: IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeMerchandiseNotAsDescribedEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder {
        type Out = IssuingDisputeMerchandiseNotAsDescribedEvidence;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "additional_documentation" => Ok(Deserialize::begin(&mut self.additional_documentation)),
                "explanation" => Ok(Deserialize::begin(&mut self.explanation)),
                "received_at" => Ok(Deserialize::begin(&mut self.received_at)),
                "return_description" => Ok(Deserialize::begin(&mut self.return_description)),
                "return_status" => Ok(Deserialize::begin(&mut self.return_status)),
                "returned_at" => Ok(Deserialize::begin(&mut self.returned_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                additional_documentation: Deserialize::default(),
                explanation: Deserialize::default(),
                received_at: Deserialize::default(),
                return_description: Deserialize::default(),
                return_status: Deserialize::default(),
                returned_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let additional_documentation = self.additional_documentation.take()?;
            let explanation = self.explanation.take()?;
            let received_at = self.received_at.take()?;
            let return_description = self.return_description.take()?;
            let return_status = self.return_status.take()?;
            let returned_at = self.returned_at.take()?;

            Some(Self::Out { additional_documentation, explanation, received_at, return_description, return_status, returned_at })
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

    impl ObjectDeser for IssuingDisputeMerchandiseNotAsDescribedEvidence {
        type Builder = IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder;
    }
};
/// Result of cardholder's attempt to return the product.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    MerchantRejected,
    Successful,
}
impl IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus::*;
        match self {
            MerchantRejected => "merchant_rejected",
            Successful => "successful",
        }
    }
}

impl std::str::FromStr for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus::*;
        match s {
            "merchant_rejected" => Ok(MerchantRejected),
            "successful" => Ok(Successful),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
