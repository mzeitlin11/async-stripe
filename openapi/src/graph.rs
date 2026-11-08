use indexmap::IndexSet;
use petgraph::prelude::DiGraphMap;

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crates::{Crate, CrateConfig};
use crate::dependencies::EdgePolicy;
use crate::exports::legacy_facade_dependencies;
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
    config: &CrateConfig,
    policy: EdgePolicy,
    placement: impl Fn(ComponentId) -> ComponentPlacement,
) -> DiGraphMap<Crate, ()> {
    let mut graph = DiGraphMap::new();
    for krate in config.all() {
        graph.add_node(krate);
        for (source, target) in legacy_facade_dependencies([krate]) {
            graph.add_edge(source, target, ());
        }
    }

    for krate in config.all() {
        let members = components.components.keys().copied().filter(|component| {
            let placement = placement(*component);
            placement.request_crate() == krate || placement.types_crate() == krate
        });
        let mut dependencies = IndexSet::new();
        for member in members {
            dependencies.extend(components.deps_for_component(member, policy));
        }
        for dependency in dependencies {
            let dependent_crate = placement(dependency).types_crate();
            if dependent_crate != krate {
                graph.add_edge(krate, dependent_crate, ());
            }
        }
    }
    graph
}

impl crate::codegen::CodeGen {
    /// Preserve the original graph command's paths and DOT labels. The richer
    /// analysis bundle is additive and does not replace this tooling interface.
    pub fn legacy_graphs(&self) -> anyhow::Result<crate::output::GeneratedWorkspace> {
        use petgraph::dot::{Config, Dot};
        use petgraph::prelude::DiGraphMap;

        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
        struct CrateLabel<'a>(&'a str);
        impl std::fmt::Debug for CrateLabel<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Crate").field("base_name", &self.0).finish()
            }
        }

        let source = self.components.gen_component_dep_graph(self.edge_policy());
        let mut components = DiGraphMap::<_, ()>::new();
        for node in source.nodes() {
            components.add_node(self.components.component_name(node));
        }
        for (from, to, _) in source.all_edges() {
            components.add_edge(
                self.components.component_name(from),
                self.components.component_name(to),
                (),
            );
        }
        let source = gen_inferred_crate_dep_graph(
            &self.components,
            &self.config,
            self.edge_policy(),
            |component| self.output_locations().placement(component),
        );
        let mut crates = DiGraphMap::<_, ()>::new();
        for node in source.nodes() {
            crates.add_node(CrateLabel(node.base_name(&self.config)));
        }
        for (from, to, _) in source.all_edges() {
            crates.add_edge(
                CrateLabel(from.base_name(&self.config)),
                CrateLabel(to.base_name(&self.config)),
                (),
            );
        }
        let mut output = crate::output::GeneratedWorkspace::default();
        output.write(
            "graphs/components_graph.txt",
            format!("{:?}", Dot::with_config(&components, &[Config::EdgeNoLabel])),
        )?;
        output.write(
            "graphs/crate_graph.txt",
            format!("{:?}", Dot::with_config(&crates, &[Config::EdgeNoLabel])),
        )?;
        Ok(output)
    }
}
