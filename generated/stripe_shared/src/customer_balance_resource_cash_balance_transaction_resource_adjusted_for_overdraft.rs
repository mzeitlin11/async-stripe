#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
    /// The [Balance Transaction](https://stripe.com/docs/api/balance_transactions/object) that corresponds to funds taken out of your Stripe balance.
    pub balance_transaction: stripe_types::Expandable<stripe_shared::BalanceTransaction>,
    /// The [Cash Balance Transaction](https://stripe.com/docs/api/cash_balance_transactions/object) that brought the customer balance negative, triggering the clawback of funds.
    pub linked_transaction: stripe_types::Expandable<stripe_shared::CustomerCashBalanceTransaction>,
}
#[cfg(feature = "min-ser")]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder {
    balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    linked_transaction: Option<stripe_types::Expandable<stripe_shared::CustomerCashBalanceTransaction>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft>,
        builder: CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder,
    }

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "balance_transaction" => Ok(Deserialize::begin(&mut self.balance_transaction)),
                "linked_transaction" => Ok(Deserialize::begin(&mut self.linked_transaction)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { balance_transaction: Deserialize::default(), linked_transaction: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let balance_transaction = self.balance_transaction.take()?;
            let linked_transaction = self.linked_transaction.take()?;

            Some(Self::Out { balance_transaction, linked_transaction })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
        type Builder = CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder;
    }
};
