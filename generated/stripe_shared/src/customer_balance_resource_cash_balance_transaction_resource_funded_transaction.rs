#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
    pub bank_transfer: stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer,
}
#[cfg(feature = "min-ser")]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder {
    bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>,
        builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder,
    }

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank_transfer" => Ok(Deserialize::begin(&mut self.bank_transfer)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank_transfer: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_transfer = self.bank_transfer.take()?;

            Some(Self::Out { bank_transfer })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
        type Builder = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder;
    }
};
