#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountTosAcceptance {
    /// The Unix timestamp marking when the account representative accepted their service agreement
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted their service agreement
    pub ip: Option<String>,
    /// The user's service agreement type
    pub service_agreement: Option<String>,
    /// The user agent of the browser from which the account representative accepted their service agreement.
    pub user_agent: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountTosAcceptanceBuilder {
    date: Option<Option<stripe_types::Timestamp>>,
    ip: Option<Option<String>>,
    service_agreement: Option<Option<String>>,
    user_agent: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountTosAcceptance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountTosAcceptance>,
        builder: AccountTosAcceptanceBuilder,
    }

    impl Visitor for Place<AccountTosAcceptance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountTosAcceptanceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountTosAcceptanceBuilder {
        type Out = AccountTosAcceptance;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "date" => Ok(Deserialize::begin(&mut self.date)),
                "ip" => Ok(Deserialize::begin(&mut self.ip)),
                "service_agreement" => Ok(Deserialize::begin(&mut self.service_agreement)),
                "user_agent" => Ok(Deserialize::begin(&mut self.user_agent)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { date: Deserialize::default(), ip: Deserialize::default(), service_agreement: Deserialize::default(), user_agent: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let date = self.date.take()?;
            let ip = self.ip.take()?;
            let service_agreement = self.service_agreement.take()?;
            let user_agent = self.user_agent.take()?;

            Some(Self::Out { date, ip, service_agreement, user_agent })
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

    impl ObjectDeser for AccountTosAcceptance {
        type Builder = AccountTosAcceptanceBuilder;
    }
};
