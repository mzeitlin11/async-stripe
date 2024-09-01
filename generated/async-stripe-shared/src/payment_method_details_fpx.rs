#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsFpx {
    /// Account holder type, if provided. Can be one of `individual` or `company`.
    pub account_holder_type: Option<PaymentMethodDetailsFpxAccountHolderType>,
    /// The customer's bank.
    /// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
    pub bank: PaymentMethodDetailsFpxBank,
    /// Unique transaction id generated by FPX for every request from the merchant
    pub transaction_id: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsFpxBuilder {
    account_holder_type: Option<Option<PaymentMethodDetailsFpxAccountHolderType>>,
    bank: Option<PaymentMethodDetailsFpxBank>,
    transaction_id: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsFpx {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsFpx>,
        builder: PaymentMethodDetailsFpxBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsFpx> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsFpxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsFpxBuilder {
        type Out = PaymentMethodDetailsFpx;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => Deserialize::begin(&mut self.account_holder_type),
                "bank" => Deserialize::begin(&mut self.bank),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_type: Deserialize::default(),
                bank: Deserialize::default(),
                transaction_id: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account_holder_type), Some(bank), Some(transaction_id)) =
                (self.account_holder_type, self.bank, self.transaction_id.take())
            else {
                return None;
            };
            Some(Self::Out { account_holder_type, bank, transaction_id })
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

    impl ObjectDeser for PaymentMethodDetailsFpx {
        type Builder = PaymentMethodDetailsFpxBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsFpx {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsFpxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_type" => b.account_holder_type = FromValueOpt::from_value(v),
                    "bank" => b.bank = FromValueOpt::from_value(v),
                    "transaction_id" => b.transaction_id = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Account holder type, if provided. Can be one of `individual` or `company`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsFpxAccountHolderType {
    Company,
    Individual,
}
impl PaymentMethodDetailsFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsFpxAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsFpxAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsFpxAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsFpxAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodDetailsFpxAccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsFpxAccountHolderType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDetailsFpxAccountHolderType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsFpxAccountHolderType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsFpxAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodDetailsFpxAccountHolderType")
        })
    }
}
/// The customer's bank.
/// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDetailsFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl PaymentMethodDetailsFpxBank {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsFpxBank::*;
        match self {
            AffinBank => "affin_bank",
            Agrobank => "agrobank",
            AllianceBank => "alliance_bank",
            Ambank => "ambank",
            BankIslam => "bank_islam",
            BankMuamalat => "bank_muamalat",
            BankOfChina => "bank_of_china",
            BankRakyat => "bank_rakyat",
            Bsn => "bsn",
            Cimb => "cimb",
            DeutscheBank => "deutsche_bank",
            HongLeongBank => "hong_leong_bank",
            Hsbc => "hsbc",
            Kfh => "kfh",
            Maybank2e => "maybank2e",
            Maybank2u => "maybank2u",
            Ocbc => "ocbc",
            PbEnterprise => "pb_enterprise",
            PublicBank => "public_bank",
            Rhb => "rhb",
            StandardChartered => "standard_chartered",
            Uob => "uob",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsFpxBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsFpxBank::*;
        match s {
            "affin_bank" => Ok(AffinBank),
            "agrobank" => Ok(Agrobank),
            "alliance_bank" => Ok(AllianceBank),
            "ambank" => Ok(Ambank),
            "bank_islam" => Ok(BankIslam),
            "bank_muamalat" => Ok(BankMuamalat),
            "bank_of_china" => Ok(BankOfChina),
            "bank_rakyat" => Ok(BankRakyat),
            "bsn" => Ok(Bsn),
            "cimb" => Ok(Cimb),
            "deutsche_bank" => Ok(DeutscheBank),
            "hong_leong_bank" => Ok(HongLeongBank),
            "hsbc" => Ok(Hsbc),
            "kfh" => Ok(Kfh),
            "maybank2e" => Ok(Maybank2e),
            "maybank2u" => Ok(Maybank2u),
            "ocbc" => Ok(Ocbc),
            "pb_enterprise" => Ok(PbEnterprise),
            "public_bank" => Ok(PublicBank),
            "rhb" => Ok(Rhb),
            "standard_chartered" => Ok(StandardChartered),
            "uob" => Ok(Uob),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodDetailsFpxBank {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsFpxBank> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsFpxBank::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsFpxBank);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsFpxBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
