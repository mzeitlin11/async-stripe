#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceCustomFieldsNumericBuilder {
    maximum_length: Option<Option<i64>>,
    minimum_length: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceCustomFieldsNumeric {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomFieldsNumeric>,
        builder: PaymentLinksResourceCustomFieldsNumericBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomFieldsNumeric> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceCustomFieldsNumericBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCustomFieldsNumericBuilder {
        type Out = PaymentLinksResourceCustomFieldsNumeric;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "maximum_length" => Ok(Deserialize::begin(&mut self.maximum_length)),
                "minimum_length" => Ok(Deserialize::begin(&mut self.minimum_length)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { maximum_length: Deserialize::default(), minimum_length: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let maximum_length = self.maximum_length.take()?;
            let minimum_length = self.minimum_length.take()?;

            Some(Self::Out { maximum_length, minimum_length })
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

    impl ObjectDeser for PaymentLinksResourceCustomFieldsNumeric {
        type Builder = PaymentLinksResourceCustomFieldsNumericBuilder;
    }
};
