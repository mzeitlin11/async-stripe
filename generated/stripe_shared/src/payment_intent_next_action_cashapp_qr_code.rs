#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionCashappQrCode {
    /// The date (unix timestamp) when the QR code expires.
    pub expires_at: stripe_types::Timestamp,
    /// The image_url_png string used to render QR code
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionCashappQrCodeBuilder {
    expires_at: Option<stripe_types::Timestamp>,
    image_url_png: Option<String>,
    image_url_svg: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionCashappQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionCashappQrCode>,
        builder: PaymentIntentNextActionCashappQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionCashappQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionCashappQrCodeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionCashappQrCodeBuilder {
        type Out = PaymentIntentNextActionCashappQrCode;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "image_url_png" => Ok(Deserialize::begin(&mut self.image_url_png)),
                "image_url_svg" => Ok(Deserialize::begin(&mut self.image_url_svg)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { expires_at: Deserialize::default(), image_url_png: Deserialize::default(), image_url_svg: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let expires_at = self.expires_at.take()?;
            let image_url_png = self.image_url_png.take()?;
            let image_url_svg = self.image_url_svg.take()?;

            Some(Self::Out { expires_at, image_url_png, image_url_svg })
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

    impl ObjectDeser for PaymentIntentNextActionCashappQrCode {
        type Builder = PaymentIntentNextActionCashappQrCodeBuilder;
    }
};
