/// A Tax Calculation allows you to calculate the tax to collect from your customer.
///
/// Related guide: [Calculate tax in your custom payment flow](https://stripe.com/docs/tax/custom)
///
/// For more details see <<https://stripe.com/docs/api/tax/calculations/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxCalculation {
    /// Total after taxes.
    pub amount_total: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<String>,
    pub customer_details: stripe_misc::TaxProductResourceCustomerDetails,
    /// Timestamp of date at which the tax calculation will expire.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the calculation.
    pub id: Option<stripe_misc::TaxCalculationId>,
    /// The list of items the customer is purchasing.
    pub line_items: Option<stripe_types::List<stripe_misc::TaxCalculationLineItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The shipping cost details for the calculation.
    pub shipping_cost: Option<stripe_misc::TaxProductResourceTaxCalculationShippingCost>,
    /// The amount of tax to be collected on top of the line item prices.
    pub tax_amount_exclusive: i64,
    /// The amount of tax already included in the line item prices.
    pub tax_amount_inclusive: i64,
    /// Breakdown of individual tax amounts that add up to the total.
    pub tax_breakdown: Vec<stripe_misc::TaxProductResourceTaxBreakdown>,
    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: stripe_types::Timestamp,
}
#[cfg(feature = "min-ser")]
pub struct TaxCalculationBuilder {
    amount_total: Option<i64>,
    currency: Option<stripe_types::Currency>,
    customer: Option<Option<String>>,
    customer_details: Option<stripe_misc::TaxProductResourceCustomerDetails>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<Option<stripe_misc::TaxCalculationId>>,
    line_items: Option<Option<stripe_types::List<stripe_misc::TaxCalculationLineItem>>>,
    livemode: Option<bool>,
    shipping_cost: Option<Option<stripe_misc::TaxProductResourceTaxCalculationShippingCost>>,
    tax_amount_exclusive: Option<i64>,
    tax_amount_inclusive: Option<i64>,
    tax_breakdown: Option<Vec<stripe_misc::TaxProductResourceTaxBreakdown>>,
    tax_date: Option<stripe_types::Timestamp>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxCalculation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxCalculation>,
        builder: TaxCalculationBuilder,
    }

    impl Visitor for Place<TaxCalculation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxCalculationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxCalculationBuilder {
        type Out = TaxCalculation;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_total" => Ok(Deserialize::begin(&mut self.amount_total)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "customer_details" => Ok(Deserialize::begin(&mut self.customer_details)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "line_items" => Ok(Deserialize::begin(&mut self.line_items)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "shipping_cost" => Ok(Deserialize::begin(&mut self.shipping_cost)),
                "tax_amount_exclusive" => Ok(Deserialize::begin(&mut self.tax_amount_exclusive)),
                "tax_amount_inclusive" => Ok(Deserialize::begin(&mut self.tax_amount_inclusive)),
                "tax_breakdown" => Ok(Deserialize::begin(&mut self.tax_breakdown)),
                "tax_date" => Ok(Deserialize::begin(&mut self.tax_date)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount_total: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                customer_details: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                line_items: Deserialize::default(),
                livemode: Deserialize::default(),
                shipping_cost: Deserialize::default(),
                tax_amount_exclusive: Deserialize::default(),
                tax_amount_inclusive: Deserialize::default(),
                tax_breakdown: Deserialize::default(),
                tax_date: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_total = self.amount_total.take()?;
            let currency = self.currency.take()?;
            let customer = self.customer.take()?;
            let customer_details = self.customer_details.take()?;
            let expires_at = self.expires_at.take()?;
            let id = self.id.take()?;
            let line_items = self.line_items.take()?;
            let livemode = self.livemode.take()?;
            let shipping_cost = self.shipping_cost.take()?;
            let tax_amount_exclusive = self.tax_amount_exclusive.take()?;
            let tax_amount_inclusive = self.tax_amount_inclusive.take()?;
            let tax_breakdown = self.tax_breakdown.take()?;
            let tax_date = self.tax_date.take()?;

            Some(Self::Out {
                amount_total,
                currency,
                customer,
                customer_details,
                expires_at,
                id,
                line_items,
                livemode,
                shipping_cost,
                tax_amount_exclusive,
                tax_amount_inclusive,
                tax_breakdown,
                tax_date,
            })
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

    impl ObjectDeser for TaxCalculation {
        type Builder = TaxCalculationBuilder;
    }
};
impl stripe_types::Object for TaxCalculation {
    type Id = Option<stripe_misc::TaxCalculationId>;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxCalculationId);
