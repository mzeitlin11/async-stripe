/// An AccountSession allows a Connect platform to grant access to a connected account in Connect embedded components.
///
/// We recommend that you create an AccountSession each time you need to display an embedded component
/// to your user. Do not save AccountSessions to your database as they expire relatively
/// quickly, and cannot be used more than once.
///
/// Related guide: [Connect embedded components](https://stripe.com/docs/connect/get-started-connect-embedded-components).
///
/// For more details see <<https://stripe.com/docs/api/account_sessions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountSession {
    /// The ID of the account the AccountSession was created for
    pub account: String,
    /// The client secret of this AccountSession.
    /// Used on the client to set up secure access to the given `account`.
    ///
    /// The client secret can be used to provide access to `account` from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the connected account.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    ///
    /// Refer to our docs to [setup Connect embedded components](https://stripe.com/docs/connect/get-started-connect-embedded-components) and learn about how `client_secret` should be handled.
    pub client_secret: String,
    pub components: stripe_connect::ConnectEmbeddedAccountSessionCreateComponents,
    /// The timestamp at which this AccountSession will expire.
    pub expires_at: stripe_types::Timestamp,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[cfg(feature = "min-ser")]
pub struct AccountSessionBuilder {
    account: Option<String>,
    client_secret: Option<String>,
    components: Option<stripe_connect::ConnectEmbeddedAccountSessionCreateComponents>,
    expires_at: Option<stripe_types::Timestamp>,
    livemode: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountSession>,
        builder: AccountSessionBuilder,
    }

    impl Visitor for Place<AccountSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountSessionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountSessionBuilder {
        type Out = AccountSession;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account" => Ok(Deserialize::begin(&mut self.account)),
                "client_secret" => Ok(Deserialize::begin(&mut self.client_secret)),
                "components" => Ok(Deserialize::begin(&mut self.components)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account: Deserialize::default(), client_secret: Deserialize::default(), components: Deserialize::default(), expires_at: Deserialize::default(), livemode: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account = self.account.take()?;
            let client_secret = self.client_secret.take()?;
            let components = self.components.take()?;
            let expires_at = self.expires_at.take()?;
            let livemode = self.livemode.take()?;

            Some(Self::Out { account, client_secret, components, expires_at, livemode })
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

    impl ObjectDeser for AccountSession {
        type Builder = AccountSessionBuilder;
    }
};
