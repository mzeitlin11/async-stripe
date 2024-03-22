/// Defines how a subscription behaves when a free trial ends.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionsTrialsResourceEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionsTrialsResourceEndBehaviorBuilder {
    missing_payment_method: Option<SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionsTrialsResourceEndBehavior {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsTrialsResourceEndBehavior>,
        builder: SubscriptionsTrialsResourceEndBehaviorBuilder,
    }

    impl Visitor for Place<SubscriptionsTrialsResourceEndBehavior> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionsTrialsResourceEndBehaviorBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionsTrialsResourceEndBehaviorBuilder {
        type Out = SubscriptionsTrialsResourceEndBehavior;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "missing_payment_method" => Ok(Deserialize::begin(&mut self.missing_payment_method)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { missing_payment_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let missing_payment_method = self.missing_payment_method.take()?;

            Some(Self::Out { missing_payment_method })
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

    impl ObjectDeser for SubscriptionsTrialsResourceEndBehavior {
        type Builder = SubscriptionsTrialsResourceEndBehaviorBuilder;
    }
};
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}
impl SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
        }
    }
}

impl std::str::FromStr for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
