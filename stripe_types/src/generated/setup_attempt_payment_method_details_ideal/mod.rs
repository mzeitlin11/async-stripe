#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsIdeal {
    /// The customer's bank.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
    pub bank: Option<SetupAttemptPaymentMethodDetailsIdealBank>,
    /// The Bank Identifier Code of the customer's bank.
    pub bic: Option<SetupAttemptPaymentMethodDetailsIdealBic>,
    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit: Option<stripe_types::Expandable<stripe_types::PaymentMethod>>,
    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate: Option<stripe_types::Expandable<stripe_types::Mandate>>,
    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    /// Owner's verified full name.
    ///
    /// Values are verified or provided by iDEAL directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}
/// The customer's bank.
///
/// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupAttemptPaymentMethodDetailsIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
}

impl SetupAttemptPaymentMethodDetailsIdealBank {
    pub fn as_str(self) -> &'static str {
        use SetupAttemptPaymentMethodDetailsIdealBank::*;
        match self {
            AbnAmro => "abn_amro",
            AsnBank => "asn_bank",
            Bunq => "bunq",
            Handelsbanken => "handelsbanken",
            Ing => "ing",
            Knab => "knab",
            Moneyou => "moneyou",
            Rabobank => "rabobank",
            Regiobank => "regiobank",
            Revolut => "revolut",
            SnsBank => "sns_bank",
            TriodosBank => "triodos_bank",
            VanLanschot => "van_lanschot",
            Yoursafe => "yoursafe",
        }
    }
}

impl std::str::FromStr for SetupAttemptPaymentMethodDetailsIdealBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupAttemptPaymentMethodDetailsIdealBank::*;
        match s {
            "abn_amro" => Ok(AbnAmro),
            "asn_bank" => Ok(AsnBank),
            "bunq" => Ok(Bunq),
            "handelsbanken" => Ok(Handelsbanken),
            "ing" => Ok(Ing),
            "knab" => Ok(Knab),
            "moneyou" => Ok(Moneyou),
            "rabobank" => Ok(Rabobank),
            "regiobank" => Ok(Regiobank),
            "revolut" => Ok(Revolut),
            "sns_bank" => Ok(SnsBank),
            "triodos_bank" => Ok(TriodosBank),
            "van_lanschot" => Ok(VanLanschot),
            "yoursafe" => Ok(Yoursafe),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupAttemptPaymentMethodDetailsIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupAttemptPaymentMethodDetailsIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupAttemptPaymentMethodDetailsIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupAttemptPaymentMethodDetailsIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupAttemptPaymentMethodDetailsIdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SetupAttemptPaymentMethodDetailsIdealBank"))
    }
}
/// The Bank Identifier Code of the customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupAttemptPaymentMethodDetailsIdealBic {
    Abnanl2a,
    Asnbnl21,
    Bitsnl2a,
    Bunqnl2a,
    Fvlbnl22,
    Handnl2a,
    Ingbnl2a,
    Knabnl2h,
    Moyonl21,
    Rabonl2u,
    Rbrbnl21,
    Revoie23,
    Revolt21,
    Snsbnl2a,
    Trionl2u,
}

impl SetupAttemptPaymentMethodDetailsIdealBic {
    pub fn as_str(self) -> &'static str {
        use SetupAttemptPaymentMethodDetailsIdealBic::*;
        match self {
            Abnanl2a => "ABNANL2A",
            Asnbnl21 => "ASNBNL21",
            Bitsnl2a => "BITSNL2A",
            Bunqnl2a => "BUNQNL2A",
            Fvlbnl22 => "FVLBNL22",
            Handnl2a => "HANDNL2A",
            Ingbnl2a => "INGBNL2A",
            Knabnl2h => "KNABNL2H",
            Moyonl21 => "MOYONL21",
            Rabonl2u => "RABONL2U",
            Rbrbnl21 => "RBRBNL21",
            Revoie23 => "REVOIE23",
            Revolt21 => "REVOLT21",
            Snsbnl2a => "SNSBNL2A",
            Trionl2u => "TRIONL2U",
        }
    }
}

impl std::str::FromStr for SetupAttemptPaymentMethodDetailsIdealBic {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupAttemptPaymentMethodDetailsIdealBic::*;
        match s {
            "ABNANL2A" => Ok(Abnanl2a),
            "ASNBNL21" => Ok(Asnbnl21),
            "BITSNL2A" => Ok(Bitsnl2a),
            "BUNQNL2A" => Ok(Bunqnl2a),
            "FVLBNL22" => Ok(Fvlbnl22),
            "HANDNL2A" => Ok(Handnl2a),
            "INGBNL2A" => Ok(Ingbnl2a),
            "KNABNL2H" => Ok(Knabnl2h),
            "MOYONL21" => Ok(Moyonl21),
            "RABONL2U" => Ok(Rabonl2u),
            "RBRBNL21" => Ok(Rbrbnl21),
            "REVOIE23" => Ok(Revoie23),
            "REVOLT21" => Ok(Revolt21),
            "SNSBNL2A" => Ok(Snsbnl2a),
            "TRIONL2U" => Ok(Trionl2u),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupAttemptPaymentMethodDetailsIdealBic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupAttemptPaymentMethodDetailsIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupAttemptPaymentMethodDetailsIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupAttemptPaymentMethodDetailsIdealBic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupAttemptPaymentMethodDetailsIdealBic {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SetupAttemptPaymentMethodDetailsIdealBic"))
    }
}
