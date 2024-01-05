#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<PaymentMethodUsBankAccountAccountHolderType>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<PaymentMethodUsBankAccountAccountType>,
    /// The name of the bank.
    pub bank_name: Option<String>,
    /// The ID of the Financial Connections Account used to create the payment method.
    pub financial_connections_account: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// Contains information about US bank account networks that can be used.
    pub networks: Option<stripe_shared::UsBankAccountNetworks>,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
    /// Contains information about the future reusability of this PaymentMethod.
    pub status_details: Option<stripe_shared::PaymentMethodUsBankAccountStatusDetails>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodUsBankAccountBuilder {
    account_holder_type: Option<Option<PaymentMethodUsBankAccountAccountHolderType>>,
    account_type: Option<Option<PaymentMethodUsBankAccountAccountType>>,
    bank_name: Option<Option<String>>,
    financial_connections_account: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    networks: Option<Option<stripe_shared::UsBankAccountNetworks>>,
    routing_number: Option<Option<String>>,
    status_details: Option<Option<stripe_shared::PaymentMethodUsBankAccountStatusDetails>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodUsBankAccount>,
        builder: PaymentMethodUsBankAccountBuilder,
    }

    impl Visitor for Place<PaymentMethodUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodUsBankAccountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodUsBankAccountBuilder {
        type Out = PaymentMethodUsBankAccount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "account_holder_type" => Ok(Deserialize::begin(&mut self.account_holder_type)),
                "account_type" => Ok(Deserialize::begin(&mut self.account_type)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "financial_connections_account" => Ok(Deserialize::begin(&mut self.financial_connections_account)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "networks" => Ok(Deserialize::begin(&mut self.networks)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),
                "status_details" => Ok(Deserialize::begin(&mut self.status_details)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account_holder_type: Deserialize::default(),
                account_type: Deserialize::default(),
                bank_name: Deserialize::default(),
                financial_connections_account: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                networks: Deserialize::default(),
                routing_number: Deserialize::default(),
                status_details: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_holder_type = self.account_holder_type.take()?;
            let account_type = self.account_type.take()?;
            let bank_name = self.bank_name.take()?;
            let financial_connections_account = self.financial_connections_account.take()?;
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;
            let networks = self.networks.take()?;
            let routing_number = self.routing_number.take()?;
            let status_details = self.status_details.take()?;

            Some(Self::Out { account_holder_type, account_type, bank_name, financial_connections_account, fingerprint, last4, networks, routing_number, status_details })
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

    impl ObjectDeser for PaymentMethodUsBankAccount {
        type Builder = PaymentMethodUsBankAccountBuilder;
    }
};
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl PaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for PaymentMethodUsBankAccountAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodUsBankAccountAccountHolderType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodUsBankAccountAccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodUsBankAccountAccountHolderType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodUsBankAccountAccountHolderType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}
impl PaymentMethodUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for PaymentMethodUsBankAccountAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodUsBankAccountAccountType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodUsBankAccountAccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodUsBankAccountAccountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodUsBankAccountAccountType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
