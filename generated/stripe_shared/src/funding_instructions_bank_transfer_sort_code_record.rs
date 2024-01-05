/// Sort Code Records contain U.K. bank account details per the sort code format.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferSortCodeRecord {
    /// The name of the person or business that owns the bank account
    pub account_holder_name: String,
    /// The account number
    pub account_number: String,
    /// The six-digit sort code
    pub sort_code: String,
}
#[cfg(feature = "min-ser")]
pub struct FundingInstructionsBankTransferSortCodeRecordBuilder {
    account_holder_name: Option<String>,
    account_number: Option<String>,
    sort_code: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransferSortCodeRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferSortCodeRecord>,
        builder: FundingInstructionsBankTransferSortCodeRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferSortCodeRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FundingInstructionsBankTransferSortCodeRecordBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferSortCodeRecordBuilder {
        type Out = FundingInstructionsBankTransferSortCodeRecord;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "account_holder_name" => Ok(Deserialize::begin(&mut self.account_holder_name)),
                "account_number" => Ok(Deserialize::begin(&mut self.account_number)),
                "sort_code" => Ok(Deserialize::begin(&mut self.sort_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account_holder_name: Deserialize::default(), account_number: Deserialize::default(), sort_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_holder_name = self.account_holder_name.take()?;
            let account_number = self.account_number.take()?;
            let sort_code = self.sort_code.take()?;

            Some(Self::Out { account_holder_name, account_number, sort_code })
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

    impl ObjectDeser for FundingInstructionsBankTransferSortCodeRecord {
        type Builder = FundingInstructionsBankTransferSortCodeRecordBuilder;
    }
};
