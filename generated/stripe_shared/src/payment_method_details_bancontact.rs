#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsBancontact {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    /// Can be one of `en`, `de`, `fr`, or `nl`
    pub preferred_language: Option<PaymentMethodDetailsBancontactPreferredLanguage>,
    /// Owner's verified full name. Values are verified or provided by Bancontact directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub verified_name: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsBancontactBuilder {
    bank_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
    generated_sepa_debit: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    generated_sepa_debit_mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    iban_last4: Option<Option<String>>,
    preferred_language: Option<Option<PaymentMethodDetailsBancontactPreferredLanguage>>,
    verified_name: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsBancontact {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsBancontact>,
        builder: PaymentMethodDetailsBancontactBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsBancontact> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsBancontactBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsBancontactBuilder {
        type Out = PaymentMethodDetailsBancontact;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.bank_code),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "bic" => Deserialize::begin(&mut self.bic),
                "generated_sepa_debit" => Deserialize::begin(&mut self.generated_sepa_debit),
                "generated_sepa_debit_mandate" => {
                    Deserialize::begin(&mut self.generated_sepa_debit_mandate)
                }
                "iban_last4" => Deserialize::begin(&mut self.iban_last4),
                "preferred_language" => Deserialize::begin(&mut self.preferred_language),
                "verified_name" => Deserialize::begin(&mut self.verified_name),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                bic: Deserialize::default(),
                generated_sepa_debit: Deserialize::default(),
                generated_sepa_debit_mandate: Deserialize::default(),
                iban_last4: Deserialize::default(),
                preferred_language: Deserialize::default(),
                verified_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                bank_code: self.bank_code.take()?,
                bank_name: self.bank_name.take()?,
                bic: self.bic.take()?,
                generated_sepa_debit: self.generated_sepa_debit.take()?,
                generated_sepa_debit_mandate: self.generated_sepa_debit_mandate.take()?,
                iban_last4: self.iban_last4.take()?,
                preferred_language: self.preferred_language?,
                verified_name: self.verified_name.take()?,
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

    impl ObjectDeser for PaymentMethodDetailsBancontact {
        type Builder = PaymentMethodDetailsBancontactBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsBancontact {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsBancontactBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_code" => b.bank_code = Some(FromValueOpt::from_value(v)?),
                    "bank_name" => b.bank_name = Some(FromValueOpt::from_value(v)?),
                    "bic" => b.bic = Some(FromValueOpt::from_value(v)?),
                    "generated_sepa_debit" => {
                        b.generated_sepa_debit = Some(FromValueOpt::from_value(v)?)
                    }
                    "generated_sepa_debit_mandate" => {
                        b.generated_sepa_debit_mandate = Some(FromValueOpt::from_value(v)?)
                    }
                    "iban_last4" => b.iban_last4 = Some(FromValueOpt::from_value(v)?),
                    "preferred_language" => {
                        b.preferred_language = Some(FromValueOpt::from_value(v)?)
                    }
                    "verified_name" => b.verified_name = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
/// Can be one of `en`, `de`, `fr`, or `nl`
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}
impl PaymentMethodDetailsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsBancontactPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Fr => "fr",
            Nl => "nl",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsBancontactPreferredLanguage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsBancontactPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsBancontactPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodDetailsBancontactPreferredLanguage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsBancontactPreferredLanguage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDetailsBancontactPreferredLanguage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsBancontactPreferredLanguage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsBancontactPreferredLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDetailsBancontactPreferredLanguage",
            )
        })
    }
}
