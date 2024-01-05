/// InboundTransfers contains inbound transfers features for a FinancialAccount.
#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceInboundTransfers {
    pub ach: Option<stripe_treasury::TreasuryFinancialAccountsResourceAchToggleSettings>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryFinancialAccountsResourceInboundTransfersBuilder {
    ach: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceAchToggleSettings>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountsResourceInboundTransfers {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceInboundTransfers>,
        builder: TreasuryFinancialAccountsResourceInboundTransfersBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceInboundTransfers> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryFinancialAccountsResourceInboundTransfersBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceInboundTransfersBuilder {
        type Out = TreasuryFinancialAccountsResourceInboundTransfers;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "ach" => Ok(Deserialize::begin(&mut self.ach)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { ach: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let ach = self.ach.take()?;

            Some(Self::Out { ach })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceInboundTransfers {
        type Builder = TreasuryFinancialAccountsResourceInboundTransfersBuilder;
    }
};
