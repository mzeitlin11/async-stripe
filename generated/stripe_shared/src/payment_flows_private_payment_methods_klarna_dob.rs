#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: Option<i64>,
    /// The month of birth, between 1 and 12.
    pub month: Option<i64>,
    /// The four-digit year of birth.
    pub year: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaDobBuilder {
    day: Option<Option<i64>>,
    month: Option<Option<i64>>,
    year: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsKlarnaDob {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsKlarnaDob>,
        builder: PaymentFlowsPrivatePaymentMethodsKlarnaDobBuilder,
    }

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsKlarnaDob> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentFlowsPrivatePaymentMethodsKlarnaDobBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsKlarnaDobBuilder {
        type Out = PaymentFlowsPrivatePaymentMethodsKlarnaDob;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "day" => Ok(Deserialize::begin(&mut self.day)),
                "month" => Ok(Deserialize::begin(&mut self.month)),
                "year" => Ok(Deserialize::begin(&mut self.year)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { day: Deserialize::default(), month: Deserialize::default(), year: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let day = self.day.take()?;
            let month = self.month.take()?;
            let year = self.year.take()?;

            Some(Self::Out { day, month, year })
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

    impl ObjectDeser for PaymentFlowsPrivatePaymentMethodsKlarnaDob {
        type Builder = PaymentFlowsPrivatePaymentMethodsKlarnaDobBuilder;
    }
};
