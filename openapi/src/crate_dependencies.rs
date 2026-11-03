use petgraph::algo::is_cyclic_directed;
use petgraph::prelude::DiGraphMap;

use crate::component_catalog::ComponentId;
use crate::crates::{ALL_CRATES, Crate};
use crate::item_catalog::ItemCatalog;
use crate::item_layout::{ItemAddress, ItemLayout};
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
) -> anyhow::Result<DiGraphMap<Crate, ()>> {
    let modules = module_dependencies(items, requests, item_layout)?;
    let mut graph = DiGraphMap::new();
    for &krate in &*ALL_CRATES {
        graph.add_node(krate);
        // Preserve the mandatory shared dependency used by domain crate
        // facades. Their public reexports are not represented by RustType.
        if krate != Crate::SHARED {
            graph.add_edge(krate, Crate::SHARED, ());
        }
    }

    for (item, record) in items.iter() {
        // Public facades are consumers too, including IDs emitted separately
        // from their models and shared overrides such as ApiVersion.
        let facade = if record.origin.path.is_empty() {
            match &record.origin.type_root {
                TypeRoot::Declaration(
                    DeclarationRoot::Component(component) | DeclarationRoot::ObjectId(component),
                ) => Some(component_layout.placement(*component).request_crate()),
                TypeRoot::Declaration(DeclarationRoot::Shared(_)) => Some(Crate::SHARED),
                _ => None,
            }
        } else {
            None
        };
        if let Some(source) = facade
            && let ItemAddress::CrateRoot(target) = item_layout.placement(item).address()
            && source != target
        {
            graph.add_edge(source, target, ());
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
