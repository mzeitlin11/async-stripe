#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingDisputeOtherEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeOtherEvidenceProductType>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingDisputeOtherEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    explanation: Option<Option<String>>,
    product_description: Option<Option<String>>,
    product_type: Option<Option<IssuingDisputeOtherEvidenceProductType>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingDisputeOtherEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeOtherEvidence>,
        builder: IssuingDisputeOtherEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeOtherEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingDisputeOtherEvidenceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingDisputeOtherEvidenceBuilder {
        type Out = IssuingDisputeOtherEvidence;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "additional_documentation" => Ok(Deserialize::begin(&mut self.additional_documentation)),
                "explanation" => Ok(Deserialize::begin(&mut self.explanation)),
                "product_description" => Ok(Deserialize::begin(&mut self.product_description)),
                "product_type" => Ok(Deserialize::begin(&mut self.product_type)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { additional_documentation: Deserialize::default(), explanation: Deserialize::default(), product_description: Deserialize::default(), product_type: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let additional_documentation = self.additional_documentation.take()?;
            let explanation = self.explanation.take()?;
            let product_description = self.product_description.take()?;
            let product_type = self.product_type.take()?;

            Some(Self::Out { additional_documentation, explanation, product_description, product_type })
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

    impl ObjectDeser for IssuingDisputeOtherEvidence {
        type Builder = IssuingDisputeOtherEvidenceBuilder;
    }
};
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingDisputeOtherEvidenceProductType {
    Merchandise,
    Service,
}
impl IssuingDisputeOtherEvidenceProductType {
    pub fn as_str(self) -> &'static str {
        use IssuingDisputeOtherEvidenceProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
        }
    }
}

impl std::str::FromStr for IssuingDisputeOtherEvidenceProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeOtherEvidenceProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingDisputeOtherEvidenceProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingDisputeOtherEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeOtherEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingDisputeOtherEvidenceProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingDisputeOtherEvidenceProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingDisputeOtherEvidenceProductType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingDisputeOtherEvidenceProductType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingDisputeOtherEvidenceProductType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingDisputeOtherEvidenceProductType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
