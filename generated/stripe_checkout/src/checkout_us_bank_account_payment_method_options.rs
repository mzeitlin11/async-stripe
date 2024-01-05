#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CheckoutUsBankAccountPaymentMethodOptions {
    pub financial_connections: Option<stripe_shared::LinkedAccountOptionsUsBankAccount>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    pub setup_future_usage: Option<CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage>,
    /// Bank account verification method.
    pub verification_method: Option<CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod>,
}
#[cfg(feature = "min-ser")]
pub struct CheckoutUsBankAccountPaymentMethodOptionsBuilder {
    financial_connections: Option<Option<stripe_shared::LinkedAccountOptionsUsBankAccount>>,
    setup_future_usage: Option<Option<CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage>>,
    verification_method: Option<Option<CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutUsBankAccountPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutUsBankAccountPaymentMethodOptions>,
        builder: CheckoutUsBankAccountPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutUsBankAccountPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CheckoutUsBankAccountPaymentMethodOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CheckoutUsBankAccountPaymentMethodOptionsBuilder {
        type Out = CheckoutUsBankAccountPaymentMethodOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "financial_connections" => Ok(Deserialize::begin(&mut self.financial_connections)),
                "setup_future_usage" => Ok(Deserialize::begin(&mut self.setup_future_usage)),
                "verification_method" => Ok(Deserialize::begin(&mut self.verification_method)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { financial_connections: Deserialize::default(), setup_future_usage: Deserialize::default(), verification_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let financial_connections = self.financial_connections.take()?;
            let setup_future_usage = self.setup_future_usage.take()?;
            let verification_method = self.verification_method.take()?;

            Some(Self::Out { financial_connections, setup_future_usage, verification_method })
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

    impl ObjectDeser for CheckoutUsBankAccountPaymentMethodOptions {
        type Builder = CheckoutUsBankAccountPaymentMethodOptionsBuilder;
    }
};
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
}
impl CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
        }
    }
}

impl std::str::FromStr for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
