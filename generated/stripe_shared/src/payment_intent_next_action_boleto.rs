#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionBoleto {
    /// The timestamp after which the boleto expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The URL to the hosted boleto voucher page, which allows customers to view the boleto voucher.
    pub hosted_voucher_url: Option<String>,
    /// The boleto number.
    pub number: Option<String>,
    /// The URL to the downloadable boleto voucher PDF.
    pub pdf: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionBoletoBuilder {
    expires_at: Option<Option<stripe_types::Timestamp>>,
    hosted_voucher_url: Option<Option<String>>,
    number: Option<Option<String>>,
    pdf: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionBoleto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionBoleto>,
        builder: PaymentIntentNextActionBoletoBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionBoleto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionBoletoBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionBoletoBuilder {
        type Out = PaymentIntentNextActionBoleto;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "hosted_voucher_url" => Ok(Deserialize::begin(&mut self.hosted_voucher_url)),
                "number" => Ok(Deserialize::begin(&mut self.number)),
                "pdf" => Ok(Deserialize::begin(&mut self.pdf)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { expires_at: Deserialize::default(), hosted_voucher_url: Deserialize::default(), number: Deserialize::default(), pdf: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let expires_at = self.expires_at.take()?;
            let hosted_voucher_url = self.hosted_voucher_url.take()?;
            let number = self.number.take()?;
            let pdf = self.pdf.take()?;

            Some(Self::Out { expires_at, hosted_voucher_url, number, pdf })
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

    impl ObjectDeser for PaymentIntentNextActionBoleto {
        type Builder = PaymentIntentNextActionBoletoBuilder;
    }
};
