#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CancellationDetails {
    /// Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the user.
    pub comment: Option<String>,
    /// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
    pub feedback: Option<CancellationDetailsFeedback>,
    /// Why this subscription was canceled.
    pub reason: Option<CancellationDetailsReason>,
}
#[cfg(feature = "min-ser")]
pub struct CancellationDetailsBuilder {
    comment: Option<Option<String>>,
    feedback: Option<Option<CancellationDetailsFeedback>>,
    reason: Option<Option<CancellationDetailsReason>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CancellationDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CancellationDetails>,
        builder: CancellationDetailsBuilder,
    }

    impl Visitor for Place<CancellationDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CancellationDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CancellationDetailsBuilder {
        type Out = CancellationDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "comment" => Ok(Deserialize::begin(&mut self.comment)),
                "feedback" => Ok(Deserialize::begin(&mut self.feedback)),
                "reason" => Ok(Deserialize::begin(&mut self.reason)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { comment: Deserialize::default(), feedback: Deserialize::default(), reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let comment = self.comment.take()?;
            let feedback = self.feedback.take()?;
            let reason = self.reason.take()?;

            Some(Self::Out { comment, feedback, reason })
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

    impl ObjectDeser for CancellationDetails {
        type Builder = CancellationDetailsBuilder;
    }
};
/// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CancellationDetailsFeedback {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}
impl CancellationDetailsFeedback {
    pub fn as_str(self) -> &'static str {
        use CancellationDetailsFeedback::*;
        match self {
            CustomerService => "customer_service",
            LowQuality => "low_quality",
            MissingFeatures => "missing_features",
            Other => "other",
            SwitchedService => "switched_service",
            TooComplex => "too_complex",
            TooExpensive => "too_expensive",
            Unused => "unused",
        }
    }
}

impl std::str::FromStr for CancellationDetailsFeedback {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CancellationDetailsFeedback::*;
        match s {
            "customer_service" => Ok(CustomerService),
            "low_quality" => Ok(LowQuality),
            "missing_features" => Ok(MissingFeatures),
            "other" => Ok(Other),
            "switched_service" => Ok(SwitchedService),
            "too_complex" => Ok(TooComplex),
            "too_expensive" => Ok(TooExpensive),
            "unused" => Ok(Unused),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CancellationDetailsFeedback {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CancellationDetailsFeedback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CancellationDetailsFeedback {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CancellationDetailsFeedback"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CancellationDetailsFeedback {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CancellationDetailsFeedback> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CancellationDetailsFeedback::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Why this subscription was canceled.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CancellationDetailsReason {
    CancellationRequested,
    PaymentDisputed,
    PaymentFailed,
}
impl CancellationDetailsReason {
    pub fn as_str(self) -> &'static str {
        use CancellationDetailsReason::*;
        match self {
            CancellationRequested => "cancellation_requested",
            PaymentDisputed => "payment_disputed",
            PaymentFailed => "payment_failed",
        }
    }
}

impl std::str::FromStr for CancellationDetailsReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CancellationDetailsReason::*;
        match s {
            "cancellation_requested" => Ok(CancellationRequested),
            "payment_disputed" => Ok(PaymentDisputed),
            "payment_failed" => Ok(PaymentFailed),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CancellationDetailsReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CancellationDetailsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CancellationDetailsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CancellationDetailsReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CancellationDetailsReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CancellationDetailsReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CancellationDetailsReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CancellationDetailsReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CancellationDetailsReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
