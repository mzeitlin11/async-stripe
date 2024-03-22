#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: Option<u64>,
    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: Option<PaymentMethodDetailsCardInstallmentsPlanInterval>,
    /// Type of installment plan, one of `fixed_count`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PaymentMethodDetailsCardInstallmentsPlanType,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsCardInstallmentsPlanBuilder {
    count: Option<Option<u64>>,
    interval: Option<Option<PaymentMethodDetailsCardInstallmentsPlanInterval>>,
    type_: Option<PaymentMethodDetailsCardInstallmentsPlanType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsCardInstallmentsPlan {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardInstallmentsPlan>,
        builder: PaymentMethodDetailsCardInstallmentsPlanBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardInstallmentsPlan> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsCardInstallmentsPlanBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardInstallmentsPlanBuilder {
        type Out = PaymentMethodDetailsCardInstallmentsPlan;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "count" => Ok(Deserialize::begin(&mut self.count)),
                "interval" => Ok(Deserialize::begin(&mut self.interval)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { count: Deserialize::default(), interval: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let count = self.count.take()?;
            let interval = self.interval.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { count, interval, type_ })
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

    impl ObjectDeser for PaymentMethodDetailsCardInstallmentsPlan {
        type Builder = PaymentMethodDetailsCardInstallmentsPlanBuilder;
    }
};
/// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
/// One of `month`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCardInstallmentsPlanInterval {
    Month,
}
impl PaymentMethodDetailsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCardInstallmentsPlanInterval::*;
        match self {
            Month => "month",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardInstallmentsPlanInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardInstallmentsPlanInterval::*;
        match s {
            "month" => Ok(Month),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodDetailsCardInstallmentsPlanInterval"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCardInstallmentsPlanInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsCardInstallmentsPlanInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCardInstallmentsPlanType {
    FixedCount,
}
impl PaymentMethodDetailsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCardInstallmentsPlanType::*;
        match self {
            FixedCount => "fixed_count",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardInstallmentsPlanType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardInstallmentsPlanType::*;
        match s {
            "fixed_count" => Ok(FixedCount),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodDetailsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodDetailsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsCardInstallmentsPlanType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardInstallmentsPlanType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodDetailsCardInstallmentsPlanType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodDetailsCardInstallmentsPlanType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCardInstallmentsPlanType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsCardInstallmentsPlanType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
