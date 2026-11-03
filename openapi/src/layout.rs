use std::collections::HashMap;

use anyhow::Context;

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crates::Crate;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ComponentPlacement {
    request_crate: Crate,
    types_crate: Crate,
}

impl ComponentPlacement {
    pub const fn request_crate(self) -> Crate {
        self.request_crate
    }

    pub const fn types_crate(self) -> Crate {
        self.types_crate
    }

    pub fn types_split_from_requests(self) -> bool {
        self.request_crate != self.types_crate
    }
}

#[derive(Debug, Clone)]
pub struct ComponentLayout {
    placements: HashMap<ComponentId, ComponentPlacement>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ComponentLayoutDraft {
    placements: HashMap<ComponentId, Option<ComponentPlacement>>,
}

impl ComponentLayoutDraft {
    pub fn insert_initial(&mut self, component: ComponentId, krate: Option<Crate>) {
        let placement =
            krate.map(|krate| ComponentPlacement { request_crate: krate, types_crate: krate });
        assert!(self.placements.insert(component, placement).is_none());
    }

    pub fn placement(&self, component: ComponentId) -> Option<ComponentPlacement> {
        self.placements[&component]
    }

    pub fn assign(&mut self, component: ComponentId, krate: Crate) {
        let slot = self.placements.get_mut(&component).expect("component had no layout slot");
        *slot = Some(ComponentPlacement { request_crate: krate, types_crate: krate });
    }

    pub fn share_types(&mut self, component: ComponentId) {
        let placement = self
            .placements
            .get_mut(&component)
            .and_then(Option::as_mut)
            .expect("component had no crate assignment");
        placement.types_crate = Crate::SHARED;
    }
}

impl ComponentLayout {
    #[cfg(test)]
    pub(crate) fn for_test(placements: &[(ComponentId, Crate, Crate)]) -> Self {
        Self {
            placements: placements
                .iter()
                .map(|&(component, request_crate, types_crate)| {
                    (component, ComponentPlacement { request_crate, types_crate })
                })
                .collect(),
        }
    }

    pub(crate) fn from_draft(
        components: &Components,
        draft: &ComponentLayoutDraft,
    ) -> anyhow::Result<Self> {
        let mut placements = HashMap::with_capacity(components.components.len());
        for &id in components.components.keys() {
            let name = components.component_name(id);
            let placement = draft
                .placement(id)
                .with_context(|| format!("component had no crate assignment: {name}"))?;
            let previous = placements.insert(id, placement);
            anyhow::ensure!(previous.is_none(), "duplicate layout placement for {name}");
        }
        let component_layout = Self { placements };
        component_layout.validate(components, draft)?;
        Ok(component_layout)
    }

    pub fn len(&self) -> usize {
        self.placements.len()
    }

    #[track_caller]
    pub(crate) fn placement(&self, component: ComponentId) -> ComponentPlacement {
        self.placements[&component]
    }

    fn validate(
        &self,
        components: &Components,
        draft: &ComponentLayoutDraft,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.len() == components.components.len(),
            "component layout and retained component counts differed"
        );
        for &id in components.components.keys() {
            let name = components.component_name(id);
            let placement = self.placement(id);
            anyhow::ensure!(
                draft.placement(id) == Some(placement),
                "frozen component placement did not match draft for {name}"
            );
        }
        Ok(())
    }
}
