#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CountrySpecVerificationFieldDetails {
    /// Additional fields which are only required for some users.
    pub additional: Vec<String>,
    /// Fields which every account must eventually provide.
    pub minimum: Vec<String>,
}
#[cfg(feature = "min-ser")]
pub struct CountrySpecVerificationFieldDetailsBuilder {
    additional: Option<Vec<String>>,
    minimum: Option<Vec<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CountrySpecVerificationFieldDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CountrySpecVerificationFieldDetails>,
        builder: CountrySpecVerificationFieldDetailsBuilder,
    }

    impl Visitor for Place<CountrySpecVerificationFieldDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CountrySpecVerificationFieldDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CountrySpecVerificationFieldDetailsBuilder {
        type Out = CountrySpecVerificationFieldDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "additional" => Ok(Deserialize::begin(&mut self.additional)),
                "minimum" => Ok(Deserialize::begin(&mut self.minimum)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { additional: Deserialize::default(), minimum: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let additional = self.additional.take()?;
            let minimum = self.minimum.take()?;

            Some(Self::Out { additional, minimum })
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

    impl ObjectDeser for CountrySpecVerificationFieldDetails {
        type Builder = CountrySpecVerificationFieldDetailsBuilder;
    }
};
