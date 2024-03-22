#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsFlow {
    pub after_completion: stripe_billing::PortalFlowsFlowAfterCompletion,
    /// Configuration when `flow.type=subscription_cancel`.
    pub subscription_cancel: Option<stripe_billing::PortalFlowsFlowSubscriptionCancel>,
    /// Configuration when `flow.type=subscription_update`.
    pub subscription_update: Option<stripe_billing::PortalFlowsFlowSubscriptionUpdate>,
    /// Configuration when `flow.type=subscription_update_confirm`.
    pub subscription_update_confirm: Option<stripe_billing::PortalFlowsFlowSubscriptionUpdateConfirm>,
    /// Type of flow that the customer will go through.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PortalFlowsFlowType,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsFlowBuilder {
    after_completion: Option<stripe_billing::PortalFlowsFlowAfterCompletion>,
    subscription_cancel: Option<Option<stripe_billing::PortalFlowsFlowSubscriptionCancel>>,
    subscription_update: Option<Option<stripe_billing::PortalFlowsFlowSubscriptionUpdate>>,
    subscription_update_confirm: Option<Option<stripe_billing::PortalFlowsFlowSubscriptionUpdateConfirm>>,
    type_: Option<PortalFlowsFlowType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsFlow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlow>,
        builder: PortalFlowsFlowBuilder,
    }

    impl Visitor for Place<PortalFlowsFlow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsFlowBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsFlowBuilder {
        type Out = PortalFlowsFlow;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "after_completion" => Ok(Deserialize::begin(&mut self.after_completion)),
                "subscription_cancel" => Ok(Deserialize::begin(&mut self.subscription_cancel)),
                "subscription_update" => Ok(Deserialize::begin(&mut self.subscription_update)),
                "subscription_update_confirm" => Ok(Deserialize::begin(&mut self.subscription_update_confirm)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                after_completion: Deserialize::default(),
                subscription_cancel: Deserialize::default(),
                subscription_update: Deserialize::default(),
                subscription_update_confirm: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let after_completion = self.after_completion.take()?;
            let subscription_cancel = self.subscription_cancel.take()?;
            let subscription_update = self.subscription_update.take()?;
            let subscription_update_confirm = self.subscription_update_confirm.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { after_completion, subscription_cancel, subscription_update, subscription_update_confirm, type_ })
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

    impl ObjectDeser for PortalFlowsFlow {
        type Builder = PortalFlowsFlowBuilder;
    }
};
/// Type of flow that the customer will go through.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalFlowsFlowType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
}
impl PortalFlowsFlowType {
    pub fn as_str(self) -> &'static str {
        use PortalFlowsFlowType::*;
        match self {
            PaymentMethodUpdate => "payment_method_update",
            SubscriptionCancel => "subscription_cancel",
            SubscriptionUpdate => "subscription_update",
            SubscriptionUpdateConfirm => "subscription_update_confirm",
        }
    }
}

impl std::str::FromStr for PortalFlowsFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsFlowType::*;
        match s {
            "payment_method_update" => Ok(PaymentMethodUpdate),
            "subscription_cancel" => Ok(SubscriptionCancel),
            "subscription_update" => Ok(SubscriptionUpdate),
            "subscription_update_confirm" => Ok(SubscriptionUpdateConfirm),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PortalFlowsFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalFlowsFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalFlowsFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PortalFlowsFlowType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PortalFlowsFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PortalFlowsFlowType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalFlowsFlowType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
