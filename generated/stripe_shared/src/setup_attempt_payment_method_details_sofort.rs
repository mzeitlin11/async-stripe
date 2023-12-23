#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsSofort {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    /// Preferred language of the Sofort authorization page that the customer is redirected to.
    /// Can be one of `en`, `de`, `fr`, or `nl`
    pub preferred_language: Option<SetupAttemptPaymentMethodDetailsSofortPreferredLanguage>,
    /// Owner's verified full name. Values are verified or provided by Sofort directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub verified_name: Option<String>,
}
/// Preferred language of the Sofort authorization page that the customer is redirected to.
/// Can be one of `en`, `de`, `fr`, or `nl`
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}
impl SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use SetupAttemptPaymentMethodDetailsSofortPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Fr => "fr",
            Nl => "nl",
        }
    }
}

impl std::str::FromStr for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupAttemptPaymentMethodDetailsSofortPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage",
            )
        })
    }
}
