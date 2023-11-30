#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedTerminalConfiguration {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_configuration::TerminalConfigurationId,
}
impl stripe_types::Object for DeletedTerminalConfiguration {
    type Id = stripe_terminal::terminal_configuration::TerminalConfigurationId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
