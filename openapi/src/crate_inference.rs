use std::collections::HashMap;
use std::fmt;

use anyhow::bail;
use indexmap::{IndexMap, IndexSet};
use petgraph::Direction;
use petgraph::algo::is_cyclic_directed;
use tracing::{debug, trace, warn};

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crates::Crate;
use crate::dependencies::EdgePolicy;
use crate::graph::{ComponentGraph, gen_inferred_crate_dep_graph};
use crate::layout::ComponentLayoutDraft;
use crate::types::ComponentName;

/// A warning emitted when a component's crate had to be auto-resolved
/// because normal inference couldn't determine a unique crate.
#[derive(Debug, Clone)]
pub struct CrateInferenceWarning {
    pub component: ComponentName,
    pub chosen_crate: Crate,
    pub depended_on_by: Vec<ComponentName>,
    pub candidate_crates: Vec<Crate>,
}

impl fmt::Display for CrateInferenceWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Auto-assigned `{}` to crate `{}` (depended on by: {}, candidate crates: {})",
            self.component,
            self.chosen_crate.base_name(),
            self.depended_on_by.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "),
            self.candidate_crates
                .iter()
                .map(|c| c.base_name().to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

impl Components {
    #[tracing::instrument(skip_all)]
    pub(crate) fn infer_all_crate_assignments(
        &self,
        layout: &mut ComponentLayoutDraft,
        policy: EdgePolicy,
    ) -> anyhow::Result<Vec<CrateInferenceWarning>> {
        // If a component includes requests that have URLs building off another component,
        // place with that component. This automates determinations like `external_account`
        // ending up in the same crate as `account` since all its requests start with `/account`.
        let mut new_assignments: IndexMap<ComponentId, Crate> = IndexMap::new();
        for &component_id in self.components.keys() {
            let path = self.component_name(component_id);
            let krate = layout.placement(component_id);
            if krate.is_some() {
                continue;
            }
            let words = path.split('_');
            let first_two = words.take(2).collect::<Vec<_>>().join("_");
            let first_word = path.split_once('_').map(|f| f.0);
            for &other_id in self.components.keys() {
                let other_path = self.component_name(other_id);
                let Some(placement) = layout.placement(other_id) else {
                    continue;
                };
                if self.is_nested_resource_of(component_id, other_id) {
                    new_assignments.insert(component_id, placement.request_crate());
                    break;
                }

                if other_path.as_ref() == first_two {
                    new_assignments.insert(component_id, placement.request_crate());
                    break;
                }
                if first_word == Some(other_path.as_ref()) {
                    new_assignments.insert(component_id, placement.request_crate());
                    break;
                }
            }
        }
        debug!("Inferred based on naming: {new_assignments:#?} ");
        for (component, krate) in new_assignments {
            layout.assign(component, krate);
        }

        let component_graph = self.gen_component_dep_graph(policy);
        infer_crates_using_deps(
            self,
            layout,
            &component_graph,
            Self::maybe_infer_crate_by_what_depends_on_it,
        );
        infer_crates_using_deps(self, layout, &component_graph, Self::maybe_infer_crate_by_deps);
        let warnings = self.resolve_missing_crates(layout, &component_graph);
        self.assign_paths_required_to_share_types(layout, &component_graph);
        self.validate_crate_assignment(layout, policy)?;
        Ok(warnings)
    }

    /// For components with no assigned crate, auto-resolve by picking the most
    #[tracing::instrument(skip_all)]
    fn resolve_missing_crates(
        &self,
        layout: &mut ComponentLayoutDraft,
        graph: &ComponentGraph,
    ) -> Vec<CrateInferenceWarning> {
        let components_without_crates: Vec<ComponentId> = self
            .components
            .keys()
            .copied()
            .filter(|component| layout.placement(*component).is_none())
            .collect();

        if components_without_crates.is_empty() {
            return Vec::new();
        }

        let mut resolutions = Vec::new();

        for missing_id in &components_without_crates {
            let depended_on_ids =
                graph.neighbors_directed(*missing_id, Direction::Incoming).collect::<Vec<_>>();
            let depended_on = depended_on_ids
                .iter()
                .map(|id| self.component_name(*id).clone())
                .collect::<Vec<_>>();
            let candidate_crates: Vec<Crate> = depended_on_ids
                .iter()
                .filter_map(|id| layout.placement(*id).map(|placement| placement.request_crate()))
                .collect();

            // Pick the most common crate, breaking ties alphabetically
            let chosen = if candidate_crates.is_empty() {
                Crate::SHARED
            } else {
                let mut counts: HashMap<Crate, usize> = HashMap::new();
                for c in &candidate_crates {
                    *counts.entry(*c).or_default() += 1;
                }
                let max_count = *counts.values().max().unwrap();
                let mut best: Vec<Crate> =
                    counts.into_iter().filter(|(_, c)| *c == max_count).map(|(k, _)| k).collect();
                best.sort_by_key(|c| c.base_name());
                best[0]
            };

            let unique_crates: Vec<Crate> =
                candidate_crates.iter().copied().collect::<IndexSet<_>>().into_iter().collect();

            let missing = self.component_name(*missing_id);
            warn!(
                "Auto-assigning component {missing} to crate {} (depended on by {depended_on:?}, \
                 candidate crates: {unique_crates:?}). Consider adding a path entry in gen_crates.toml.",
                chosen.base_name()
            );

            resolutions.push((
                *missing_id,
                CrateInferenceWarning {
                    component: missing.clone(),
                    chosen_crate: chosen,
                    depended_on_by: depended_on,
                    candidate_crates: unique_crates,
                },
            ));
        }

        for (component, warning) in &resolutions {
            layout.assign(*component, warning.chosen_crate);
        }

        resolutions.into_iter().map(|(_, warning)| warning).collect()
    }

    fn validate_crate_assignment(
        &self,
        layout: &ComponentLayoutDraft,
        policy: EdgePolicy,
    ) -> anyhow::Result<()> {
        let graph = gen_inferred_crate_dep_graph(self, policy, |component| {
            layout.placement(component).expect("component had no crate assignment")
        });
        let deps_for_shared_crate = graph.neighbors(Crate::SHARED).collect::<Vec<_>>();
        if !deps_for_shared_crate.is_empty() {
            bail!(
                "Shared types crate should not have dependencies, but has dependencies {deps_for_shared_crate:#?}!",
            );
        }
        if is_cyclic_directed(&graph) {
            bail!("Crate dependency graph contains a cycle!");
        }
        let requests_in_types = self
            .components
            .iter()
            .filter(|(component, _)| {
                layout.placement(**component).unwrap().request_crate() == Crate::SHARED
                    && self.has_requests(**component)
            })
            .map(|(component, _)| self.component_name(*component))
            .collect::<Vec<_>>();
        if !requests_in_types.is_empty() {
            bail!("Components have requests, not allowed in types crate: {requests_in_types:#?}");
        }
        Ok(())
    }

    fn assign_paths_required_to_share_types(
        &self,
        layout: &mut ComponentLayoutDraft,
        graph: &ComponentGraph,
    ) {
        // Paths for components required to live in the `async-stripe-shared` crate. Adding `event` is
        // used for webhooks
        const PATHS_IN_TYPES: &[&str] = &["event"];

        loop {
            let mut required: Vec<ComponentId> = vec![];
            for &component_id in self.components.keys() {
                let path = self.component_name(component_id);
                let placement = layout.placement(component_id).unwrap();
                if placement.types_crate() == Crate::SHARED {
                    continue;
                }
                if PATHS_IN_TYPES.contains(&path.as_ref()) {
                    required.push(component_id);
                    continue;
                }
                let my_crate = placement.request_crate();
                let depended_on =
                    graph.neighbors_directed(component_id, Direction::Incoming).collect::<Vec<_>>();
                let depended_crates = depended_on
                    .iter()
                    .map(|id| layout.placement(*id).unwrap().types_crate())
                    .filter(|c| *c != my_crate)
                    .collect::<IndexSet<_>>();
                if !depended_crates.is_empty() {
                    required.push(component_id);
                }
            }

            let done = required.is_empty();
            if done {
                break;
            }
            for req in required {
                layout.share_types(req);
            }
        }
    }

    fn maybe_infer_crate_by_what_depends_on_it(
        &self,
        layout: &ComponentLayoutDraft,
        component: ComponentId,
        graph: &ComponentGraph,
    ) -> Option<Crate> {
        let depended_on_by = graph
            .neighbors_directed(component, Direction::Incoming)
            .map(|id| layout.placement(id).map(|placement| placement.request_crate()))
            .collect::<Option<Vec<_>>>()?;

        let first = depended_on_by.first()?;
        if depended_on_by.iter().all(|d| d == first) { Some(*first) } else { None }
    }

    fn maybe_infer_crate_by_deps(
        &self,
        layout: &ComponentLayoutDraft,
        component: ComponentId,
        graph: &ComponentGraph,
    ) -> Option<Crate> {
        let known_dependents = graph
            .neighbors(component)
            .map(|id| layout.placement(id).map(|placement| placement.request_crate()))
            .collect::<Option<IndexSet<_>>>()?;

        let first = known_dependents.first()?;
        if known_dependents.iter().all(|d| d == first) { Some(*first) } else { None }
    }
}

fn infer_crates_using_deps(
    components: &Components,
    layout: &mut ComponentLayoutDraft,
    graph: &ComponentGraph,
    infer_test: fn(
        &Components,
        &ComponentLayoutDraft,
        ComponentId,
        &ComponentGraph,
    ) -> Option<Crate>,
) {
    loop {
        let mut new_assignments: IndexMap<ComponentId, Crate> = IndexMap::new();
        for &component in components.components.keys() {
            if layout.placement(component).is_some() {
                continue;
            }
            if let Some(assignment) = infer_test(components, layout, component, graph) {
                new_assignments.insert(component, assignment);
            }
        }
        let no_new_assignments = new_assignments.is_empty();

        trace!("Inferred {new_assignments:#?}");
        for (component, krate) in new_assignments {
            layout.assign(component, krate);
        }
        if no_new_assignments {
            break;
        }
    }
}
