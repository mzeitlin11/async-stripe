#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReceivedPaymentMethodDetailsFinancialAccount {
    /// The FinancialAccount ID.
    pub id: String,
    /// The rails the ReceivedCredit was sent over. A FinancialAccount can only send funds over `stripe`.
    pub network: ReceivedPaymentMethodDetailsFinancialAccountNetwork,
}
#[cfg(feature = "min-ser")]
pub struct ReceivedPaymentMethodDetailsFinancialAccountBuilder {
    id: Option<String>,
    network: Option<ReceivedPaymentMethodDetailsFinancialAccountNetwork>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ReceivedPaymentMethodDetailsFinancialAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReceivedPaymentMethodDetailsFinancialAccount>,
        builder: ReceivedPaymentMethodDetailsFinancialAccountBuilder,
    }

    impl Visitor for Place<ReceivedPaymentMethodDetailsFinancialAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ReceivedPaymentMethodDetailsFinancialAccountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ReceivedPaymentMethodDetailsFinancialAccountBuilder {
        type Out = ReceivedPaymentMethodDetailsFinancialAccount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "network" => Ok(Deserialize::begin(&mut self.network)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), network: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let id = self.id.take()?;
            let network = self.network.take()?;

            Some(Self::Out { id, network })
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

    impl ObjectDeser for ReceivedPaymentMethodDetailsFinancialAccount {
        type Builder = ReceivedPaymentMethodDetailsFinancialAccountBuilder;
    }
};
/// The rails the ReceivedCredit was sent over. A FinancialAccount can only send funds over `stripe`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
}
impl ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use ReceivedPaymentMethodDetailsFinancialAccountNetwork::*;
        match self {
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReceivedPaymentMethodDetailsFinancialAccountNetwork::*;
        match s {
            "stripe" => Ok(Stripe),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ReceivedPaymentMethodDetailsFinancialAccountNetwork"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ReceivedPaymentMethodDetailsFinancialAccountNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReceivedPaymentMethodDetailsFinancialAccountNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
