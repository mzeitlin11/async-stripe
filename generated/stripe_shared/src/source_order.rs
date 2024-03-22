#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceOrder {
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The email address of the customer placing the order.
    pub email: Option<String>,
    /// List of items constituting the order.
    pub items: Option<Vec<stripe_shared::SourceOrderItem>>,
    pub shipping: Option<stripe_shared::Shipping>,
}
#[cfg(feature = "min-ser")]
pub struct SourceOrderBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    email: Option<Option<String>>,
    items: Option<Option<Vec<stripe_shared::SourceOrderItem>>>,
    shipping: Option<Option<stripe_shared::Shipping>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceOrder {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceOrder>,
        builder: SourceOrderBuilder,
    }

    impl Visitor for Place<SourceOrder> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceOrderBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceOrderBuilder {
        type Out = SourceOrder;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "email" => Ok(Deserialize::begin(&mut self.email)),
                "items" => Ok(Deserialize::begin(&mut self.items)),
                "shipping" => Ok(Deserialize::begin(&mut self.shipping)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), currency: Deserialize::default(), email: Deserialize::default(), items: Deserialize::default(), shipping: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let currency = self.currency.take()?;
            let email = self.email.take()?;
            let items = self.items.take()?;
            let shipping = self.shipping.take()?;

            Some(Self::Out { amount, currency, email, items, shipping })
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

    impl ObjectDeser for SourceOrder {
        type Builder = SourceOrderBuilder;
    }
};
