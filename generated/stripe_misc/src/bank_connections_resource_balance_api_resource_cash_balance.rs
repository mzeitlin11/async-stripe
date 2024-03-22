#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankConnectionsResourceBalanceApiResourceCashBalance {
    /// The funds available to the account holder. Typically this is the current balance less any holds.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub available: Option<std::collections::HashMap<String, i64>>,
}
#[cfg(feature = "min-ser")]
pub struct BankConnectionsResourceBalanceApiResourceCashBalanceBuilder {
    available: Option<Option<std::collections::HashMap<String, i64>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BankConnectionsResourceBalanceApiResourceCashBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceBalanceApiResourceCashBalance>,
        builder: BankConnectionsResourceBalanceApiResourceCashBalanceBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceBalanceApiResourceCashBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: BankConnectionsResourceBalanceApiResourceCashBalanceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BankConnectionsResourceBalanceApiResourceCashBalanceBuilder {
        type Out = BankConnectionsResourceBalanceApiResourceCashBalance;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "available" => Ok(Deserialize::begin(&mut self.available)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { available: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let available = self.available.take()?;

            Some(Self::Out { available })
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

    impl ObjectDeser for BankConnectionsResourceBalanceApiResourceCashBalance {
        type Builder = BankConnectionsResourceBalanceApiResourceCashBalanceBuilder;
    }
};
