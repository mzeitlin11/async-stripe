#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ConnectCollectionTransfer {
    /// Amount transferred, in cents (or local equivalent).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the account that funds are being collected for.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ConnectCollectionTransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[cfg(feature = "min-ser")]
pub struct ConnectCollectionTransferBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    destination: Option<stripe_types::Expandable<stripe_shared::Account>>,
    id: Option<stripe_shared::ConnectCollectionTransferId>,
    livemode: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ConnectCollectionTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectCollectionTransfer>,
        builder: ConnectCollectionTransferBuilder,
    }

    impl Visitor for Place<ConnectCollectionTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ConnectCollectionTransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ConnectCollectionTransferBuilder {
        type Out = ConnectCollectionTransfer;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "destination" => Ok(Deserialize::begin(&mut self.destination)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), currency: Deserialize::default(), destination: Deserialize::default(), id: Deserialize::default(), livemode: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let currency = self.currency.take()?;
            let destination = self.destination.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;

            Some(Self::Out { amount, currency, destination, id, livemode })
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

    impl ObjectDeser for ConnectCollectionTransfer {
        type Builder = ConnectCollectionTransferBuilder;
    }
};
impl stripe_types::Object for ConnectCollectionTransfer {
    type Id = stripe_shared::ConnectCollectionTransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ConnectCollectionTransferId, "connct_");
