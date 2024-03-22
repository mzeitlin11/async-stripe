#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OutboundPaymentsPaymentMethodDetails {
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    pub financial_account: Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccount>,
    /// The type of the payment method used in the OutboundPayment.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: OutboundPaymentsPaymentMethodDetailsType,
    pub us_bank_account: Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "min-ser")]
pub struct OutboundPaymentsPaymentMethodDetailsBuilder {
    billing_details: Option<stripe_treasury::TreasurySharedResourceBillingDetails>,
    financial_account: Option<Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccount>>,
    type_: Option<OutboundPaymentsPaymentMethodDetailsType>,
    us_bank_account: Option<Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccount>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for OutboundPaymentsPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<OutboundPaymentsPaymentMethodDetails>,
        builder: OutboundPaymentsPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<OutboundPaymentsPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: OutboundPaymentsPaymentMethodDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for OutboundPaymentsPaymentMethodDetailsBuilder {
        type Out = OutboundPaymentsPaymentMethodDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "billing_details" => Ok(Deserialize::begin(&mut self.billing_details)),
                "financial_account" => Ok(Deserialize::begin(&mut self.financial_account)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { billing_details: Deserialize::default(), financial_account: Deserialize::default(), type_: Deserialize::default(), us_bank_account: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let billing_details = self.billing_details.take()?;
            let financial_account = self.financial_account.take()?;
            let type_ = self.type_.take()?;
            let us_bank_account = self.us_bank_account.take()?;

            Some(Self::Out { billing_details, financial_account, type_, us_bank_account })
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

    impl ObjectDeser for OutboundPaymentsPaymentMethodDetails {
        type Builder = OutboundPaymentsPaymentMethodDetailsBuilder;
    }
};
/// The type of the payment method used in the OutboundPayment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
}
impl OutboundPaymentsPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsType::*;
        match self {
            FinancialAccount => "financial_account",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundPaymentsPaymentMethodDetailsType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<OutboundPaymentsPaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(OutboundPaymentsPaymentMethodDetailsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
