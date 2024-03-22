#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedCard {
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: Option<stripe_types::Currency>,
    /// Always true for a deleted object
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::CardId,
}
#[cfg(feature = "min-ser")]
pub struct DeletedCardBuilder {
    currency: Option<Option<stripe_types::Currency>>,
    deleted: Option<stripe_types::AlwaysTrue>,
    id: Option<stripe_shared::CardId>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DeletedCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DeletedCard>,
        builder: DeletedCardBuilder,
    }

    impl Visitor for Place<DeletedCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DeletedCardBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DeletedCardBuilder {
        type Out = DeletedCard;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "deleted" => Ok(Deserialize::begin(&mut self.deleted)),
                "id" => Ok(Deserialize::begin(&mut self.id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { currency: Deserialize::default(), deleted: Deserialize::default(), id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let currency = self.currency.take()?;
            let deleted = self.deleted.take()?;
            let id = self.id.take()?;

            Some(Self::Out { currency, deleted, id })
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

    impl ObjectDeser for DeletedCard {
        type Builder = DeletedCardBuilder;
    }
};
impl stripe_types::Object for DeletedCard {
    type Id = stripe_shared::CardId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
