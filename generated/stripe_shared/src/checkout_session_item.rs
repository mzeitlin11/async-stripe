/// A line item.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CheckoutSessionItem {
    /// Total discount amount applied. If no discounts were applied, defaults to 0.
    pub amount_discount: i64,
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total tax amount applied. If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    /// Total after discounts and taxes.
    pub amount_total: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    /// Often useful for displaying to users.
    /// Defaults to product name.
    pub description: String,
    /// The discounts applied to the line item.
    pub discounts: Option<Vec<stripe_shared::LineItemsDiscountAmount>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::CheckoutSessionItemId,
    /// The price used to generate the line item.
    pub price: Option<stripe_shared::Price>,
    /// The quantity of products being purchased.
    pub quantity: Option<u64>,
    /// The taxes applied to the line item.
    pub taxes: Option<Vec<stripe_shared::LineItemsTaxAmount>>,
}
#[cfg(feature = "min-ser")]
pub struct CheckoutSessionItemBuilder {
    amount_discount: Option<i64>,
    amount_subtotal: Option<i64>,
    amount_tax: Option<i64>,
    amount_total: Option<i64>,
    currency: Option<stripe_types::Currency>,
    description: Option<String>,
    discounts: Option<Option<Vec<stripe_shared::LineItemsDiscountAmount>>>,
    id: Option<stripe_shared::CheckoutSessionItemId>,
    price: Option<Option<stripe_shared::Price>>,
    quantity: Option<Option<u64>>,
    taxes: Option<Option<Vec<stripe_shared::LineItemsTaxAmount>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutSessionItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutSessionItem>,
        builder: CheckoutSessionItemBuilder,
    }

    impl Visitor for Place<CheckoutSessionItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CheckoutSessionItemBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CheckoutSessionItemBuilder {
        type Out = CheckoutSessionItem;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_discount" => Ok(Deserialize::begin(&mut self.amount_discount)),
                "amount_subtotal" => Ok(Deserialize::begin(&mut self.amount_subtotal)),
                "amount_tax" => Ok(Deserialize::begin(&mut self.amount_tax)),
                "amount_total" => Ok(Deserialize::begin(&mut self.amount_total)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "discounts" => Ok(Deserialize::begin(&mut self.discounts)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "price" => Ok(Deserialize::begin(&mut self.price)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),
                "taxes" => Ok(Deserialize::begin(&mut self.taxes)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount_discount: Deserialize::default(),
                amount_subtotal: Deserialize::default(),
                amount_tax: Deserialize::default(),
                amount_total: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                discounts: Deserialize::default(),
                id: Deserialize::default(),
                price: Deserialize::default(),
                quantity: Deserialize::default(),
                taxes: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_discount = self.amount_discount.take()?;
            let amount_subtotal = self.amount_subtotal.take()?;
            let amount_tax = self.amount_tax.take()?;
            let amount_total = self.amount_total.take()?;
            let currency = self.currency.take()?;
            let description = self.description.take()?;
            let discounts = self.discounts.take()?;
            let id = self.id.take()?;
            let price = self.price.take()?;
            let quantity = self.quantity.take()?;
            let taxes = self.taxes.take()?;

            Some(Self::Out { amount_discount, amount_subtotal, amount_tax, amount_total, currency, description, discounts, id, price, quantity, taxes })
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

    impl ObjectDeser for CheckoutSessionItem {
        type Builder = CheckoutSessionItemBuilder;
    }
};
impl stripe_types::Object for CheckoutSessionItem {
    type Id = stripe_shared::CheckoutSessionItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CheckoutSessionItemId, "li_");
