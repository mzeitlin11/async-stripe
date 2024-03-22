#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceLineItem {
    /// The amount, in cents (or local equivalent).
    pub amount: i64,
    /// The integer amount in cents (or local equivalent) representing the amount for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
    /// If true, discounts will apply to this line item. Always false for prorations.
    pub discountable: bool,
    /// The discounts applied to the invoice line item.
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::InvoiceLineItemId,
    /// The ID of the [invoice item](https://stripe.com/docs/api/invoiceitems) associated with this line item if any.
    pub invoice_item: Option<stripe_types::Expandable<stripe_shared::InvoiceItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription` this will reflect the metadata of the subscription that caused the line item to be created.
    pub metadata: std::collections::HashMap<String, String>,
    pub period: stripe_shared::InvoiceLineItemPeriod,
    /// The plan of the subscription, if the line item is a subscription or a proration.
    pub plan: Option<stripe_shared::Plan>,
    /// The price of the line item.
    pub price: Option<stripe_shared::Price>,
    /// Whether this is a proration.
    pub proration: bool,
    /// Additional details for proration line items
    pub proration_details: Option<stripe_shared::InvoicesResourceLineItemsProrationDetails>,
    /// The quantity of the subscription, if the line item is a subscription or a proration.
    pub quantity: Option<u64>,
    /// The subscription that the invoice item pertains to, if any.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// The subscription item that generated this line item.
    /// Left empty if the line item is not an explicit result of a subscription.
    pub subscription_item: Option<stripe_types::Expandable<stripe_shared::SubscriptionItem>>,
    /// The amount of tax calculated per tax rate for this line item
    pub tax_amounts: Option<Vec<stripe_shared::InvoiceTaxAmount>>,
    /// The tax rates which apply to the line item.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
    /// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: InvoiceLineItemType,
    /// The amount in cents (or local equivalent) representing the unit amount for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceLineItemBuilder {
    amount: Option<i64>,
    amount_excluding_tax: Option<Option<i64>>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    discount_amounts: Option<Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>>,
    discountable: Option<bool>,
    discounts: Option<Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>>,
    id: Option<stripe_shared::InvoiceLineItemId>,
    invoice_item: Option<Option<stripe_types::Expandable<stripe_shared::InvoiceItem>>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    period: Option<stripe_shared::InvoiceLineItemPeriod>,
    plan: Option<Option<stripe_shared::Plan>>,
    price: Option<Option<stripe_shared::Price>>,
    proration: Option<bool>,
    proration_details: Option<Option<stripe_shared::InvoicesResourceLineItemsProrationDetails>>,
    quantity: Option<Option<u64>>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    subscription_item: Option<Option<stripe_types::Expandable<stripe_shared::SubscriptionItem>>>,
    tax_amounts: Option<Option<Vec<stripe_shared::InvoiceTaxAmount>>>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
    type_: Option<InvoiceLineItemType>,
    unit_amount_excluding_tax: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceLineItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceLineItem>,
        builder: InvoiceLineItemBuilder,
    }

    impl Visitor for Place<InvoiceLineItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceLineItemBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceLineItemBuilder {
        type Out = InvoiceLineItem;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_excluding_tax" => Ok(Deserialize::begin(&mut self.amount_excluding_tax)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "discount_amounts" => Ok(Deserialize::begin(&mut self.discount_amounts)),
                "discountable" => Ok(Deserialize::begin(&mut self.discountable)),
                "discounts" => Ok(Deserialize::begin(&mut self.discounts)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "invoice_item" => Ok(Deserialize::begin(&mut self.invoice_item)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "period" => Ok(Deserialize::begin(&mut self.period)),
                "plan" => Ok(Deserialize::begin(&mut self.plan)),
                "price" => Ok(Deserialize::begin(&mut self.price)),
                "proration" => Ok(Deserialize::begin(&mut self.proration)),
                "proration_details" => Ok(Deserialize::begin(&mut self.proration_details)),
                "quantity" => Ok(Deserialize::begin(&mut self.quantity)),
                "subscription" => Ok(Deserialize::begin(&mut self.subscription)),
                "subscription_item" => Ok(Deserialize::begin(&mut self.subscription_item)),
                "tax_amounts" => Ok(Deserialize::begin(&mut self.tax_amounts)),
                "tax_rates" => Ok(Deserialize::begin(&mut self.tax_rates)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "unit_amount_excluding_tax" => Ok(Deserialize::begin(&mut self.unit_amount_excluding_tax)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_excluding_tax: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                discount_amounts: Deserialize::default(),
                discountable: Deserialize::default(),
                discounts: Deserialize::default(),
                id: Deserialize::default(),
                invoice_item: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                period: Deserialize::default(),
                plan: Deserialize::default(),
                price: Deserialize::default(),
                proration: Deserialize::default(),
                proration_details: Deserialize::default(),
                quantity: Deserialize::default(),
                subscription: Deserialize::default(),
                subscription_item: Deserialize::default(),
                tax_amounts: Deserialize::default(),
                tax_rates: Deserialize::default(),
                type_: Deserialize::default(),
                unit_amount_excluding_tax: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_excluding_tax = self.amount_excluding_tax.take()?;
            let currency = self.currency.take()?;
            let description = self.description.take()?;
            let discount_amounts = self.discount_amounts.take()?;
            let discountable = self.discountable.take()?;
            let discounts = self.discounts.take()?;
            let id = self.id.take()?;
            let invoice_item = self.invoice_item.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let period = self.period.take()?;
            let plan = self.plan.take()?;
            let price = self.price.take()?;
            let proration = self.proration.take()?;
            let proration_details = self.proration_details.take()?;
            let quantity = self.quantity.take()?;
            let subscription = self.subscription.take()?;
            let subscription_item = self.subscription_item.take()?;
            let tax_amounts = self.tax_amounts.take()?;
            let tax_rates = self.tax_rates.take()?;
            let type_ = self.type_.take()?;
            let unit_amount_excluding_tax = self.unit_amount_excluding_tax.take()?;

            Some(Self::Out {
                amount,
                amount_excluding_tax,
                currency,
                description,
                discount_amounts,
                discountable,
                discounts,
                id,
                invoice_item,
                livemode,
                metadata,
                period,
                plan,
                price,
                proration,
                proration_details,
                quantity,
                subscription,
                subscription_item,
                tax_amounts,
                tax_rates,
                type_,
                unit_amount_excluding_tax,
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

    impl ObjectDeser for InvoiceLineItem {
        type Builder = InvoiceLineItemBuilder;
    }
};
/// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceLineItemType {
    Invoiceitem,
    Subscription,
}
impl InvoiceLineItemType {
    pub fn as_str(self) -> &'static str {
        use InvoiceLineItemType::*;
        match self {
            Invoiceitem => "invoiceitem",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for InvoiceLineItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceLineItemType::*;
        match s {
            "invoiceitem" => Ok(Invoiceitem),
            "subscription" => Ok(Subscription),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoiceLineItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoiceLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceLineItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceLineItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoiceLineItemType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceLineItemType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoiceLineItemType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceLineItemType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for InvoiceLineItem {
    type Id = stripe_shared::InvoiceLineItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(InvoiceLineItemId);
