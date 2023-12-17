#[derive(Clone, Debug, serde::Deserialize)]
#[non_exhaustive]
#[serde(tag = "object")]
pub enum EventObject {
    #[serde(rename = "account")]
    Account(stripe_shared::Account),
    #[serde(rename = "capability")]
    Capability(stripe_shared::Capability),
    #[serde(rename = "application")]
    Application(stripe_shared::Application),
    #[serde(rename = "application_fee")]
    ApplicationFee(stripe_shared::ApplicationFee),
    #[serde(rename = "fee_refund")]
    ApplicationFeeRefund(stripe_shared::ApplicationFeeRefund),
    #[cfg(feature = "stripe_core")]
    #[serde(rename = "balance")]
    Balance(stripe_core::Balance),
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[cfg(feature = "stripe_billing")]
    #[serde(rename = "billing_portal.configuration")]
    BillingPortalConfiguration(stripe_billing::BillingPortalConfiguration),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[cfg(feature = "stripe_checkout")]
    #[serde(rename = "checkout.session")]
    CheckoutSession(stripe_checkout::CheckoutSession),
    #[serde(rename = "coupon")]
    Coupon(stripe_shared::Coupon),
    #[serde(rename = "customer")]
    Customer(stripe_shared::Customer),
    #[serde(rename = "discount")]
    Discount(stripe_shared::Discount),
    #[serde(rename = "dispute")]
    Dispute(stripe_shared::Dispute),
    #[serde(rename = "file")]
    File(stripe_shared::File),
    #[serde(rename = "invoice")]
    Invoice(stripe_shared::Invoice),
    #[serde(rename = "invoiceitem")]
    InvoiceItem(stripe_shared::InvoiceItem),
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization(stripe_shared::IssuingAuthorization),
    #[serde(rename = "issuing.card")]
    IssuingCard(stripe_shared::IssuingCard),
    #[serde(rename = "issuing.cardholder")]
    IssuingCardholder(stripe_shared::IssuingCardholder),
    #[serde(rename = "issuing.dispute")]
    IssuingDispute(stripe_shared::IssuingDispute),
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction(stripe_shared::IssuingTransaction),
    #[serde(rename = "mandate")]
    Mandate(stripe_shared::Mandate),
    #[serde(rename = "payment_intent")]
    PaymentIntent(stripe_shared::PaymentIntent),
    #[serde(rename = "payment_link")]
    PaymentLink(stripe_shared::PaymentLink),
    #[serde(rename = "payment_method")]
    PaymentMethod(stripe_shared::PaymentMethod),
    #[serde(rename = "payout")]
    Payout(stripe_shared::Payout),
    #[serde(rename = "person")]
    Person(stripe_shared::Person),
    #[serde(rename = "plan")]
    Plan(stripe_shared::Plan),
    #[serde(rename = "price")]
    Price(stripe_shared::Price),
    #[serde(rename = "product")]
    Product(stripe_shared::Product),
    #[serde(rename = "promotion_code")]
    PromotionCode(stripe_shared::PromotionCode),
    #[serde(rename = "quote")]
    Quote(stripe_shared::Quote),
    #[serde(rename = "refund")]
    Refund(stripe_shared::Refund),
    #[serde(rename = "review")]
    Review(stripe_shared::Review),
    #[serde(rename = "setup_intent")]
    SetupIntent(stripe_shared::SetupIntent),
    #[serde(rename = "subscription")]
    Subscription(stripe_shared::Subscription),
    #[serde(rename = "subscription_schedule")]
    SubscriptionSchedule(stripe_shared::SubscriptionSchedule),
    #[serde(rename = "tax_id")]
    TaxId(stripe_shared::TaxId),
    #[serde(rename = "tax_rate")]
    TaxRate(stripe_shared::TaxRate),
    #[serde(rename = "test_helpers.test_clock")]
    TestHelpersTestClock(stripe_shared::TestHelpersTestClock),
    #[serde(rename = "topup")]
    Topup(stripe_shared::Topup),
    #[serde(rename = "transfer")]
    Transfer(stripe_shared::Transfer),
    #[serde(other)]
    Unknown,
}
