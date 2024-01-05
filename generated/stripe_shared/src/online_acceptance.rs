#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OnlineAcceptance {
    /// The customer accepts the mandate from this IP address.
    pub ip_address: Option<String>,
    /// The customer accepts the mandate using the user agent of the browser.
    pub user_agent: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct OnlineAcceptanceBuilder {
    ip_address: Option<Option<String>>,
    user_agent: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for OnlineAcceptance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<OnlineAcceptance>,
        builder: OnlineAcceptanceBuilder,
    }

    impl Visitor for Place<OnlineAcceptance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: OnlineAcceptanceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for OnlineAcceptanceBuilder {
        type Out = OnlineAcceptance;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "ip_address" => Ok(Deserialize::begin(&mut self.ip_address)),
                "user_agent" => Ok(Deserialize::begin(&mut self.user_agent)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { ip_address: Deserialize::default(), user_agent: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let ip_address = self.ip_address.take()?;
            let user_agent = self.user_agent.take()?;

            Some(Self::Out { ip_address, user_agent })
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

    impl ObjectDeser for OnlineAcceptance {
        type Builder = OnlineAcceptanceBuilder;
    }
};
