#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsIdeal {
    /// The customer's bank.
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `n26`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
    pub bank: Option<PaymentMethodDetailsIdealBank>,
    /// The Bank Identifier Code of the customer's bank.
    pub bic: Option<PaymentMethodDetailsIdealBic>,
    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    /// Owner's verified full name. Values are verified or provided by iDEAL directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub verified_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsIdealBuilder {
    bank: Option<Option<PaymentMethodDetailsIdealBank>>,
    bic: Option<Option<PaymentMethodDetailsIdealBic>>,
    generated_sepa_debit: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    generated_sepa_debit_mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    iban_last4: Option<Option<String>>,
    verified_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsIdeal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsIdeal>,
        builder: PaymentMethodDetailsIdealBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsIdeal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsIdealBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsIdealBuilder {
        type Out = PaymentMethodDetailsIdeal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank" => Ok(Deserialize::begin(&mut self.bank)),
                "bic" => Ok(Deserialize::begin(&mut self.bic)),
                "generated_sepa_debit" => Ok(Deserialize::begin(&mut self.generated_sepa_debit)),
                "generated_sepa_debit_mandate" => Ok(Deserialize::begin(&mut self.generated_sepa_debit_mandate)),
                "iban_last4" => Ok(Deserialize::begin(&mut self.iban_last4)),
                "verified_name" => Ok(Deserialize::begin(&mut self.verified_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                bank: Deserialize::default(),
                bic: Deserialize::default(),
                generated_sepa_debit: Deserialize::default(),
                generated_sepa_debit_mandate: Deserialize::default(),
                iban_last4: Deserialize::default(),
                verified_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank = self.bank.take()?;
            let bic = self.bic.take()?;
            let generated_sepa_debit = self.generated_sepa_debit.take()?;
            let generated_sepa_debit_mandate = self.generated_sepa_debit_mandate.take()?;
            let iban_last4 = self.iban_last4.take()?;
            let verified_name = self.verified_name.take()?;

            Some(Self::Out { bank, bic, generated_sepa_debit, generated_sepa_debit_mandate, iban_last4, verified_name })
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

    impl ObjectDeser for PaymentMethodDetailsIdeal {
        type Builder = PaymentMethodDetailsIdealBuilder;
    }
};
/// The customer's bank.
/// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `n26`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDetailsIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl PaymentMethodDetailsIdealBank {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsIdealBank::*;
        match self {
            AbnAmro => "abn_amro",
            AsnBank => "asn_bank",
            Bunq => "bunq",
            Handelsbanken => "handelsbanken",
            Ing => "ing",
            Knab => "knab",
            Moneyou => "moneyou",
            N26 => "n26",
            Rabobank => "rabobank",
            Regiobank => "regiobank",
            Revolut => "revolut",
            SnsBank => "sns_bank",
            TriodosBank => "triodos_bank",
            VanLanschot => "van_lanschot",
            Yoursafe => "yoursafe",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsIdealBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsIdealBank::*;
        match s {
            "abn_amro" => Ok(AbnAmro),
            "asn_bank" => Ok(AsnBank),
            "bunq" => Ok(Bunq),
            "handelsbanken" => Ok(Handelsbanken),
            "ing" => Ok(Ing),
            "knab" => Ok(Knab),
            "moneyou" => Ok(Moneyou),
            "n26" => Ok(N26),
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
impl AsRef<str> for PaymentMethodDetailsIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodDetailsIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsIdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodDetailsIdealBank {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsIdealBank> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsIdealBank::from_str(s).unwrap_or(PaymentMethodDetailsIdealBank::Unknown));
        Ok(())
    }
}
/// The Bank Identifier Code of the customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDetailsIdealBic {
    Abnanl2a,
    Asnbnl21,
    Bitsnl2a,
    Bunqnl2a,
    Fvlbnl22,
    Handnl2a,
    Ingbnl2a,
    Knabnl2h,
    Moyonl21,
    Ntsbdeb1,
    Rabonl2u,
    Rbrbnl21,
    Revoie23,
    Revolt21,
    Snsbnl2a,
    Trionl2u,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl PaymentMethodDetailsIdealBic {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsIdealBic::*;
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
            Ntsbdeb1 => "NTSBDEB1",
            Rabonl2u => "RABONL2U",
            Rbrbnl21 => "RBRBNL21",
            Revoie23 => "REVOIE23",
            Revolt21 => "REVOLT21",
            Snsbnl2a => "SNSBNL2A",
            Trionl2u => "TRIONL2U",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsIdealBic {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsIdealBic::*;
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
            "NTSBDEB1" => Ok(Ntsbdeb1),
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
impl AsRef<str> for PaymentMethodDetailsIdealBic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodDetailsIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsIdealBic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsIdealBic {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodDetailsIdealBic {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsIdealBic> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsIdealBic::from_str(s).unwrap_or(PaymentMethodDetailsIdealBic::Unknown));
        Ok(())
    }
}
