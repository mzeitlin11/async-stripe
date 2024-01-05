#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingTransactionFlightData {
    /// The time that the flight departed.
    pub departure_at: Option<i64>,
    /// The name of the passenger.
    pub passenger_name: Option<String>,
    /// Whether the ticket is refundable.
    pub refundable: Option<bool>,
    /// The legs of the trip.
    pub segments: Option<Vec<stripe_shared::IssuingTransactionFlightDataLeg>>,
    /// The travel agency that issued the ticket.
    pub travel_agency: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTransactionFlightDataBuilder {
    departure_at: Option<Option<i64>>,
    passenger_name: Option<Option<String>>,
    refundable: Option<Option<bool>>,
    segments: Option<Option<Vec<stripe_shared::IssuingTransactionFlightDataLeg>>>,
    travel_agency: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionFlightData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFlightData>,
        builder: IssuingTransactionFlightDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionFlightData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTransactionFlightDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTransactionFlightDataBuilder {
        type Out = IssuingTransactionFlightData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "departure_at" => Ok(Deserialize::begin(&mut self.departure_at)),
                "passenger_name" => Ok(Deserialize::begin(&mut self.passenger_name)),
                "refundable" => Ok(Deserialize::begin(&mut self.refundable)),
                "segments" => Ok(Deserialize::begin(&mut self.segments)),
                "travel_agency" => Ok(Deserialize::begin(&mut self.travel_agency)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                departure_at: Deserialize::default(),
                passenger_name: Deserialize::default(),
                refundable: Deserialize::default(),
                segments: Deserialize::default(),
                travel_agency: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let departure_at = self.departure_at.take()?;
            let passenger_name = self.passenger_name.take()?;
            let refundable = self.refundable.take()?;
            let segments = self.segments.take()?;
            let travel_agency = self.travel_agency.take()?;

            Some(Self::Out { departure_at, passenger_name, refundable, segments, travel_agency })
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

    impl ObjectDeser for IssuingTransactionFlightData {
        type Builder = IssuingTransactionFlightDataBuilder;
    }
};
