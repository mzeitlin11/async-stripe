#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceLineItemPeriod {
    /// The end of the period, which must be greater than or equal to the start. This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period. This value is inclusive.
    pub start: stripe_types::Timestamp,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceLineItemPeriodBuilder {
    end: Option<stripe_types::Timestamp>,
    start: Option<stripe_types::Timestamp>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceLineItemPeriod {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceLineItemPeriod>,
        builder: InvoiceLineItemPeriodBuilder,
    }

    impl Visitor for Place<InvoiceLineItemPeriod> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceLineItemPeriodBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceLineItemPeriodBuilder {
        type Out = InvoiceLineItemPeriod;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "end" => Ok(Deserialize::begin(&mut self.end)),
                "start" => Ok(Deserialize::begin(&mut self.start)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { end: Deserialize::default(), start: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let end = self.end.take()?;
            let start = self.start.take()?;

            Some(Self::Out { end, start })
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

    impl ObjectDeser for InvoiceLineItemPeriod {
        type Builder = InvoiceLineItemPeriodBuilder;
    }
};
