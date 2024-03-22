#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransfer {
    /// The country of the bank account to fund
    pub country: String,
    /// A list of financial addresses that can be used to fund a particular balance
    pub financial_addresses: Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>,
    /// The bank_transfer type
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: FundingInstructionsBankTransferType,
}
#[cfg(feature = "min-ser")]
pub struct FundingInstructionsBankTransferBuilder {
    country: Option<String>,
    financial_addresses: Option<Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>>,
    type_: Option<FundingInstructionsBankTransferType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransfer>,
        builder: FundingInstructionsBankTransferBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FundingInstructionsBankTransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferBuilder {
        type Out = FundingInstructionsBankTransfer;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "financial_addresses" => Ok(Deserialize::begin(&mut self.financial_addresses)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { country: Deserialize::default(), financial_addresses: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let country = self.country.take()?;
            let financial_addresses = self.financial_addresses.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { country, financial_addresses, type_ })
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

    impl ObjectDeser for FundingInstructionsBankTransfer {
        type Builder = FundingInstructionsBankTransferBuilder;
    }
};
/// The bank_transfer type
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsBankTransferType {
    EuBankTransfer,
    JpBankTransfer,
}
impl FundingInstructionsBankTransferType {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for FundingInstructionsBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for FundingInstructionsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FundingInstructionsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FundingInstructionsBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FundingInstructionsBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for FundingInstructionsBankTransferType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FundingInstructionsBankTransferType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<FundingInstructionsBankTransferType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FundingInstructionsBankTransferType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
