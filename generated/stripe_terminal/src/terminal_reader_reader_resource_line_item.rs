/// Represents a line item to be displayed on the reader
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceLineItem {
    /// The amount of the line item.
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Description of the line item.
    pub description: String,
    /// The quantity of the line item.
    pub quantity: u64,
}
#[cfg(feature = "min-ser")]
pub struct TerminalReaderReaderResourceLineItemBuilder {
    amount: Option<i64>,
    description: Option<String>,
    quantity: Option<u64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalReaderReaderResourceLineItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceLineItem>,
        builder: TerminalReaderReaderResourceLineItemBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceLineItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TerminalReaderReaderResourceLineItemBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceLineItemBuilder {
        type Out = TerminalReaderReaderResourceLineItem;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), description: Deserialize::default(), quantity: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let description = self.description.take()?;
            let quantity = self.quantity.take()?;

            Some(Self::Out { amount, description, quantity })
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

    impl ObjectDeser for TerminalReaderReaderResourceLineItem {
        type Builder = TerminalReaderReaderResourceLineItemBuilder;
    }
};
