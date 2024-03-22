#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeAchDebit {
    pub bank_name: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeAchDebitBuilder {
    bank_name: Option<Option<String>>,
    country: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
    type_: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeAchDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAchDebit>,
        builder: SourceTypeAchDebitBuilder,
    }

    impl Visitor for Place<SourceTypeAchDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeAchDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeAchDebitBuilder {
        type Out = SourceTypeAchDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                bank_name: Deserialize::default(),
                country: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                routing_number: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_name = self.bank_name.take()?;
            let country = self.country.take()?;
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;
            let routing_number = self.routing_number.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { bank_name, country, fingerprint, last4, routing_number, type_ })
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

    impl ObjectDeser for SourceTypeAchDebit {
        type Builder = SourceTypeAchDebitBuilder;
    }
};
