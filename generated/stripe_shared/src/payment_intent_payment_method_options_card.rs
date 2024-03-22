#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptionsCard {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentIntentPaymentMethodOptionsCardCaptureMethod>,
    /// Installment details for this payment (Mexico only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    pub installments: Option<stripe_shared::PaymentMethodOptionsCardInstallments>,
    /// Configuration options for setting up an eMandate for cards issued in India.
    pub mandate_options: Option<stripe_shared::PaymentMethodOptionsCardMandateOptions>,
    /// Selected network to process this payment intent on.
    /// Depends on the available networks of the card attached to the payment intent.
    /// Can be only set confirm-time.
    pub network: Option<PaymentIntentPaymentMethodOptionsCardNetwork>,
    /// Request ability to [capture beyond the standard authorization validity window](https://stripe.com/docs/payments/extended-authorization) for this PaymentIntent.
    pub request_extended_authorization: Option<PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization>,
    /// Request ability to [increment](https://stripe.com/docs/payments/incremental-authorization) for this PaymentIntent.
    pub request_incremental_authorization: Option<PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization>,
    /// Request ability to make [multiple captures](https://stripe.com/docs/payments/multicapture) for this PaymentIntent.
    pub request_multicapture: Option<PaymentIntentPaymentMethodOptionsCardRequestMulticapture>,
    /// Request ability to [overcapture](https://stripe.com/docs/payments/overcapture) for this PaymentIntent.
    pub request_overcapture: Option<PaymentIntentPaymentMethodOptionsCardRequestOvercapture>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentIntentPaymentMethodOptionsCardSetupFutureUsage>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    pub statement_descriptor_suffix_kana: Option<String>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    pub statement_descriptor_suffix_kanji: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentPaymentMethodOptionsCardBuilder {
    capture_method: Option<Option<PaymentIntentPaymentMethodOptionsCardCaptureMethod>>,
    installments: Option<Option<stripe_shared::PaymentMethodOptionsCardInstallments>>,
    mandate_options: Option<Option<stripe_shared::PaymentMethodOptionsCardMandateOptions>>,
    network: Option<Option<PaymentIntentPaymentMethodOptionsCardNetwork>>,
    request_extended_authorization: Option<Option<PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization>>,
    request_incremental_authorization: Option<Option<PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization>>,
    request_multicapture: Option<Option<PaymentIntentPaymentMethodOptionsCardRequestMulticapture>>,
    request_overcapture: Option<Option<PaymentIntentPaymentMethodOptionsCardRequestOvercapture>>,
    request_three_d_secure: Option<Option<PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure>>,
    setup_future_usage: Option<Option<PaymentIntentPaymentMethodOptionsCardSetupFutureUsage>>,
    statement_descriptor_suffix_kana: Option<Option<String>>,
    statement_descriptor_suffix_kanji: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentPaymentMethodOptionsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentPaymentMethodOptionsCard>,
        builder: PaymentIntentPaymentMethodOptionsCardBuilder,
    }

    impl Visitor for Place<PaymentIntentPaymentMethodOptionsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentPaymentMethodOptionsCardBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentPaymentMethodOptionsCardBuilder {
        type Out = PaymentIntentPaymentMethodOptionsCard;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "capture_method" => Ok(Deserialize::begin(&mut self.capture_method)),
                "installments" => Ok(Deserialize::begin(&mut self.installments)),
                "mandate_options" => Ok(Deserialize::begin(&mut self.mandate_options)),
                "network" => Ok(Deserialize::begin(&mut self.network)),
                "request_extended_authorization" => Ok(Deserialize::begin(&mut self.request_extended_authorization)),
                "request_incremental_authorization" => Ok(Deserialize::begin(&mut self.request_incremental_authorization)),
                "request_multicapture" => Ok(Deserialize::begin(&mut self.request_multicapture)),
                "request_overcapture" => Ok(Deserialize::begin(&mut self.request_overcapture)),
                "request_three_d_secure" => Ok(Deserialize::begin(&mut self.request_three_d_secure)),
                "setup_future_usage" => Ok(Deserialize::begin(&mut self.setup_future_usage)),
                "statement_descriptor_suffix_kana" => Ok(Deserialize::begin(&mut self.statement_descriptor_suffix_kana)),
                "statement_descriptor_suffix_kanji" => Ok(Deserialize::begin(&mut self.statement_descriptor_suffix_kanji)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                capture_method: Deserialize::default(),
                installments: Deserialize::default(),
                mandate_options: Deserialize::default(),
                network: Deserialize::default(),
                request_extended_authorization: Deserialize::default(),
                request_incremental_authorization: Deserialize::default(),
                request_multicapture: Deserialize::default(),
                request_overcapture: Deserialize::default(),
                request_three_d_secure: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
                statement_descriptor_suffix_kana: Deserialize::default(),
                statement_descriptor_suffix_kanji: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let capture_method = self.capture_method.take()?;
            let installments = self.installments.take()?;
            let mandate_options = self.mandate_options.take()?;
            let network = self.network.take()?;
            let request_extended_authorization = self.request_extended_authorization.take()?;
            let request_incremental_authorization = self.request_incremental_authorization.take()?;
            let request_multicapture = self.request_multicapture.take()?;
            let request_overcapture = self.request_overcapture.take()?;
            let request_three_d_secure = self.request_three_d_secure.take()?;
            let setup_future_usage = self.setup_future_usage.take()?;
            let statement_descriptor_suffix_kana = self.statement_descriptor_suffix_kana.take()?;
            let statement_descriptor_suffix_kanji = self.statement_descriptor_suffix_kanji.take()?;

            Some(Self::Out {
                capture_method,
                installments,
                mandate_options,
                network,
                request_extended_authorization,
                request_incremental_authorization,
                request_multicapture,
                request_overcapture,
                request_three_d_secure,
                setup_future_usage,
                statement_descriptor_suffix_kana,
                statement_descriptor_suffix_kanji,
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

    impl ObjectDeser for PaymentIntentPaymentMethodOptionsCard {
        type Builder = PaymentIntentPaymentMethodOptionsCardBuilder;
    }
};
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    Manual,
}
impl PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardCaptureMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsCardCaptureMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsCardCaptureMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Selected network to process this payment intent on.
/// Depends on the available networks of the card attached to the payment intent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}
impl PaymentIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardNetwork::*;
        match self {
            Amex => "amex",
            CartesBancaires => "cartes_bancaires",
            Diners => "diners",
            Discover => "discover",
            EftposAu => "eftpos_au",
            Interac => "interac",
            Jcb => "jcb",
            Mastercard => "mastercard",
            Unionpay => "unionpay",
            Unknown => "unknown",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardNetwork::*;
        match s {
            "amex" => Ok(Amex),
            "cartes_bancaires" => Ok(CartesBancaires),
            "diners" => Ok(Diners),
            "discover" => Ok(Discover),
            "eftpos_au" => Ok(EftposAu),
            "interac" => Ok(Interac),
            "jcb" => Ok(Jcb),
            "mastercard" => Ok(Mastercard),
            "unionpay" => Ok(Unionpay),
            "unknown" => Ok(Unknown),
            "visa" => Ok(Visa),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardNetwork"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsCardNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsCardNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Request ability to [capture beyond the standard authorization validity window](https://stripe.com/docs/payments/extended-authorization) for this PaymentIntent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    IfAvailable,
    Never,
}
impl PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Request ability to [increment](https://stripe.com/docs/payments/incremental-authorization) for this PaymentIntent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    IfAvailable,
    Never,
}
impl PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Request ability to make [multiple captures](https://stripe.com/docs/payments/multicapture) for this PaymentIntent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    IfAvailable,
    Never,
}
impl PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestMulticapture::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestMulticapture::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardRequestMulticapture"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsCardRequestMulticapture> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsCardRequestMulticapture::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Request ability to [overcapture](https://stripe.com/docs/payments/overcapture) for this PaymentIntent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    IfAvailable,
    Never,
}
impl PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestOvercapture::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestOvercapture::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardRequestOvercapture"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsCardRequestOvercapture> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsCardRequestOvercapture::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Permitted values include: `automatic` or `any`.
/// If not provided, defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    ChallengeOnly,
}
impl PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            ChallengeOnly => "challenge_only",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge_only" => Ok(ChallengeOnly),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::from_str(s).map_err(|_| miniserde::Error)?);
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
pub enum PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentIntentPaymentMethodOptionsCardSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentIntentPaymentMethodOptionsCardSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
