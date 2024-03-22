#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionWechatPayDisplayQrCode {
    /// The data being used to generate QR code
    pub data: String,
    /// The URL to the hosted WeChat Pay instructions page, which allows customers to view the WeChat Pay QR code.
    pub hosted_instructions_url: String,
    /// The base64 image data for a pre-generated QR code
    pub image_data_url: String,
    /// The image_url_png string used to render QR code
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionWechatPayDisplayQrCodeBuilder {
    data: Option<String>,
    hosted_instructions_url: Option<String>,
    image_data_url: Option<String>,
    image_url_png: Option<String>,
    image_url_svg: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionWechatPayDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionWechatPayDisplayQrCode>,
        builder: PaymentIntentNextActionWechatPayDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionWechatPayDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionWechatPayDisplayQrCodeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionWechatPayDisplayQrCodeBuilder {
        type Out = PaymentIntentNextActionWechatPayDisplayQrCode;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "data" => Ok(Deserialize::begin(&mut self.data)),
                "hosted_instructions_url" => Ok(Deserialize::begin(&mut self.hosted_instructions_url)),
                "image_data_url" => Ok(Deserialize::begin(&mut self.image_data_url)),
                "image_url_png" => Ok(Deserialize::begin(&mut self.image_url_png)),
                "image_url_svg" => Ok(Deserialize::begin(&mut self.image_url_svg)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                data: Deserialize::default(),
                hosted_instructions_url: Deserialize::default(),
                image_data_url: Deserialize::default(),
                image_url_png: Deserialize::default(),
                image_url_svg: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let data = self.data.take()?;
            let hosted_instructions_url = self.hosted_instructions_url.take()?;
            let image_data_url = self.image_data_url.take()?;
            let image_url_png = self.image_url_png.take()?;
            let image_url_svg = self.image_url_svg.take()?;

            Some(Self::Out { data, hosted_instructions_url, image_data_url, image_url_png, image_url_svg })
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

    impl ObjectDeser for PaymentIntentNextActionWechatPayDisplayQrCode {
        type Builder = PaymentIntentNextActionWechatPayDisplayQrCodeBuilder;
    }
};
