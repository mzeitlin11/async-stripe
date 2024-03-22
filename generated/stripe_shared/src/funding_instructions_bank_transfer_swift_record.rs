/// SWIFT Records contain U.S. bank account details per the SWIFT format.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferSwiftRecord {
    /// The account number
    pub account_number: String,
    /// The bank name
    pub bank_name: String,
    /// The SWIFT code
    pub swift_code: String,
}
#[cfg(feature = "min-ser")]
pub struct FundingInstructionsBankTransferSwiftRecordBuilder {
    account_number: Option<String>,
    bank_name: Option<String>,
    swift_code: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransferSwiftRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferSwiftRecord>,
        builder: FundingInstructionsBankTransferSwiftRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferSwiftRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FundingInstructionsBankTransferSwiftRecordBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferSwiftRecordBuilder {
        type Out = FundingInstructionsBankTransferSwiftRecord;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account_number" => Ok(Deserialize::begin(&mut self.account_number)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "swift_code" => Ok(Deserialize::begin(&mut self.swift_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account_number: Deserialize::default(), bank_name: Deserialize::default(), swift_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_number = self.account_number.take()?;
            let bank_name = self.bank_name.take()?;
            let swift_code = self.swift_code.take()?;

            Some(Self::Out { account_number, bank_name, swift_code })
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

    impl ObjectDeser for FundingInstructionsBankTransferSwiftRecord {
        type Builder = FundingInstructionsBankTransferSwiftRecordBuilder;
    }
};
