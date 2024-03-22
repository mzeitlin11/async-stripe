#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionWechatPayRedirectToAndroidApp {
    /// app_id is the APP ID registered on WeChat open platform
    pub app_id: String,
    /// nonce_str is a random string
    pub nonce_str: String,
    /// package is static value
    pub package: String,
    /// an unique merchant ID assigned by WeChat Pay
    pub partner_id: String,
    /// an unique trading ID assigned by WeChat Pay
    pub prepay_id: String,
    /// A signature
    pub sign: String,
    /// Specifies the current time in epoch format
    pub timestamp: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder {
    app_id: Option<String>,
    nonce_str: Option<String>,
    package: Option<String>,
    partner_id: Option<String>,
    prepay_id: Option<String>,
    sign: Option<String>,
    timestamp: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionWechatPayRedirectToAndroidApp {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionWechatPayRedirectToAndroidApp>,
        builder: PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionWechatPayRedirectToAndroidApp> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder {
        type Out = PaymentIntentNextActionWechatPayRedirectToAndroidApp;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "app_id" => Ok(Deserialize::begin(&mut self.app_id)),
                "nonce_str" => Ok(Deserialize::begin(&mut self.nonce_str)),
                "package" => Ok(Deserialize::begin(&mut self.package)),
                "partner_id" => Ok(Deserialize::begin(&mut self.partner_id)),
                "prepay_id" => Ok(Deserialize::begin(&mut self.prepay_id)),
                "sign" => Ok(Deserialize::begin(&mut self.sign)),
                "timestamp" => Ok(Deserialize::begin(&mut self.timestamp)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                app_id: Deserialize::default(),
                nonce_str: Deserialize::default(),
                package: Deserialize::default(),
                partner_id: Deserialize::default(),
                prepay_id: Deserialize::default(),
                sign: Deserialize::default(),
                timestamp: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let app_id = self.app_id.take()?;
            let nonce_str = self.nonce_str.take()?;
            let package = self.package.take()?;
            let partner_id = self.partner_id.take()?;
            let prepay_id = self.prepay_id.take()?;
            let sign = self.sign.take()?;
            let timestamp = self.timestamp.take()?;

            Some(Self::Out { app_id, nonce_str, package, partner_id, prepay_id, sign, timestamp })
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

    impl ObjectDeser for PaymentIntentNextActionWechatPayRedirectToAndroidApp {
        type Builder = PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder;
    }
};
