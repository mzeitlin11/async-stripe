#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardShippingCustoms {
    /// A registration number used for customs in Europe.
    /// See [<https://www.gov.uk/eori>](https://www.gov.uk/eori) for the UK and [<https://ec.europa.eu/taxation_customs/business/customs-procedures-import-and-export/customs-procedures/economic-operators-registration-and-identification-number-eori_en>](https://ec.europa.eu/taxation_customs/business/customs-procedures-import-and-export/customs-procedures/economic-operators-registration-and-identification-number-eori_en) for the EU.
    pub eori_number: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardShippingCustomsBuilder {
    eori_number: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardShippingCustoms {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardShippingCustoms>,
        builder: IssuingCardShippingCustomsBuilder,
    }

    impl Visitor for Place<IssuingCardShippingCustoms> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardShippingCustomsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardShippingCustomsBuilder {
        type Out = IssuingCardShippingCustoms;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "eori_number" => Ok(Deserialize::begin(&mut self.eori_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { eori_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let eori_number = self.eori_number.take()?;

            Some(Self::Out { eori_number })
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

    impl ObjectDeser for IssuingCardShippingCustoms {
        type Builder = IssuingCardShippingCustomsBuilder;
    }
};
