#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankConnectionsResourceBalance {
    /// The time that the external institution calculated this balance.
    /// Measured in seconds since the Unix epoch.
    pub as_of: stripe_types::Timestamp,
    pub cash: Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCashBalance>,
    pub credit: Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCreditBalance>,
    /// The balances owed to (or by) the account holder.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub current: std::collections::HashMap<String, i64>,
    /// The `type` of the balance.
    /// An additional hash is included on the balance with a name matching this value.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: BankConnectionsResourceBalanceType,
}
#[cfg(feature = "min-ser")]
pub struct BankConnectionsResourceBalanceBuilder {
    as_of: Option<stripe_types::Timestamp>,
    cash: Option<Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCashBalance>>,
    credit: Option<Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCreditBalance>>,
    current: Option<std::collections::HashMap<String, i64>>,
    type_: Option<BankConnectionsResourceBalanceType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BankConnectionsResourceBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceBalance>,
        builder: BankConnectionsResourceBalanceBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: BankConnectionsResourceBalanceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BankConnectionsResourceBalanceBuilder {
        type Out = BankConnectionsResourceBalance;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "as_of" => Ok(Deserialize::begin(&mut self.as_of)),
                "cash" => Ok(Deserialize::begin(&mut self.cash)),
                "credit" => Ok(Deserialize::begin(&mut self.credit)),
                "current" => Ok(Deserialize::begin(&mut self.current)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { as_of: Deserialize::default(), cash: Deserialize::default(), credit: Deserialize::default(), current: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let as_of = self.as_of.take()?;
            let cash = self.cash.take()?;
            let credit = self.credit.take()?;
            let current = self.current.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { as_of, cash, credit, current, type_ })
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

    impl ObjectDeser for BankConnectionsResourceBalance {
        type Builder = BankConnectionsResourceBalanceBuilder;
    }
};
/// The `type` of the balance.
/// An additional hash is included on the balance with a name matching this value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceBalanceType {
    Cash,
    Credit,
}
impl BankConnectionsResourceBalanceType {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceBalanceType::*;
        match self {
            Cash => "cash",
            Credit => "credit",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceBalanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceBalanceType::*;
        match s {
            "cash" => Ok(Cash),
            "credit" => Ok(Credit),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for BankConnectionsResourceBalanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceBalanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceBalanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceBalanceType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankConnectionsResourceBalanceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<BankConnectionsResourceBalanceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankConnectionsResourceBalanceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
