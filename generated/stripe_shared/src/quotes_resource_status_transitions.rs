#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct QuotesResourceStatusTransitions {
    /// The time that the quote was accepted. Measured in seconds since Unix epoch.
    pub accepted_at: Option<stripe_types::Timestamp>,
    /// The time that the quote was canceled. Measured in seconds since Unix epoch.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// The time that the quote was finalized. Measured in seconds since Unix epoch.
    pub finalized_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct QuotesResourceStatusTransitionsBuilder {
    accepted_at: Option<Option<stripe_types::Timestamp>>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    finalized_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceStatusTransitions>,
        builder: QuotesResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<QuotesResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: QuotesResourceStatusTransitionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for QuotesResourceStatusTransitionsBuilder {
        type Out = QuotesResourceStatusTransitions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "accepted_at" => Ok(Deserialize::begin(&mut self.accepted_at)),
                "canceled_at" => Ok(Deserialize::begin(&mut self.canceled_at)),
                "finalized_at" => Ok(Deserialize::begin(&mut self.finalized_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { accepted_at: Deserialize::default(), canceled_at: Deserialize::default(), finalized_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let accepted_at = self.accepted_at.take()?;
            let canceled_at = self.canceled_at.take()?;
            let finalized_at = self.finalized_at.take()?;

            Some(Self::Out { accepted_at, canceled_at, finalized_at })
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

    impl ObjectDeser for QuotesResourceStatusTransitions {
        type Builder = QuotesResourceStatusTransitionsBuilder;
    }
};
