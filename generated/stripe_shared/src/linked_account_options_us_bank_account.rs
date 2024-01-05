#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LinkedAccountOptionsUsBankAccount {
    /// The list of permissions to request. The `payment_method` permission must be included.
    pub permissions: Option<Vec<LinkedAccountOptionsUsBankAccountPermissions>>,
    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<LinkedAccountOptionsUsBankAccountPrefetch>>,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    pub return_url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct LinkedAccountOptionsUsBankAccountBuilder {
    permissions: Option<Option<Vec<LinkedAccountOptionsUsBankAccountPermissions>>>,
    prefetch: Option<Option<Vec<LinkedAccountOptionsUsBankAccountPrefetch>>>,
    return_url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for LinkedAccountOptionsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LinkedAccountOptionsUsBankAccount>,
        builder: LinkedAccountOptionsUsBankAccountBuilder,
    }

    impl Visitor for Place<LinkedAccountOptionsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: LinkedAccountOptionsUsBankAccountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for LinkedAccountOptionsUsBankAccountBuilder {
        type Out = LinkedAccountOptionsUsBankAccount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "permissions" => Ok(Deserialize::begin(&mut self.permissions)),
                "prefetch" => Ok(Deserialize::begin(&mut self.prefetch)),
                "return_url" => Ok(Deserialize::begin(&mut self.return_url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { permissions: Deserialize::default(), prefetch: Deserialize::default(), return_url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let permissions = self.permissions.take()?;
            let prefetch = self.prefetch.take()?;
            let return_url = self.return_url.take()?;

            Some(Self::Out { permissions, prefetch, return_url })
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

    impl ObjectDeser for LinkedAccountOptionsUsBankAccount {
        type Builder = LinkedAccountOptionsUsBankAccountBuilder;
    }
};
/// The list of permissions to request. The `payment_method` permission must be included.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LinkedAccountOptionsUsBankAccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl LinkedAccountOptionsUsBankAccountPermissions {
    pub fn as_str(self) -> &'static str {
        use LinkedAccountOptionsUsBankAccountPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for LinkedAccountOptionsUsBankAccountPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LinkedAccountOptionsUsBankAccountPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for LinkedAccountOptionsUsBankAccountPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for LinkedAccountOptionsUsBankAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for LinkedAccountOptionsUsBankAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for LinkedAccountOptionsUsBankAccountPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LinkedAccountOptionsUsBankAccountPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for LinkedAccountOptionsUsBankAccountPermissions"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LinkedAccountOptionsUsBankAccountPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<LinkedAccountOptionsUsBankAccountPermissions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(LinkedAccountOptionsUsBankAccountPermissions::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Data features requested to be retrieved upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LinkedAccountOptionsUsBankAccountPrefetch {
    Balances,
}
impl LinkedAccountOptionsUsBankAccountPrefetch {
    pub fn as_str(self) -> &'static str {
        use LinkedAccountOptionsUsBankAccountPrefetch::*;
        match self {
            Balances => "balances",
        }
    }
}

impl std::str::FromStr for LinkedAccountOptionsUsBankAccountPrefetch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LinkedAccountOptionsUsBankAccountPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for LinkedAccountOptionsUsBankAccountPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for LinkedAccountOptionsUsBankAccountPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for LinkedAccountOptionsUsBankAccountPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for LinkedAccountOptionsUsBankAccountPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LinkedAccountOptionsUsBankAccountPrefetch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for LinkedAccountOptionsUsBankAccountPrefetch"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LinkedAccountOptionsUsBankAccountPrefetch {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<LinkedAccountOptionsUsBankAccountPrefetch> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(LinkedAccountOptionsUsBankAccountPrefetch::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
