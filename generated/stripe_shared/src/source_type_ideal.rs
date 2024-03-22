#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeIdeal {
    pub bank: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeIdealBuilder {
    bank: Option<Option<String>>,
    bic: Option<Option<String>>,
    iban_last4: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeIdeal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeIdeal>,
        builder: SourceTypeIdealBuilder,
    }

    impl Visitor for Place<SourceTypeIdeal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeIdealBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeIdealBuilder {
        type Out = SourceTypeIdeal;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "bank" => Ok(Deserialize::begin(&mut self.bank)),
                "bic" => Ok(Deserialize::begin(&mut self.bic)),
                "iban_last4" => Ok(Deserialize::begin(&mut self.iban_last4)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank: Deserialize::default(), bic: Deserialize::default(), iban_last4: Deserialize::default(), statement_descriptor: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank = self.bank.take()?;
            let bic = self.bic.take()?;
            let iban_last4 = self.iban_last4.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;

            Some(Self::Out { bank, bic, iban_last4, statement_descriptor })
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

    impl ObjectDeser for SourceTypeIdeal {
        type Builder = SourceTypeIdealBuilder;
    }
};
