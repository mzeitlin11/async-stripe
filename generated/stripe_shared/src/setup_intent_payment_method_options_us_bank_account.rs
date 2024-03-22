#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsUsBankAccount {
    pub financial_connections: Option<stripe_shared::LinkedAccountOptionsUsBankAccount>,
    /// Bank account verification method.
    pub verification_method: Option<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentPaymentMethodOptionsUsBankAccountBuilder {
    financial_connections: Option<Option<stripe_shared::LinkedAccountOptionsUsBankAccount>>,
    verification_method: Option<Option<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentPaymentMethodOptionsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsUsBankAccount>,
        builder: SetupIntentPaymentMethodOptionsUsBankAccountBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentPaymentMethodOptionsUsBankAccountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsUsBankAccountBuilder {
        type Out = SetupIntentPaymentMethodOptionsUsBankAccount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "financial_connections" => Ok(Deserialize::begin(&mut self.financial_connections)),
                "verification_method" => Ok(Deserialize::begin(&mut self.verification_method)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { financial_connections: Deserialize::default(), verification_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let financial_connections = self.financial_connections.take()?;
            let verification_method = self.verification_method.take()?;

            Some(Self::Out { financial_connections, verification_method })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsUsBankAccount {
        type Builder = SetupIntentPaymentMethodOptionsUsBankAccountBuilder;
    }
};
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
