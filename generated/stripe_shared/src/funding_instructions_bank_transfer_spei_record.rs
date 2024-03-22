/// SPEI Records contain Mexico bank account details per the SPEI format.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferSpeiRecord {
    /// The three-digit bank code
    pub bank_code: String,
    /// The short banking institution name
    pub bank_name: String,
    /// The CLABE number
    pub clabe: String,
}
#[cfg(feature = "min-ser")]
pub struct FundingInstructionsBankTransferSpeiRecordBuilder {
    bank_code: Option<String>,
    bank_name: Option<String>,
    clabe: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransferSpeiRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferSpeiRecord>,
        builder: FundingInstructionsBankTransferSpeiRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferSpeiRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FundingInstructionsBankTransferSpeiRecordBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferSpeiRecordBuilder {
        type Out = FundingInstructionsBankTransferSpeiRecord;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "bank_code" => Ok(Deserialize::begin(&mut self.bank_code)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "clabe" => Ok(Deserialize::begin(&mut self.clabe)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank_code: Deserialize::default(), bank_name: Deserialize::default(), clabe: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_code = self.bank_code.take()?;
            let bank_name = self.bank_name.take()?;
            let clabe = self.clabe.take()?;

            Some(Self::Out { bank_code, bank_name, clabe })
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

    impl ObjectDeser for FundingInstructionsBankTransferSpeiRecord {
        type Builder = FundingInstructionsBankTransferSpeiRecordBuilder;
    }
};
