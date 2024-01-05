#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsAfterpayClearpay {
    /// The Afterpay order ID associated with this payment intent.
    pub order_id: Option<String>,
    /// Order identifier shown to the merchant in Afterpay’s online portal.
    pub reference: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsAfterpayClearpayBuilder {
    order_id: Option<Option<String>>,
    reference: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsAfterpayClearpay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsAfterpayClearpay>,
        builder: PaymentMethodDetailsAfterpayClearpayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsAfterpayClearpay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsAfterpayClearpayBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsAfterpayClearpayBuilder {
        type Out = PaymentMethodDetailsAfterpayClearpay;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "order_id" => Ok(Deserialize::begin(&mut self.order_id)),
                "reference" => Ok(Deserialize::begin(&mut self.reference)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { order_id: Deserialize::default(), reference: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let order_id = self.order_id.take()?;
            let reference = self.reference.take()?;

            Some(Self::Out { order_id, reference })
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

    impl ObjectDeser for PaymentMethodDetailsAfterpayClearpay {
        type Builder = PaymentMethodDetailsAfterpayClearpayBuilder;
    }
};
