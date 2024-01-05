#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionAutomaticTax {
    /// Indicates whether automatic tax is enabled for the session
    pub enabled: bool,
    /// The status of the most recent automated tax calculation for this session.
    pub status: Option<PaymentPagesCheckoutSessionAutomaticTaxStatus>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionAutomaticTaxBuilder {
    enabled: Option<bool>,
    status: Option<Option<PaymentPagesCheckoutSessionAutomaticTaxStatus>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionAutomaticTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionAutomaticTax>,
        builder: PaymentPagesCheckoutSessionAutomaticTaxBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionAutomaticTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionAutomaticTaxBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionAutomaticTaxBuilder {
        type Out = PaymentPagesCheckoutSessionAutomaticTax;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),
                "status" => Ok(Deserialize::begin(&mut self.status)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default(), status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let enabled = self.enabled.take()?;
            let status = self.status.take()?;

            Some(Self::Out { enabled, status })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionAutomaticTax {
        type Builder = PaymentPagesCheckoutSessionAutomaticTaxBuilder;
    }
};
/// The status of the most recent automated tax calculation for this session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}
impl PaymentPagesCheckoutSessionAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionAutomaticTaxStatus::*;
        match self {
            Complete => "complete",
            Failed => "failed",
            RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionAutomaticTaxStatus::*;
        match s {
            "complete" => Ok(Complete),
            "failed" => Ok(Failed),
            "requires_location_inputs" => Ok(RequiresLocationInputs),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentPagesCheckoutSessionAutomaticTaxStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentPagesCheckoutSessionAutomaticTaxStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentPagesCheckoutSessionAutomaticTaxStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
