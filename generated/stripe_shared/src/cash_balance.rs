/// A customer's `Cash balance` represents real funds.
/// Customers can add funds to their cash balance by sending a bank transfer.
/// These funds can be used for payment and can eventually be paid out to your bank account.
///
/// For more details see <<https://stripe.com/docs/api/cash_balance/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CashBalance {
    /// A hash of all cash balances available to this customer.
    /// You cannot delete a customer with any cash balances, even if the balance is 0.
    /// Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub available: Option<std::collections::HashMap<String, i64>>,
    /// The ID of the customer whose cash balance this object represents.
    pub customer: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub settings: stripe_shared::CustomerBalanceCustomerBalanceSettings,
}
#[cfg(feature = "min-ser")]
pub struct CashBalanceBuilder {
    available: Option<Option<std::collections::HashMap<String, i64>>>,
    customer: Option<String>,
    livemode: Option<bool>,
    settings: Option<stripe_shared::CustomerBalanceCustomerBalanceSettings>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CashBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CashBalance>,
        builder: CashBalanceBuilder,
    }

    impl Visitor for Place<CashBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CashBalanceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CashBalanceBuilder {
        type Out = CashBalance;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "available" => Ok(Deserialize::begin(&mut self.available)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "settings" => Ok(Deserialize::begin(&mut self.settings)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { available: Deserialize::default(), customer: Deserialize::default(), livemode: Deserialize::default(), settings: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let available = self.available.take()?;
            let customer = self.customer.take()?;
            let livemode = self.livemode.take()?;
            let settings = self.settings.take()?;

            Some(Self::Out { available, customer, livemode, settings })
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

    impl ObjectDeser for CashBalance {
        type Builder = CashBalanceBuilder;
    }
};
