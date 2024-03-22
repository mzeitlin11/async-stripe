/// Change to a FinancialAccount's balance
#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryTransactionsResourceBalanceImpact {
    /// The change made to funds the user can spend right now.
    pub cash: i64,
    /// The change made to funds that are not spendable yet, but will become available at a later time.
    pub inbound_pending: i64,
    /// The change made to funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: i64,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryTransactionsResourceBalanceImpactBuilder {
    cash: Option<i64>,
    inbound_pending: Option<i64>,
    outbound_pending: Option<i64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryTransactionsResourceBalanceImpact {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryTransactionsResourceBalanceImpact>,
        builder: TreasuryTransactionsResourceBalanceImpactBuilder,
    }

    impl Visitor for Place<TreasuryTransactionsResourceBalanceImpact> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryTransactionsResourceBalanceImpactBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryTransactionsResourceBalanceImpactBuilder {
        type Out = TreasuryTransactionsResourceBalanceImpact;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "cash" => Ok(Deserialize::begin(&mut self.cash)),
                "inbound_pending" => Ok(Deserialize::begin(&mut self.inbound_pending)),
                "outbound_pending" => Ok(Deserialize::begin(&mut self.outbound_pending)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { cash: Deserialize::default(), inbound_pending: Deserialize::default(), outbound_pending: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let cash = self.cash.take()?;
            let inbound_pending = self.inbound_pending.take()?;
            let outbound_pending = self.outbound_pending.take()?;

            Some(Self::Out { cash, inbound_pending, outbound_pending })
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

    impl ObjectDeser for TreasuryTransactionsResourceBalanceImpact {
        type Builder = TreasuryTransactionsResourceBalanceImpactBuilder;
    }
};
