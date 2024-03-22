#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsAfterCompletionHostedConfirmation {
    /// A custom message to display to the customer after the flow is completed.
    pub custom_message: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsAfterCompletionHostedConfirmationBuilder {
    custom_message: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsAfterCompletionHostedConfirmation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsAfterCompletionHostedConfirmation>,
        builder: PortalFlowsAfterCompletionHostedConfirmationBuilder,
    }

    impl Visitor for Place<PortalFlowsAfterCompletionHostedConfirmation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsAfterCompletionHostedConfirmationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsAfterCompletionHostedConfirmationBuilder {
        type Out = PortalFlowsAfterCompletionHostedConfirmation;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "custom_message" => Ok(Deserialize::begin(&mut self.custom_message)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { custom_message: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let custom_message = self.custom_message.take()?;

            Some(Self::Out { custom_message })
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

    impl ObjectDeser for PortalFlowsAfterCompletionHostedConfirmation {
        type Builder = PortalFlowsAfterCompletionHostedConfirmationBuilder;
    }
};
