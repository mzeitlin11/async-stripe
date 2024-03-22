#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalLoginPage {
    /// If `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer portal.
    ///
    /// If `false`, the previously generated `url`, if any, will be deactivated.
    pub enabled: bool,
    /// A shareable URL to the hosted portal login page.
    /// Your customers will be able to log in with their [email](https://stripe.com/docs/api/customers/object#customer_object-email) and receive a link to their customer portal.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PortalLoginPageBuilder {
    enabled: Option<bool>,
    url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalLoginPage {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalLoginPage>,
        builder: PortalLoginPageBuilder,
    }

    impl Visitor for Place<PortalLoginPage> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalLoginPageBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalLoginPageBuilder {
        type Out = PortalLoginPage;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let enabled = self.enabled.take()?;
            let url = self.url.take()?;

            Some(Self::Out { enabled, url })
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

    impl ObjectDeser for PortalLoginPage {
        type Builder = PortalLoginPageBuilder;
    }
};
