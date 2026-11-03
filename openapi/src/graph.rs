use indexmap::IndexSet;
use petgraph::prelude::DiGraphMap;

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crates::{ALL_CRATES, Crate};
use crate::dependencies::EdgePolicy;
use crate::layout::ComponentPlacement;

pub type ComponentGraph = DiGraphMap<ComponentId, ()>;

impl Components {
    /// Generate a dependency graph with an edge from A to B implying that A depends on B
    pub fn gen_component_dep_graph(&self, policy: EdgePolicy) -> ComponentGraph {
        let mut graph = DiGraphMap::new();
        for &component in self.components.keys() {
            graph.add_node(component);
        }

        for &component_id in self.components.keys() {
            let deps = self.deps_for_component(component_id, policy);
            for dependency_id in deps {
                // Don't clutter with self edges since they aren't particularly meaningful
                // in this context
                if dependency_id != component_id {
                    graph.add_edge(component_id, dependency_id, ());
                }
            }
        }
        graph
    }

    /// Generate dependencies between model type definitions, excluding request
    /// parameters and return wrappers.
    pub fn gen_component_type_dep_graph(&self, policy: EdgePolicy) -> ComponentGraph {
        let mut graph = DiGraphMap::new();
        for &component in self.components.keys() {
            graph.add_node(component);
        }

        for &component_id in self.components.keys() {
            for dependency in self.deps_for_component_type(component_id, policy) {
                if dependency != component_id {
                    graph.add_edge(component_id, dependency, ());
                }
            }
        }
        graph
    }
}

/// Project component dependencies through a placement into workspace crate
/// dependencies for layout inference. Emission uses lowered Rust dependencies.
pub(crate) fn gen_inferred_crate_dep_graph(
    components: &Components,
    policy: EdgePolicy,
    placement: impl Fn(ComponentId) -> ComponentPlacement,
) -> DiGraphMap<Crate, ()> {
    let mut graph = DiGraphMap::new();
    for krate in &*ALL_CRATES {
        graph.add_node(*krate);
        if *krate != Crate::SHARED {
            graph.add_edge(*krate, Crate::SHARED, ());
        }
    }

    for krate in &*ALL_CRATES {
        let members = components.components.keys().copied().filter(|component| {
            let placement = placement(*component);
            placement.request_crate() == *krate || placement.types_crate() == *krate
        });
        let mut dependencies = IndexSet::new();
        for member in members {
            dependencies.extend(components.deps_for_component(member, policy));
        }
        for dependency in dependencies {
            let dependent_crate = placement(dependency).types_crate();
            if dependent_crate != *krate {
                graph.add_edge(*krate, dependent_crate, ());
            }
        }
    }
    graph
}
