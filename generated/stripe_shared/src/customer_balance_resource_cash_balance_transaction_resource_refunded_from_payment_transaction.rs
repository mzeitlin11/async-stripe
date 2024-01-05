#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction {
    /// The [Refund](https://stripe.com/docs/api/refunds/object) that moved these funds into the customer's cash balance.
    pub refund: stripe_types::Expandable<stripe_shared::Refund>,
}
#[cfg(feature = "min-ser")]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder {
    refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>,
        builder: CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder,
    }

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "refund" => Ok(Deserialize::begin(&mut self.refund)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { refund: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let refund = self.refund.take()?;

            Some(Self::Out { refund })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction {
        type Builder = CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransactionBuilder;
    }
};
