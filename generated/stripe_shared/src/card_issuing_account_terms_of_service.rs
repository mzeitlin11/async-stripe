#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CardIssuingAccountTermsOfService {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    pub ip: Option<String>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    pub user_agent: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct CardIssuingAccountTermsOfServiceBuilder {
    date: Option<Option<stripe_types::Timestamp>>,
    ip: Option<Option<String>>,
    user_agent: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CardIssuingAccountTermsOfService {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CardIssuingAccountTermsOfService>,
        builder: CardIssuingAccountTermsOfServiceBuilder,
    }

    impl Visitor for Place<CardIssuingAccountTermsOfService> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CardIssuingAccountTermsOfServiceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CardIssuingAccountTermsOfServiceBuilder {
        type Out = CardIssuingAccountTermsOfService;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "date" => Ok(Deserialize::begin(&mut self.date)),
                "ip" => Ok(Deserialize::begin(&mut self.ip)),
                "user_agent" => Ok(Deserialize::begin(&mut self.user_agent)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { date: Deserialize::default(), ip: Deserialize::default(), user_agent: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let date = self.date.take()?;
            let ip = self.ip.take()?;
            let user_agent = self.user_agent.take()?;

            Some(Self::Out { date, ip, user_agent })
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

    impl ObjectDeser for CardIssuingAccountTermsOfService {
        type Builder = CardIssuingAccountTermsOfServiceBuilder;
    }
};
