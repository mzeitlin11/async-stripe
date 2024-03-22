#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
    /// Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub province: String,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder {
    province: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard>,
        builder: TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "province" => Ok(Deserialize::begin(&mut self.province)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { province: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let province = self.province.take()?;

            Some(Self::Out { province })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
        type Builder = TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder;
    }
};
