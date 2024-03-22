#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
    /// Timestamp describing when the Transaction changed status to `posted`.
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when the Transaction changed status to `void`.
    pub void_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder {
    posted_at: Option<Option<stripe_types::Timestamp>>,
    void_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions>,
        builder: TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder {
        type Out = TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "posted_at" => Ok(Deserialize::begin(&mut self.posted_at)),
                "void_at" => Ok(Deserialize::begin(&mut self.void_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { posted_at: Deserialize::default(), void_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let posted_at = self.posted_at.take()?;
            let void_at = self.void_at.take()?;

            Some(Self::Out { posted_at, void_at })
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

    impl ObjectDeser for TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
        type Builder = TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder;
    }
};
