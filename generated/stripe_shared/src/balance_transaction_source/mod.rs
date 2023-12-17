/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum BalanceTransactionSource {
    #[serde(rename = "application_fee")]
    ApplicationFee(stripe_shared::ApplicationFee),
    #[serde(rename = "charge")]
    Charge(stripe_shared::Charge),
    #[serde(rename = "connect_collection_transfer")]
    ConnectCollectionTransfer(stripe_shared::ConnectCollectionTransfer),
    #[serde(rename = "customer_cash_balance_transaction")]
    CustomerCashBalanceTransaction(stripe_shared::CustomerCashBalanceTransaction),
    #[serde(rename = "dispute")]
    Dispute(stripe_shared::Dispute),
    #[serde(rename = "fee_refund")]
    ApplicationFeeRefund(stripe_shared::ApplicationFeeRefund),
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization(stripe_shared::IssuingAuthorization),
    #[serde(rename = "issuing.dispute")]
    IssuingDispute(stripe_shared::IssuingDispute),
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction(stripe_shared::IssuingTransaction),
    #[serde(rename = "payout")]
    Payout(stripe_shared::Payout),
    #[serde(rename = "platform_tax_fee")]
    PlatformTaxFee(stripe_shared::PlatformTaxFee),
    #[serde(rename = "refund")]
    Refund(stripe_shared::Refund),
    #[serde(rename = "reserve_transaction")]
    ReserveTransaction(stripe_shared::ReserveTransaction),
    #[serde(rename = "tax_deducted_at_source")]
    TaxDeductedAtSource(stripe_shared::TaxDeductedAtSource),
    #[serde(rename = "topup")]
    Topup(stripe_shared::Topup),
    #[serde(rename = "transfer")]
    Transfer(stripe_shared::Transfer),
    #[serde(rename = "transfer_reversal")]
    TransferReversal(stripe_shared::TransferReversal),
}
impl stripe_types::Object for BalanceTransactionSource {
    type Id = String;
    fn id(&self) -> Option<&str> {
        match self {
            Self::ApplicationFee(v) => Some(v.id.as_str()),
            Self::Charge(v) => Some(v.id.as_str()),
            Self::ConnectCollectionTransfer(v) => Some(v.id.as_str()),
            Self::CustomerCashBalanceTransaction(v) => Some(v.id.as_str()),
            Self::Dispute(v) => Some(v.id.as_str()),
            Self::ApplicationFeeRefund(v) => Some(v.id.as_str()),
            Self::IssuingAuthorization(v) => Some(v.id.as_str()),
            Self::IssuingDispute(v) => Some(v.id.as_str()),
            Self::IssuingTransaction(v) => Some(v.id.as_str()),
            Self::Payout(v) => Some(v.id.as_str()),
            Self::PlatformTaxFee(v) => Some(v.id.as_str()),
            Self::Refund(v) => Some(v.id.as_str()),
            Self::ReserveTransaction(v) => Some(v.id.as_str()),
            Self::TaxDeductedAtSource(v) => Some(v.id.as_str()),
            Self::Topup(v) => Some(v.id.as_str()),
            Self::Transfer(v) => Some(v.id.as_str()),
            Self::TransferReversal(v) => Some(v.id.as_str()),
        }
    }
}
