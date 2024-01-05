/// Settings related to Outbound Payments features on a Financial Account
#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceOutboundPayments {
    pub ach: Option<stripe_treasury::TreasuryFinancialAccountsResourceAchToggleSettings>,
    pub us_domestic_wire: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryFinancialAccountsResourceOutboundPaymentsBuilder {
    ach: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceAchToggleSettings>>,
    us_domestic_wire: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountsResourceOutboundPayments {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceOutboundPayments>,
        builder: TreasuryFinancialAccountsResourceOutboundPaymentsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceOutboundPayments> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryFinancialAccountsResourceOutboundPaymentsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceOutboundPaymentsBuilder {
        type Out = TreasuryFinancialAccountsResourceOutboundPayments;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "ach" => Ok(Deserialize::begin(&mut self.ach)),
                "us_domestic_wire" => Ok(Deserialize::begin(&mut self.us_domestic_wire)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { ach: Deserialize::default(), us_domestic_wire: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let ach = self.ach.take()?;
            let us_domestic_wire = self.us_domestic_wire.take()?;

            Some(Self::Out { ach, us_domestic_wire })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceOutboundPayments {
        type Builder = TreasuryFinancialAccountsResourceOutboundPaymentsBuilder;
    }
};
