/// The resource representing a Stripe Polymorphic
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(not(feature = "min-ser"), serde(tag = "object"))]
pub enum BalanceTransactionSource {
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "application_fee"))]
    ApplicationFee(stripe_shared::ApplicationFee),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "charge"))]
    Charge(stripe_shared::Charge),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "connect_collection_transfer"))]
    ConnectCollectionTransfer(stripe_shared::ConnectCollectionTransfer),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "customer_cash_balance_transaction"))]
    CustomerCashBalanceTransaction(stripe_shared::CustomerCashBalanceTransaction),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "dispute"))]
    Dispute(stripe_shared::Dispute),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "fee_refund"))]
    ApplicationFeeRefund(stripe_shared::ApplicationFeeRefund),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "issuing.authorization"))]
    IssuingAuthorization(stripe_shared::IssuingAuthorization),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "issuing.dispute"))]
    IssuingDispute(stripe_shared::IssuingDispute),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "issuing.transaction"))]
    IssuingTransaction(stripe_shared::IssuingTransaction),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "payout"))]
    Payout(stripe_shared::Payout),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "platform_tax_fee"))]
    PlatformTaxFee(stripe_shared::PlatformTaxFee),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "refund"))]
    Refund(stripe_shared::Refund),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "reserve_transaction"))]
    ReserveTransaction(stripe_shared::ReserveTransaction),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "tax_deducted_at_source"))]
    TaxDeductedAtSource(stripe_shared::TaxDeductedAtSource),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "topup"))]
    Topup(stripe_shared::Topup),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "transfer"))]
    Transfer(stripe_shared::Transfer),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "transfer_reversal"))]
    TransferReversal(stripe_shared::TransferReversal),
}

#[cfg(feature = "min-ser")]
#[derive(Default)]
pub struct BalanceTransactionSourceBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::{from_str, to_string};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<BalanceTransactionSource>,
        builder: BalanceTransactionSourceBuilder,
    }

    impl Deserialize for BalanceTransactionSource {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<BalanceTransactionSource> {
        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
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

    impl stripe_types::MapBuilder for BalanceTransactionSourceBuilder {
        type Out = BalanceTransactionSource;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (obj_key, object) = self.inner.finish_inner()?;
            let obj_str = to_string(&object);
            Some(match obj_key.as_str() {
                "application_fee" => BalanceTransactionSource::ApplicationFee(from_str(&obj_str).ok()?),
                "charge" => BalanceTransactionSource::Charge(from_str(&obj_str).ok()?),
                "connect_collection_transfer" => BalanceTransactionSource::ConnectCollectionTransfer(from_str(&obj_str).ok()?),
                "customer_cash_balance_transaction" => BalanceTransactionSource::CustomerCashBalanceTransaction(from_str(&obj_str).ok()?),
                "dispute" => BalanceTransactionSource::Dispute(from_str(&obj_str).ok()?),
                "fee_refund" => BalanceTransactionSource::ApplicationFeeRefund(from_str(&obj_str).ok()?),
                "issuing.authorization" => BalanceTransactionSource::IssuingAuthorization(from_str(&obj_str).ok()?),
                "issuing.dispute" => BalanceTransactionSource::IssuingDispute(from_str(&obj_str).ok()?),
                "issuing.transaction" => BalanceTransactionSource::IssuingTransaction(from_str(&obj_str).ok()?),
                "payout" => BalanceTransactionSource::Payout(from_str(&obj_str).ok()?),
                "platform_tax_fee" => BalanceTransactionSource::PlatformTaxFee(from_str(&obj_str).ok()?),
                "refund" => BalanceTransactionSource::Refund(from_str(&obj_str).ok()?),
                "reserve_transaction" => BalanceTransactionSource::ReserveTransaction(from_str(&obj_str).ok()?),
                "tax_deducted_at_source" => BalanceTransactionSource::TaxDeductedAtSource(from_str(&obj_str).ok()?),
                "topup" => BalanceTransactionSource::Topup(from_str(&obj_str).ok()?),
                "transfer" => BalanceTransactionSource::Transfer(from_str(&obj_str).ok()?),
                "transfer_reversal" => BalanceTransactionSource::TransferReversal(from_str(&obj_str).ok()?),

                _ => return None,
            })
        }
    }

    impl stripe_types::ObjectDeser for BalanceTransactionSource {
        type Builder = BalanceTransactionSourceBuilder;
    }
};

impl stripe_types::Object for BalanceTransactionSource {
    type Id = smol_str::SmolStr;
    fn id(&self) -> &Self::Id {
        match self {
            Self::ApplicationFee(v) => v.id.inner(),
            Self::Charge(v) => v.id.inner(),
            Self::ConnectCollectionTransfer(v) => v.id.inner(),
            Self::CustomerCashBalanceTransaction(v) => v.id.inner(),
            Self::Dispute(v) => v.id.inner(),
            Self::ApplicationFeeRefund(v) => v.id.inner(),
            Self::IssuingAuthorization(v) => v.id.inner(),
            Self::IssuingDispute(v) => v.id.inner(),
            Self::IssuingTransaction(v) => v.id.inner(),
            Self::Payout(v) => v.id.inner(),
            Self::PlatformTaxFee(v) => v.id.inner(),
            Self::Refund(v) => v.id.inner(),
            Self::ReserveTransaction(v) => v.id.inner(),
            Self::TaxDeductedAtSource(v) => v.id.inner(),
            Self::Topup(v) => v.id.inner(),
            Self::Transfer(v) => v.id.inner(),
            Self::TransferReversal(v) => v.id.inner(),
        }
    }
}
