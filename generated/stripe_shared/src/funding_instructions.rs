/// Each customer has a [`balance`](https://stripe.com/docs/api/customers/object#customer_object-balance) that is.
/// automatically applied to future invoices and payments using the `customer_balance` payment method.
/// Customers can fund this balance by initiating a bank transfer to any account in the
/// `financial_addresses` field.
/// Related guide: [Customer balance funding instructions](https://stripe.com/docs/payments/customer-balance/funding-instructions).
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FundingInstructions {
    pub bank_transfer: stripe_shared::FundingInstructionsBankTransfer,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The `funding_type` of the returned instructions
    pub funding_type: FundingInstructionsFundingType,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[cfg(feature = "min-ser")]
pub struct FundingInstructionsBuilder {
    bank_transfer: Option<stripe_shared::FundingInstructionsBankTransfer>,
    currency: Option<stripe_types::Currency>,
    funding_type: Option<FundingInstructionsFundingType>,
    livemode: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructions>,
        builder: FundingInstructionsBuilder,
    }

    impl Visitor for Place<FundingInstructions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FundingInstructionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FundingInstructionsBuilder {
        type Out = FundingInstructions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank_transfer" => Ok(Deserialize::begin(&mut self.bank_transfer)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "funding_type" => Ok(Deserialize::begin(&mut self.funding_type)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank_transfer: Deserialize::default(), currency: Deserialize::default(), funding_type: Deserialize::default(), livemode: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_transfer = self.bank_transfer.take()?;
            let currency = self.currency.take()?;
            let funding_type = self.funding_type.take()?;
            let livemode = self.livemode.take()?;

            Some(Self::Out { bank_transfer, currency, funding_type, livemode })
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

    impl ObjectDeser for FundingInstructions {
        type Builder = FundingInstructionsBuilder;
    }
};
/// The `funding_type` of the returned instructions
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsFundingType {
    BankTransfer,
}
impl FundingInstructionsFundingType {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for FundingInstructionsFundingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for FundingInstructionsFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FundingInstructionsFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FundingInstructionsFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for FundingInstructionsFundingType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FundingInstructionsFundingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<FundingInstructionsFundingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FundingInstructionsFundingType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
