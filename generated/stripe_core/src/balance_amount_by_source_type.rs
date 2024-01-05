#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BalanceAmountBySourceType {
    /// Amount for bank account.
    pub bank_account: Option<i64>,
    /// Amount for card.
    pub card: Option<i64>,
    /// Amount for FPX.
    pub fpx: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct BalanceAmountBySourceTypeBuilder {
    bank_account: Option<Option<i64>>,
    card: Option<Option<i64>>,
    fpx: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BalanceAmountBySourceType {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceAmountBySourceType>,
        builder: BalanceAmountBySourceTypeBuilder,
    }

    impl Visitor for Place<BalanceAmountBySourceType> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: BalanceAmountBySourceTypeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BalanceAmountBySourceTypeBuilder {
        type Out = BalanceAmountBySourceType;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank_account" => Ok(Deserialize::begin(&mut self.bank_account)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "fpx" => Ok(Deserialize::begin(&mut self.fpx)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank_account: Deserialize::default(), card: Deserialize::default(), fpx: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_account = self.bank_account.take()?;
            let card = self.card.take()?;
            let fpx = self.fpx.take()?;

            Some(Self::Out { bank_account, card, fpx })
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

    impl ObjectDeser for BalanceAmountBySourceType {
        type Builder = BalanceAmountBySourceTypeBuilder;
    }
};
