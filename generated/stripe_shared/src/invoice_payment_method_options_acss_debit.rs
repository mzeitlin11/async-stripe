#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsAcssDebit {
    pub mandate_options: Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebitMandateOptions>,
    /// Bank account verification method.
    pub verification_method: Option<InvoicePaymentMethodOptionsAcssDebitVerificationMethod>,
}
#[cfg(feature = "min-ser")]
pub struct InvoicePaymentMethodOptionsAcssDebitBuilder {
    mandate_options: Option<Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebitMandateOptions>>,
    verification_method: Option<Option<InvoicePaymentMethodOptionsAcssDebitVerificationMethod>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicePaymentMethodOptionsAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsAcssDebit>,
        builder: InvoicePaymentMethodOptionsAcssDebitBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoicePaymentMethodOptionsAcssDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsAcssDebitBuilder {
        type Out = InvoicePaymentMethodOptionsAcssDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "mandate_options" => Ok(Deserialize::begin(&mut self.mandate_options)),
                "verification_method" => Ok(Deserialize::begin(&mut self.verification_method)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { mandate_options: Deserialize::default(), verification_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let mandate_options = self.mandate_options.take()?;
            let verification_method = self.verification_method.take()?;

            Some(Self::Out { mandate_options, verification_method })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsAcssDebit {
        type Builder = InvoicePaymentMethodOptionsAcssDebitBuilder;
    }
};
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoicePaymentMethodOptionsAcssDebitVerificationMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoicePaymentMethodOptionsAcssDebitVerificationMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoicePaymentMethodOptionsAcssDebitVerificationMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
