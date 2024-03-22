#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountBacsDebitPaymentsSettings {
    /// The Bacs Direct Debit display name for this account.
    /// For payments made with Bacs Direct Debit, this name appears on the mandate as the statement descriptor.
    /// Mobile banking apps display it as the name of the business.
    /// To use custom branding, set the Bacs Direct Debit Display Name during or right after creation.
    /// Custom branding incurs an additional monthly fee for the platform.
    /// The fee appears 5 business days after requesting Bacs.
    /// If you don't set the display name before requesting Bacs capability, it's automatically set as "Stripe" and the account is onboarded to Stripe branding, which is free.
    pub display_name: Option<String>,
    /// The Bacs Direct Debit Service user number for this account.
    /// For payments made with Bacs Direct Debit, this number is a unique identifier of the account with our banking partners.
    pub service_user_number: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountBacsDebitPaymentsSettingsBuilder {
    display_name: Option<Option<String>>,
    service_user_number: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountBacsDebitPaymentsSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountBacsDebitPaymentsSettings>,
        builder: AccountBacsDebitPaymentsSettingsBuilder,
    }

    impl Visitor for Place<AccountBacsDebitPaymentsSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountBacsDebitPaymentsSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountBacsDebitPaymentsSettingsBuilder {
        type Out = AccountBacsDebitPaymentsSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "display_name" => Ok(Deserialize::begin(&mut self.display_name)),
                "service_user_number" => Ok(Deserialize::begin(&mut self.service_user_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { display_name: Deserialize::default(), service_user_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let display_name = self.display_name.take()?;
            let service_user_number = self.service_user_number.take()?;

            Some(Self::Out { display_name, service_user_number })
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

    impl ObjectDeser for AccountBacsDebitPaymentsSettings {
        type Builder = AccountBacsDebitPaymentsSettingsBuilder;
    }
};
