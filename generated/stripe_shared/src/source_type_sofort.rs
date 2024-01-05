#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeSofort {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub country: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeSofortBuilder {
    bank_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
    country: Option<Option<String>>,
    iban_last4: Option<Option<String>>,
    preferred_language: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeSofort {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeSofort>,
        builder: SourceTypeSofortBuilder,
    }

    impl Visitor for Place<SourceTypeSofort> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeSofortBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeSofortBuilder {
        type Out = SourceTypeSofort;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank_code" => Ok(Deserialize::begin(&mut self.bank_code)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "bic" => Ok(Deserialize::begin(&mut self.bic)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "iban_last4" => Ok(Deserialize::begin(&mut self.iban_last4)),
                "preferred_language" => Ok(Deserialize::begin(&mut self.preferred_language)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                bank_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                bic: Deserialize::default(),
                country: Deserialize::default(),
                iban_last4: Deserialize::default(),
                preferred_language: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_code = self.bank_code.take()?;
            let bank_name = self.bank_name.take()?;
            let bic = self.bic.take()?;
            let country = self.country.take()?;
            let iban_last4 = self.iban_last4.take()?;
            let preferred_language = self.preferred_language.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;

            Some(Self::Out { bank_code, bank_name, bic, country, iban_last4, preferred_language, statement_descriptor })
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

    impl ObjectDeser for SourceTypeSofort {
        type Builder = SourceTypeSofortBuilder;
    }
};
