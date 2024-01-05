#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {
    /// The last 4 digits of the account number of the sender of the funding.
    pub account_number_last4: Option<String>,
    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
    /// The sort code of the bank of the sender of the funding
    pub sort_code: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder {
    account_number_last4: Option<Option<String>>,
    sender_name: Option<Option<String>>,
    sort_code: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>,
        builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder,
    }

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "account_number_last4" => Ok(Deserialize::begin(&mut self.account_number_last4)),
                "sender_name" => Ok(Deserialize::begin(&mut self.sender_name)),
                "sort_code" => Ok(Deserialize::begin(&mut self.sort_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account_number_last4: Deserialize::default(), sender_name: Deserialize::default(), sort_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_number_last4 = self.account_number_last4.take()?;
            let sender_name = self.sender_name.take()?;
            let sort_code = self.sort_code.take()?;

            Some(Self::Out { account_number_last4, sender_name, sort_code })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {
        type Builder = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder;
    }
};
