#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTransactionChfCreditTransferData {
    /// Reference associated with the transfer.
    pub reference: Option<String>,
    /// Sender's country address.
    pub sender_address_country: Option<String>,
    /// Sender's line 1 address.
    pub sender_address_line1: Option<String>,
    /// Sender's bank account IBAN.
    pub sender_iban: Option<String>,
    /// Sender's name.
    pub sender_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTransactionChfCreditTransferDataBuilder {
    reference: Option<Option<String>>,
    sender_address_country: Option<Option<String>>,
    sender_address_line1: Option<Option<String>>,
    sender_iban: Option<Option<String>>,
    sender_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTransactionChfCreditTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionChfCreditTransferData>,
        builder: SourceTransactionChfCreditTransferDataBuilder,
    }

    impl Visitor for Place<SourceTransactionChfCreditTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTransactionChfCreditTransferDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTransactionChfCreditTransferDataBuilder {
        type Out = SourceTransactionChfCreditTransferData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "sender_address_country" => Ok(Deserialize::begin(&mut self.sender_address_country)),
                "sender_address_line1" => Ok(Deserialize::begin(&mut self.sender_address_line1)),
                "sender_iban" => Ok(Deserialize::begin(&mut self.sender_iban)),
                "sender_name" => Ok(Deserialize::begin(&mut self.sender_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                reference: Deserialize::default(),
                sender_address_country: Deserialize::default(),
                sender_address_line1: Deserialize::default(),
                sender_iban: Deserialize::default(),
                sender_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let reference = self.reference.take()?;
            let sender_address_country = self.sender_address_country.take()?;
            let sender_address_line1 = self.sender_address_line1.take()?;
            let sender_iban = self.sender_iban.take()?;
            let sender_name = self.sender_name.take()?;

            Some(Self::Out { reference, sender_address_country, sender_address_line1, sender_iban, sender_name })
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

    impl ObjectDeser for SourceTransactionChfCreditTransferData {
        type Builder = SourceTransactionChfCreditTransferDataBuilder;
    }
};
