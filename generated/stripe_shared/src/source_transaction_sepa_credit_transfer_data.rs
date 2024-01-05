#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTransactionSepaCreditTransferData {
    /// Reference associated with the transfer.
    pub reference: Option<String>,
    /// Sender's bank account IBAN.
    pub sender_iban: Option<String>,
    /// Sender's name.
    pub sender_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTransactionSepaCreditTransferDataBuilder {
    reference: Option<Option<String>>,
    sender_iban: Option<Option<String>>,
    sender_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTransactionSepaCreditTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionSepaCreditTransferData>,
        builder: SourceTransactionSepaCreditTransferDataBuilder,
    }

    impl Visitor for Place<SourceTransactionSepaCreditTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTransactionSepaCreditTransferDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTransactionSepaCreditTransferDataBuilder {
        type Out = SourceTransactionSepaCreditTransferData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "sender_iban" => Ok(Deserialize::begin(&mut self.sender_iban)),
                "sender_name" => Ok(Deserialize::begin(&mut self.sender_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { reference: Deserialize::default(), sender_iban: Deserialize::default(), sender_name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let reference = self.reference.take()?;
            let sender_iban = self.sender_iban.take()?;
            let sender_name = self.sender_name.take()?;

            Some(Self::Out { reference, sender_iban, sender_name })
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

    impl ObjectDeser for SourceTransactionSepaCreditTransferData {
        type Builder = SourceTransactionSepaCreditTransferDataBuilder;
    }
};
