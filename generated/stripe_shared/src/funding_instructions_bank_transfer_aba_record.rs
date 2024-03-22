/// ABA Records contain U.S. bank account details per the ABA format.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferAbaRecord {
    /// The ABA account number
    pub account_number: String,
    /// The bank name
    pub bank_name: String,
    /// The ABA routing number
    pub routing_number: String,
}
#[cfg(feature = "min-ser")]
pub struct FundingInstructionsBankTransferAbaRecordBuilder {
    account_number: Option<String>,
    bank_name: Option<String>,
    routing_number: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransferAbaRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferAbaRecord>,
        builder: FundingInstructionsBankTransferAbaRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferAbaRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FundingInstructionsBankTransferAbaRecordBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferAbaRecordBuilder {
        type Out = FundingInstructionsBankTransferAbaRecord;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account_number" => Ok(Deserialize::begin(&mut self.account_number)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account_number: Deserialize::default(), bank_name: Deserialize::default(), routing_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_number = self.account_number.take()?;
            let bank_name = self.bank_name.take()?;
            let routing_number = self.routing_number.take()?;

            Some(Self::Out { account_number, bank_name, routing_number })
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

    impl ObjectDeser for FundingInstructionsBankTransferAbaRecord {
        type Builder = FundingInstructionsBankTransferAbaRecordBuilder;
    }
};
