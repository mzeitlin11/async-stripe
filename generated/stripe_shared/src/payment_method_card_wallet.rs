#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodCardWallet {
    pub amex_express_checkout: Option<stripe_shared::PaymentMethodCardWalletAmexExpressCheckout>,
    pub apple_pay: Option<stripe_shared::PaymentMethodCardWalletApplePay>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    pub google_pay: Option<stripe_shared::PaymentMethodCardWalletGooglePay>,
    pub link: Option<stripe_shared::PaymentMethodCardWalletLink>,
    pub masterpass: Option<stripe_shared::PaymentMethodCardWalletMasterpass>,
    pub samsung_pay: Option<stripe_shared::PaymentMethodCardWalletSamsungPay>,
    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PaymentMethodCardWalletType,
    pub visa_checkout: Option<stripe_shared::PaymentMethodCardWalletVisaCheckout>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodCardWalletBuilder {
    amex_express_checkout: Option<Option<stripe_shared::PaymentMethodCardWalletAmexExpressCheckout>>,
    apple_pay: Option<Option<stripe_shared::PaymentMethodCardWalletApplePay>>,
    dynamic_last4: Option<Option<String>>,
    google_pay: Option<Option<stripe_shared::PaymentMethodCardWalletGooglePay>>,
    link: Option<Option<stripe_shared::PaymentMethodCardWalletLink>>,
    masterpass: Option<Option<stripe_shared::PaymentMethodCardWalletMasterpass>>,
    samsung_pay: Option<Option<stripe_shared::PaymentMethodCardWalletSamsungPay>>,
    type_: Option<PaymentMethodCardWalletType>,
    visa_checkout: Option<Option<stripe_shared::PaymentMethodCardWalletVisaCheckout>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodCardWallet {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCardWallet>,
        builder: PaymentMethodCardWalletBuilder,
    }

    impl Visitor for Place<PaymentMethodCardWallet> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodCardWalletBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodCardWalletBuilder {
        type Out = PaymentMethodCardWallet;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amex_express_checkout" => Ok(Deserialize::begin(&mut self.amex_express_checkout)),
                "apple_pay" => Ok(Deserialize::begin(&mut self.apple_pay)),
                "dynamic_last4" => Ok(Deserialize::begin(&mut self.dynamic_last4)),
                "google_pay" => Ok(Deserialize::begin(&mut self.google_pay)),
                "link" => Ok(Deserialize::begin(&mut self.link)),
                "masterpass" => Ok(Deserialize::begin(&mut self.masterpass)),
                "samsung_pay" => Ok(Deserialize::begin(&mut self.samsung_pay)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "visa_checkout" => Ok(Deserialize::begin(&mut self.visa_checkout)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amex_express_checkout: Deserialize::default(),
                apple_pay: Deserialize::default(),
                dynamic_last4: Deserialize::default(),
                google_pay: Deserialize::default(),
                link: Deserialize::default(),
                masterpass: Deserialize::default(),
                samsung_pay: Deserialize::default(),
                type_: Deserialize::default(),
                visa_checkout: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amex_express_checkout = self.amex_express_checkout.take()?;
            let apple_pay = self.apple_pay.take()?;
            let dynamic_last4 = self.dynamic_last4.take()?;
            let google_pay = self.google_pay.take()?;
            let link = self.link.take()?;
            let masterpass = self.masterpass.take()?;
            let samsung_pay = self.samsung_pay.take()?;
            let type_ = self.type_.take()?;
            let visa_checkout = self.visa_checkout.take()?;

            Some(Self::Out { amex_express_checkout, apple_pay, dynamic_last4, google_pay, link, masterpass, samsung_pay, type_, visa_checkout })
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

    impl ObjectDeser for PaymentMethodCardWallet {
        type Builder = PaymentMethodCardWalletBuilder;
    }
};
/// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
/// An additional hash is included on the Wallet subhash with a name matching this value.
/// It contains additional information specific to the card wallet type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Link,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}
impl PaymentMethodCardWalletType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodCardWalletType::*;
        match self {
            AmexExpressCheckout => "amex_express_checkout",
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            Link => "link",
            Masterpass => "masterpass",
            SamsungPay => "samsung_pay",
            VisaCheckout => "visa_checkout",
        }
    }
}

impl std::str::FromStr for PaymentMethodCardWalletType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodCardWalletType::*;
        match s {
            "amex_express_checkout" => Ok(AmexExpressCheckout),
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "link" => Ok(Link),
            "masterpass" => Ok(Masterpass),
            "samsung_pay" => Ok(SamsungPay),
            "visa_checkout" => Ok(VisaCheckout),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodCardWalletType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodCardWalletType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodCardWalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodCardWalletType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodCardWalletType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodCardWalletType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodCardWalletType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
