/// Account Links are the means by which a Connect platform grants a connected account permission to access.
/// Stripe-hosted applications, such as Connect Onboarding.
///
/// Related guide: [Connect Onboarding](https://stripe.com/docs/connect/custom/hosted-onboarding)
///
/// For more details see <<https://stripe.com/docs/api/account_links/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountLink {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The timestamp at which this account link will expire.
    pub expires_at: stripe_types::Timestamp,
    /// The URL for the account link.
    pub url: String,
}
#[cfg(feature = "min-ser")]
pub struct AccountLinkBuilder {
    created: Option<stripe_types::Timestamp>,
    expires_at: Option<stripe_types::Timestamp>,
    url: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountLink>,
        builder: AccountLinkBuilder,
    }

    impl Visitor for Place<AccountLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountLinkBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountLinkBuilder {
        type Out = AccountLink;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { created: Deserialize::default(), expires_at: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let expires_at = self.expires_at.take()?;
            let url = self.url.take()?;

            Some(Self::Out { created, expires_at, url })
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

    impl ObjectDeser for AccountLink {
        type Builder = AccountLinkBuilder;
    }
};
