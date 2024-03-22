#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentNextActionRedirectToUrl {
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    /// The URL you must redirect your customer to in order to authenticate.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentNextActionRedirectToUrlBuilder {
    return_url: Option<Option<String>>,
    url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentNextActionRedirectToUrl {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentNextActionRedirectToUrl>,
        builder: SetupIntentNextActionRedirectToUrlBuilder,
    }

    impl Visitor for Place<SetupIntentNextActionRedirectToUrl> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentNextActionRedirectToUrlBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentNextActionRedirectToUrlBuilder {
        type Out = SetupIntentNextActionRedirectToUrl;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "return_url" => Ok(Deserialize::begin(&mut self.return_url)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { return_url: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let return_url = self.return_url.take()?;
            let url = self.url.take()?;

            Some(Self::Out { return_url, url })
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

    impl ObjectDeser for SetupIntentNextActionRedirectToUrl {
        type Builder = SetupIntentNextActionRedirectToUrlBuilder;
    }
};
