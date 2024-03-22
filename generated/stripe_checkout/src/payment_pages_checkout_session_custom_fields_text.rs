#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,
    /// The value entered by the customer.
    pub value: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionCustomFieldsTextBuilder {
    maximum_length: Option<Option<i64>>,
    minimum_length: Option<Option<i64>>,
    value: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCustomFieldsText {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFieldsText>,
        builder: PaymentPagesCheckoutSessionCustomFieldsTextBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFieldsText> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionCustomFieldsTextBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomFieldsTextBuilder {
        type Out = PaymentPagesCheckoutSessionCustomFieldsText;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "maximum_length" => Ok(Deserialize::begin(&mut self.maximum_length)),
                "minimum_length" => Ok(Deserialize::begin(&mut self.minimum_length)),
                "value" => Ok(Deserialize::begin(&mut self.value)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { maximum_length: Deserialize::default(), minimum_length: Deserialize::default(), value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let maximum_length = self.maximum_length.take()?;
            let minimum_length = self.minimum_length.take()?;
            let value = self.value.take()?;

            Some(Self::Out { maximum_length, minimum_length, value })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomFieldsText {
        type Builder = PaymentPagesCheckoutSessionCustomFieldsTextBuilder;
    }
};
