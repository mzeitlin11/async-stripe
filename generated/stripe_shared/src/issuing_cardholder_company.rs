#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardholderCompany {
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardholderCompanyBuilder {
    tax_id_provided: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardholderCompany {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderCompany>,
        builder: IssuingCardholderCompanyBuilder,
    }

    impl Visitor for Place<IssuingCardholderCompany> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardholderCompanyBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardholderCompanyBuilder {
        type Out = IssuingCardholderCompany;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "tax_id_provided" => Ok(Deserialize::begin(&mut self.tax_id_provided)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { tax_id_provided: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let tax_id_provided = self.tax_id_provided.take()?;

            Some(Self::Out { tax_id_provided })
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

    impl ObjectDeser for IssuingCardholderCompany {
        type Builder = IssuingCardholderCompanyBuilder;
    }
};
