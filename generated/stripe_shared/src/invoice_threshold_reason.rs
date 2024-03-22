#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceThresholdReason {
    /// The total invoice amount threshold boundary if it triggered the threshold invoice.
    pub amount_gte: Option<i64>,
    /// Indicates which line items triggered a threshold invoice.
    pub item_reasons: Vec<stripe_shared::InvoiceItemThresholdReason>,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceThresholdReasonBuilder {
    amount_gte: Option<Option<i64>>,
    item_reasons: Option<Vec<stripe_shared::InvoiceItemThresholdReason>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceThresholdReason {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceThresholdReason>,
        builder: InvoiceThresholdReasonBuilder,
    }

    impl Visitor for Place<InvoiceThresholdReason> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceThresholdReasonBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceThresholdReasonBuilder {
        type Out = InvoiceThresholdReason;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_gte" => Ok(Deserialize::begin(&mut self.amount_gte)),
                "item_reasons" => Ok(Deserialize::begin(&mut self.item_reasons)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount_gte: Deserialize::default(), item_reasons: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_gte = self.amount_gte.take()?;
            let item_reasons = self.item_reasons.take()?;

            Some(Self::Out { amount_gte, item_reasons })
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

    impl ObjectDeser for InvoiceThresholdReason {
        type Builder = InvoiceThresholdReasonBuilder;
    }
};
