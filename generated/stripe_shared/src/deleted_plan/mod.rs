#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedPlan {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::plan::PlanId,
}
impl stripe_types::Object for DeletedPlan {
    type Id = stripe_shared::plan::PlanId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
