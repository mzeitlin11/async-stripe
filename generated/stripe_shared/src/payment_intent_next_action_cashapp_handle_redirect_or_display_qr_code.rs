#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
    /// The URL to the hosted Cash App Pay instructions page, which allows customers to view the QR code, and supports QR code refreshing on expiration.
    pub hosted_instructions_url: String,
    /// The url for mobile redirect based auth
    pub mobile_auth_url: String,
    pub qr_code: stripe_shared::PaymentIntentNextActionCashappQrCode,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder {
    hosted_instructions_url: Option<String>,
    mobile_auth_url: Option<String>,
    qr_code: Option<stripe_shared::PaymentIntentNextActionCashappQrCode>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
        builder: PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder {
        type Out = PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "hosted_instructions_url" => Ok(Deserialize::begin(&mut self.hosted_instructions_url)),
                "mobile_auth_url" => Ok(Deserialize::begin(&mut self.mobile_auth_url)),
                "qr_code" => Ok(Deserialize::begin(&mut self.qr_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { hosted_instructions_url: Deserialize::default(), mobile_auth_url: Deserialize::default(), qr_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let hosted_instructions_url = self.hosted_instructions_url.take()?;
            let mobile_auth_url = self.mobile_auth_url.take()?;
            let qr_code = self.qr_code.take()?;

            Some(Self::Out { hosted_instructions_url, mobile_auth_url, qr_code })
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

    impl ObjectDeser for PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
        type Builder = PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder;
    }
};
