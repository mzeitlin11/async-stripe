#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountTreasurySettings {
    pub tos_acceptance: Option<stripe_shared::AccountTermsOfService>,
}
#[cfg(feature = "min-ser")]
pub struct AccountTreasurySettingsBuilder {
    tos_acceptance: Option<Option<stripe_shared::AccountTermsOfService>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountTreasurySettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountTreasurySettings>,
        builder: AccountTreasurySettingsBuilder,
    }

    impl Visitor for Place<AccountTreasurySettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountTreasurySettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountTreasurySettingsBuilder {
        type Out = AccountTreasurySettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "tos_acceptance" => Ok(Deserialize::begin(&mut self.tos_acceptance)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { tos_acceptance: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let tos_acceptance = self.tos_acceptance.take()?;

            Some(Self::Out { tos_acceptance })
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

    impl ObjectDeser for AccountTreasurySettings {
        type Builder = AccountTreasurySettingsBuilder;
    }
};
