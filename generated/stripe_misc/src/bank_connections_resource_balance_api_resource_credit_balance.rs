#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalance {
    /// The credit that has been used by the account holder.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub used: Option<std::collections::HashMap<String, i64>>,
}
#[cfg(feature = "min-ser")]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder {
    used: Option<Option<std::collections::HashMap<String, i64>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BankConnectionsResourceBalanceApiResourceCreditBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceBalanceApiResourceCreditBalance>,
        builder: BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceBalanceApiResourceCreditBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder {
        type Out = BankConnectionsResourceBalanceApiResourceCreditBalance;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "used" => Ok(Deserialize::begin(&mut self.used)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { used: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let used = self.used.take()?;

            Some(Self::Out { used })
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

    impl ObjectDeser for BankConnectionsResourceBalanceApiResourceCreditBalance {
        type Builder = BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder;
    }
};
