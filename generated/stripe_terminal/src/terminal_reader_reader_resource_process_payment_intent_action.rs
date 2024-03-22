/// Represents a reader action to process a payment intent
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceProcessPaymentIntentAction {
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
    pub process_config: Option<stripe_terminal::TerminalReaderReaderResourceProcessConfig>,
}
#[cfg(feature = "min-ser")]
pub struct TerminalReaderReaderResourceProcessPaymentIntentActionBuilder {
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    process_config: Option<Option<stripe_terminal::TerminalReaderReaderResourceProcessConfig>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalReaderReaderResourceProcessPaymentIntentAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceProcessPaymentIntentAction>,
        builder: TerminalReaderReaderResourceProcessPaymentIntentActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceProcessPaymentIntentAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TerminalReaderReaderResourceProcessPaymentIntentActionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceProcessPaymentIntentActionBuilder {
        type Out = TerminalReaderReaderResourceProcessPaymentIntentAction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "payment_intent" => Ok(Deserialize::begin(&mut self.payment_intent)),
                "process_config" => Ok(Deserialize::begin(&mut self.process_config)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { payment_intent: Deserialize::default(), process_config: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let payment_intent = self.payment_intent.take()?;
            let process_config = self.process_config.take()?;

            Some(Self::Out { payment_intent, process_config })
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

    impl ObjectDeser for TerminalReaderReaderResourceProcessPaymentIntentAction {
        type Builder = TerminalReaderReaderResourceProcessPaymentIntentActionBuilder;
    }
};
