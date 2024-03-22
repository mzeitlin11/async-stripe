#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryReceivedDebitsResourceStatusTransitions {
    /// Timestamp describing when the DebitReversal changed status to `completed`.
    pub completed_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryReceivedDebitsResourceStatusTransitionsBuilder {
    completed_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryReceivedDebitsResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedDebitsResourceStatusTransitions>,
        builder: TreasuryReceivedDebitsResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedDebitsResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryReceivedDebitsResourceStatusTransitionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryReceivedDebitsResourceStatusTransitionsBuilder {
        type Out = TreasuryReceivedDebitsResourceStatusTransitions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "completed_at" => Ok(Deserialize::begin(&mut self.completed_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { completed_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let completed_at = self.completed_at.take()?;

            Some(Self::Out { completed_at })
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

    impl ObjectDeser for TreasuryReceivedDebitsResourceStatusTransitions {
        type Builder = TreasuryReceivedDebitsResourceStatusTransitionsBuilder;
    }
};
