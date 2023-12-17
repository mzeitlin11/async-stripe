#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedTerminalLocation {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_location::TerminalLocationId,
}
impl stripe_types::Object for DeletedTerminalLocation {
    type Id = stripe_terminal::terminal_location::TerminalLocationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
