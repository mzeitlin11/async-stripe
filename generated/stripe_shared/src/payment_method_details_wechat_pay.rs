#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsWechatPay {
    /// Uniquely identifies this particular WeChat Pay account.
    /// You can use this attribute to check whether two WeChat accounts are the same.
    pub fingerprint: Option<String>,
    /// Transaction ID of this particular WeChat Pay transaction.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsWechatPayBuilder {
    fingerprint: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsWechatPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsWechatPay>,
        builder: PaymentMethodDetailsWechatPayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsWechatPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsWechatPayBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsWechatPayBuilder {
        type Out = PaymentMethodDetailsWechatPay;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "transaction_id" => Ok(Deserialize::begin(&mut self.transaction_id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { fingerprint: Deserialize::default(), transaction_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let fingerprint = self.fingerprint.take()?;
            let transaction_id = self.transaction_id.take()?;

            Some(Self::Out { fingerprint, transaction_id })
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

    impl ObjectDeser for PaymentMethodDetailsWechatPay {
        type Builder = PaymentMethodDetailsWechatPayBuilder;
    }
};
