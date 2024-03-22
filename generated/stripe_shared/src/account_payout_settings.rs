#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountPayoutSettings {
    /// A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account.
    /// See our [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details.
    /// Default value is `false` for Custom accounts, otherwise `true`.
    pub debit_negative_balances: bool,
    pub schedule: stripe_shared::TransferSchedule,
    /// The text that appears on the bank account statement for payouts.
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountPayoutSettingsBuilder {
    debit_negative_balances: Option<bool>,
    schedule: Option<stripe_shared::TransferSchedule>,
    statement_descriptor: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountPayoutSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountPayoutSettings>,
        builder: AccountPayoutSettingsBuilder,
    }

    impl Visitor for Place<AccountPayoutSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountPayoutSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountPayoutSettingsBuilder {
        type Out = AccountPayoutSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "debit_negative_balances" => Ok(Deserialize::begin(&mut self.debit_negative_balances)),
                "schedule" => Ok(Deserialize::begin(&mut self.schedule)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { debit_negative_balances: Deserialize::default(), schedule: Deserialize::default(), statement_descriptor: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let debit_negative_balances = self.debit_negative_balances.take()?;
            let schedule = self.schedule.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;

            Some(Self::Out { debit_negative_balances, schedule, statement_descriptor })
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

    impl ObjectDeser for AccountPayoutSettings {
        type Builder = AccountPayoutSettingsBuilder;
    }
};
