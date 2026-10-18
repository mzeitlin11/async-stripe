use std::path::PathBuf;

use heck::ToSnakeCase;
use indexmap::IndexSet;

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crates::Crate;
use crate::item_catalog::{ItemCatalog, ItemId};
use crate::item_layout::EmissionSite;
use crate::layout::{ComponentLayout, ComponentPlacement};

/// Resolves frozen component placement into crate and file destinations.
#[derive(Copy, Clone)]
pub(crate) struct OutputLocations<'a> {
    pub(crate) components: &'a Components,
    component_layout: &'a ComponentLayout,
}

impl<'a> OutputLocations<'a> {
    pub(crate) const fn new(
        components: &'a Components,
        component_layout: &'a ComponentLayout,
    ) -> Self {
        Self { components, component_layout }
    }

    pub(crate) fn placement(self, component: ComponentId) -> ComponentPlacement {
        self.component_layout.placement(component)
    }

    pub(crate) fn request_crate(self, component: ComponentId) -> Crate {
        self.placement(component).request_crate()
    }

    pub(crate) fn types_crate(self, component: ComponentId) -> Crate {
        self.placement(component).types_crate()
    }

    pub(crate) fn types_split_from_requests(self, component: ComponentId) -> bool {
        self.placement(component).types_split_from_requests()
    }

    pub(crate) fn crate_members(self, krate: Crate) -> IndexSet<ComponentId> {
        self.components
            .components
            .keys()
            .copied()
            .filter(|component| {
                let placement = self.placement(*component);
                placement.request_crate() == krate || placement.types_crate() == krate
            })
            .collect()
    }

    pub(crate) fn requests_module_path(self, component: ComponentId) -> PathBuf {
        self.requests_folder_path(component).join("mod.rs")
    }

    pub(crate) fn requests_content_path(self, component: ComponentId) -> PathBuf {
        self.requests_folder_path(component).join("requests.rs")
    }

    pub(crate) fn standalone_module(self, item: ItemId, items: &ItemCatalog) -> String {
        let ident = &items.get(item).metadata.ident;
        self.components
            .extra_types
            .get(ident)
            .map_or_else(|| ident.to_snake_case(), |typ| typ.mod_path.clone())
    }

    /// Resolve the selected emission site rather than the declaration's origin.
    pub(crate) fn emission_path(self, site: EmissionSite, items: &ItemCatalog) -> PathBuf {
        match site {
            EmissionSite::ComponentTypes(component) => self.types_content_path(component),
            EmissionSite::ComponentRequests(component) => self.requests_content_path(component),
            EmissionSite::Standalone { item, krate } => {
                krate.get_path().join(format!("{}.rs", self.standalone_module(item, items)))
            }
            EmissionSite::Webhooks => PathBuf::from("async-stripe-webhook/mod.rs"),
        }
    }

    pub(crate) fn types_content_path(self, component: ComponentId) -> PathBuf {
        let object = self.components.get_by_id(component);
        if !self.components.has_requests(component) || self.types_split_from_requests(component) {
            self.types_crate(component).get_path().join(format!("{}.rs", object.mod_path()))
        } else {
            self.requests_folder_path(component).join("types.rs")
        }
    }

    fn requests_folder_path(self, component: ComponentId) -> PathBuf {
        let object = self.components.get_by_id(component);
        self.request_crate(component).get_path().join(object.mod_path())
    }
}
