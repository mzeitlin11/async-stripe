#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingTransactionNetworkData {
    /// A code created by Stripe which is shared with the merchant to validate the authorization.
    /// This field will be populated if the authorization message was approved.
    /// The code typically starts with the letter "S", followed by a six-digit number.
    /// For example, "S498162".
    /// Please note that the code is not guaranteed to be unique across authorizations.
    pub authorization_code: Option<String>,
    /// The date the transaction was processed by the card network.
    /// This can be different from the date the seller recorded the transaction depending on when the acquirer submits the transaction to the network.
    pub processing_date: Option<String>,
    /// Unique identifier for the authorization assigned by the card network used to match subsequent messages, disputes, and transactions.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTransactionNetworkDataBuilder {
    authorization_code: Option<Option<String>>,
    processing_date: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionNetworkData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionNetworkData>,
        builder: IssuingTransactionNetworkDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionNetworkData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTransactionNetworkDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTransactionNetworkDataBuilder {
        type Out = IssuingTransactionNetworkData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "authorization_code" => Ok(Deserialize::begin(&mut self.authorization_code)),
                "processing_date" => Ok(Deserialize::begin(&mut self.processing_date)),
                "transaction_id" => Ok(Deserialize::begin(&mut self.transaction_id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { authorization_code: Deserialize::default(), processing_date: Deserialize::default(), transaction_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let authorization_code = self.authorization_code.take()?;
            let processing_date = self.processing_date.take()?;
            let transaction_id = self.transaction_id.take()?;

            Some(Self::Out { authorization_code, processing_date, transaction_id })
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

    impl ObjectDeser for IssuingTransactionNetworkData {
        type Builder = IssuingTransactionNetworkDataBuilder;
    }
};
