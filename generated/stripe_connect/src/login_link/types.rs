/// Login Links are single-use login link for an Express account to access their Stripe dashboard.
///
/// For more details see <<https://stripe.com/docs/api/account/login_link>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LoginLink {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The URL for the login link.
    pub url: String,
}
#[cfg(feature = "min-ser")]
pub struct LoginLinkBuilder {
    created: Option<stripe_types::Timestamp>,
    url: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for LoginLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LoginLink>,
        builder: LoginLinkBuilder,
    }

    impl Visitor for Place<LoginLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: LoginLinkBuilder::deser_default() }))
        }
    }

    impl MapBuilder for LoginLinkBuilder {
        type Out = LoginLink;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { created: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let url = self.url.take()?;

            Some(Self::Out { created, url })
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

    impl ObjectDeser for LoginLink {
        type Builder = LoginLinkBuilder;
    }
};
