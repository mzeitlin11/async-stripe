#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomFieldsDropdown {
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: Vec<stripe_shared::PaymentLinksResourceCustomFieldsDropdownOption>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceCustomFieldsDropdownBuilder {
    options: Option<Vec<stripe_shared::PaymentLinksResourceCustomFieldsDropdownOption>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceCustomFieldsDropdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomFieldsDropdown>,
        builder: PaymentLinksResourceCustomFieldsDropdownBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomFieldsDropdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceCustomFieldsDropdownBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCustomFieldsDropdownBuilder {
        type Out = PaymentLinksResourceCustomFieldsDropdown;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "options" => Ok(Deserialize::begin(&mut self.options)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { options: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let options = self.options.take()?;

            Some(Self::Out { options })
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

    impl ObjectDeser for PaymentLinksResourceCustomFieldsDropdown {
        type Builder = PaymentLinksResourceCustomFieldsDropdownBuilder;
    }
};
