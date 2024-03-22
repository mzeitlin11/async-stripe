#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFieldsDropdown {
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: Vec<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsOption>,
    /// The option selected by the customer. This will be the `value` for the option.
    pub value: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder {
    options: Option<Vec<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsOption>>,
    value: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCustomFieldsDropdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFieldsDropdown>,
        builder: PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFieldsDropdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder {
        type Out = PaymentPagesCheckoutSessionCustomFieldsDropdown;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "options" => Ok(Deserialize::begin(&mut self.options)),
                "value" => Ok(Deserialize::begin(&mut self.value)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { options: Deserialize::default(), value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let options = self.options.take()?;
            let value = self.value.take()?;

            Some(Self::Out { options, value })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomFieldsDropdown {
        type Builder = PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder;
    }
};
