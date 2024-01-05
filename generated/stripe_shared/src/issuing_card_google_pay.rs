#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardGooglePay {
    /// Google Pay Eligibility
    pub eligible: bool,
    /// Reason the card is ineligible for Google Pay
    pub ineligible_reason: Option<IssuingCardGooglePayIneligibleReason>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardGooglePayBuilder {
    eligible: Option<bool>,
    ineligible_reason: Option<Option<IssuingCardGooglePayIneligibleReason>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardGooglePay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardGooglePay>,
        builder: IssuingCardGooglePayBuilder,
    }

    impl Visitor for Place<IssuingCardGooglePay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardGooglePayBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardGooglePayBuilder {
        type Out = IssuingCardGooglePay;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "eligible" => Ok(Deserialize::begin(&mut self.eligible)),
                "ineligible_reason" => Ok(Deserialize::begin(&mut self.ineligible_reason)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { eligible: Deserialize::default(), ineligible_reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let eligible = self.eligible.take()?;
            let ineligible_reason = self.ineligible_reason.take()?;

            Some(Self::Out { eligible, ineligible_reason })
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

    impl ObjectDeser for IssuingCardGooglePay {
        type Builder = IssuingCardGooglePayBuilder;
    }
};
/// Reason the card is ineligible for Google Pay
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardGooglePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}
impl IssuingCardGooglePayIneligibleReason {
    pub fn as_str(self) -> &'static str {
        use IssuingCardGooglePayIneligibleReason::*;
        match self {
            MissingAgreement => "missing_agreement",
            MissingCardholderContact => "missing_cardholder_contact",
            UnsupportedRegion => "unsupported_region",
        }
    }
}

impl std::str::FromStr for IssuingCardGooglePayIneligibleReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardGooglePayIneligibleReason::*;
        match s {
            "missing_agreement" => Ok(MissingAgreement),
            "missing_cardholder_contact" => Ok(MissingCardholderContact),
            "unsupported_region" => Ok(UnsupportedRegion),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingCardGooglePayIneligibleReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardGooglePayIneligibleReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardGooglePayIneligibleReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardGooglePayIneligibleReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingCardGooglePayIneligibleReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingCardGooglePayIneligibleReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardGooglePayIneligibleReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
