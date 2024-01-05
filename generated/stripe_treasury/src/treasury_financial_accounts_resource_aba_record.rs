/// ABA Records contain U.S. bank account details per the ABA format.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceAbaRecord {
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    /// The account number.
    pub account_number: Option<String>,
    /// The last four characters of the account number.
    pub account_number_last4: String,
    /// Name of the bank.
    pub bank_name: String,
    /// Routing number for the account.
    pub routing_number: String,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryFinancialAccountsResourceAbaRecordBuilder {
    account_holder_name: Option<String>,
    account_number: Option<Option<String>>,
    account_number_last4: Option<String>,
    bank_name: Option<String>,
    routing_number: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountsResourceAbaRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceAbaRecord>,
        builder: TreasuryFinancialAccountsResourceAbaRecordBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceAbaRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryFinancialAccountsResourceAbaRecordBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceAbaRecordBuilder {
        type Out = TreasuryFinancialAccountsResourceAbaRecord;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "account_holder_name" => Ok(Deserialize::begin(&mut self.account_holder_name)),
                "account_number" => Ok(Deserialize::begin(&mut self.account_number)),
                "account_number_last4" => Ok(Deserialize::begin(&mut self.account_number_last4)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account_holder_name: Deserialize::default(),
                account_number: Deserialize::default(),
                account_number_last4: Deserialize::default(),
                bank_name: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_holder_name = self.account_holder_name.take()?;
            let account_number = self.account_number.take()?;
            let account_number_last4 = self.account_number_last4.take()?;
            let bank_name = self.bank_name.take()?;
            let routing_number = self.routing_number.take()?;

            Some(Self::Out { account_holder_name, account_number, account_number_last4, bank_name, routing_number })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceAbaRecord {
        type Builder = TreasuryFinancialAccountsResourceAbaRecordBuilder;
    }
};
