#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeSepaDebit {
    pub bank_code: Option<String>,
    pub branch_code: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub mandate_reference: Option<String>,
    pub mandate_url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeSepaDebitBuilder {
    bank_code: Option<Option<String>>,
    branch_code: Option<Option<String>>,
    country: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate_reference: Option<Option<String>>,
    mandate_url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeSepaDebit>,
        builder: SourceTypeSepaDebitBuilder,
    }

    impl Visitor for Place<SourceTypeSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeSepaDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeSepaDebitBuilder {
        type Out = SourceTypeSepaDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank_code" => Ok(Deserialize::begin(&mut self.bank_code)),
                "branch_code" => Ok(Deserialize::begin(&mut self.branch_code)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "mandate_reference" => Ok(Deserialize::begin(&mut self.mandate_reference)),
                "mandate_url" => Ok(Deserialize::begin(&mut self.mandate_url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                bank_code: Deserialize::default(),
                branch_code: Deserialize::default(),
                country: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                mandate_reference: Deserialize::default(),
                mandate_url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_code = self.bank_code.take()?;
            let branch_code = self.branch_code.take()?;
            let country = self.country.take()?;
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;
            let mandate_reference = self.mandate_reference.take()?;
            let mandate_url = self.mandate_url.take()?;

            Some(Self::Out { bank_code, branch_code, country, fingerprint, last4, mandate_reference, mandate_url })
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

    impl ObjectDeser for SourceTypeSepaDebit {
        type Builder = SourceTypeSepaDebitBuilder;
    }
};
