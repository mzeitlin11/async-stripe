#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingTransactionFlightDataLeg {
    /// The three-letter IATA airport code of the flight's destination.
    pub arrival_airport_code: Option<String>,
    /// The airline carrier code.
    pub carrier: Option<String>,
    /// The three-letter IATA airport code that the flight departed from.
    pub departure_airport_code: Option<String>,
    /// The flight number.
    pub flight_number: Option<String>,
    /// The flight's service class.
    pub service_class: Option<String>,
    /// Whether a stopover is allowed on this flight.
    pub stopover_allowed: Option<bool>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTransactionFlightDataLegBuilder {
    arrival_airport_code: Option<Option<String>>,
    carrier: Option<Option<String>>,
    departure_airport_code: Option<Option<String>>,
    flight_number: Option<Option<String>>,
    service_class: Option<Option<String>>,
    stopover_allowed: Option<Option<bool>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionFlightDataLeg {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFlightDataLeg>,
        builder: IssuingTransactionFlightDataLegBuilder,
    }

    impl Visitor for Place<IssuingTransactionFlightDataLeg> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTransactionFlightDataLegBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTransactionFlightDataLegBuilder {
        type Out = IssuingTransactionFlightDataLeg;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "arrival_airport_code" => Ok(Deserialize::begin(&mut self.arrival_airport_code)),
                "carrier" => Ok(Deserialize::begin(&mut self.carrier)),
                "departure_airport_code" => Ok(Deserialize::begin(&mut self.departure_airport_code)),
                "flight_number" => Ok(Deserialize::begin(&mut self.flight_number)),
                "service_class" => Ok(Deserialize::begin(&mut self.service_class)),
                "stopover_allowed" => Ok(Deserialize::begin(&mut self.stopover_allowed)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                arrival_airport_code: Deserialize::default(),
                carrier: Deserialize::default(),
                departure_airport_code: Deserialize::default(),
                flight_number: Deserialize::default(),
                service_class: Deserialize::default(),
                stopover_allowed: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let arrival_airport_code = self.arrival_airport_code.take()?;
            let carrier = self.carrier.take()?;
            let departure_airport_code = self.departure_airport_code.take()?;
            let flight_number = self.flight_number.take()?;
            let service_class = self.service_class.take()?;
            let stopover_allowed = self.stopover_allowed.take()?;

            Some(Self::Out { arrival_airport_code, carrier, departure_airport_code, flight_number, service_class, stopover_allowed })
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

    impl ObjectDeser for IssuingTransactionFlightDataLeg {
        type Builder = IssuingTransactionFlightDataLegBuilder;
    }
};
