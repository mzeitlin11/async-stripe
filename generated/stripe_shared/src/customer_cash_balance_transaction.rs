/// Customers with certain payments enabled have a cash balance, representing funds that were paid
/// by the customer to a merchant, but have not yet been allocated to a payment.
/// Cash Balance Transactions.
/// represent when funds are moved into or out of this balance.
/// This includes funding by the customer, allocation.
/// to payments, and refunds to the customer.
///
/// For more details see <<https://stripe.com/docs/api/cash_balance_transactions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerCashBalanceTransaction {
    pub adjusted_for_overdraft: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft>,
    pub applied_to_payment: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The customer whose available cash balance changed as a result of this transaction.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// The total available cash balance for the specified currency after this transaction was applied.
    /// Represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub ending_balance: i64,
    pub funded: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>,
    /// Unique identifier for the object.
    pub id: stripe_shared::CustomerCashBalanceTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The amount by which the cash balance changed, represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// A positive value represents funds being added to the cash balance, a negative value represents funds being removed from the cash balance.
    pub net_amount: i64,
    pub refunded_from_payment: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>,
    /// The type of the cash balance transaction.
    /// New types may be added in future.
    /// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: CustomerCashBalanceTransactionType,
    pub unapplied_from_payment: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction>,
}
#[cfg(feature = "min-ser")]
pub struct CustomerCashBalanceTransactionBuilder {
    adjusted_for_overdraft: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft>>,
    applied_to_payment: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    ending_balance: Option<i64>,
    funded: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>>,
    id: Option<stripe_shared::CustomerCashBalanceTransactionId>,
    livemode: Option<bool>,
    net_amount: Option<i64>,
    refunded_from_payment: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>>,
    type_: Option<CustomerCashBalanceTransactionType>,
    unapplied_from_payment: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerCashBalanceTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerCashBalanceTransaction>,
        builder: CustomerCashBalanceTransactionBuilder,
    }

    impl Visitor for Place<CustomerCashBalanceTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomerCashBalanceTransactionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomerCashBalanceTransactionBuilder {
        type Out = CustomerCashBalanceTransaction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "adjusted_for_overdraft" => Ok(Deserialize::begin(&mut self.adjusted_for_overdraft)),
                "applied_to_payment" => Ok(Deserialize::begin(&mut self.applied_to_payment)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "ending_balance" => Ok(Deserialize::begin(&mut self.ending_balance)),
                "funded" => Ok(Deserialize::begin(&mut self.funded)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "net_amount" => Ok(Deserialize::begin(&mut self.net_amount)),
                "refunded_from_payment" => Ok(Deserialize::begin(&mut self.refunded_from_payment)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "unapplied_from_payment" => Ok(Deserialize::begin(&mut self.unapplied_from_payment)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                adjusted_for_overdraft: Deserialize::default(),
                applied_to_payment: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                ending_balance: Deserialize::default(),
                funded: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                net_amount: Deserialize::default(),
                refunded_from_payment: Deserialize::default(),
                type_: Deserialize::default(),
                unapplied_from_payment: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let adjusted_for_overdraft = self.adjusted_for_overdraft.take()?;
            let applied_to_payment = self.applied_to_payment.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let customer = self.customer.take()?;
            let ending_balance = self.ending_balance.take()?;
            let funded = self.funded.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let net_amount = self.net_amount.take()?;
            let refunded_from_payment = self.refunded_from_payment.take()?;
            let type_ = self.type_.take()?;
            let unapplied_from_payment = self.unapplied_from_payment.take()?;

            Some(Self::Out {
                adjusted_for_overdraft,
                applied_to_payment,
                created,
                currency,
                customer,
                ending_balance,
                funded,
                id,
                livemode,
                net_amount,
                refunded_from_payment,
                type_,
                unapplied_from_payment,
            })
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

    impl ObjectDeser for CustomerCashBalanceTransaction {
        type Builder = CustomerCashBalanceTransactionBuilder;
    }
};
/// The type of the cash balance transaction.
/// New types may be added in future.
/// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerCashBalanceTransactionType {
    AdjustedForOverdraft,
    AppliedToPayment,
    Funded,
    FundingReversed,
    RefundedFromPayment,
    ReturnCanceled,
    ReturnInitiated,
    TransferredToBalance,
    UnappliedFromPayment,
}
impl CustomerCashBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        use CustomerCashBalanceTransactionType::*;
        match self {
            AdjustedForOverdraft => "adjusted_for_overdraft",
            AppliedToPayment => "applied_to_payment",
            Funded => "funded",
            FundingReversed => "funding_reversed",
            RefundedFromPayment => "refunded_from_payment",
            ReturnCanceled => "return_canceled",
            ReturnInitiated => "return_initiated",
            TransferredToBalance => "transferred_to_balance",
            UnappliedFromPayment => "unapplied_from_payment",
        }
    }
}

impl std::str::FromStr for CustomerCashBalanceTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerCashBalanceTransactionType::*;
        match s {
            "adjusted_for_overdraft" => Ok(AdjustedForOverdraft),
            "applied_to_payment" => Ok(AppliedToPayment),
            "funded" => Ok(Funded),
            "funding_reversed" => Ok(FundingReversed),
            "refunded_from_payment" => Ok(RefundedFromPayment),
            "return_canceled" => Ok(ReturnCanceled),
            "return_initiated" => Ok(ReturnInitiated),
            "transferred_to_balance" => Ok(TransferredToBalance),
            "unapplied_from_payment" => Ok(UnappliedFromPayment),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CustomerCashBalanceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CustomerCashBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerCashBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CustomerCashBalanceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerCashBalanceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerCashBalanceTransactionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerCashBalanceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CustomerCashBalanceTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerCashBalanceTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for CustomerCashBalanceTransaction {
    type Id = stripe_shared::CustomerCashBalanceTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CustomerCashBalanceTransactionId);
