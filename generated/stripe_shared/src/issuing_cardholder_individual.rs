#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardholderIndividual {
    /// Information related to the card_issuing program for this cardholder.
    pub card_issuing: Option<stripe_shared::IssuingCardholderCardIssuing>,
    /// The date of birth of this cardholder.
    pub dob: Option<stripe_shared::IssuingCardholderIndividualDob>,
    /// The first name of this cardholder.
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    pub first_name: Option<String>,
    /// The last name of this cardholder.
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    pub last_name: Option<String>,
    /// Government-issued ID document for this cardholder.
    pub verification: Option<stripe_shared::IssuingCardholderVerification>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardholderIndividualBuilder {
    card_issuing: Option<Option<stripe_shared::IssuingCardholderCardIssuing>>,
    dob: Option<Option<stripe_shared::IssuingCardholderIndividualDob>>,
    first_name: Option<Option<String>>,
    last_name: Option<Option<String>>,
    verification: Option<Option<stripe_shared::IssuingCardholderVerification>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardholderIndividual {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderIndividual>,
        builder: IssuingCardholderIndividualBuilder,
    }

    impl Visitor for Place<IssuingCardholderIndividual> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardholderIndividualBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardholderIndividualBuilder {
        type Out = IssuingCardholderIndividual;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "card_issuing" => Ok(Deserialize::begin(&mut self.card_issuing)),
                "dob" => Ok(Deserialize::begin(&mut self.dob)),
                "first_name" => Ok(Deserialize::begin(&mut self.first_name)),
                "last_name" => Ok(Deserialize::begin(&mut self.last_name)),
                "verification" => Ok(Deserialize::begin(&mut self.verification)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { card_issuing: Deserialize::default(), dob: Deserialize::default(), first_name: Deserialize::default(), last_name: Deserialize::default(), verification: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let card_issuing = self.card_issuing.take()?;
            let dob = self.dob.take()?;
            let first_name = self.first_name.take()?;
            let last_name = self.last_name.take()?;
            let verification = self.verification.take()?;

            Some(Self::Out { card_issuing, dob, first_name, last_name, verification })
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

    impl ObjectDeser for IssuingCardholderIndividual {
        type Builder = IssuingCardholderIndividualBuilder;
    }
};
