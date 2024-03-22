#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ChargeTransferData {
    /// The amount transferred to the destination account, if specified.
    /// By default, the entire charge amount is transferred to the destination account.
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
}
#[cfg(feature = "min-ser")]
pub struct ChargeTransferDataBuilder {
    amount: Option<Option<i64>>,
    destination: Option<stripe_types::Expandable<stripe_shared::Account>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ChargeTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ChargeTransferData>,
        builder: ChargeTransferDataBuilder,
    }

    impl Visitor for Place<ChargeTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ChargeTransferDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ChargeTransferDataBuilder {
        type Out = ChargeTransferData;
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

    impl ObjectDeser for ChargeTransferData {
        type Builder = ChargeTransferDataBuilder;
    }
};
