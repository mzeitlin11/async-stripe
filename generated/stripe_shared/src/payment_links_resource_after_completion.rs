#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceAfterCompletion {
    pub hosted_confirmation: Option<stripe_shared::PaymentLinksResourceCompletionBehaviorConfirmationPage>,
    pub redirect: Option<stripe_shared::PaymentLinksResourceCompletionBehaviorRedirect>,
    /// The specified behavior after the purchase is complete.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PaymentLinksResourceAfterCompletionType,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceAfterCompletionBuilder {
    hosted_confirmation: Option<Option<stripe_shared::PaymentLinksResourceCompletionBehaviorConfirmationPage>>,
    redirect: Option<Option<stripe_shared::PaymentLinksResourceCompletionBehaviorRedirect>>,
    type_: Option<PaymentLinksResourceAfterCompletionType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceAfterCompletion {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceAfterCompletion>,
        builder: PaymentLinksResourceAfterCompletionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceAfterCompletion> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceAfterCompletionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceAfterCompletionBuilder {
        type Out = PaymentLinksResourceAfterCompletion;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "hosted_confirmation" => Ok(Deserialize::begin(&mut self.hosted_confirmation)),
                "redirect" => Ok(Deserialize::begin(&mut self.redirect)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { hosted_confirmation: Deserialize::default(), redirect: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let hosted_confirmation = self.hosted_confirmation.take()?;
            let redirect = self.redirect.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { hosted_confirmation, redirect, type_ })
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

    impl ObjectDeser for PaymentLinksResourceAfterCompletion {
        type Builder = PaymentLinksResourceAfterCompletionBuilder;
    }
};
/// The specified behavior after the purchase is complete.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceAfterCompletionType {
    HostedConfirmation,
    Redirect,
}
impl PaymentLinksResourceAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceAfterCompletionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "redirect" => Ok(Redirect),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinksResourceAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourceAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinksResourceAfterCompletionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinksResourceAfterCompletionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinksResourceAfterCompletionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinksResourceAfterCompletionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
