use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::item_catalog::{ItemCatalog, ItemCatalogDraft};
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
    /// Finalize representation-dependent properties and canonical requests
    /// together. Layout and rendering only receive this completed model.
    pub(crate) fn from_draft(
        components: &Components,
        draft: ItemCatalogDraft,
    ) -> anyhow::Result<Self> {
        let items = draft.finish(components)?;
        let requests = RequestCatalog::build(components, &items)?;
        Ok(Self { items, requests })
    }

    pub(crate) fn items(&self) -> &ItemCatalog {
        &self.items
    }

    pub(crate) fn requests_for_component(&self, component: ComponentId) -> &[RequestModel] {
        self.requests.for_component(component)
    }

    pub(crate) fn requests(&self) -> impl Iterator<Item = (ComponentId, &[RequestModel])> {
        self.requests.iter()
    }
}
