#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    pub amount_percent: Option<f64>,
    /// The account where funds from the payment will be transferred to upon payment success.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionTransferDataBuilder {
    amount_percent: Option<Option<f64>>,
    destination: Option<stripe_types::Expandable<stripe_shared::Account>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionTransferData>,
        builder: SubscriptionTransferDataBuilder,
    }

    impl Visitor for Place<SubscriptionTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionTransferDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionTransferDataBuilder {
        type Out = SubscriptionTransferData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount_percent" => Ok(Deserialize::begin(&mut self.amount_percent)),
                "destination" => Ok(Deserialize::begin(&mut self.destination)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount_percent: Deserialize::default(), destination: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_percent = self.amount_percent.take()?;
            let destination = self.destination.take()?;

            Some(Self::Out { amount_percent, destination })
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

    impl ObjectDeser for SubscriptionTransferData {
        type Builder = SubscriptionTransferDataBuilder;
    }
};
