#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedDiscount {
    /// The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode.
    /// Will not be present for subscription mode.
    pub checkout_session: Option<String>,
    pub coupon: stripe_shared::Coupon,
    /// The ID of the customer associated with this discount.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Always true for a deleted object
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// The ID of the discount object.
    /// Discounts cannot be fetched by ID.
    /// Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: stripe_shared::DiscountId,
    /// The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    pub invoice: Option<String>,
    /// The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    pub invoice_item: Option<String>,
    /// The promotion code applied to create this discount.
    pub promotion_code: Option<stripe_types::Expandable<stripe_shared::PromotionCode>>,
    /// Date that the coupon was applied.
    pub start: stripe_types::Timestamp,
    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct DeletedDiscountBuilder {
    checkout_session: Option<Option<String>>,
    coupon: Option<stripe_shared::Coupon>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    deleted: Option<stripe_types::AlwaysTrue>,
    id: Option<stripe_shared::DiscountId>,
    invoice: Option<Option<String>>,
    invoice_item: Option<Option<String>>,
    promotion_code: Option<Option<stripe_types::Expandable<stripe_shared::PromotionCode>>>,
    start: Option<stripe_types::Timestamp>,
    subscription: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DeletedDiscount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DeletedDiscount>,
        builder: DeletedDiscountBuilder,
    }

    impl Visitor for Place<DeletedDiscount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DeletedDiscountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DeletedDiscountBuilder {
        type Out = DeletedDiscount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "checkout_session" => Ok(Deserialize::begin(&mut self.checkout_session)),
                "coupon" => Ok(Deserialize::begin(&mut self.coupon)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "deleted" => Ok(Deserialize::begin(&mut self.deleted)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "invoice" => Ok(Deserialize::begin(&mut self.invoice)),
                "invoice_item" => Ok(Deserialize::begin(&mut self.invoice_item)),
                "promotion_code" => Ok(Deserialize::begin(&mut self.promotion_code)),
                "start" => Ok(Deserialize::begin(&mut self.start)),
                "subscription" => Ok(Deserialize::begin(&mut self.subscription)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                checkout_session: Deserialize::default(),
                coupon: Deserialize::default(),
                customer: Deserialize::default(),
                deleted: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                invoice_item: Deserialize::default(),
                promotion_code: Deserialize::default(),
                start: Deserialize::default(),
                subscription: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let checkout_session = self.checkout_session.take()?;
            let coupon = self.coupon.take()?;
            let customer = self.customer.take()?;
            let deleted = self.deleted.take()?;
            let id = self.id.take()?;
            let invoice = self.invoice.take()?;
            let invoice_item = self.invoice_item.take()?;
            let promotion_code = self.promotion_code.take()?;
            let start = self.start.take()?;
            let subscription = self.subscription.take()?;

            Some(Self::Out { checkout_session, coupon, customer, deleted, id, invoice, invoice_item, promotion_code, start, subscription })
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

    impl ObjectDeser for DeletedDiscount {
        type Builder = DeletedDiscountBuilder;
    }
};
impl stripe_types::Object for DeletedDiscount {
    type Id = stripe_shared::DiscountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
