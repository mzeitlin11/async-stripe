#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    pub preferred_language: PaymentMethodOptionsBancontactPreferredLanguage,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentMethodOptionsBancontactSetupFutureUsage>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodOptionsBancontactBuilder {
    preferred_language: Option<PaymentMethodOptionsBancontactPreferredLanguage>,
    setup_future_usage: Option<Option<PaymentMethodOptionsBancontactSetupFutureUsage>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodOptionsBancontact {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsBancontact>,
        builder: PaymentMethodOptionsBancontactBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsBancontact> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodOptionsBancontactBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsBancontactBuilder {
        type Out = PaymentMethodOptionsBancontact;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "preferred_language" => Ok(Deserialize::begin(&mut self.preferred_language)),
                "setup_future_usage" => Ok(Deserialize::begin(&mut self.setup_future_usage)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { preferred_language: Deserialize::default(), setup_future_usage: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let preferred_language = self.preferred_language.take()?;
            let setup_future_usage = self.setup_future_usage.take()?;

            Some(Self::Out { preferred_language, setup_future_usage })
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

    impl ObjectDeser for PaymentMethodOptionsBancontact {
        type Builder = PaymentMethodOptionsBancontactBuilder;
    }
};
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}
impl PaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsBancontactPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Fr => "fr",
            Nl => "nl",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsBancontactPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsBancontactPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodOptionsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsBancontactPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsBancontactPreferredLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodOptionsBancontactPreferredLanguage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptionsBancontactPreferredLanguage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsBancontactPreferredLanguage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodOptionsBancontactPreferredLanguage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}
impl PaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsBancontactSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsBancontactSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsBancontactSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodOptionsBancontactSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodOptionsBancontactSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsBancontactSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsBancontactSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsBancontactSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodOptionsBancontactSetupFutureUsage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptionsBancontactSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsBancontactSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodOptionsBancontactSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
