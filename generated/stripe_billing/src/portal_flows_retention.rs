#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsRetention {
    /// Configuration when `retention.type=coupon_offer`.
    pub coupon_offer: Option<stripe_billing::PortalFlowsCouponOffer>,
    /// Type of retention strategy that will be used.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PortalFlowsRetentionType,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsRetentionBuilder {
    coupon_offer: Option<Option<stripe_billing::PortalFlowsCouponOffer>>,
    type_: Option<PortalFlowsRetentionType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsRetention {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsRetention>,
        builder: PortalFlowsRetentionBuilder,
    }

    impl Visitor for Place<PortalFlowsRetention> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsRetentionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsRetentionBuilder {
        type Out = PortalFlowsRetention;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "coupon_offer" => Ok(Deserialize::begin(&mut self.coupon_offer)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { coupon_offer: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let coupon_offer = self.coupon_offer.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { coupon_offer, type_ })
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

    impl ObjectDeser for PortalFlowsRetention {
        type Builder = PortalFlowsRetentionBuilder;
    }
};
/// Type of retention strategy that will be used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalFlowsRetentionType {
    CouponOffer,
}
impl PortalFlowsRetentionType {
    pub fn as_str(self) -> &'static str {
        use PortalFlowsRetentionType::*;
        match self {
            CouponOffer => "coupon_offer",
        }
    }
}

impl std::str::FromStr for PortalFlowsRetentionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsRetentionType::*;
        match s {
            "coupon_offer" => Ok(CouponOffer),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PortalFlowsRetentionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PortalFlowsRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalFlowsRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalFlowsRetentionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalFlowsRetentionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PortalFlowsRetentionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PortalFlowsRetentionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PortalFlowsRetentionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalFlowsRetentionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
