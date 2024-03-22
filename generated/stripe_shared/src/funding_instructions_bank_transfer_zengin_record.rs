/// Zengin Records contain Japan bank account details per the Zengin format.
#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferZenginRecord {
    /// The account holder name
    pub account_holder_name: Option<String>,
    /// The account number
    pub account_number: Option<String>,
    /// The bank account type. In Japan, this can only be `futsu` or `toza`.
    pub account_type: Option<String>,
    /// The bank code of the account
    pub bank_code: Option<String>,
    /// The bank name of the account
    pub bank_name: Option<String>,
    /// The branch code of the account
    pub branch_code: Option<String>,
    /// The branch name of the account
    pub branch_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct FundingInstructionsBankTransferZenginRecordBuilder {
    account_holder_name: Option<Option<String>>,
    account_number: Option<Option<String>>,
    account_type: Option<Option<String>>,
    bank_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    branch_code: Option<Option<String>>,
    branch_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransferZenginRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferZenginRecord>,
        builder: FundingInstructionsBankTransferZenginRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferZenginRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FundingInstructionsBankTransferZenginRecordBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferZenginRecordBuilder {
        type Out = FundingInstructionsBankTransferZenginRecord;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account_holder_name" => Ok(Deserialize::begin(&mut self.account_holder_name)),
                "account_number" => Ok(Deserialize::begin(&mut self.account_number)),
                "account_type" => Ok(Deserialize::begin(&mut self.account_type)),
                "bank_code" => Ok(Deserialize::begin(&mut self.bank_code)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "branch_code" => Ok(Deserialize::begin(&mut self.branch_code)),
                "branch_name" => Ok(Deserialize::begin(&mut self.branch_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account_holder_name: Deserialize::default(),
                account_number: Deserialize::default(),
                account_type: Deserialize::default(),
                bank_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                branch_code: Deserialize::default(),
                branch_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_holder_name = self.account_holder_name.take()?;
            let account_number = self.account_number.take()?;
            let account_type = self.account_type.take()?;
            let bank_code = self.bank_code.take()?;
            let bank_name = self.bank_name.take()?;
            let branch_code = self.branch_code.take()?;
            let branch_name = self.branch_name.take()?;

            Some(Self::Out { account_holder_name, account_number, account_type, bank_code, bank_name, branch_code, branch_name })
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

    impl ObjectDeser for FundingInstructionsBankTransferZenginRecord {
        type Builder = FundingInstructionsBankTransferZenginRecordBuilder;
    }
};
