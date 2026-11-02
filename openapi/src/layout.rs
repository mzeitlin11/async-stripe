use std::collections::{BTreeSet, HashMap};

use anyhow::Context;

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crates::Crate;
use crate::item_catalog::ItemCatalog;
use crate::item_layout::{EmissionSite, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::model_availability::{Availability, ModelAvailability};
use crate::rust_type::DeclarationRoot;

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
    /// Honor inherent decoder colocation before rendering the baseline opaque
    /// experiment. This uses declarations, independent of selected features.
    pub(crate) fn with_record_decoder_colocation(
        &self,
        items: &ItemCatalog,
        item_layout: &ItemLayout,
        availability: &ModelAvailability,
    ) -> Self {
        let crate_for_site = |site: EmissionSite| site.krate(self);
        let mut shared = availability
            .dependencies()
            .keys()
            .copied()
            .filter(|&site| crate_for_site(site) == Some(Crate::SHARED))
            .collect::<std::collections::HashSet<_>>();
        let mut edges = availability
            .dependencies()
            .iter()
            .flat_map(|(&source, targets)| targets.iter().map(move |&target| (source, target)))
            .collect::<Vec<_>>();
        for accessor in items.record_accessors() {
            let owner = item_layout.placement(accessor.owner).emission_site();
            let payload = item_layout.placement(accessor.payload).emission_site();
            if crate_for_site(owner) != crate_for_site(payload) {
                shared.extend([owner, payload]);
            }
            edges.extend([(owner, payload), (payload, owner)]);
        }
        loop {
            let before = shared.len();
            for &(source, target) in &edges {
                if shared.contains(&source) {
                    shared.insert(target);
                }
            }
            if shared.len() == before {
                break;
            }
        }
        let mut placed = self.clone();
        for (&component, placement) in &mut placed.placements {
            if shared
                .contains(&item_layout.placement(items.component_root(component)).emission_site())
            {
                placement.types_crate = Crate::SHARED;
            }
        }
        placed
    }

    /// Place model modules by their represented consumers while retaining the
    /// existing request crates and public component facades.
    ///
    /// The baseline supplies service-owner labels, not semantic dependency
    /// edges. Availability already contains the actual emitted module graph.
    pub(crate) fn with_consumer_model_crates(
        &self,
        components: &Components,
        items: &ItemCatalog,
        item_layout: &ItemLayout,
        availability: &ModelAvailability,
        feature_crates: &HashMap<String, Crate>,
    ) -> anyhow::Result<Self> {
        for feature in availability.features() {
            anyhow::ensure!(
                feature_crates.contains_key(feature),
                "model feature has no consumer crate: {feature}"
            );
        }
        let mut consumers = availability
            .dependencies()
            .keys()
            .map(|&site| {
                let mut users = BTreeSet::new();
                if let EmissionSite::ComponentRequests(component) = site {
                    users.insert(self.placement(component).request_crate());
                }
                match availability.site(site) {
                    Availability::Always | Availability::Disabled => {
                        if !matches!(site, EmissionSite::ComponentRequests(_)) {
                            users.insert(Crate::SHARED);
                        }
                    }
                    Availability::Features(features) => {
                        users.extend(features.iter().map(|feature| feature_crates[feature]));
                    }
                }
                (site, users)
            })
            .collect::<HashMap<_, _>>();
        for (item, record) in items.iter() {
            let site = item_layout.placement(item).emission_site();
            let users = consumers.get_mut(&site).context("item has no availability site")?;
            if record.is_reference_foundation()
                || matches!(
                    record.origin.type_root,
                    TypeRoot::Declaration(DeclarationRoot::Shared(_))
                )
            {
                users.insert(Crate::SHARED);
            }
            // A public alias consumes a type even when its service has no
            // request or model field referring to that type.
            if record.origin.path.is_empty()
                && let TypeRoot::Declaration(
                    DeclarationRoot::Component(component) | DeclarationRoot::ObjectId(component),
                ) = record.origin.type_root
            {
                users.insert(self.placement(component).request_crate());
            }
        }
        let mut dependencies = availability
            .dependencies()
            .iter()
            .flat_map(|(&source, targets)| targets.iter().map(move |&target| (source, target)))
            .collect::<Vec<_>>();
        for accessor in items.record_accessors() {
            let owner = item_layout.placement(accessor.owner).emission_site();
            let payload = item_layout.placement(accessor.payload).emission_site();
            // Inherent methods require the receiver and payload declarations
            // in one crate. Propagate both ways, then through their children;
            // merging only the immediate pair can leave Shared -> domain cycles.
            dependencies.extend([(owner, payload), (payload, owner)]);
        }
        loop {
            let mut changed = false;
            for &(source, target) in &dependencies {
                let required = consumers
                    .get(&source)
                    .context("placement dependency has no source module")?
                    .clone();
                let users = consumers
                    .get_mut(&target)
                    .context("placement dependency has no target module")?;
                let before = users.len();
                users.extend(required);
                changed |= users.len() != before;
            }
            if !changed {
                break;
            }
        }
        let mut placed = self.clone();
        for &component in components.components.keys() {
            let site = item_layout.placement(items.component_root(component)).emission_site();
            let users = consumers.get(&site).context("component has no availability site")?;
            let types_crate = if users.len() == 1 {
                *users.first().expect("single consumer disappeared")
            } else {
                Crate::SHARED
            };
            placed
                .placements
                .get_mut(&component)
                .context("component has no placement")?
                .types_crate = types_crate;
        }
        Ok(placed)
    }

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

#[cfg(test)]
mod tests;
