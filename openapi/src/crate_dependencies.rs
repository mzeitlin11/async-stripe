use anyhow::Context as _;
use petgraph::algo::is_cyclic_directed;
use petgraph::prelude::DiGraphMap;

use crate::component_catalog::ComponentId;
use crate::crates::{ALL_CRATES, Crate};
use crate::item_catalog::ItemCatalog;
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::layout::ComponentLayout;
use crate::request_model::RequestModel;
use crate::rust_object::{ObjectIdentity, RustObject};
use crate::rust_type::{DeclarationRoot, RustType};

/// Dependencies required by lowered declarations, request APIs, and public facades.
/// Unlike the spec graph used to choose placement, this includes ID references
/// and references introduced by model rewrites, without applying an edge policy.
pub(crate) fn crate_dependencies<'a>(
    items: &ItemCatalog,
    requests: impl IntoIterator<Item = (ComponentId, &'a [RequestModel])>,
    component_layout: &ComponentLayout,
    item_layout: &ItemLayout,
) -> anyhow::Result<DiGraphMap<Crate, ()>> {
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
        let Some(object) = record.object() else { continue };
        let source = item_layout.placement(item).emission_site();
        let mut add_type = |typ: &RustType| {
            add_type_dependencies(&mut graph, source, typ, component_layout, item_layout)
                .with_context(|| {
                    format!("invalid dependencies for item {} ({item:?})", record.metadata.ident)
                })
        };
        match &object.definition {
            RustObject::Struct(strukt) => {
                for field in &strukt.fields {
                    add_type(&field.rust_type)?;
                }
            }
            RustObject::Enum(variants) => {
                for typ in variants.iter().filter_map(|variant| variant.rust_type.as_ref()) {
                    add_type(typ)?;
                }
            }
            RustObject::FieldlessEnum(_) => {}
        }
        // The associated ID type in an Object impl is a separate consumer.
        if let Some(ObjectIdentity::IdType(typ)) = &object.identity {
            add_type(typ)?;
        }
    }
    for (component, requests) in requests {
        let source = EmissionSite::ComponentRequests(component);
        for request in requests {
            // Roots that only reference existing items introduce no declaration,
            // so walking the item arena alone would miss these dependencies.
            let types = request
                .path_params
                .iter()
                .map(|param| &param.typ)
                .chain(request.params.iter().map(|root| &root.typ))
                .chain(std::iter::once(&request.returned.typ));
            for typ in types {
                add_type_dependencies(&mut graph, source, typ, component_layout, item_layout)
                    .with_context(|| {
                        format!("invalid dependencies for request {}", request.ident)
                    })?;
            }
            // The request builder exposes parameter fields in its constructor
            // and setters even when their containing declaration lives elsewhere.
            if let Some(params_root) = &request.params
                && let Some(RustObject::Struct(params)) = items.object_for_type(&params_root.typ)
            {
                let param_site = item_layout
                    .placement(params_root.typ.item_id().expect("parameter struct has no item"))
                    .emission_site();
                for field in &params.fields {
                    anyhow::ensure!(
                        param_site == source || field.required || field.vis.is_public(),
                        "request {} cannot set private parameter field {} in another emission site",
                        request.ident,
                        field.field_name
                    );
                    add_type_dependencies(
                        &mut graph,
                        source,
                        &field.rust_type,
                        component_layout,
                        item_layout,
                    )
                    .with_context(|| {
                        format!("invalid constructor dependencies for request {}", request.ident)
                    })?;
                }
            }
        }
    }
    Ok(graph)
}

fn add_type_dependencies(
    graph: &mut DiGraphMap<Crate, ()>,
    source: EmissionSite,
    typ: &RustType,
    component_layout: &ComponentLayout,
    item_layout: &ItemLayout,
) -> anyhow::Result<()> {
    match typ {
        RustType::Item(item) => {
            let target = item_layout.placement(*item);
            match target.address() {
                ItemAddress::Local => anyhow::ensure!(
                    target.emission_site() == source,
                    "local item {item:?} is not visible from {source:?}; emitted in {:?}",
                    target.emission_site(),
                ),
                ItemAddress::CrateRoot(target_crate) => {
                    // Webhooks belong to a handwritten crate with its own manifest.
                    if let Some(source_crate) = source.krate(component_layout)
                        && source_crate != target_crate
                    {
                        graph.add_edge(source_crate, target_crate, ());
                    }
                }
            }
        }
        RustType::Container(container) => add_type_dependencies(
            graph,
            source,
            container.value_typ(),
            component_layout,
            item_layout,
        )?,
        RustType::Reference(inner) => {
            add_type_dependencies(graph, source, inner, component_layout, item_layout)?
        }
        RustType::Primitive(_) | RustType::External(_) => {}
        RustType::Object(_, _) | RustType::Path(_) => {
            anyhow::bail!("unlowered type in crate dependencies: {typ:?}")
        }
    }
    Ok(())
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
