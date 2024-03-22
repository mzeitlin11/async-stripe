#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ApplePayDomain {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub domain_name: String,
    /// Unique identifier for the object.
    pub id: stripe_misc::ApplePayDomainId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[cfg(feature = "min-ser")]
pub struct ApplePayDomainBuilder {
    created: Option<stripe_types::Timestamp>,
    domain_name: Option<String>,
    id: Option<stripe_misc::ApplePayDomainId>,
    livemode: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ApplePayDomain {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ApplePayDomain>,
        builder: ApplePayDomainBuilder,
    }

    impl Visitor for Place<ApplePayDomain> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ApplePayDomainBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ApplePayDomainBuilder {
        type Out = ApplePayDomain;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "domain_name" => Ok(Deserialize::begin(&mut self.domain_name)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { created: Deserialize::default(), domain_name: Deserialize::default(), id: Deserialize::default(), livemode: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let domain_name = self.domain_name.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;

            Some(Self::Out { created, domain_name, id, livemode })
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

    impl ObjectDeser for ApplePayDomain {
        type Builder = ApplePayDomainBuilder;
    }
};
impl stripe_types::Object for ApplePayDomain {
    type Id = stripe_misc::ApplePayDomainId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ApplePayDomainId);
