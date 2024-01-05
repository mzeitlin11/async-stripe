#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct QuotesResourceComputed {
    /// The definitive totals and line items the customer will be charged on a recurring basis.
    /// Takes into account the line items with recurring prices and discounts with `duration=forever` coupons only.
    /// Defaults to `null` if no inputted line items with recurring prices.
    pub recurring: Option<stripe_shared::QuotesResourceRecurring>,
    pub upfront: stripe_shared::QuotesResourceUpfront,
}
#[cfg(feature = "min-ser")]
pub struct QuotesResourceComputedBuilder {
    recurring: Option<Option<stripe_shared::QuotesResourceRecurring>>,
    upfront: Option<stripe_shared::QuotesResourceUpfront>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceComputed {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceComputed>,
        builder: QuotesResourceComputedBuilder,
    }

    impl Visitor for Place<QuotesResourceComputed> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: QuotesResourceComputedBuilder::deser_default() }))
        }
    }

    impl MapBuilder for QuotesResourceComputedBuilder {
        type Out = QuotesResourceComputed;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "recurring" => Ok(Deserialize::begin(&mut self.recurring)),
                "upfront" => Ok(Deserialize::begin(&mut self.upfront)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { recurring: Deserialize::default(), upfront: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let recurring = self.recurring.take()?;
            let upfront = self.upfront.take()?;

            Some(Self::Out { recurring, upfront })
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

    impl ObjectDeser for QuotesResourceComputed {
        type Builder = QuotesResourceComputedBuilder;
    }
};
