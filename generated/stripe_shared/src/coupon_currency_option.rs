#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CouponCurrencyOption {
    /// Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: i64,
}
#[cfg(feature = "min-ser")]
pub struct CouponCurrencyOptionBuilder {
    amount_off: Option<i64>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CouponCurrencyOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CouponCurrencyOption>,
        builder: CouponCurrencyOptionBuilder,
    }

    impl Visitor for Place<CouponCurrencyOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CouponCurrencyOptionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CouponCurrencyOptionBuilder {
        type Out = CouponCurrencyOption;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount_off" => Ok(Deserialize::begin(&mut self.amount_off)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount_off: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_off = self.amount_off.take()?;

            Some(Self::Out { amount_off })
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

    impl ObjectDeser for CouponCurrencyOption {
        type Builder = CouponCurrencyOptionBuilder;
    }
};
