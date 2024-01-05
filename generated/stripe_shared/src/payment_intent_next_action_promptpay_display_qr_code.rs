#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionPromptpayDisplayQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: String,
    /// The URL to the hosted PromptPay instructions page, which allows customers to view the PromptPay QR code.
    pub hosted_instructions_url: String,
    /// The PNG path used to render the QR code, can be used as the source in an HTML img tag
    pub image_url_png: String,
    /// The SVG path used to render the QR code, can be used as the source in an HTML img tag
    pub image_url_svg: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionPromptpayDisplayQrCodeBuilder {
    data: Option<String>,
    hosted_instructions_url: Option<String>,
    image_url_png: Option<String>,
    image_url_svg: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionPromptpayDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionPromptpayDisplayQrCode>,
        builder: PaymentIntentNextActionPromptpayDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionPromptpayDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionPromptpayDisplayQrCodeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionPromptpayDisplayQrCodeBuilder {
        type Out = PaymentIntentNextActionPromptpayDisplayQrCode;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "data" => Ok(Deserialize::begin(&mut self.data)),
                "hosted_instructions_url" => Ok(Deserialize::begin(&mut self.hosted_instructions_url)),
                "image_url_png" => Ok(Deserialize::begin(&mut self.image_url_png)),
                "image_url_svg" => Ok(Deserialize::begin(&mut self.image_url_svg)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { data: Deserialize::default(), hosted_instructions_url: Deserialize::default(), image_url_png: Deserialize::default(), image_url_svg: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let data = self.data.take()?;
            let hosted_instructions_url = self.hosted_instructions_url.take()?;
            let image_url_png = self.image_url_png.take()?;
            let image_url_svg = self.image_url_svg.take()?;

            Some(Self::Out { data, hosted_instructions_url, image_url_png, image_url_svg })
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

    impl ObjectDeser for PaymentIntentNextActionPromptpayDisplayQrCode {
        type Builder = PaymentIntentNextActionPromptpayDisplayQrCodeBuilder;
    }
};
