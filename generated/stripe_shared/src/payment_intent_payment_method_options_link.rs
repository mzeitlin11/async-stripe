#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptionsLink {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentIntentPaymentMethodOptionsLinkCaptureMethod>,
    /// \[Deprecated\] This is a legacy parameter that no longer has any function.
    pub persistent_token: Option<String>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentPaymentMethodOptionsLinkBuilder {
    capture_method: Option<Option<PaymentIntentPaymentMethodOptionsLinkCaptureMethod>>,
    persistent_token: Option<Option<String>>,
    setup_future_usage: Option<Option<PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentPaymentMethodOptionsLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentPaymentMethodOptionsLink>,
        builder: PaymentIntentPaymentMethodOptionsLinkBuilder,
    }

    impl Visitor for Place<PaymentIntentPaymentMethodOptionsLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentPaymentMethodOptionsLinkBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentPaymentMethodOptionsLinkBuilder {
        type Out = PaymentIntentPaymentMethodOptionsLink;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "capture_method" => Ok(Deserialize::begin(&mut self.capture_method)),
                "persistent_token" => Ok(Deserialize::begin(&mut self.persistent_token)),
                "setup_future_usage" => Ok(Deserialize::begin(&mut self.setup_future_usage)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { capture_method: Deserialize::default(), persistent_token: Deserialize::default(), setup_future_usage: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let capture_method = self.capture_method.take()?;
            let persistent_token = self.persistent_token.take()?;
            let setup_future_usage = self.setup_future_usage.take()?;

            Some(Self::Out { capture_method, persistent_token, setup_future_usage })
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

    impl ObjectDeser for PaymentIntentPaymentMethodOptionsLink {
        type Builder = PaymentIntentPaymentMethodOptionsLinkBuilder;
    }
};
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}
impl PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsLinkCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsLinkCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsLinkCaptureMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsLinkCaptureMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsLinkCaptureMethod::from_str(s).map_err(|_| miniserde::Error)?);
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
pub enum PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}
impl PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
