/// For more details see <<https://stripe.com/docs/api/application_fees/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ApplicationFee {
    /// ID of the Stripe account this fee was taken from.
    pub account: stripe_types::Expandable<stripe_shared::Account>,
    /// Amount earned, in cents (or local equivalent).
    pub amount: i64,
    /// Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the fee if a partial refund was issued).
    pub amount_refunded: i64,
    /// ID of the Connect application that earned the fee.
    pub application: stripe_types::Expandable<stripe_shared::Application>,
    /// Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// ID of the charge that the application fee was taken from.
    pub charge: stripe_types::Expandable<stripe_shared::Charge>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_shared::ApplicationFeeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    pub originating_transaction: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Whether the fee has been fully refunded.
    /// If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    /// A list of refunds that have been applied to the fee.
    pub refunds: stripe_types::List<stripe_shared::ApplicationFeeRefund>,
}
#[cfg(feature = "min-ser")]
pub struct ApplicationFeeBuilder {
    account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    amount: Option<i64>,
    amount_refunded: Option<i64>,
    application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    id: Option<stripe_shared::ApplicationFeeId>,
    livemode: Option<bool>,
    originating_transaction: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    refunded: Option<bool>,
    refunds: Option<stripe_types::List<stripe_shared::ApplicationFeeRefund>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ApplicationFee {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ApplicationFee>,
        builder: ApplicationFeeBuilder,
    }

    impl Visitor for Place<ApplicationFee> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ApplicationFeeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ApplicationFeeBuilder {
        type Out = ApplicationFee;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "account" => Ok(Deserialize::begin(&mut self.account)),
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_refunded" => Ok(Deserialize::begin(&mut self.amount_refunded)),
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "balance_transaction" => Ok(Deserialize::begin(&mut self.balance_transaction)),
                "charge" => Ok(Deserialize::begin(&mut self.charge)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "originating_transaction" => Ok(Deserialize::begin(&mut self.originating_transaction)),
                "refunded" => Ok(Deserialize::begin(&mut self.refunded)),
                "refunds" => Ok(Deserialize::begin(&mut self.refunds)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                amount: Deserialize::default(),
                amount_refunded: Deserialize::default(),
                application: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                charge: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                originating_transaction: Deserialize::default(),
                refunded: Deserialize::default(),
                refunds: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account = self.account.take()?;
            let amount = self.amount.take()?;
            let amount_refunded = self.amount_refunded.take()?;
            let application = self.application.take()?;
            let balance_transaction = self.balance_transaction.take()?;
            let charge = self.charge.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let originating_transaction = self.originating_transaction.take()?;
            let refunded = self.refunded.take()?;
            let refunds = self.refunds.take()?;

            Some(Self::Out { account, amount, amount_refunded, application, balance_transaction, charge, created, currency, id, livemode, originating_transaction, refunded, refunds })
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

    impl ObjectDeser for ApplicationFee {
        type Builder = ApplicationFeeBuilder;
    }
};
impl stripe_types::Object for ApplicationFee {
    type Id = stripe_shared::ApplicationFeeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ApplicationFeeId, "fee_");
