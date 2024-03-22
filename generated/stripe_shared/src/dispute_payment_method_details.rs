#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DisputePaymentMethodDetails {
    /// Card specific dispute details.
    pub card: Option<stripe_shared::DisputePaymentMethodDetailsCard>,
    /// Payment method type.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: DisputePaymentMethodDetailsType,
}
#[cfg(feature = "min-ser")]
pub struct DisputePaymentMethodDetailsBuilder {
    card: Option<Option<stripe_shared::DisputePaymentMethodDetailsCard>>,
    type_: Option<DisputePaymentMethodDetailsType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DisputePaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputePaymentMethodDetails>,
        builder: DisputePaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<DisputePaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DisputePaymentMethodDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DisputePaymentMethodDetailsBuilder {
        type Out = DisputePaymentMethodDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { card: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let card = self.card.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { card, type_ })
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

    impl ObjectDeser for DisputePaymentMethodDetails {
        type Builder = DisputePaymentMethodDetailsBuilder;
    }
};
/// Payment method type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DisputePaymentMethodDetailsType {
    Card,
}
impl DisputePaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use DisputePaymentMethodDetailsType::*;
        match self {
            Card => "card",
        }
    }
}

impl std::str::FromStr for DisputePaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputePaymentMethodDetailsType::*;
        match s {
            "card" => Ok(Card),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for DisputePaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for DisputePaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputePaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DisputePaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisputePaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for DisputePaymentMethodDetailsType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DisputePaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<DisputePaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputePaymentMethodDetailsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
