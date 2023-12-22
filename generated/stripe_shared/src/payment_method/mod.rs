/// PaymentMethod objects represent your customer's payment instruments.
/// You can use them with [PaymentIntents](https://stripe.com/docs/payments/payment-intents) to collect payments or save them to
/// Customer objects to store instrument details for future payments.
///
/// Related guides: [Payment Methods](https://stripe.com/docs/payments/payment-methods) and [More Payment Scenarios](https://stripe.com/docs/payments/more-payment-scenarios).
///
/// For more details see <<https://stripe.com/docs/api/payment_methods/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_shared::PaymentMethodAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_shared::PaymentMethodAffirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodAfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_shared::PaymentMethodAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_shared::PaymentMethodBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_shared::PaymentMethodBancontact>,
    pub billing_details: stripe_shared::BillingDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_shared::PaymentMethodBlik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_shared::PaymentMethodBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_shared::PaymentMethodCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_shared::PaymentMethodCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_shared::PaymentMethodCashapp>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_shared::PaymentMethodCustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_shared::PaymentMethodEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_shared::PaymentMethodFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_shared::PaymentMethodGiropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_shared::PaymentMethodGrabpay>,
    /// Unique identifier for the object.
    pub id: stripe_shared::payment_method::PaymentMethodId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_shared::PaymentMethodIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_shared::PaymentMethodInteracPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_shared::PaymentMethodKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_shared::PaymentMethodKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_shared::PaymentMethodLink>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_shared::PaymentMethodOxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_shared::PaymentMethodP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_shared::PaymentMethodPaynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_shared::PaymentMethodPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_shared::PaymentMethodPix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_shared::PaymentMethodPromptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<stripe_shared::RadarRadarOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_shared::PaymentMethodRevolutPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_shared::PaymentMethodSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_shared::PaymentMethodSofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_shared::PaymentMethodUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_shared::PaymentMethodWechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_shared::PaymentMethodZip>,
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CardPresent,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl PaymentMethodType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            CardPresent => "card_present",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            InteracPresent => "interac_present",
            Klarna => "klarna",
            Konbini => "konbini",
            Link => "link",
            Oxxo => "oxxo",
            P24 => "p24",
            Paynow => "paynow",
            Paypal => "paypal",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentMethodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "card_present" => Ok(CardPresent),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "interac_present" => Ok(InteracPresent),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(PaymentMethodType::Unknown))
    }
}
impl stripe_types::Object for PaymentMethod {
    type Id = stripe_shared::payment_method::PaymentMethodId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PaymentMethodId, "pm_" | "card_" | "src_" | "ba_");
