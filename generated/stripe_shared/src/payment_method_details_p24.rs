#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsP24 {
    /// The customer's bank.
    /// Can be one of `ing`, `citi_handlowy`, `tmobile_usbugi_bankowe`, `plus_bank`, `etransfer_pocztowy24`, `banki_spbdzielcze`, `bank_nowy_bfg_sa`, `getin_bank`, `blik`, `noble_pay`, `ideabank`, `envelobank`, `santander_przelew24`, `nest_przelew`, `mbank_mtransfer`, `inteligo`, `pbac_z_ipko`, `bnp_paribas`, `credit_agricole`, `toyota_bank`, `bank_pekao_sa`, `volkswagen_bank`, `bank_millennium`, `alior_bank`, or `boz`.
    pub bank: Option<PaymentMethodDetailsP24Bank>,
    /// Unique reference for this Przelewy24 payment.
    pub reference: Option<String>,
    /// Owner's verified full name. Values are verified or provided by Przelewy24 directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    /// Przelewy24 rarely provides this information so the attribute is usually empty.
    pub verified_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsP24Builder {
    bank: Option<Option<PaymentMethodDetailsP24Bank>>,
    reference: Option<Option<String>>,
    verified_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsP24 {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsP24>,
        builder: PaymentMethodDetailsP24Builder,
    }

    impl Visitor for Place<PaymentMethodDetailsP24> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsP24Builder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsP24Builder {
        type Out = PaymentMethodDetailsP24;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "bank" => Ok(Deserialize::begin(&mut self.bank)),
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "verified_name" => Ok(Deserialize::begin(&mut self.verified_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank: Deserialize::default(), reference: Deserialize::default(), verified_name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank = self.bank.take()?;
            let reference = self.reference.take()?;
            let verified_name = self.verified_name.take()?;

            Some(Self::Out { bank, reference, verified_name })
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

    impl ObjectDeser for PaymentMethodDetailsP24 {
        type Builder = PaymentMethodDetailsP24Builder;
    }
};
/// The customer's bank.
/// Can be one of `ing`, `citi_handlowy`, `tmobile_usbugi_bankowe`, `plus_bank`, `etransfer_pocztowy24`, `banki_spbdzielcze`, `bank_nowy_bfg_sa`, `getin_bank`, `blik`, `noble_pay`, `ideabank`, `envelobank`, `santander_przelew24`, `nest_przelew`, `mbank_mtransfer`, `inteligo`, `pbac_z_ipko`, `bnp_paribas`, `credit_agricole`, `toyota_bank`, `bank_pekao_sa`, `volkswagen_bank`, `bank_millennium`, `alior_bank`, or `boz`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDetailsP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl PaymentMethodDetailsP24Bank {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsP24Bank::*;
        match self {
            AliorBank => "alior_bank",
            BankMillennium => "bank_millennium",
            BankNowyBfgSa => "bank_nowy_bfg_sa",
            BankPekaoSa => "bank_pekao_sa",
            BankiSpbdzielcze => "banki_spbdzielcze",
            Blik => "blik",
            BnpParibas => "bnp_paribas",
            Boz => "boz",
            CitiHandlowy => "citi_handlowy",
            CreditAgricole => "credit_agricole",
            Envelobank => "envelobank",
            EtransferPocztowy24 => "etransfer_pocztowy24",
            GetinBank => "getin_bank",
            Ideabank => "ideabank",
            Ing => "ing",
            Inteligo => "inteligo",
            MbankMtransfer => "mbank_mtransfer",
            NestPrzelew => "nest_przelew",
            NoblePay => "noble_pay",
            PbacZIpko => "pbac_z_ipko",
            PlusBank => "plus_bank",
            SantanderPrzelew24 => "santander_przelew24",
            TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            ToyotaBank => "toyota_bank",
            VolkswagenBank => "volkswagen_bank",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsP24Bank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsP24Bank::*;
        match s {
            "alior_bank" => Ok(AliorBank),
            "bank_millennium" => Ok(BankMillennium),
            "bank_nowy_bfg_sa" => Ok(BankNowyBfgSa),
            "bank_pekao_sa" => Ok(BankPekaoSa),
            "banki_spbdzielcze" => Ok(BankiSpbdzielcze),
            "blik" => Ok(Blik),
            "bnp_paribas" => Ok(BnpParibas),
            "boz" => Ok(Boz),
            "citi_handlowy" => Ok(CitiHandlowy),
            "credit_agricole" => Ok(CreditAgricole),
            "envelobank" => Ok(Envelobank),
            "etransfer_pocztowy24" => Ok(EtransferPocztowy24),
            "getin_bank" => Ok(GetinBank),
            "ideabank" => Ok(Ideabank),
            "ing" => Ok(Ing),
            "inteligo" => Ok(Inteligo),
            "mbank_mtransfer" => Ok(MbankMtransfer),
            "nest_przelew" => Ok(NestPrzelew),
            "noble_pay" => Ok(NoblePay),
            "pbac_z_ipko" => Ok(PbacZIpko),
            "plus_bank" => Ok(PlusBank),
            "santander_przelew24" => Ok(SantanderPrzelew24),
            "tmobile_usbugi_bankowe" => Ok(TmobileUsbugiBankowe),
            "toyota_bank" => Ok(ToyotaBank),
            "volkswagen_bank" => Ok(VolkswagenBank),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodDetailsP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodDetailsP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsP24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsP24Bank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodDetailsP24Bank {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsP24Bank> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsP24Bank::from_str(s).unwrap_or(PaymentMethodDetailsP24Bank::Unknown));
        Ok(())
    }
}
