#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OutboundPaymentsPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// The US bank account network used to send funds.
    pub network: OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder {
    account_holder_type: Option<Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType>>,
    account_type: Option<Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType>>,
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    network: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork>,
    routing_number: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for OutboundPaymentsPaymentMethodDetailsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
        builder: OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder,
    }

    impl Visitor for Place<OutboundPaymentsPaymentMethodDetailsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder {
        type Out = OutboundPaymentsPaymentMethodDetailsUsBankAccount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account_holder_type" => Ok(Deserialize::begin(&mut self.account_holder_type)),
                "account_type" => Ok(Deserialize::begin(&mut self.account_type)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "network" => Ok(Deserialize::begin(&mut self.network)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account_holder_type: Deserialize::default(),
                account_type: Deserialize::default(),
                bank_name: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                network: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_holder_type = self.account_holder_type.take()?;
            let account_type = self.account_type.take()?;
            let bank_name = self.bank_name.take()?;
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;
            let network = self.network.take()?;
            let routing_number = self.routing_number.take()?;

            Some(Self::Out { account_holder_type, account_type, bank_name, fingerprint, last4, network, routing_number })
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

    impl ObjectDeser for OutboundPaymentsPaymentMethodDetailsUsBankAccount {
        type Builder = OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder;
    }
};
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}
impl OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The US bank account network used to send funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}
impl OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
