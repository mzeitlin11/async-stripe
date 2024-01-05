#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerBalanceCustomerBalanceSettings {
    /// The configuration for how funds that land in the customer cash balance are reconciled.
    pub reconciliation_mode: CustomerBalanceCustomerBalanceSettingsReconciliationMode,
    /// A flag to indicate if reconciliation mode returned is the user's default or is specific to this customer cash balance.
    pub using_merchant_default: bool,
}
#[cfg(feature = "min-ser")]
pub struct CustomerBalanceCustomerBalanceSettingsBuilder {
    reconciliation_mode: Option<CustomerBalanceCustomerBalanceSettingsReconciliationMode>,
    using_merchant_default: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceCustomerBalanceSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerBalanceCustomerBalanceSettings>,
        builder: CustomerBalanceCustomerBalanceSettingsBuilder,
    }

    impl Visitor for Place<CustomerBalanceCustomerBalanceSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomerBalanceCustomerBalanceSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomerBalanceCustomerBalanceSettingsBuilder {
        type Out = CustomerBalanceCustomerBalanceSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "reconciliation_mode" => Ok(Deserialize::begin(&mut self.reconciliation_mode)),
                "using_merchant_default" => Ok(Deserialize::begin(&mut self.using_merchant_default)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { reconciliation_mode: Deserialize::default(), using_merchant_default: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let reconciliation_mode = self.reconciliation_mode.take()?;
            let using_merchant_default = self.using_merchant_default.take()?;

            Some(Self::Out { reconciliation_mode, using_merchant_default })
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

    impl ObjectDeser for CustomerBalanceCustomerBalanceSettings {
        type Builder = CustomerBalanceCustomerBalanceSettingsBuilder;
    }
};
/// The configuration for how funds that land in the customer cash balance are reconciled.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}
impl CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        use CustomerBalanceCustomerBalanceSettingsReconciliationMode::*;
        match self {
            Automatic => "automatic",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerBalanceCustomerBalanceSettingsReconciliationMode::*;
        match s {
            "automatic" => Ok(Automatic),
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerBalanceCustomerBalanceSettingsReconciliationMode"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CustomerBalanceCustomerBalanceSettingsReconciliationMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerBalanceCustomerBalanceSettingsReconciliationMode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
