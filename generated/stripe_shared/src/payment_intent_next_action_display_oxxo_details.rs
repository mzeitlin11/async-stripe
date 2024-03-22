#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionDisplayOxxoDetails {
    /// The timestamp after which the OXXO voucher expires.
    pub expires_after: Option<stripe_types::Timestamp>,
    /// The URL for the hosted OXXO voucher page, which allows customers to view and print an OXXO voucher.
    pub hosted_voucher_url: Option<String>,
    /// OXXO reference number.
    pub number: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionDisplayOxxoDetailsBuilder {
    expires_after: Option<Option<stripe_types::Timestamp>>,
    hosted_voucher_url: Option<Option<String>>,
    number: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionDisplayOxxoDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionDisplayOxxoDetails>,
        builder: PaymentIntentNextActionDisplayOxxoDetailsBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionDisplayOxxoDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionDisplayOxxoDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionDisplayOxxoDetailsBuilder {
        type Out = PaymentIntentNextActionDisplayOxxoDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "expires_after" => Ok(Deserialize::begin(&mut self.expires_after)),
                "hosted_voucher_url" => Ok(Deserialize::begin(&mut self.hosted_voucher_url)),
                "number" => Ok(Deserialize::begin(&mut self.number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { expires_after: Deserialize::default(), hosted_voucher_url: Deserialize::default(), number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let expires_after = self.expires_after.take()?;
            let hosted_voucher_url = self.hosted_voucher_url.take()?;
            let number = self.number.take()?;

            Some(Self::Out { expires_after, hosted_voucher_url, number })
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

    impl ObjectDeser for PaymentIntentNextActionDisplayOxxoDetails {
        type Builder = PaymentIntentNextActionDisplayOxxoDetailsBuilder;
    }
};
