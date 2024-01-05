#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingDisputeTreasury {
    /// The Treasury [DebitReversal](https://stripe.com/docs/api/treasury/debit_reversals) representing this Issuing dispute.
    pub debit_reversal: Option<String>,
    /// The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_debits) that is being disputed.
    pub received_debit: String,
}
#[cfg(feature = "min-ser")]
pub struct IssuingDisputeTreasuryBuilder {
    debit_reversal: Option<Option<String>>,
    received_debit: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingDisputeTreasury {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeTreasury>,
        builder: IssuingDisputeTreasuryBuilder,
    }

    impl Visitor for Place<IssuingDisputeTreasury> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingDisputeTreasuryBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingDisputeTreasuryBuilder {
        type Out = IssuingDisputeTreasury;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "debit_reversal" => Ok(Deserialize::begin(&mut self.debit_reversal)),
                "received_debit" => Ok(Deserialize::begin(&mut self.received_debit)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { debit_reversal: Deserialize::default(), received_debit: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let debit_reversal = self.debit_reversal.take()?;
            let received_debit = self.received_debit.take()?;

            Some(Self::Out { debit_reversal, received_debit })
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

    impl ObjectDeser for IssuingDisputeTreasury {
        type Builder = IssuingDisputeTreasuryBuilder;
    }
};
