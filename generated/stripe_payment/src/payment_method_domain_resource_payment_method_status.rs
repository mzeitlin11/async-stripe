/// Indicates the status of a specific payment method on a payment method domain.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDomainResourcePaymentMethodStatus {
    /// The status of the payment method on the domain.
    pub status: PaymentMethodDomainResourcePaymentMethodStatusStatus,
    pub status_details: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatusDetails>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDomainResourcePaymentMethodStatusBuilder {
    status: Option<PaymentMethodDomainResourcePaymentMethodStatusStatus>,
    status_details: Option<Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatusDetails>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDomainResourcePaymentMethodStatus {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDomainResourcePaymentMethodStatus>,
        builder: PaymentMethodDomainResourcePaymentMethodStatusBuilder,
    }

    impl Visitor for Place<PaymentMethodDomainResourcePaymentMethodStatus> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDomainResourcePaymentMethodStatusBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDomainResourcePaymentMethodStatusBuilder {
        type Out = PaymentMethodDomainResourcePaymentMethodStatus;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "status_details" => Ok(Deserialize::begin(&mut self.status_details)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { status: Deserialize::default(), status_details: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let status = self.status.take()?;
            let status_details = self.status_details.take()?;

            Some(Self::Out { status, status_details })
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

    impl ObjectDeser for PaymentMethodDomainResourcePaymentMethodStatus {
        type Builder = PaymentMethodDomainResourcePaymentMethodStatusBuilder;
    }
};
/// The status of the payment method on the domain.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDomainResourcePaymentMethodStatusStatus {
    Active,
    Inactive,
}
impl PaymentMethodDomainResourcePaymentMethodStatusStatus {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDomainResourcePaymentMethodStatusStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDomainResourcePaymentMethodStatusStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodDomainResourcePaymentMethodStatusStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodDomainResourcePaymentMethodStatusStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDomainResourcePaymentMethodStatusStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
