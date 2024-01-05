#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalFlowsAfterCompletionRedirect {
    /// The URL the customer will be redirected to after the flow is completed.
    pub return_url: String,
}
#[cfg(feature = "min-ser")]
pub struct PortalFlowsAfterCompletionRedirectBuilder {
    return_url: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsAfterCompletionRedirect {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsAfterCompletionRedirect>,
        builder: PortalFlowsAfterCompletionRedirectBuilder,
    }

    impl Visitor for Place<PortalFlowsAfterCompletionRedirect> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalFlowsAfterCompletionRedirectBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalFlowsAfterCompletionRedirectBuilder {
        type Out = PortalFlowsAfterCompletionRedirect;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "return_url" => Ok(Deserialize::begin(&mut self.return_url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { return_url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let return_url = self.return_url.take()?;

            Some(Self::Out { return_url })
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

    impl ObjectDeser for PortalFlowsAfterCompletionRedirect {
        type Builder = PortalFlowsAfterCompletionRedirectBuilder;
    }
};
