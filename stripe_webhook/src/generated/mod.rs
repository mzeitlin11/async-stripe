#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[non_exhaustive]
#[cfg_attr(not(feature = "min-ser"), serde(tag = "object"))]
pub enum EventObject {
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "account"))]
    Account(stripe_shared::Account),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "capability"))]
    Capability(stripe_shared::Capability),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "application"))]
    Application(stripe_shared::Application),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "application_fee"))]
    ApplicationFee(stripe_shared::ApplicationFee),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "fee_refund"))]
    ApplicationFeeRefund(stripe_shared::ApplicationFeeRefund),
    #[cfg(feature = "stripe_core")]
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "balance"))]
    Balance(stripe_core::Balance),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg(feature = "stripe_billing")]
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "billing_portal.configuration"))]
    BillingPortalConfiguration(stripe_billing::BillingPortalConfiguration),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg(feature = "stripe_checkout")]
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "checkout.session"))]
    CheckoutSession(stripe_checkout::CheckoutSession),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "coupon"))]
    Coupon(stripe_shared::Coupon),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "customer"))]
    Customer(stripe_shared::Customer),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "discount"))]
    Discount(stripe_shared::Discount),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "dispute"))]
    Dispute(stripe_shared::Dispute),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "file"))]
    File(stripe_shared::File),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "invoice"))]
    Invoice(stripe_shared::Invoice),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "invoiceitem"))]
    InvoiceItem(stripe_shared::InvoiceItem),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "issuing.authorization"))]
    IssuingAuthorization(stripe_shared::IssuingAuthorization),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "issuing.card"))]
    IssuingCard(stripe_shared::IssuingCard),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "issuing.cardholder"))]
    IssuingCardholder(stripe_shared::IssuingCardholder),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "issuing.dispute"))]
    IssuingDispute(stripe_shared::IssuingDispute),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "issuing.transaction"))]
    IssuingTransaction(stripe_shared::IssuingTransaction),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "mandate"))]
    Mandate(stripe_shared::Mandate),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "payment_intent"))]
    PaymentIntent(stripe_shared::PaymentIntent),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "payment_link"))]
    PaymentLink(stripe_shared::PaymentLink),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "payment_method"))]
    PaymentMethod(stripe_shared::PaymentMethod),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "payout"))]
    Payout(stripe_shared::Payout),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "person"))]
    Person(stripe_shared::Person),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "plan"))]
    Plan(stripe_shared::Plan),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "price"))]
    Price(stripe_shared::Price),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "product"))]
    Product(stripe_shared::Product),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "promotion_code"))]
    PromotionCode(stripe_shared::PromotionCode),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "quote"))]
    Quote(stripe_shared::Quote),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "refund"))]
    Refund(stripe_shared::Refund),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "review"))]
    Review(stripe_shared::Review),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "setup_intent"))]
    SetupIntent(stripe_shared::SetupIntent),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "subscription"))]
    Subscription(stripe_shared::Subscription),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "subscription_schedule"))]
    SubscriptionSchedule(stripe_shared::SubscriptionSchedule),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "tax_id"))]
    TaxId(stripe_shared::TaxId),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "tax_rate"))]
    TaxRate(stripe_shared::TaxRate),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "test_helpers.test_clock"))]
    TestHelpersTestClock(stripe_shared::TestHelpersTestClock),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "topup"))]
    Topup(stripe_shared::Topup),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "transfer"))]
    Transfer(stripe_shared::Transfer),
    #[cfg_attr(not(feature = "min-ser"), serde(other))]
    Unknown,
}

#[cfg(feature = "min-ser")]
#[derive(Default)]
pub struct EventObjectBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::{from_str, to_string};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<EventObject>,
        builder: EventObjectBuilder,
    }

    impl Deserialize for EventObject {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<EventObject> {
        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
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

    impl stripe_types::MapBuilder for EventObjectBuilder {
        type Out = EventObject;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (obj_key, object) = self.inner.finish_inner()?;
            let obj_str = to_string(&object);
            Some(match obj_key.as_str() {
                "account" => EventObject::Account(from_str(&obj_str).ok()?),
                "capability" => EventObject::Capability(from_str(&obj_str).ok()?),
                "application" => EventObject::Application(from_str(&obj_str).ok()?),
                "application_fee" => EventObject::ApplicationFee(from_str(&obj_str).ok()?),
                "fee_refund" => EventObject::ApplicationFeeRefund(from_str(&obj_str).ok()?),
                #[cfg(feature = "stripe_core")]
                "balance" => EventObject::Balance(from_str(&obj_str).ok()?),
                "bank_account" => EventObject::BankAccount(from_str(&obj_str).ok()?),
                #[cfg(feature = "stripe_billing")]
                "billing_portal.configuration" => {
                    EventObject::BillingPortalConfiguration(from_str(&obj_str).ok()?)
                }
                "card" => EventObject::Card(from_str(&obj_str).ok()?),
                #[cfg(feature = "stripe_checkout")]
                "checkout.session" => EventObject::CheckoutSession(from_str(&obj_str).ok()?),
                "coupon" => EventObject::Coupon(from_str(&obj_str).ok()?),
                "customer" => EventObject::Customer(from_str(&obj_str).ok()?),
                "discount" => EventObject::Discount(from_str(&obj_str).ok()?),
                "dispute" => EventObject::Dispute(from_str(&obj_str).ok()?),
                "file" => EventObject::File(from_str(&obj_str).ok()?),
                "invoice" => EventObject::Invoice(from_str(&obj_str).ok()?),
                "invoiceitem" => EventObject::InvoiceItem(from_str(&obj_str).ok()?),
                "issuing.authorization" => {
                    EventObject::IssuingAuthorization(from_str(&obj_str).ok()?)
                }
                "issuing.card" => EventObject::IssuingCard(from_str(&obj_str).ok()?),
                "issuing.cardholder" => EventObject::IssuingCardholder(from_str(&obj_str).ok()?),
                "issuing.dispute" => EventObject::IssuingDispute(from_str(&obj_str).ok()?),
                "issuing.transaction" => EventObject::IssuingTransaction(from_str(&obj_str).ok()?),
                "mandate" => EventObject::Mandate(from_str(&obj_str).ok()?),
                "payment_intent" => EventObject::PaymentIntent(from_str(&obj_str).ok()?),
                "payment_link" => EventObject::PaymentLink(from_str(&obj_str).ok()?),
                "payment_method" => EventObject::PaymentMethod(from_str(&obj_str).ok()?),
                "payout" => EventObject::Payout(from_str(&obj_str).ok()?),
                "person" => EventObject::Person(from_str(&obj_str).ok()?),
                "plan" => EventObject::Plan(from_str(&obj_str).ok()?),
                "price" => EventObject::Price(from_str(&obj_str).ok()?),
                "product" => EventObject::Product(from_str(&obj_str).ok()?),
                "promotion_code" => EventObject::PromotionCode(from_str(&obj_str).ok()?),
                "quote" => EventObject::Quote(from_str(&obj_str).ok()?),
                "refund" => EventObject::Refund(from_str(&obj_str).ok()?),
                "review" => EventObject::Review(from_str(&obj_str).ok()?),
                "setup_intent" => EventObject::SetupIntent(from_str(&obj_str).ok()?),
                "subscription" => EventObject::Subscription(from_str(&obj_str).ok()?),
                "subscription_schedule" => {
                    EventObject::SubscriptionSchedule(from_str(&obj_str).ok()?)
                }
                "tax_id" => EventObject::TaxId(from_str(&obj_str).ok()?),
                "tax_rate" => EventObject::TaxRate(from_str(&obj_str).ok()?),
                "test_helpers.test_clock" => {
                    EventObject::TestHelpersTestClock(from_str(&obj_str).ok()?)
                }
                "topup" => EventObject::Topup(from_str(&obj_str).ok()?),
                "transfer" => EventObject::Transfer(from_str(&obj_str).ok()?),

                _ => return None,
            })
        }
    }

    impl stripe_types::ObjectDeser for EventObject {
        type Builder = EventObjectBuilder;
    }
};
