#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RadarReviewResourceLocation {
    /// The city where the payment originated.
    pub city: Option<String>,
    /// Two-letter ISO code representing the country where the payment originated.
    pub country: Option<String>,
    /// The geographic latitude where the payment originated.
    pub latitude: Option<f64>,
    /// The geographic longitude where the payment originated.
    pub longitude: Option<f64>,
    /// The state/county/province/region where the payment originated.
    pub region: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct RadarReviewResourceLocationBuilder {
    city: Option<Option<String>>,
    country: Option<Option<String>>,
    latitude: Option<Option<f64>>,
    longitude: Option<Option<f64>>,
    region: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for RadarReviewResourceLocation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarReviewResourceLocation>,
        builder: RadarReviewResourceLocationBuilder,
    }

    impl Visitor for Place<RadarReviewResourceLocation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RadarReviewResourceLocationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RadarReviewResourceLocationBuilder {
        type Out = RadarReviewResourceLocation;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "city" => Ok(Deserialize::begin(&mut self.city)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "latitude" => Ok(Deserialize::begin(&mut self.latitude)),
                "longitude" => Ok(Deserialize::begin(&mut self.longitude)),
                "region" => Ok(Deserialize::begin(&mut self.region)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { city: Deserialize::default(), country: Deserialize::default(), latitude: Deserialize::default(), longitude: Deserialize::default(), region: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let city = self.city.take()?;
            let country = self.country.take()?;
            let latitude = self.latitude.take()?;
            let longitude = self.longitude.take()?;
            let region = self.region.take()?;

            Some(Self::Out { city, country, latitude, longitude, region })
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

    impl ObjectDeser for RadarReviewResourceLocation {
        type Builder = RadarReviewResourceLocationBuilder;
    }
};
