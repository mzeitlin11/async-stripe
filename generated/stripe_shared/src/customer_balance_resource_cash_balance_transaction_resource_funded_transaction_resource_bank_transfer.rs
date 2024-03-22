#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
    pub eu_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer>,
    pub gb_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>,
    pub jp_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>,
    /// The user-supplied reference field on the bank transfer.
    pub reference: Option<String>,
    /// The funding method type used to fund the customer balance.
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType,
    pub us_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer>,
}
#[cfg(feature = "min-ser")]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder {
    eu_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer>>,
    gb_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>>,
    jp_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>>,
    reference: Option<Option<String>>,
    type_: Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType>,
    us_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer>,
        builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder,
    }

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "eu_bank_transfer" => Ok(Deserialize::begin(&mut self.eu_bank_transfer)),
                "gb_bank_transfer" => Ok(Deserialize::begin(&mut self.gb_bank_transfer)),
                "jp_bank_transfer" => Ok(Deserialize::begin(&mut self.jp_bank_transfer)),
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "us_bank_transfer" => Ok(Deserialize::begin(&mut self.us_bank_transfer)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                eu_bank_transfer: Deserialize::default(),
                gb_bank_transfer: Deserialize::default(),
                jp_bank_transfer: Deserialize::default(),
                reference: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_transfer: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let eu_bank_transfer = self.eu_bank_transfer.take()?;
            let gb_bank_transfer = self.gb_bank_transfer.take()?;
            let jp_bank_transfer = self.jp_bank_transfer.take()?;
            let reference = self.reference.take()?;
            let type_ = self.type_.take()?;
            let us_bank_transfer = self.us_bank_transfer.take()?;

            Some(Self::Out { eu_bank_transfer, gb_bank_transfer, jp_bank_transfer, reference, type_, us_bank_transfer })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
        type Builder = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder;
    }
};
/// The funding method type used to fund the customer balance.
/// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}
impl CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    pub fn as_str(self) -> &'static str {
        use CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl std::str::FromStr for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            "us_bank_transfer" => Ok(UsBankTransfer),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
