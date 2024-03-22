/// Iban Records contain E.U. bank account details per the SEPA format.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferIbanRecord {
    /// The name of the person or business that owns the bank account
    pub account_holder_name: String,
    /// The BIC/SWIFT code of the account.
    pub bic: String,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// The IBAN of the account.
    pub iban: String,
}
#[cfg(feature = "min-ser")]
pub struct FundingInstructionsBankTransferIbanRecordBuilder {
    account_holder_name: Option<String>,
    bic: Option<String>,
    country: Option<String>,
    iban: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransferIbanRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferIbanRecord>,
        builder: FundingInstructionsBankTransferIbanRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferIbanRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FundingInstructionsBankTransferIbanRecordBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferIbanRecordBuilder {
        type Out = FundingInstructionsBankTransferIbanRecord;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account_holder_name" => Ok(Deserialize::begin(&mut self.account_holder_name)),
                "bic" => Ok(Deserialize::begin(&mut self.bic)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "iban" => Ok(Deserialize::begin(&mut self.iban)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account_holder_name: Deserialize::default(), bic: Deserialize::default(), country: Deserialize::default(), iban: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_holder_name = self.account_holder_name.take()?;
            let bic = self.bic.take()?;
            let country = self.country.take()?;
            let iban = self.iban.take()?;

            Some(Self::Out { account_holder_name, bic, country, iban })
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

    impl ObjectDeser for FundingInstructionsBankTransferIbanRecord {
        type Builder = FundingInstructionsBankTransferIbanRecordBuilder;
    }
};
