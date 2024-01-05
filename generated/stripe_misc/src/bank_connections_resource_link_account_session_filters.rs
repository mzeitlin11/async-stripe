#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankConnectionsResourceLinkAccountSessionFilters {
    /// List of countries from which to filter accounts.
    pub countries: Option<Vec<String>>,
}
#[cfg(feature = "min-ser")]
pub struct BankConnectionsResourceLinkAccountSessionFiltersBuilder {
    countries: Option<Option<Vec<String>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BankConnectionsResourceLinkAccountSessionFilters {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceLinkAccountSessionFilters>,
        builder: BankConnectionsResourceLinkAccountSessionFiltersBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceLinkAccountSessionFilters> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: BankConnectionsResourceLinkAccountSessionFiltersBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BankConnectionsResourceLinkAccountSessionFiltersBuilder {
        type Out = BankConnectionsResourceLinkAccountSessionFilters;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "countries" => Ok(Deserialize::begin(&mut self.countries)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { countries: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let countries = self.countries.take()?;

            Some(Self::Out { countries })
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

    impl ObjectDeser for BankConnectionsResourceLinkAccountSessionFilters {
        type Builder = BankConnectionsResourceLinkAccountSessionFiltersBuilder;
    }
};
