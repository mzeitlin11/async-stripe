use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::item_catalog::{ItemCatalog, ItemMetadata};
use crate::request_model::{RequestCatalog, RequestModel};

/// The complete canonical Rust model produced from inferred components.
///
/// Items are allocated and lowered first because request models refer to their
/// canonical item identities. Consumers should use this model rather than
/// constructing either catalog independently.
pub(crate) struct RustModel {
    items: ItemCatalog,
    requests: RequestCatalog,
}

impl RustModel {
    pub(crate) fn build(
        components: &Components,
        component_metadata: impl Fn(ComponentId) -> ItemMetadata,
    ) -> anyhow::Result<Self> {
        let items = ItemCatalog::build(components, component_metadata)?;
        let requests = RequestCatalog::build(components, &items)?;
        Ok(Self { items, requests })
    }

    pub(crate) fn items(&self) -> &ItemCatalog {
        &self.items
    }

    pub(crate) fn requests_for_component(&self, component: ComponentId) -> &[RequestModel] {
        self.requests.for_component(component)
    }
}
