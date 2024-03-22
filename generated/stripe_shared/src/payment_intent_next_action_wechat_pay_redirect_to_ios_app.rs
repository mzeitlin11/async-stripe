#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionWechatPayRedirectToIosApp {
    /// An universal link that redirect to WeChat Pay app
    pub native_url: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionWechatPayRedirectToIosAppBuilder {
    native_url: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionWechatPayRedirectToIosApp {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionWechatPayRedirectToIosApp>,
        builder: PaymentIntentNextActionWechatPayRedirectToIosAppBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionWechatPayRedirectToIosApp> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionWechatPayRedirectToIosAppBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionWechatPayRedirectToIosAppBuilder {
        type Out = PaymentIntentNextActionWechatPayRedirectToIosApp;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "native_url" => Ok(Deserialize::begin(&mut self.native_url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { native_url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let native_url = self.native_url.take()?;

            Some(Self::Out { native_url })
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

    impl ObjectDeser for PaymentIntentNextActionWechatPayRedirectToIosApp {
        type Builder = PaymentIntentNextActionWechatPayRedirectToIosAppBuilder;
    }
};
