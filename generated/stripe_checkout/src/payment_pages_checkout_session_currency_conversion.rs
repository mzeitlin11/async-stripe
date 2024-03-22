#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCurrencyConversion {
    /// Total of all items in source currency before discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total of all items in source currency after discounts and taxes are applied.
    pub amount_total: i64,
    /// Exchange rate used to convert source currency amounts to customer currency amounts
    pub fx_rate: String,
    /// Creation currency of the CheckoutSession before localization
    pub source_currency: stripe_types::Currency,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionCurrencyConversionBuilder {
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    fx_rate: Option<String>,
    source_currency: Option<stripe_types::Currency>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCurrencyConversion {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCurrencyConversion>,
        builder: PaymentPagesCheckoutSessionCurrencyConversionBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCurrencyConversion> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionCurrencyConversionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCurrencyConversionBuilder {
        type Out = PaymentPagesCheckoutSessionCurrencyConversion;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_subtotal" => Ok(Deserialize::begin(&mut self.amount_subtotal)),
                "amount_total" => Ok(Deserialize::begin(&mut self.amount_total)),
                "fx_rate" => Ok(Deserialize::begin(&mut self.fx_rate)),
                "source_currency" => Ok(Deserialize::begin(&mut self.source_currency)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount_subtotal: Deserialize::default(), amount_total: Deserialize::default(), fx_rate: Deserialize::default(), source_currency: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_subtotal = self.amount_subtotal.take()?;
            let amount_total = self.amount_total.take()?;
            let fx_rate = self.fx_rate.take()?;
            let source_currency = self.source_currency.take()?;

            Some(Self::Out { amount_subtotal, amount_total, fx_rate, source_currency })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCurrencyConversion {
        type Builder = PaymentPagesCheckoutSessionCurrencyConversionBuilder;
    }
};
