#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceItemThresholdReason {
    /// The IDs of the line items that triggered the threshold invoice.
    pub line_item_ids: Vec<String>,
    /// The quantity threshold boundary that applied to the given line item.
    pub usage_gte: i64,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceItemThresholdReasonBuilder {
    line_item_ids: Option<Vec<String>>,
    usage_gte: Option<i64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceItemThresholdReason {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceItemThresholdReason>,
        builder: InvoiceItemThresholdReasonBuilder,
    }

    impl Visitor for Place<InvoiceItemThresholdReason> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceItemThresholdReasonBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceItemThresholdReasonBuilder {
        type Out = InvoiceItemThresholdReason;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "line_item_ids" => Ok(Deserialize::begin(&mut self.line_item_ids)),
                "usage_gte" => Ok(Deserialize::begin(&mut self.usage_gte)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { line_item_ids: Deserialize::default(), usage_gte: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let line_item_ids = self.line_item_ids.take()?;
            let usage_gte = self.usage_gte.take()?;

            Some(Self::Out { line_item_ids, usage_gte })
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

    impl ObjectDeser for InvoiceItemThresholdReason {
        type Builder = InvoiceItemThresholdReasonBuilder;
    }
};
