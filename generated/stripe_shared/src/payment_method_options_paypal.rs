#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodOptionsPaypal {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentMethodOptionsPaypalCaptureMethod>,
    /// Preferred locale of the PayPal checkout page that the customer is redirected to.
    pub preferred_locale: Option<String>,
    /// A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID.
    /// This must be a globally unique ID if you have configured in your PayPal settings to block multiple payments per invoice ID.
    pub reference: Option<String>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentMethodOptionsPaypalSetupFutureUsage>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodOptionsPaypalBuilder {
    capture_method: Option<Option<PaymentMethodOptionsPaypalCaptureMethod>>,
    preferred_locale: Option<Option<String>>,
    reference: Option<Option<String>>,
    setup_future_usage: Option<Option<PaymentMethodOptionsPaypalSetupFutureUsage>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodOptionsPaypal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsPaypal>,
        builder: PaymentMethodOptionsPaypalBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsPaypal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodOptionsPaypalBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsPaypalBuilder {
        type Out = PaymentMethodOptionsPaypal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "capture_method" => Ok(Deserialize::begin(&mut self.capture_method)),
                "preferred_locale" => Ok(Deserialize::begin(&mut self.preferred_locale)),
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "setup_future_usage" => Ok(Deserialize::begin(&mut self.setup_future_usage)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { capture_method: Deserialize::default(), preferred_locale: Deserialize::default(), reference: Deserialize::default(), setup_future_usage: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let capture_method = self.capture_method.take()?;
            let preferred_locale = self.preferred_locale.take()?;
            let reference = self.reference.take()?;
            let setup_future_usage = self.setup_future_usage.take()?;

            Some(Self::Out { capture_method, preferred_locale, reference, setup_future_usage })
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

    impl ObjectDeser for PaymentMethodOptionsPaypal {
        type Builder = PaymentMethodOptionsPaypalBuilder;
    }
};
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsPaypalCaptureMethod {
    Manual,
}
impl PaymentMethodOptionsPaypalCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsPaypalCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsPaypalCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsPaypalCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodOptionsPaypalCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodOptionsPaypalCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsPaypalCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsPaypalCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsPaypalCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodOptionsPaypalCaptureMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptionsPaypalCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsPaypalCaptureMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodOptionsPaypalCaptureMethod::from_str(s).map_err(|_| miniserde::Error)?);
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
pub enum PaymentMethodOptionsPaypalSetupFutureUsage {
    None,
    OffSession,
}
impl PaymentMethodOptionsPaypalSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsPaypalSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsPaypalSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsPaypalSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodOptionsPaypalSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodOptionsPaypalSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsPaypalSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsPaypalSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsPaypalSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodOptionsPaypalSetupFutureUsage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptionsPaypalSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsPaypalSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodOptionsPaypalSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
