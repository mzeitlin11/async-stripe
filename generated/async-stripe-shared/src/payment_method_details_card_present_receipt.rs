#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardPresentReceipt {
    /// The type of account being debited or credited
    pub account_type: Option<PaymentMethodDetailsCardPresentReceiptAccountType>,
    /// EMV tag 9F26, cryptogram generated by the integrated circuit chip.
    pub application_cryptogram: Option<String>,
    /// Mnenomic of the Application Identifier.
    pub application_preferred_name: Option<String>,
    /// Identifier for this transaction.
    pub authorization_code: Option<String>,
    /// EMV tag 8A. A code returned by the card issuer.
    pub authorization_response_code: Option<String>,
    /// Describes the method used by the cardholder to verify ownership of the card.
    /// One of the following: `approval`, `failure`, `none`, `offline_pin`, `offline_pin_and_signature`, `online_pin`, or `signature`.
    pub cardholder_verification_method: Option<String>,
    /// EMV tag 84. Similar to the application identifier stored on the integrated circuit chip.
    pub dedicated_file_name: Option<String>,
    /// The outcome of a series of EMV functions performed by the card reader.
    pub terminal_verification_results: Option<String>,
    /// An indication of various EMV functions performed during the transaction.
    pub transaction_status_information: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCardPresentReceiptBuilder {
    account_type: Option<Option<PaymentMethodDetailsCardPresentReceiptAccountType>>,
    application_cryptogram: Option<Option<String>>,
    application_preferred_name: Option<Option<String>>,
    authorization_code: Option<Option<String>>,
    authorization_response_code: Option<Option<String>>,
    cardholder_verification_method: Option<Option<String>>,
    dedicated_file_name: Option<Option<String>>,
    terminal_verification_results: Option<Option<String>>,
    transaction_status_information: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsCardPresentReceipt {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardPresentReceipt>,
        builder: PaymentMethodDetailsCardPresentReceiptBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardPresentReceipt> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCardPresentReceiptBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardPresentReceiptBuilder {
        type Out = PaymentMethodDetailsCardPresentReceipt;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_type" => Deserialize::begin(&mut self.account_type),
                "application_cryptogram" => Deserialize::begin(&mut self.application_cryptogram),
                "application_preferred_name" => {
                    Deserialize::begin(&mut self.application_preferred_name)
                }
                "authorization_code" => Deserialize::begin(&mut self.authorization_code),
                "authorization_response_code" => {
                    Deserialize::begin(&mut self.authorization_response_code)
                }
                "cardholder_verification_method" => {
                    Deserialize::begin(&mut self.cardholder_verification_method)
                }
                "dedicated_file_name" => Deserialize::begin(&mut self.dedicated_file_name),
                "terminal_verification_results" => {
                    Deserialize::begin(&mut self.terminal_verification_results)
                }
                "transaction_status_information" => {
                    Deserialize::begin(&mut self.transaction_status_information)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_type: Deserialize::default(),
                application_cryptogram: Deserialize::default(),
                application_preferred_name: Deserialize::default(),
                authorization_code: Deserialize::default(),
                authorization_response_code: Deserialize::default(),
                cardholder_verification_method: Deserialize::default(),
                dedicated_file_name: Deserialize::default(),
                terminal_verification_results: Deserialize::default(),
                transaction_status_information: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_type),
                Some(application_cryptogram),
                Some(application_preferred_name),
                Some(authorization_code),
                Some(authorization_response_code),
                Some(cardholder_verification_method),
                Some(dedicated_file_name),
                Some(terminal_verification_results),
                Some(transaction_status_information),
            ) = (
                self.account_type,
                self.application_cryptogram.take(),
                self.application_preferred_name.take(),
                self.authorization_code.take(),
                self.authorization_response_code.take(),
                self.cardholder_verification_method.take(),
                self.dedicated_file_name.take(),
                self.terminal_verification_results.take(),
                self.transaction_status_information.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_type,
                application_cryptogram,
                application_preferred_name,
                authorization_code,
                authorization_response_code,
                cardholder_verification_method,
                dedicated_file_name,
                terminal_verification_results,
                transaction_status_information,
            })
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

    impl ObjectDeser for PaymentMethodDetailsCardPresentReceipt {
        type Builder = PaymentMethodDetailsCardPresentReceiptBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsCardPresentReceipt {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsCardPresentReceiptBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_type" => b.account_type = FromValueOpt::from_value(v),
                    "application_cryptogram" => {
                        b.application_cryptogram = FromValueOpt::from_value(v)
                    }
                    "application_preferred_name" => {
                        b.application_preferred_name = FromValueOpt::from_value(v)
                    }
                    "authorization_code" => b.authorization_code = FromValueOpt::from_value(v),
                    "authorization_response_code" => {
                        b.authorization_response_code = FromValueOpt::from_value(v)
                    }
                    "cardholder_verification_method" => {
                        b.cardholder_verification_method = FromValueOpt::from_value(v)
                    }
                    "dedicated_file_name" => b.dedicated_file_name = FromValueOpt::from_value(v),
                    "terminal_verification_results" => {
                        b.terminal_verification_results = FromValueOpt::from_value(v)
                    }
                    "transaction_status_information" => {
                        b.transaction_status_information = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of account being debited or credited
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCardPresentReceiptAccountType {
    Checking,
    Credit,
    Prepaid,
    Unknown,
}
impl PaymentMethodDetailsCardPresentReceiptAccountType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCardPresentReceiptAccountType::*;
        match self {
            Checking => "checking",
            Credit => "credit",
            Prepaid => "prepaid",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardPresentReceiptAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardPresentReceiptAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "credit" => Ok(Credit),
            "prepaid" => Ok(Prepaid),
            "unknown" => Ok(Unknown),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCardPresentReceiptAccountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDetailsCardPresentReceiptAccountType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsCardPresentReceiptAccountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDetailsCardPresentReceiptAccountType",
            )
        })
    }
}
