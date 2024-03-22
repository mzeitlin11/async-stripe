#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionAfterExpirationRecovery {
    /// Enables user redeemable promotion codes on the recovered Checkout Sessions. Defaults to `false`
    pub allow_promotion_codes: bool,
    /// If `true`, a recovery url will be generated to recover this Checkout Session if it
    /// expires before a transaction is completed. It will be attached to the
    /// Checkout Session object upon expiration.
    pub enabled: bool,
    /// The timestamp at which the recovery URL will expire.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// URL that creates a new Checkout Session when clicked that is a copy of this expired Checkout Session.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder {
    allow_promotion_codes: Option<bool>,
    enabled: Option<bool>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionAfterExpirationRecovery {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionAfterExpirationRecovery>,
        builder: PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionAfterExpirationRecovery> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder {
        type Out = PaymentPagesCheckoutSessionAfterExpirationRecovery;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "allow_promotion_codes" => Ok(Deserialize::begin(&mut self.allow_promotion_codes)),
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { allow_promotion_codes: Deserialize::default(), enabled: Deserialize::default(), expires_at: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let allow_promotion_codes = self.allow_promotion_codes.take()?;
            let enabled = self.enabled.take()?;
            let expires_at = self.expires_at.take()?;
            let url = self.url.take()?;

            Some(Self::Out { allow_promotion_codes, enabled, expires_at, url })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionAfterExpirationRecovery {
        type Builder = PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder;
    }
};
