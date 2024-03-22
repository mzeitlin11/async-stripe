#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CheckoutCustomerBalancePaymentMethodOptions {
    pub bank_transfer: Option<stripe_checkout::CheckoutCustomerBalanceBankTransferPaymentMethodOptions>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    pub funding_type: Option<CheckoutCustomerBalancePaymentMethodOptionsFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    pub setup_future_usage: Option<CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage>,
}
#[cfg(feature = "min-ser")]
pub struct CheckoutCustomerBalancePaymentMethodOptionsBuilder {
    bank_transfer: Option<Option<stripe_checkout::CheckoutCustomerBalanceBankTransferPaymentMethodOptions>>,
    funding_type: Option<Option<CheckoutCustomerBalancePaymentMethodOptionsFundingType>>,
    setup_future_usage: Option<Option<CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutCustomerBalancePaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutCustomerBalancePaymentMethodOptions>,
        builder: CheckoutCustomerBalancePaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutCustomerBalancePaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CheckoutCustomerBalancePaymentMethodOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CheckoutCustomerBalancePaymentMethodOptionsBuilder {
        type Out = CheckoutCustomerBalancePaymentMethodOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "bank_transfer" => Ok(Deserialize::begin(&mut self.bank_transfer)),
                "funding_type" => Ok(Deserialize::begin(&mut self.funding_type)),
                "setup_future_usage" => Ok(Deserialize::begin(&mut self.setup_future_usage)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank_transfer: Deserialize::default(), funding_type: Deserialize::default(), setup_future_usage: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_transfer = self.bank_transfer.take()?;
            let funding_type = self.funding_type.take()?;
            let setup_future_usage = self.setup_future_usage.take()?;

            Some(Self::Out { bank_transfer, funding_type, setup_future_usage })
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

    impl ObjectDeser for CheckoutCustomerBalancePaymentMethodOptions {
        type Builder = CheckoutCustomerBalancePaymentMethodOptionsBuilder;
    }
};
/// The funding method type to be used when there are not enough funds in the customer balance.
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    BankTransfer,
}
impl CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    pub fn as_str(self) -> &'static str {
        use CheckoutCustomerBalancePaymentMethodOptionsFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCustomerBalancePaymentMethodOptionsFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutCustomerBalancePaymentMethodOptionsFundingType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutCustomerBalancePaymentMethodOptionsFundingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutCustomerBalancePaymentMethodOptionsFundingType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    None,
}
impl CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
