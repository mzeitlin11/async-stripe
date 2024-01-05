#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionsResourcePaymentSettings {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    pub payment_method_options: Option<stripe_shared::SubscriptionsResourcePaymentMethodOptions>,
    /// The list of payment method types to provide to every invoice created by the subscription.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<SubscriptionsResourcePaymentSettingsPaymentMethodTypes>>,
    /// Either `off`, or `on_subscription`.
    /// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    pub save_default_payment_method: Option<SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionsResourcePaymentSettingsBuilder {
    payment_method_options: Option<Option<stripe_shared::SubscriptionsResourcePaymentMethodOptions>>,
    payment_method_types: Option<Option<Vec<SubscriptionsResourcePaymentSettingsPaymentMethodTypes>>>,
    save_default_payment_method: Option<Option<SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionsResourcePaymentSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourcePaymentSettings>,
        builder: SubscriptionsResourcePaymentSettingsBuilder,
    }

    impl Visitor for Place<SubscriptionsResourcePaymentSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionsResourcePaymentSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionsResourcePaymentSettingsBuilder {
        type Out = SubscriptionsResourcePaymentSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "payment_method_options" => Ok(Deserialize::begin(&mut self.payment_method_options)),
                "payment_method_types" => Ok(Deserialize::begin(&mut self.payment_method_types)),
                "save_default_payment_method" => Ok(Deserialize::begin(&mut self.save_default_payment_method)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { payment_method_options: Deserialize::default(), payment_method_types: Deserialize::default(), save_default_payment_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let payment_method_options = self.payment_method_options.take()?;
            let payment_method_types = self.payment_method_types.take()?;
            let save_default_payment_method = self.save_default_payment_method.take()?;

            Some(Self::Out { payment_method_options, payment_method_types, save_default_payment_method })
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

    impl ObjectDeser for SubscriptionsResourcePaymentSettings {
        type Builder = SubscriptionsResourcePaymentSettingsBuilder;
    }
};
/// The list of payment method types to provide to every invoice created by the subscription.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use SubscriptionsResourcePaymentSettingsPaymentMethodTypes::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            AcssDebit => "acss_debit",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Konbini => "konbini",
            Link => "link",
            Paynow => "paynow",
            Paypal => "paypal",
            Promptpay => "promptpay",
            SepaCreditTransfer => "sepa_credit_transfer",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourcePaymentSettingsPaymentMethodTypes::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "acss_debit" => Ok(AcssDebit),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "promptpay" => Ok(Promptpay),
            "sepa_credit_transfer" => Ok(SepaCreditTransfer),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SubscriptionsResourcePaymentSettingsPaymentMethodTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionsResourcePaymentSettingsPaymentMethodTypes::from_str(s).unwrap_or(SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Unknown));
        Ok(())
    }
}
/// Either `off`, or `on_subscription`.
/// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}
impl SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::*;
        match self {
            Off => "off",
            OnSubscription => "on_subscription",
        }
    }
}

impl std::str::FromStr for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::*;
        match s {
            "off" => Ok(Off),
            "on_subscription" => Ok(OnSubscription),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
