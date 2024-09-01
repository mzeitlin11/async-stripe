#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsSofort {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    /// Preferred language of the SOFORT authorization page that the customer is redirected to.
    /// Can be one of `de`, `en`, `es`, `fr`, `it`, `nl`, or `pl`
    pub preferred_language: Option<PaymentMethodDetailsSofortPreferredLanguage>,
    /// Owner's verified full name. Values are verified or provided by SOFORT directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub verified_name: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsSofortBuilder {
    bank_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
    country: Option<Option<String>>,
    generated_sepa_debit: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    generated_sepa_debit_mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    iban_last4: Option<Option<String>>,
    preferred_language: Option<Option<PaymentMethodDetailsSofortPreferredLanguage>>,
    verified_name: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsSofort {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsSofort>,
        builder: PaymentMethodDetailsSofortBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsSofort> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsSofortBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsSofortBuilder {
        type Out = PaymentMethodDetailsSofort;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.bank_code),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "bic" => Deserialize::begin(&mut self.bic),
                "country" => Deserialize::begin(&mut self.country),
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
                country: Deserialize::default(),
                generated_sepa_debit: Deserialize::default(),
                generated_sepa_debit_mandate: Deserialize::default(),
                iban_last4: Deserialize::default(),
                preferred_language: Deserialize::default(),
                verified_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(bank_code),
                Some(bank_name),
                Some(bic),
                Some(country),
                Some(generated_sepa_debit),
                Some(generated_sepa_debit_mandate),
                Some(iban_last4),
                Some(preferred_language),
                Some(verified_name),
            ) = (
                self.bank_code.take(),
                self.bank_name.take(),
                self.bic.take(),
                self.country.take(),
                self.generated_sepa_debit.take(),
                self.generated_sepa_debit_mandate.take(),
                self.iban_last4.take(),
                self.preferred_language,
                self.verified_name.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                bank_code,
                bank_name,
                bic,
                country,
                generated_sepa_debit,
                generated_sepa_debit_mandate,
                iban_last4,
                preferred_language,
                verified_name,
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

    impl ObjectDeser for PaymentMethodDetailsSofort {
        type Builder = PaymentMethodDetailsSofortBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsSofort {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsSofortBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_code" => b.bank_code = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "bic" => b.bic = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "generated_sepa_debit" => b.generated_sepa_debit = FromValueOpt::from_value(v),
                    "generated_sepa_debit_mandate" => {
                        b.generated_sepa_debit_mandate = FromValueOpt::from_value(v)
                    }
                    "iban_last4" => b.iban_last4 = FromValueOpt::from_value(v),
                    "preferred_language" => b.preferred_language = FromValueOpt::from_value(v),
                    "verified_name" => b.verified_name = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Preferred language of the SOFORT authorization page that the customer is redirected to.
/// Can be one of `de`, `en`, `es`, `fr`, `it`, `nl`, or `pl`
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}
impl PaymentMethodDetailsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsSofortPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Es => "es",
            Fr => "fr",
            It => "it",
            Nl => "nl",
            Pl => "pl",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsSofortPreferredLanguage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsSofortPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "es" => Ok(Es),
            "fr" => Ok(Fr),
            "it" => Ok(It),
            "nl" => Ok(Nl),
            "pl" => Ok(Pl),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsSofortPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodDetailsSofortPreferredLanguage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsSofortPreferredLanguage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDetailsSofortPreferredLanguage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsSofortPreferredLanguage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsSofortPreferredLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDetailsSofortPreferredLanguage",
            )
        })
    }
}
