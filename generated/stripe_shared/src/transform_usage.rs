#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TransformUsage {
    /// Divide usage by this number.
    pub divide_by: i64,
    /// After division, either round the result `up` or `down`.
    pub round: TransformUsageRound,
}
#[cfg(feature = "min-ser")]
pub struct TransformUsageBuilder {
    divide_by: Option<i64>,
    round: Option<TransformUsageRound>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TransformUsage {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TransformUsage>,
        builder: TransformUsageBuilder,
    }

    impl Visitor for Place<TransformUsage> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TransformUsageBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TransformUsageBuilder {
        type Out = TransformUsage;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "divide_by" => Ok(Deserialize::begin(&mut self.divide_by)),
                "round" => Ok(Deserialize::begin(&mut self.round)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { divide_by: Deserialize::default(), round: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let divide_by = self.divide_by.take()?;
            let round = self.round.take()?;

            Some(Self::Out { divide_by, round })
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

    impl ObjectDeser for TransformUsage {
        type Builder = TransformUsageBuilder;
    }
};
/// After division, either round the result `up` or `down`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TransformUsageRound {
    Down,
    Up,
}
impl TransformUsageRound {
    pub fn as_str(self) -> &'static str {
        use TransformUsageRound::*;
        match self {
            Down => "down",
            Up => "up",
        }
    }
}

impl std::str::FromStr for TransformUsageRound {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransformUsageRound::*;
        match s {
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TransformUsageRound {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TransformUsageRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TransformUsageRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TransformUsageRound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransformUsageRound {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TransformUsageRound"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransformUsageRound {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TransformUsageRound> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TransformUsageRound::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
