#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedRadarValueListItem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_fraud::radar_value_list_item::RadarValueListItemId,
}
impl stripe_types::Object for DeletedRadarValueListItem {
    type Id = stripe_fraud::radar_value_list_item::RadarValueListItemId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
