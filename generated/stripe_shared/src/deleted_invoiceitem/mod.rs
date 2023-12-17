#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedInvoiceitem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::invoice_item::InvoiceItemId,
}
impl stripe_types::Object for DeletedInvoiceitem {
    type Id = stripe_shared::invoice_item::InvoiceItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
