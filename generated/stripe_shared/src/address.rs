#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Address {
    /// City, district, suburb, town, or village.
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    pub line2: Option<String>,
    /// ZIP or postal code.
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    pub state: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct AddressBuilder {
    city: Option<Option<String>>,
    country: Option<Option<String>>,
    line1: Option<Option<String>>,
    line2: Option<Option<String>>,
    postal_code: Option<Option<String>>,
    state: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Address {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Address>,
        builder: AddressBuilder,
    }

    impl Visitor for Place<Address> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AddressBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AddressBuilder {
        type Out = Address;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "city" => Ok(Deserialize::begin(&mut self.city)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "line1" => Ok(Deserialize::begin(&mut self.line1)),
                "line2" => Ok(Deserialize::begin(&mut self.line2)),
                "postal_code" => Ok(Deserialize::begin(&mut self.postal_code)),
                "state" => Ok(Deserialize::begin(&mut self.state)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                city: Deserialize::default(),
                country: Deserialize::default(),
                line1: Deserialize::default(),
                line2: Deserialize::default(),
                postal_code: Deserialize::default(),
                state: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let city = self.city.take()?;
            let country = self.country.take()?;
            let line1 = self.line1.take()?;
            let line2 = self.line2.take()?;
            let postal_code = self.postal_code.take()?;
            let state = self.state.take()?;

            Some(Self::Out { city, country, line1, line2, postal_code, state })
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

    impl ObjectDeser for Address {
        type Builder = AddressBuilder;
    }
};
