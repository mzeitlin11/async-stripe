#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalSubscriptionCancel {
    pub cancellation_reason: stripe_billing::PortalSubscriptionCancellationReason,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    pub mode: PortalSubscriptionCancelMode,
    /// Whether to create prorations when canceling subscriptions.
    /// Possible values are `none` and `create_prorations`.
    pub proration_behavior: PortalSubscriptionCancelProrationBehavior,
}
#[cfg(feature = "min-ser")]
pub struct PortalSubscriptionCancelBuilder {
    cancellation_reason: Option<stripe_billing::PortalSubscriptionCancellationReason>,
    enabled: Option<bool>,
    mode: Option<PortalSubscriptionCancelMode>,
    proration_behavior: Option<PortalSubscriptionCancelProrationBehavior>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalSubscriptionCancel {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionCancel>,
        builder: PortalSubscriptionCancelBuilder,
    }

    impl Visitor for Place<PortalSubscriptionCancel> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalSubscriptionCancelBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalSubscriptionCancelBuilder {
        type Out = PortalSubscriptionCancel;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "cancellation_reason" => Ok(Deserialize::begin(&mut self.cancellation_reason)),
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),
                "mode" => Ok(Deserialize::begin(&mut self.mode)),
                "proration_behavior" => Ok(Deserialize::begin(&mut self.proration_behavior)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { cancellation_reason: Deserialize::default(), enabled: Deserialize::default(), mode: Deserialize::default(), proration_behavior: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let cancellation_reason = self.cancellation_reason.take()?;
            let enabled = self.enabled.take()?;
            let mode = self.mode.take()?;
            let proration_behavior = self.proration_behavior.take()?;

            Some(Self::Out { cancellation_reason, enabled, mode, proration_behavior })
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

    impl ObjectDeser for PortalSubscriptionCancel {
        type Builder = PortalSubscriptionCancelBuilder;
    }
};
/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}
impl PortalSubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        use PortalSubscriptionCancelMode::*;
        match self {
            AtPeriodEnd => "at_period_end",
            Immediately => "immediately",
        }
    }
}

impl std::str::FromStr for PortalSubscriptionCancelMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionCancelMode::*;
        match s {
            "at_period_end" => Ok(AtPeriodEnd),
            "immediately" => Ok(Immediately),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PortalSubscriptionCancelMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PortalSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalSubscriptionCancelMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalSubscriptionCancelMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PortalSubscriptionCancelMode"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PortalSubscriptionCancelMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PortalSubscriptionCancelMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalSubscriptionCancelMode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Whether to create prorations when canceling subscriptions.
/// Possible values are `none` and `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl PortalSubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use PortalSubscriptionCancelProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for PortalSubscriptionCancelProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionCancelProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PortalSubscriptionCancelProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PortalSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalSubscriptionCancelProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalSubscriptionCancelProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PortalSubscriptionCancelProrationBehavior"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PortalSubscriptionCancelProrationBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PortalSubscriptionCancelProrationBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalSubscriptionCancelProrationBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
