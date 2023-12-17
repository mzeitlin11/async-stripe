/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum PaymentSource {
    #[serde(rename = "account")]
    Account(stripe_shared::Account),
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[serde(rename = "source")]
    Source(stripe_shared::Source),
}
impl stripe_types::Object for PaymentSource {
    type Id = String;
    fn id(&self) -> Option<&str> {
        match self {
            Self::Account(v) => Some(v.id.as_str()),
            Self::BankAccount(v) => Some(v.id.as_str()),
            Self::Card(v) => Some(v.id.as_str()),
            Self::Source(v) => Some(v.id.as_str()),
        }
    }
}
