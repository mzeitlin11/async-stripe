#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CountrySpecVerificationFields {
    pub company: stripe_connect::CountrySpecVerificationFieldDetails,
    pub individual: stripe_connect::CountrySpecVerificationFieldDetails,
}
#[cfg(feature = "min-ser")]
pub struct CountrySpecVerificationFieldsBuilder {
    company: Option<stripe_connect::CountrySpecVerificationFieldDetails>,
    individual: Option<stripe_connect::CountrySpecVerificationFieldDetails>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CountrySpecVerificationFields {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CountrySpecVerificationFields>,
        builder: CountrySpecVerificationFieldsBuilder,
    }

    impl Visitor for Place<CountrySpecVerificationFields> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CountrySpecVerificationFieldsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CountrySpecVerificationFieldsBuilder {
        type Out = CountrySpecVerificationFields;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "company" => Ok(Deserialize::begin(&mut self.company)),
                "individual" => Ok(Deserialize::begin(&mut self.individual)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { company: Deserialize::default(), individual: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let company = self.company.take()?;
            let individual = self.individual.take()?;

            Some(Self::Out { company, individual })
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

    impl ObjectDeser for CountrySpecVerificationFields {
        type Builder = CountrySpecVerificationFieldsBuilder;
    }
};
