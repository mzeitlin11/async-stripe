#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    pub account_onboarding: stripe_connect::ConnectEmbeddedBaseConfigClaim,
}
#[cfg(feature = "min-ser")]
pub struct ConnectEmbeddedAccountSessionCreateComponentsBuilder {
    account_onboarding: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ConnectEmbeddedAccountSessionCreateComponents {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedAccountSessionCreateComponents>,
        builder: ConnectEmbeddedAccountSessionCreateComponentsBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedAccountSessionCreateComponents> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ConnectEmbeddedAccountSessionCreateComponentsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ConnectEmbeddedAccountSessionCreateComponentsBuilder {
        type Out = ConnectEmbeddedAccountSessionCreateComponents;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "account_onboarding" => Ok(Deserialize::begin(&mut self.account_onboarding)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account_onboarding: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_onboarding = self.account_onboarding.take()?;

            Some(Self::Out { account_onboarding })
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

    impl ObjectDeser for ConnectEmbeddedAccountSessionCreateComponents {
        type Builder = ConnectEmbeddedAccountSessionCreateComponentsBuilder;
    }
};
