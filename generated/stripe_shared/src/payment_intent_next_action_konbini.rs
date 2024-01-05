#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionKonbini {
    /// The timestamp at which the pending Konbini payment expires.
    pub expires_at: stripe_types::Timestamp,
    /// The URL for the Konbini payment instructions page, which allows customers to view and print a Konbini voucher.
    pub hosted_voucher_url: Option<String>,
    pub stores: stripe_shared::PaymentIntentNextActionKonbiniStores,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionKonbiniBuilder {
    expires_at: Option<stripe_types::Timestamp>,
    hosted_voucher_url: Option<Option<String>>,
    stores: Option<stripe_shared::PaymentIntentNextActionKonbiniStores>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionKonbini {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionKonbini>,
        builder: PaymentIntentNextActionKonbiniBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionKonbini> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionKonbiniBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionKonbiniBuilder {
        type Out = PaymentIntentNextActionKonbini;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "hosted_voucher_url" => Ok(Deserialize::begin(&mut self.hosted_voucher_url)),
                "stores" => Ok(Deserialize::begin(&mut self.stores)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { expires_at: Deserialize::default(), hosted_voucher_url: Deserialize::default(), stores: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let expires_at = self.expires_at.take()?;
            let hosted_voucher_url = self.hosted_voucher_url.take()?;
            let stores = self.stores.take()?;

            Some(Self::Out { expires_at, hosted_voucher_url, stores })
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

    impl ObjectDeser for PaymentIntentNextActionKonbini {
        type Builder = PaymentIntentNextActionKonbiniBuilder;
    }
};
