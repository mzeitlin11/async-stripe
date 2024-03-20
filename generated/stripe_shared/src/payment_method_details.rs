#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<stripe_shared::PaymentMethodDetailsAchCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<stripe_shared::PaymentMethodDetailsAchDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_shared::PaymentMethodDetailsAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_shared::PaymentMethodDetailsAffirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodDetailsAfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipayDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_shared::PaymentMethodDetailsAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_shared::PaymentMethodDetailsBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_shared::PaymentMethodDetailsBancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_shared::PaymentMethodDetailsBlik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_shared::PaymentMethodDetailsBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_shared::PaymentMethodDetailsCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_shared::PaymentMethodDetailsCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_shared::PaymentMethodDetailsCashapp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_shared::PaymentMethodDetailsCustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_shared::PaymentMethodDetailsEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_shared::PaymentMethodDetailsFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_shared::PaymentMethodDetailsGiropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_shared::PaymentMethodDetailsGrabpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_shared::PaymentMethodDetailsIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_shared::PaymentMethodDetailsInteracPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_shared::PaymentMethodDetailsKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_shared::PaymentMethodDetailsKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_shared::PaymentMethodDetailsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_shared::PaymentMethodDetailsMultibanco>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_shared::PaymentMethodDetailsOxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_shared::PaymentMethodDetailsP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_shared::PaymentMethodDetailsPaynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_shared::PaymentMethodDetailsPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_shared::PaymentMethodDetailsPix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_shared::PaymentMethodDetailsPromptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_shared::PaymentMethodDetailsRevolutPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<stripe_shared::PaymentMethodDetailsSepaCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_shared::PaymentMethodDetailsSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_shared::PaymentMethodDetailsSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_account: Option<stripe_shared::PaymentMethodDetailsStripeAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_shared::PaymentMethodDetailsSwish>,
    /// The type of transaction-specific details of the payment method used in the payment, one of `ach_credit_transfer`, `ach_debit`, `acss_debit`, `alipay`, `au_becs_debit`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `klarna`, `multibanco`, `p24`, `sepa_debit`, `sofort`, `stripe_account`, or `wechat`.
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains information specific to the payment method.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_shared::PaymentMethodDetailsUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<stripe_shared::PaymentMethodDetailsWechat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_shared::PaymentMethodDetailsWechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_shared::PaymentMethodDetailsZip>,
}
