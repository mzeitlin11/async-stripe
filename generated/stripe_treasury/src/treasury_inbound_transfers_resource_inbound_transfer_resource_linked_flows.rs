#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    /// If funds for this flow were returned after the flow went to the `succeeded` state, this field contains a reference to the ReceivedDebit return.
    pub received_debit: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder {
    received_debit: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows>,
        builder: TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder,
    }

    impl Visitor for Place<TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder {
        type Out = TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "received_debit" => Ok(Deserialize::begin(&mut self.received_debit)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { received_debit: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let received_debit = self.received_debit.take()?;

            Some(Self::Out { received_debit })
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

    impl ObjectDeser for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
        type Builder = TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder;
    }
};
