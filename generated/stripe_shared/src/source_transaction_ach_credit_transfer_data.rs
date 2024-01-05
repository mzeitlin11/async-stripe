#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTransactionAchCreditTransferData {
    /// Customer data associated with the transfer.
    pub customer_data: Option<String>,
    /// Bank account fingerprint associated with the transfer.
    pub fingerprint: Option<String>,
    /// Last 4 digits of the account number associated with the transfer.
    pub last4: Option<String>,
    /// Routing number associated with the transfer.
    pub routing_number: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTransactionAchCreditTransferDataBuilder {
    customer_data: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTransactionAchCreditTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionAchCreditTransferData>,
        builder: SourceTransactionAchCreditTransferDataBuilder,
    }

    impl Visitor for Place<SourceTransactionAchCreditTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTransactionAchCreditTransferDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTransactionAchCreditTransferDataBuilder {
        type Out = SourceTransactionAchCreditTransferData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "customer_data" => Ok(Deserialize::begin(&mut self.customer_data)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { customer_data: Deserialize::default(), fingerprint: Deserialize::default(), last4: Deserialize::default(), routing_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let customer_data = self.customer_data.take()?;
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;
            let routing_number = self.routing_number.take()?;

            Some(Self::Out { customer_data, fingerprint, last4, routing_number })
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

    impl ObjectDeser for SourceTransactionAchCreditTransferData {
        type Builder = SourceTransactionAchCreditTransferDataBuilder;
    }
};
