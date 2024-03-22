#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PromotionCodesResourceRestrictions {
    /// Promotion code restrictions defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options: Option<std::collections::HashMap<stripe_types::Currency, stripe_shared::PromotionCodeCurrencyOption>>,
    /// A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices.
    pub first_time_transaction: bool,
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: Option<i64>,
    /// Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount
    pub minimum_amount_currency: Option<stripe_types::Currency>,
}
#[cfg(feature = "min-ser")]
pub struct PromotionCodesResourceRestrictionsBuilder {
    currency_options: Option<Option<std::collections::HashMap<stripe_types::Currency, stripe_shared::PromotionCodeCurrencyOption>>>,
    first_time_transaction: Option<bool>,
    minimum_amount: Option<Option<i64>>,
    minimum_amount_currency: Option<Option<stripe_types::Currency>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PromotionCodesResourceRestrictions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PromotionCodesResourceRestrictions>,
        builder: PromotionCodesResourceRestrictionsBuilder,
    }

    impl Visitor for Place<PromotionCodesResourceRestrictions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PromotionCodesResourceRestrictionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PromotionCodesResourceRestrictionsBuilder {
        type Out = PromotionCodesResourceRestrictions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "currency_options" => Ok(Deserialize::begin(&mut self.currency_options)),
                "first_time_transaction" => Ok(Deserialize::begin(&mut self.first_time_transaction)),
                "minimum_amount" => Ok(Deserialize::begin(&mut self.minimum_amount)),
                "minimum_amount_currency" => Ok(Deserialize::begin(&mut self.minimum_amount_currency)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { currency_options: Deserialize::default(), first_time_transaction: Deserialize::default(), minimum_amount: Deserialize::default(), minimum_amount_currency: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let currency_options = self.currency_options.take()?;
            let first_time_transaction = self.first_time_transaction.take()?;
            let minimum_amount = self.minimum_amount.take()?;
            let minimum_amount_currency = self.minimum_amount_currency.take()?;

            Some(Self::Out { currency_options, first_time_transaction, minimum_amount, minimum_amount_currency })
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

    impl ObjectDeser for PromotionCodesResourceRestrictions {
        type Builder = PromotionCodesResourceRestrictionsBuilder;
    }
};
