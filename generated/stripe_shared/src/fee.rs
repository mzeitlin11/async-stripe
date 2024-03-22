#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Fee {
    /// Amount of the fee, in cents.
    pub amount: i64,
    /// ID of the Connect application that earned the fee.
    pub application: Option<String>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Type of the fee, one of: `application_fee`, `stripe_fee` or `tax`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: String,
}
#[cfg(feature = "min-ser")]
pub struct FeeBuilder {
    amount: Option<i64>,
    application: Option<Option<String>>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    type_: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Fee {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Fee>,
        builder: FeeBuilder,
    }

    impl Visitor for Place<Fee> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FeeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FeeBuilder {
        type Out = Fee;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), application: Deserialize::default(), currency: Deserialize::default(), description: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let application = self.application.take()?;
            let currency = self.currency.take()?;
            let description = self.description.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { amount, application, currency, description, type_ })
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

    impl ObjectDeser for Fee {
        type Builder = FeeBuilder;
    }
};
