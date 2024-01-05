#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourcePaymentIntentData {
    /// Indicates when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentLinksResourcePaymentIntentDataCaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    pub metadata: std::collections::HashMap<String, String>,
    /// Indicates that you intend to make future payments with the payment method collected during checkout.
    pub setup_future_usage: Option<PaymentLinksResourcePaymentIntentDataSetupFutureUsage>,
    /// Extra information about the payment.
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    pub statement_descriptor: Option<String>,
    /// Provides information about the charge that customers see on their statements.
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that's set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourcePaymentIntentDataBuilder {
    capture_method: Option<Option<PaymentLinksResourcePaymentIntentDataCaptureMethod>>,
    description: Option<Option<String>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    setup_future_usage: Option<Option<PaymentLinksResourcePaymentIntentDataSetupFutureUsage>>,
    statement_descriptor: Option<Option<String>>,
    statement_descriptor_suffix: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourcePaymentIntentData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourcePaymentIntentData>,
        builder: PaymentLinksResourcePaymentIntentDataBuilder,
    }

    impl Visitor for Place<PaymentLinksResourcePaymentIntentData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourcePaymentIntentDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourcePaymentIntentDataBuilder {
        type Out = PaymentLinksResourcePaymentIntentData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "capture_method" => Ok(Deserialize::begin(&mut self.capture_method)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "setup_future_usage" => Ok(Deserialize::begin(&mut self.setup_future_usage)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),
                "statement_descriptor_suffix" => Ok(Deserialize::begin(&mut self.statement_descriptor_suffix)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                capture_method: Deserialize::default(),
                description: Deserialize::default(),
                metadata: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                statement_descriptor_suffix: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let capture_method = self.capture_method.take()?;
            let description = self.description.take()?;
            let metadata = self.metadata.take()?;
            let setup_future_usage = self.setup_future_usage.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;
            let statement_descriptor_suffix = self.statement_descriptor_suffix.take()?;

            Some(Self::Out { capture_method, description, metadata, setup_future_usage, statement_descriptor, statement_descriptor_suffix })
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

    impl ObjectDeser for PaymentLinksResourcePaymentIntentData {
        type Builder = PaymentLinksResourcePaymentIntentDataBuilder;
    }
};
/// Indicates when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourcePaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}
impl PaymentLinksResourcePaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourcePaymentIntentDataCaptureMethod::*;
        match self {
            Automatic => "automatic",
            AutomaticAsync => "automatic_async",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourcePaymentIntentDataCaptureMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "automatic_async" => Ok(AutomaticAsync),
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinksResourcePaymentIntentDataCaptureMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinksResourcePaymentIntentDataCaptureMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinksResourcePaymentIntentDataCaptureMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Indicates that you intend to make future payments with the payment method collected during checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}
impl PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourcePaymentIntentDataSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourcePaymentIntentDataSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinksResourcePaymentIntentDataSetupFutureUsage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinksResourcePaymentIntentDataSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinksResourcePaymentIntentDataSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
