#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodCashapp {
    /// A unique and immutable identifier assigned by Cash App to every buyer.
    pub buyer_id: Option<String>,
    /// A public identifier for buyers using Cash App.
    pub cashtag: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodCashappBuilder {
    buyer_id: Option<Option<String>>,
    cashtag: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodCashapp {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCashapp>,
        builder: PaymentMethodCashappBuilder,
    }

    impl Visitor for Place<PaymentMethodCashapp> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodCashappBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodCashappBuilder {
        type Out = PaymentMethodCashapp;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "buyer_id" => Ok(Deserialize::begin(&mut self.buyer_id)),
                "cashtag" => Ok(Deserialize::begin(&mut self.cashtag)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { buyer_id: Deserialize::default(), cashtag: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let buyer_id = self.buyer_id.take()?;
            let cashtag = self.cashtag.take()?;

            Some(Self::Out { buyer_id, cashtag })
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

    impl ObjectDeser for PaymentMethodCashapp {
        type Builder = PaymentMethodCashappBuilder;
    }
};
