use std::path::PathBuf;

use indexmap::IndexSet;

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crates::Crate;
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
