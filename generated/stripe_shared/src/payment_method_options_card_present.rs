#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCardPresent {
    /// Request ability to capture this payment beyond the standard [authorization validity window](https://stripe.com/docs/terminal/features/extended-authorizations#authorization-validity).
    pub request_extended_authorization: Option<bool>,
    /// Request ability to [increment](https://stripe.com/docs/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible.
    /// Check [incremental_authorization_supported](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://stripe.com/docs/api/payment_intents/confirm) response to verify support.
    pub request_incremental_authorization_support: Option<bool>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodOptionsCardPresentBuilder {
    request_extended_authorization: Option<Option<bool>>,
    request_incremental_authorization_support: Option<Option<bool>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodOptionsCardPresent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCardPresent>,
        builder: PaymentMethodOptionsCardPresentBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCardPresent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodOptionsCardPresentBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCardPresentBuilder {
        type Out = PaymentMethodOptionsCardPresent;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "request_extended_authorization" => Ok(Deserialize::begin(&mut self.request_extended_authorization)),
                "request_incremental_authorization_support" => Ok(Deserialize::begin(&mut self.request_incremental_authorization_support)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { request_extended_authorization: Deserialize::default(), request_incremental_authorization_support: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let request_extended_authorization = self.request_extended_authorization.take()?;
            let request_incremental_authorization_support = self.request_incremental_authorization_support.take()?;

            Some(Self::Out { request_extended_authorization, request_incremental_authorization_support })
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

    impl ObjectDeser for PaymentMethodOptionsCardPresent {
        type Builder = PaymentMethodOptionsCardPresentBuilder;
    }
};
