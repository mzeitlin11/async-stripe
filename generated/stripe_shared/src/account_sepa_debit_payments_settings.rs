#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountSepaDebitPaymentsSettings {
    /// SEPA creditor identifier that identifies the company making the payment.
    pub creditor_id: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountSepaDebitPaymentsSettingsBuilder {
    creditor_id: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountSepaDebitPaymentsSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountSepaDebitPaymentsSettings>,
        builder: AccountSepaDebitPaymentsSettingsBuilder,
    }

    impl Visitor for Place<AccountSepaDebitPaymentsSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountSepaDebitPaymentsSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountSepaDebitPaymentsSettingsBuilder {
        type Out = AccountSepaDebitPaymentsSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "creditor_id" => Ok(Deserialize::begin(&mut self.creditor_id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { creditor_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let creditor_id = self.creditor_id.take()?;

            Some(Self::Out { creditor_id })
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

    impl ObjectDeser for AccountSepaDebitPaymentsSettings {
        type Builder = AccountSepaDebitPaymentsSettingsBuilder;
    }
};
