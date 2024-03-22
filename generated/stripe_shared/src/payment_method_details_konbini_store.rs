#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKonbiniStore {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<PaymentMethodDetailsKonbiniStoreChain>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsKonbiniStoreBuilder {
    chain: Option<Option<PaymentMethodDetailsKonbiniStoreChain>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsKonbiniStore {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsKonbiniStore>,
        builder: PaymentMethodDetailsKonbiniStoreBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsKonbiniStore> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsKonbiniStoreBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsKonbiniStoreBuilder {
        type Out = PaymentMethodDetailsKonbiniStore;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "chain" => Ok(Deserialize::begin(&mut self.chain)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { chain: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let chain = self.chain.take()?;

            Some(Self::Out { chain })
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

    impl ObjectDeser for PaymentMethodDetailsKonbiniStore {
        type Builder = PaymentMethodDetailsKonbiniStoreBuilder;
    }
};
/// The name of the convenience store chain where the payment was completed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsKonbiniStoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
}
impl PaymentMethodDetailsKonbiniStoreChain {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsKonbiniStoreChain::*;
        match self {
            Familymart => "familymart",
            Lawson => "lawson",
            Ministop => "ministop",
            Seicomart => "seicomart",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsKonbiniStoreChain {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsKonbiniStoreChain::*;
        match s {
            "familymart" => Ok(Familymart),
            "lawson" => Ok(Lawson),
            "ministop" => Ok(Ministop),
            "seicomart" => Ok(Seicomart),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodDetailsKonbiniStoreChain {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodDetailsKonbiniStoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsKonbiniStoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsKonbiniStoreChain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsKonbiniStoreChain {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodDetailsKonbiniStoreChain"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodDetailsKonbiniStoreChain {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsKonbiniStoreChain> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsKonbiniStoreChain::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
