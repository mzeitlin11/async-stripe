#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CheckoutCardPaymentMethodOptions {
    pub installments: Option<stripe_checkout::CheckoutCardInstallmentsOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    pub setup_future_usage: Option<CheckoutCardPaymentMethodOptionsSetupFutureUsage>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    pub statement_descriptor_suffix_kana: Option<String>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    pub statement_descriptor_suffix_kanji: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct CheckoutCardPaymentMethodOptionsBuilder {
    installments: Option<Option<stripe_checkout::CheckoutCardInstallmentsOptions>>,
    setup_future_usage: Option<Option<CheckoutCardPaymentMethodOptionsSetupFutureUsage>>,
    statement_descriptor_suffix_kana: Option<Option<String>>,
    statement_descriptor_suffix_kanji: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutCardPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutCardPaymentMethodOptions>,
        builder: CheckoutCardPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutCardPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CheckoutCardPaymentMethodOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CheckoutCardPaymentMethodOptionsBuilder {
        type Out = CheckoutCardPaymentMethodOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "installments" => Ok(Deserialize::begin(&mut self.installments)),
                "setup_future_usage" => Ok(Deserialize::begin(&mut self.setup_future_usage)),
                "statement_descriptor_suffix_kana" => Ok(Deserialize::begin(&mut self.statement_descriptor_suffix_kana)),
                "statement_descriptor_suffix_kanji" => Ok(Deserialize::begin(&mut self.statement_descriptor_suffix_kanji)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                installments: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
                statement_descriptor_suffix_kana: Deserialize::default(),
                statement_descriptor_suffix_kanji: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let installments = self.installments.take()?;
            let setup_future_usage = self.setup_future_usage.take()?;
            let statement_descriptor_suffix_kana = self.statement_descriptor_suffix_kana.take()?;
            let statement_descriptor_suffix_kanji = self.statement_descriptor_suffix_kanji.take()?;

            Some(Self::Out { installments, setup_future_usage, statement_descriptor_suffix_kana, statement_descriptor_suffix_kanji })
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

    impl ObjectDeser for CheckoutCardPaymentMethodOptions {
        type Builder = CheckoutCardPaymentMethodOptionsBuilder;
    }
};
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CheckoutCardPaymentMethodOptionsSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCardPaymentMethodOptionsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutCardPaymentMethodOptionsSetupFutureUsage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutCardPaymentMethodOptionsSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutCardPaymentMethodOptionsSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
