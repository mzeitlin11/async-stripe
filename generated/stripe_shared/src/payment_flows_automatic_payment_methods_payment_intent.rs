#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
    /// Controls whether this PaymentIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    /// To [confirm](https://stripe.com/docs/api/payment_intents/confirm) this PaymentIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the payment.
    pub allow_redirects: Option<PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects>,
    /// Automatically calculates compatible payment methods
    pub enabled: bool,
}
#[cfg(feature = "min-ser")]
pub struct PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder {
    allow_redirects: Option<Option<PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects>>,
    enabled: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAutomaticPaymentMethodsPaymentIntent>,
        builder: PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder,
    }

    impl Visitor for Place<PaymentFlowsAutomaticPaymentMethodsPaymentIntent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder {
        type Out = PaymentFlowsAutomaticPaymentMethodsPaymentIntent;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "allow_redirects" => Ok(Deserialize::begin(&mut self.allow_redirects)),
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { allow_redirects: Deserialize::default(), enabled: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let allow_redirects = self.allow_redirects.take()?;
            let enabled = self.enabled.take()?;

            Some(Self::Out { allow_redirects, enabled })
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

    impl ObjectDeser for PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
        type Builder = PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder;
    }
};
/// Controls whether this PaymentIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
/// To [confirm](https://stripe.com/docs/api/payment_intents/confirm) this PaymentIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the payment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    Always,
    Never,
}
impl PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    pub fn as_str(self) -> &'static str {
        use PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::*;
        match self {
            Always => "always",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
