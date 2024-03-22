#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionDisplayBankTransferInstructions {
    /// The remaining amount that needs to be transferred to complete the payment.
    pub amount_remaining: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// A list of financial addresses that can be used to fund the customer balance
    pub financial_addresses: Option<Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>>,
    /// A link to a hosted page that guides your customer through completing the transfer.
    pub hosted_instructions_url: Option<String>,
    /// A string identifying this payment.
    /// Instruct your customer to include this code in the reference or memo field of their bank transfer.
    pub reference: Option<String>,
    /// Type of bank transfer
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PaymentIntentNextActionDisplayBankTransferInstructionsType,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionDisplayBankTransferInstructionsBuilder {
    amount_remaining: Option<Option<i64>>,
    currency: Option<Option<stripe_types::Currency>>,
    financial_addresses: Option<Option<Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>>>,
    hosted_instructions_url: Option<Option<String>>,
    reference: Option<Option<String>>,
    type_: Option<PaymentIntentNextActionDisplayBankTransferInstructionsType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionDisplayBankTransferInstructions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionDisplayBankTransferInstructions>,
        builder: PaymentIntentNextActionDisplayBankTransferInstructionsBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionDisplayBankTransferInstructions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionDisplayBankTransferInstructionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionDisplayBankTransferInstructionsBuilder {
        type Out = PaymentIntentNextActionDisplayBankTransferInstructions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_remaining" => Ok(Deserialize::begin(&mut self.amount_remaining)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "financial_addresses" => Ok(Deserialize::begin(&mut self.financial_addresses)),
                "hosted_instructions_url" => Ok(Deserialize::begin(&mut self.hosted_instructions_url)),
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount_remaining: Deserialize::default(),
                currency: Deserialize::default(),
                financial_addresses: Deserialize::default(),
                hosted_instructions_url: Deserialize::default(),
                reference: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_remaining = self.amount_remaining.take()?;
            let currency = self.currency.take()?;
            let financial_addresses = self.financial_addresses.take()?;
            let hosted_instructions_url = self.hosted_instructions_url.take()?;
            let reference = self.reference.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { amount_remaining, currency, financial_addresses, hosted_instructions_url, reference, type_ })
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

    impl ObjectDeser for PaymentIntentNextActionDisplayBankTransferInstructions {
        type Builder = PaymentIntentNextActionDisplayBankTransferInstructionsBuilder;
    }
};
/// Type of bank transfer
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentNextActionDisplayBankTransferInstructionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}
impl PaymentIntentNextActionDisplayBankTransferInstructionsType {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentNextActionDisplayBankTransferInstructionsType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl std::str::FromStr for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentNextActionDisplayBankTransferInstructionsType::*;
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
impl AsRef<str> for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentNextActionDisplayBankTransferInstructionsType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentNextActionDisplayBankTransferInstructionsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentNextActionDisplayBankTransferInstructionsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
