#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceTransferData {
    /// The amount in cents (or local equivalent) that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    /// The connected account receiving the transfer.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceTransferDataBuilder {
    amount: Option<Option<i64>>,
    destination: Option<stripe_types::Expandable<stripe_shared::Account>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceTransferData>,
        builder: PaymentLinksResourceTransferDataBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceTransferDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceTransferDataBuilder {
        type Out = PaymentLinksResourceTransferData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "destination" => Ok(Deserialize::begin(&mut self.destination)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), destination: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let destination = self.destination.take()?;

            Some(Self::Out { amount, destination })
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

    impl ObjectDeser for PaymentLinksResourceTransferData {
        type Builder = PaymentLinksResourceTransferDataBuilder;
    }
};
