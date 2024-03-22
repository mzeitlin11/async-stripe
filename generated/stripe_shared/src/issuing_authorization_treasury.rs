#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingAuthorizationTreasury {
    /// The array of [ReceivedCredits](https://stripe.com/docs/api/treasury/received_credits) associated with this authorization.
    pub received_credits: Vec<String>,
    /// The array of [ReceivedDebits](https://stripe.com/docs/api/treasury/received_debits) associated with this authorization.
    pub received_debits: Vec<String>,
    /// The Treasury [Transaction](https://stripe.com/docs/api/treasury/transactions) associated with this authorization.
    pub transaction: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingAuthorizationTreasuryBuilder {
    received_credits: Option<Vec<String>>,
    received_debits: Option<Vec<String>>,
    transaction: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingAuthorizationTreasury {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationTreasury>,
        builder: IssuingAuthorizationTreasuryBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationTreasury> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingAuthorizationTreasuryBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingAuthorizationTreasuryBuilder {
        type Out = IssuingAuthorizationTreasury;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "received_credits" => Ok(Deserialize::begin(&mut self.received_credits)),
                "received_debits" => Ok(Deserialize::begin(&mut self.received_debits)),
                "transaction" => Ok(Deserialize::begin(&mut self.transaction)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { received_credits: Deserialize::default(), received_debits: Deserialize::default(), transaction: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let received_credits = self.received_credits.take()?;
            let received_debits = self.received_debits.take()?;
            let transaction = self.transaction.take()?;

            Some(Self::Out { received_credits, received_debits, transaction })
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

    impl ObjectDeser for IssuingAuthorizationTreasury {
        type Builder = IssuingAuthorizationTreasuryBuilder;
    }
};
