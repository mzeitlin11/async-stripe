#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingTransactionTreasury {
    /// The Treasury [ReceivedCredit](https://stripe.com/docs/api/treasury/received_credits) representing this Issuing transaction if it is a refund.
    pub received_credit: Option<String>,
    /// The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_debits) representing this Issuing transaction if it is a capture.
    pub received_debit: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTransactionTreasuryBuilder {
    received_credit: Option<Option<String>>,
    received_debit: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionTreasury {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionTreasury>,
        builder: IssuingTransactionTreasuryBuilder,
    }

    impl Visitor for Place<IssuingTransactionTreasury> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTransactionTreasuryBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTransactionTreasuryBuilder {
        type Out = IssuingTransactionTreasury;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "received_credit" => Ok(Deserialize::begin(&mut self.received_credit)),
                "received_debit" => Ok(Deserialize::begin(&mut self.received_debit)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { received_credit: Deserialize::default(), received_debit: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let received_credit = self.received_credit.take()?;
            let received_debit = self.received_debit.take()?;

            Some(Self::Out { received_credit, received_debit })
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

    impl ObjectDeser for IssuingTransactionTreasury {
        type Builder = IssuingTransactionTreasuryBuilder;
    }
};
