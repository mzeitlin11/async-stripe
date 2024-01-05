#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentNextAction {
    pub cashapp_handle_redirect_or_display_qr_code: Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    pub redirect_to_url: Option<stripe_shared::SetupIntentNextActionRedirectToUrl>,
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: String,
    /// When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    pub use_stripe_sdk: Option<stripe_types::Value>,
    pub verify_with_microdeposits: Option<stripe_shared::SetupIntentNextActionVerifyWithMicrodeposits>,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentNextActionBuilder {
    cashapp_handle_redirect_or_display_qr_code: Option<Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>>,
    redirect_to_url: Option<Option<stripe_shared::SetupIntentNextActionRedirectToUrl>>,
    type_: Option<String>,
    use_stripe_sdk: Option<Option<stripe_types::Value>>,
    verify_with_microdeposits: Option<Option<stripe_shared::SetupIntentNextActionVerifyWithMicrodeposits>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentNextAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentNextAction>,
        builder: SetupIntentNextActionBuilder,
    }

    impl Visitor for Place<SetupIntentNextAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentNextActionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentNextActionBuilder {
        type Out = SetupIntentNextAction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "cashapp_handle_redirect_or_display_qr_code" => Ok(Deserialize::begin(&mut self.cashapp_handle_redirect_or_display_qr_code)),
                "redirect_to_url" => Ok(Deserialize::begin(&mut self.redirect_to_url)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "use_stripe_sdk" => Ok(Deserialize::begin(&mut self.use_stripe_sdk)),
                "verify_with_microdeposits" => Ok(Deserialize::begin(&mut self.verify_with_microdeposits)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                cashapp_handle_redirect_or_display_qr_code: Deserialize::default(),
                redirect_to_url: Deserialize::default(),
                type_: Deserialize::default(),
                use_stripe_sdk: Deserialize::default(),
                verify_with_microdeposits: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let cashapp_handle_redirect_or_display_qr_code = self.cashapp_handle_redirect_or_display_qr_code.take()?;
            let redirect_to_url = self.redirect_to_url.take()?;
            let type_ = self.type_.take()?;
            let use_stripe_sdk = self.use_stripe_sdk.take()?;
            let verify_with_microdeposits = self.verify_with_microdeposits.take()?;

            Some(Self::Out { cashapp_handle_redirect_or_display_qr_code, redirect_to_url, type_, use_stripe_sdk, verify_with_microdeposits })
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

    impl ObjectDeser for SetupIntentNextAction {
        type Builder = SetupIntentNextActionBuilder;
    }
};
