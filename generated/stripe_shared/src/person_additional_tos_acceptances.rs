#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PersonAdditionalTosAcceptances {
    pub account: stripe_shared::PersonAdditionalTosAcceptance,
}
#[cfg(feature = "min-ser")]
pub struct PersonAdditionalTosAcceptancesBuilder {
    account: Option<stripe_shared::PersonAdditionalTosAcceptance>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PersonAdditionalTosAcceptances {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonAdditionalTosAcceptances>,
        builder: PersonAdditionalTosAcceptancesBuilder,
    }

    impl Visitor for Place<PersonAdditionalTosAcceptances> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PersonAdditionalTosAcceptancesBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PersonAdditionalTosAcceptancesBuilder {
        type Out = PersonAdditionalTosAcceptances;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account" => Ok(Deserialize::begin(&mut self.account)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account = self.account.take()?;

            Some(Self::Out { account })
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

    impl ObjectDeser for PersonAdditionalTosAcceptances {
        type Builder = PersonAdditionalTosAcceptancesBuilder;
    }
};
