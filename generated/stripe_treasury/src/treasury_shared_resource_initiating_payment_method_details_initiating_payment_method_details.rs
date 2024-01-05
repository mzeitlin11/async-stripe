#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    /// Set when `type` is `balance`.
    pub balance: Option<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance>,
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    pub financial_account: Option<stripe_treasury::ReceivedPaymentMethodDetailsFinancialAccount>,
    /// Set when `type` is `issuing_card`.
    /// This is an [Issuing Card](https://stripe.com/docs/api#issuing_cards) ID.
    pub issuing_card: Option<String>,
    /// Polymorphic type matching the originating money movement's source.
    /// This can be an external account, a Stripe balance, or a FinancialAccount.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType,
    pub us_bank_account: Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "min-ser")]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBuilder {
    balance: Option<Option<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance>>,
    billing_details: Option<stripe_treasury::TreasurySharedResourceBillingDetails>,
    financial_account: Option<Option<stripe_treasury::ReceivedPaymentMethodDetailsFinancialAccount>>,
    issuing_card: Option<Option<String>>,
    type_: Option<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType>,
    us_bank_account: Option<Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>,
        builder: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBuilder {
        type Out = TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "balance" => Ok(Deserialize::begin(&mut self.balance)),
                "billing_details" => Ok(Deserialize::begin(&mut self.billing_details)),
                "financial_account" => Ok(Deserialize::begin(&mut self.financial_account)),
                "issuing_card" => Ok(Deserialize::begin(&mut self.issuing_card)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                balance: Deserialize::default(),
                billing_details: Deserialize::default(),
                financial_account: Deserialize::default(),
                issuing_card: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let balance = self.balance.take()?;
            let billing_details = self.billing_details.take()?;
            let financial_account = self.financial_account.take()?;
            let issuing_card = self.issuing_card.take()?;
            let type_ = self.type_.take()?;
            let us_bank_account = self.us_bank_account.take()?;

            Some(Self::Out { balance, billing_details, financial_account, issuing_card, type_, us_bank_account })
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

    impl ObjectDeser for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
        type Builder = TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBuilder;
    }
};
/// Set when `type` is `balance`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    Payments,
}
impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    pub fn as_str(self) -> &'static str {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::*;
        match self {
            Payments => "payments",
        }
    }
}

impl std::str::FromStr for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::*;
        match s {
            "payments" => Ok(Payments),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Polymorphic type matching the originating money movement's source.
/// This can be an external account, a Stripe balance, or a FinancialAccount.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
}
impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::*;
        match self {
            Balance => "balance",
            FinancialAccount => "financial_account",
            IssuingCard => "issuing_card",
            Stripe => "stripe",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::*;
        match s {
            "balance" => Ok(Balance),
            "financial_account" => Ok(FinancialAccount),
            "issuing_card" => Ok(IssuingCard),
            "stripe" => Ok(Stripe),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
