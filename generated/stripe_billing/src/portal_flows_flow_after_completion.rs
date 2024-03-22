#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsFlowAfterCompletion {
    /// Configuration when `after_completion.type=hosted_confirmation`.
    pub hosted_confirmation: Option<stripe_billing::PortalFlowsAfterCompletionHostedConfirmation>,
    /// Configuration when `after_completion.type=redirect`.
    pub redirect: Option<stripe_billing::PortalFlowsAfterCompletionRedirect>,
    /// The specified type of behavior after the flow is completed.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PortalFlowsFlowAfterCompletionType,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsFlowAfterCompletionBuilder {
    hosted_confirmation: Option<Option<stripe_billing::PortalFlowsAfterCompletionHostedConfirmation>>,
    redirect: Option<Option<stripe_billing::PortalFlowsAfterCompletionRedirect>>,
    type_: Option<PortalFlowsFlowAfterCompletionType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsFlowAfterCompletion {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlowAfterCompletion>,
        builder: PortalFlowsFlowAfterCompletionBuilder,
    }

    impl Visitor for Place<PortalFlowsFlowAfterCompletion> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsFlowAfterCompletionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsFlowAfterCompletionBuilder {
        type Out = PortalFlowsFlowAfterCompletion;
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

    impl ObjectDeser for PortalFlowsFlowAfterCompletion {
        type Builder = PortalFlowsFlowAfterCompletionBuilder;
    }
};
/// The specified type of behavior after the flow is completed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalFlowsFlowAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
}
impl PortalFlowsFlowAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use PortalFlowsFlowAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            PortalHomepage => "portal_homepage",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for PortalFlowsFlowAfterCompletionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsFlowAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "portal_homepage" => Ok(PortalHomepage),
            "redirect" => Ok(Redirect),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PortalFlowsFlowAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PortalFlowsFlowAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalFlowsFlowAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalFlowsFlowAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalFlowsFlowAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PortalFlowsFlowAfterCompletionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PortalFlowsFlowAfterCompletionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PortalFlowsFlowAfterCompletionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalFlowsFlowAfterCompletionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
