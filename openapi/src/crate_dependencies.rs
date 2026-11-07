use petgraph::algo::is_cyclic_directed;
use petgraph::prelude::DiGraphMap;

use crate::component_catalog::ComponentId;
use crate::crates::{Crate, CrateConfig};
use crate::exports::{PublicExport, component_exports};
use crate::item_catalog::ItemCatalog;
use crate::item_layout::ItemLayout;
use crate::item_origin::TypeRoot;
use crate::layout::ComponentLayout;
use crate::module_dependencies::module_dependencies;
use crate::request_model::RequestModel;
use crate::rust_type::DeclarationRoot;

/// Dependencies required by lowered declarations, request APIs, and public facades.
/// Unlike the spec graph used to choose placement, this includes ID references
/// and references introduced by model rewrites, without applying an edge policy.
pub(crate) fn crate_dependencies<'a>(
    items: &ItemCatalog,
    requests: impl IntoIterator<Item = (ComponentId, &'a [RequestModel])>,
    component_layout: &ComponentLayout,
    item_layout: &ItemLayout,
    config: &CrateConfig,
) -> anyhow::Result<DiGraphMap<Crate, ()>> {
    let modules = module_dependencies(items, requests, item_layout)?;
    let mut graph = DiGraphMap::new();
    for krate in config.all() {
        graph.add_node(krate);
    }

    for (item, record) in items.iter() {
        if !record.origin.path.is_empty() {
            continue;
        }
        let exports = match record.origin.type_root {
            TypeRoot::Declaration(DeclarationRoot::Component(component)) => {
                component_exports(component, items, item_layout, component_layout)?
            }
            TypeRoot::Declaration(DeclarationRoot::Shared(_)) => {
                vec![PublicExport::item(item, Crate::SHARED, item_layout)?]
            }
            _ => Vec::new(),
        };
        for export in exports {
            if let Some((source, target)) = export.dependency() {
                graph.add_edge(source, target, ());
            }
        }
    }
    for (source, targets) in modules {
        // Webhooks belong to a handwritten crate with its own manifest.
        let Some(source) = source.krate(component_layout) else { continue };
        for target in targets {
            if let Some(target) = target.krate(component_layout)
                && source != target
            {
                graph.add_edge(source, target, ());
            }
        }
    }
    Ok(graph)
}

/// Counterfactual layouts remain available to analysis, but generation must not
/// produce manifests containing cycles in the actual Rust dependencies.
pub(crate) fn validate_crate_dependencies(graph: &DiGraphMap<Crate, ()>) -> anyhow::Result<()> {
    anyhow::ensure!(
        !is_cyclic_directed(graph),
        "lowered Rust crate dependencies contain a cycle; placement must account for the emitted types"
    );
    Ok(())
}
