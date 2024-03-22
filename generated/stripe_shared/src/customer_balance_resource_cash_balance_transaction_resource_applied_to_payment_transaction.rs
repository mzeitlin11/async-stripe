#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {
    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were applied to.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
}
#[cfg(feature = "min-ser")]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder {
    payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>,
        builder: CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder,
    }

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "payment_intent" => Ok(Deserialize::begin(&mut self.payment_intent)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { payment_intent: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let payment_intent = self.payment_intent.take()?;

            Some(Self::Out { payment_intent })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {
        type Builder = CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransactionBuilder;
    }
};
