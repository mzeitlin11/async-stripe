#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeAcssDebit {
    pub bank_address_city: Option<String>,
    pub bank_address_line_1: Option<String>,
    pub bank_address_line_2: Option<String>,
    pub bank_address_postal_code: Option<String>,
    pub bank_name: Option<String>,
    pub category: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeAcssDebitBuilder {
    bank_address_city: Option<Option<String>>,
    bank_address_line_1: Option<Option<String>>,
    bank_address_line_2: Option<Option<String>>,
    bank_address_postal_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    category: Option<Option<String>>,
    country: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAcssDebit>,
        builder: SourceTypeAcssDebitBuilder,
    }

    impl Visitor for Place<SourceTypeAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeAcssDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeAcssDebitBuilder {
        type Out = SourceTypeAcssDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank_address_city" => Ok(Deserialize::begin(&mut self.bank_address_city)),
                "bank_address_line_1" => Ok(Deserialize::begin(&mut self.bank_address_line_1)),
                "bank_address_line_2" => Ok(Deserialize::begin(&mut self.bank_address_line_2)),
                "bank_address_postal_code" => Ok(Deserialize::begin(&mut self.bank_address_postal_code)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "category" => Ok(Deserialize::begin(&mut self.category)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                bank_address_city: Deserialize::default(),
                bank_address_line_1: Deserialize::default(),
                bank_address_line_2: Deserialize::default(),
                bank_address_postal_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                category: Deserialize::default(),
                country: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_address_city = self.bank_address_city.take()?;
            let bank_address_line_1 = self.bank_address_line_1.take()?;
            let bank_address_line_2 = self.bank_address_line_2.take()?;
            let bank_address_postal_code = self.bank_address_postal_code.take()?;
            let bank_name = self.bank_name.take()?;
            let category = self.category.take()?;
            let country = self.country.take()?;
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;
            let routing_number = self.routing_number.take()?;

            Some(Self::Out { bank_address_city, bank_address_line_1, bank_address_line_2, bank_address_postal_code, bank_name, category, country, fingerprint, last4, routing_number })
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

    impl ObjectDeser for SourceTypeAcssDebit {
        type Builder = SourceTypeAcssDebitBuilder;
    }
};
