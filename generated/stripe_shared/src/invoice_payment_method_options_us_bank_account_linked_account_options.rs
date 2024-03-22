#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    /// The list of permissions to request. The `payment_method` permission must be included.
    pub permissions: Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions>>,
    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch>>,
}
#[cfg(feature = "min-ser")]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder {
    permissions: Option<Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions>>>,
    prefetch: Option<Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions>,
        builder: InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder {
        type Out = InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "permissions" => Ok(Deserialize::begin(&mut self.permissions)),
                "prefetch" => Ok(Deserialize::begin(&mut self.prefetch)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { permissions: Deserialize::default(), prefetch: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let permissions = self.permissions.take()?;
            let prefetch = self.prefetch.take()?;

            Some(Self::Out { permissions, prefetch })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
        type Builder = InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder;
    }
};
/// The list of permissions to request. The `payment_method` permission must be included.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    Balances,
    PaymentMethod,
    Transactions,
}
impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::*;
        match self {
            Balances => "balances",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Data features requested to be retrieved upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    Balances,
}
impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::*;
        match self {
            Balances => "balances",
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
