#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTransactionGbpCreditTransferData {
    /// Bank account fingerprint associated with the Stripe owned bank account receiving the transfer.
    pub fingerprint: Option<String>,
    /// The credit transfer rails the sender used to push this transfer.
    /// The possible rails are: Faster Payments, BACS, CHAPS, and wire transfers.
    /// Currently only Faster Payments is supported.
    pub funding_method: Option<String>,
    /// Last 4 digits of sender account number associated with the transfer.
    pub last4: Option<String>,
    /// Sender entered arbitrary information about the transfer.
    pub reference: Option<String>,
    /// Sender account number associated with the transfer.
    pub sender_account_number: Option<String>,
    /// Sender name associated with the transfer.
    pub sender_name: Option<String>,
    /// Sender sort code associated with the transfer.
    pub sender_sort_code: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTransactionGbpCreditTransferDataBuilder {
    fingerprint: Option<Option<String>>,
    funding_method: Option<Option<String>>,
    last4: Option<Option<String>>,
    reference: Option<Option<String>>,
    sender_account_number: Option<Option<String>>,
    sender_name: Option<Option<String>>,
    sender_sort_code: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTransactionGbpCreditTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionGbpCreditTransferData>,
        builder: SourceTransactionGbpCreditTransferDataBuilder,
    }

    impl Visitor for Place<SourceTransactionGbpCreditTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTransactionGbpCreditTransferDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTransactionGbpCreditTransferDataBuilder {
        type Out = SourceTransactionGbpCreditTransferData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "funding_method" => Ok(Deserialize::begin(&mut self.funding_method)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "sender_account_number" => Ok(Deserialize::begin(&mut self.sender_account_number)),
                "sender_name" => Ok(Deserialize::begin(&mut self.sender_name)),
                "sender_sort_code" => Ok(Deserialize::begin(&mut self.sender_sort_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                fingerprint: Deserialize::default(),
                funding_method: Deserialize::default(),
                last4: Deserialize::default(),
                reference: Deserialize::default(),
                sender_account_number: Deserialize::default(),
                sender_name: Deserialize::default(),
                sender_sort_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let fingerprint = self.fingerprint.take()?;
            let funding_method = self.funding_method.take()?;
            let last4 = self.last4.take()?;
            let reference = self.reference.take()?;
            let sender_account_number = self.sender_account_number.take()?;
            let sender_name = self.sender_name.take()?;
            let sender_sort_code = self.sender_sort_code.take()?;

            Some(Self::Out { fingerprint, funding_method, last4, reference, sender_account_number, sender_name, sender_sort_code })
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

    impl ObjectDeser for SourceTransactionGbpCreditTransferData {
        type Builder = SourceTransactionGbpCreditTransferDataBuilder;
    }
};
