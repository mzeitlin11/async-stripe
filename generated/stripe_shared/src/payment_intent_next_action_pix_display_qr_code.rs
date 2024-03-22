#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionPixDisplayQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: Option<String>,
    /// The date (unix timestamp) when the PIX expires.
    pub expires_at: Option<i64>,
    /// The URL to the hosted pix instructions page, which allows customers to view the pix QR code.
    pub hosted_instructions_url: Option<String>,
    /// The image_url_png string used to render png QR code
    pub image_url_png: Option<String>,
    /// The image_url_svg string used to render svg QR code
    pub image_url_svg: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionPixDisplayQrCodeBuilder {
    data: Option<Option<String>>,
    expires_at: Option<Option<i64>>,
    hosted_instructions_url: Option<Option<String>>,
    image_url_png: Option<Option<String>>,
    image_url_svg: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionPixDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionPixDisplayQrCode>,
        builder: PaymentIntentNextActionPixDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionPixDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionPixDisplayQrCodeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionPixDisplayQrCodeBuilder {
        type Out = PaymentIntentNextActionPixDisplayQrCode;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "data" => Ok(Deserialize::begin(&mut self.data)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "hosted_instructions_url" => Ok(Deserialize::begin(&mut self.hosted_instructions_url)),
                "image_url_png" => Ok(Deserialize::begin(&mut self.image_url_png)),
                "image_url_svg" => Ok(Deserialize::begin(&mut self.image_url_svg)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                data: Deserialize::default(),
                expires_at: Deserialize::default(),
                hosted_instructions_url: Deserialize::default(),
                image_url_png: Deserialize::default(),
                image_url_svg: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let data = self.data.take()?;
            let expires_at = self.expires_at.take()?;
            let hosted_instructions_url = self.hosted_instructions_url.take()?;
            let image_url_png = self.image_url_png.take()?;
            let image_url_svg = self.image_url_svg.take()?;

            Some(Self::Out { data, expires_at, hosted_instructions_url, image_url_png, image_url_svg })
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

    impl ObjectDeser for PaymentIntentNextActionPixDisplayQrCode {
        type Builder = PaymentIntentNextActionPixDisplayQrCodeBuilder;
    }
};
