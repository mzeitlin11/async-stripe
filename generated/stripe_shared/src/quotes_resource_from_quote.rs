#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct QuotesResourceFromQuote {
    /// Whether this quote is a revision of a different quote.
    pub is_revision: bool,
    /// The quote that was cloned.
    pub quote: stripe_types::Expandable<stripe_shared::Quote>,
}
#[cfg(feature = "min-ser")]
pub struct QuotesResourceFromQuoteBuilder {
    is_revision: Option<bool>,
    quote: Option<stripe_types::Expandable<stripe_shared::Quote>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceFromQuote {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceFromQuote>,
        builder: QuotesResourceFromQuoteBuilder,
    }

    impl Visitor for Place<QuotesResourceFromQuote> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: QuotesResourceFromQuoteBuilder::deser_default() }))
        }
    }

    impl MapBuilder for QuotesResourceFromQuoteBuilder {
        type Out = QuotesResourceFromQuote;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "is_revision" => Ok(Deserialize::begin(&mut self.is_revision)),
                "quote" => Ok(Deserialize::begin(&mut self.quote)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { is_revision: Deserialize::default(), quote: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let is_revision = self.is_revision.take()?;
            let quote = self.quote.take()?;

            Some(Self::Out { is_revision, quote })
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

    impl ObjectDeser for QuotesResourceFromQuote {
        type Builder = QuotesResourceFromQuoteBuilder;
    }
};
