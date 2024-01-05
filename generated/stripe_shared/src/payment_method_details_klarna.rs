#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKlarna {
    /// The Klarna payment method used for this transaction.
    /// Can be one of `pay_later`, `pay_now`, `pay_with_financing`, or `pay_in_installments`
    pub payment_method_category: Option<String>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    /// Can be one of `de-AT`, `en-AT`, `nl-BE`, `fr-BE`, `en-BE`, `de-DE`, `en-DE`, `da-DK`, `en-DK`, `es-ES`, `en-ES`, `fi-FI`, `sv-FI`, `en-FI`, `en-GB`, `en-IE`, `it-IT`, `en-IT`, `nl-NL`, `en-NL`, `nb-NO`, `en-NO`, `sv-SE`, `en-SE`, `en-US`, `es-US`, `fr-FR`, `en-FR`, `cs-CZ`, `en-CZ`, `el-GR`, `en-GR`, `en-AU`, `en-NZ`, `en-CA`, `fr-CA`, `pl-PL`, `en-PL`, `pt-PT`, `en-PT`, `de-CH`, `fr-CH`, `it-CH`, or `en-CH`.
    pub preferred_locale: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsKlarnaBuilder {
    payment_method_category: Option<Option<String>>,
    preferred_locale: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsKlarna>,
        builder: PaymentMethodDetailsKlarnaBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsKlarnaBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsKlarnaBuilder {
        type Out = PaymentMethodDetailsKlarna;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "payment_method_category" => Ok(Deserialize::begin(&mut self.payment_method_category)),
                "preferred_locale" => Ok(Deserialize::begin(&mut self.preferred_locale)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { payment_method_category: Deserialize::default(), preferred_locale: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let payment_method_category = self.payment_method_category.take()?;
            let preferred_locale = self.preferred_locale.take()?;

            Some(Self::Out { payment_method_category, preferred_locale })
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

    impl ObjectDeser for PaymentMethodDetailsKlarna {
        type Builder = PaymentMethodDetailsKlarnaBuilder;
    }
};
