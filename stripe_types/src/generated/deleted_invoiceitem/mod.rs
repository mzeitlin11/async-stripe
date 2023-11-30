#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedInvoiceitem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::invoiceitem::InvoiceitemId,
}
impl stripe_types::Object for DeletedInvoiceitem {
    type Id = stripe_types::invoiceitem::InvoiceitemId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
