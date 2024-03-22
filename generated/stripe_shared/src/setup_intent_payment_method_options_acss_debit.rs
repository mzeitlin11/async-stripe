#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsAcssDebit {
    /// Currency supported by the bank account
    pub currency: Option<SetupIntentPaymentMethodOptionsAcssDebitCurrency>,
    pub mandate_options: Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>,
    /// Bank account verification method.
    pub verification_method: Option<SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentPaymentMethodOptionsAcssDebitBuilder {
    currency: Option<Option<SetupIntentPaymentMethodOptionsAcssDebitCurrency>>,
    mandate_options: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>>,
    verification_method: Option<Option<SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentPaymentMethodOptionsAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsAcssDebit>,
        builder: SetupIntentPaymentMethodOptionsAcssDebitBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentPaymentMethodOptionsAcssDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsAcssDebitBuilder {
        type Out = SetupIntentPaymentMethodOptionsAcssDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "mandate_options" => Ok(Deserialize::begin(&mut self.mandate_options)),
                "verification_method" => Ok(Deserialize::begin(&mut self.verification_method)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { currency: Deserialize::default(), mandate_options: Deserialize::default(), verification_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let currency = self.currency.take()?;
            let mandate_options = self.mandate_options.take()?;
            let verification_method = self.verification_method.take()?;

            Some(Self::Out { currency, mandate_options, verification_method })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsAcssDebit {
        type Builder = SetupIntentPaymentMethodOptionsAcssDebitBuilder;
    }
};
/// Currency supported by the bank account
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}
impl SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsAcssDebitCurrency::*;
        match self {
            Cad => "cad",
            Usd => "usd",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsAcssDebitCurrency::*;
        match s {
            "cad" => Ok(Cad),
            "usd" => Ok(Usd),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentPaymentMethodOptionsAcssDebitCurrency"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupIntentPaymentMethodOptionsAcssDebitCurrency> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentPaymentMethodOptionsAcssDebitCurrency::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
