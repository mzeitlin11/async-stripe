#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PortalBusinessProfile {
    /// The messaging shown to customers in the portal.
    pub headline: Option<String>,
    /// A link to the business’s publicly available privacy policy.
    pub privacy_policy_url: Option<String>,
    /// A link to the business’s publicly available terms of service.
    pub terms_of_service_url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PortalBusinessProfileBuilder {
    headline: Option<Option<String>>,
    privacy_policy_url: Option<Option<String>>,
    terms_of_service_url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalBusinessProfile {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalBusinessProfile>,
        builder: PortalBusinessProfileBuilder,
    }

    impl Visitor for Place<PortalBusinessProfile> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PortalBusinessProfileBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PortalBusinessProfileBuilder {
        type Out = PortalBusinessProfile;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "headline" => Ok(Deserialize::begin(&mut self.headline)),
                "privacy_policy_url" => Ok(Deserialize::begin(&mut self.privacy_policy_url)),
                "terms_of_service_url" => Ok(Deserialize::begin(&mut self.terms_of_service_url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { headline: Deserialize::default(), privacy_policy_url: Deserialize::default(), terms_of_service_url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let headline = self.headline.take()?;
            let privacy_policy_url = self.privacy_policy_url.take()?;
            let terms_of_service_url = self.terms_of_service_url.take()?;

            Some(Self::Out { headline, privacy_policy_url, terms_of_service_url })
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

    impl ObjectDeser for PortalBusinessProfile {
        type Builder = PortalBusinessProfileBuilder;
    }
};
