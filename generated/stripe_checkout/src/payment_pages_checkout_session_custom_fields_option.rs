#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFieldsOption {
    /// The label for the option, displayed to the customer. Up to 100 characters.
    pub label: String,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionCustomFieldsOptionBuilder {
    label: Option<String>,
    value: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCustomFieldsOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFieldsOption>,
        builder: PaymentPagesCheckoutSessionCustomFieldsOptionBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFieldsOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionCustomFieldsOptionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomFieldsOptionBuilder {
        type Out = PaymentPagesCheckoutSessionCustomFieldsOption;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "label" => Ok(Deserialize::begin(&mut self.label)),
                "value" => Ok(Deserialize::begin(&mut self.value)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { label: Deserialize::default(), value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let label = self.label.take()?;
            let value = self.value.take()?;

            Some(Self::Out { label, value })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomFieldsOption {
        type Builder = PaymentPagesCheckoutSessionCustomFieldsOptionBuilder;
    }
};
