#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionAlipayHandleRedirect {
    /// The native data to be used with Alipay SDK you must redirect your customer to in order to authenticate the payment in an Android App.
    pub native_data: Option<String>,
    /// The native URL you must redirect your customer to in order to authenticate the payment in an iOS App.
    pub native_url: Option<String>,
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    /// The URL you must redirect your customer to in order to authenticate the payment.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionAlipayHandleRedirectBuilder {
    native_data: Option<Option<String>>,
    native_url: Option<Option<String>>,
    return_url: Option<Option<String>>,
    url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionAlipayHandleRedirect {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionAlipayHandleRedirect>,
        builder: PaymentIntentNextActionAlipayHandleRedirectBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionAlipayHandleRedirect> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionAlipayHandleRedirectBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionAlipayHandleRedirectBuilder {
        type Out = PaymentIntentNextActionAlipayHandleRedirect;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "native_data" => Ok(Deserialize::begin(&mut self.native_data)),
                "native_url" => Ok(Deserialize::begin(&mut self.native_url)),
                "return_url" => Ok(Deserialize::begin(&mut self.return_url)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { native_data: Deserialize::default(), native_url: Deserialize::default(), return_url: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let native_data = self.native_data.take()?;
            let native_url = self.native_url.take()?;
            let return_url = self.return_url.take()?;
            let url = self.url.take()?;

            Some(Self::Out { native_data, native_url, return_url, url })
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

    impl ObjectDeser for PaymentIntentNextActionAlipayHandleRedirect {
        type Builder = PaymentIntentNextActionAlipayHandleRedirectBuilder;
    }
};
