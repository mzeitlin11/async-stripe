use std::collections::{HashMap, HashSet};

use anyhow::Context as _;

use crate::component_catalog::ComponentId;
use crate::item_catalog::ItemCatalog;
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::request_model::RequestModel;
use crate::rust_object::RustObject;
use crate::rust_type::RustType;

/// Actual type uses between emitted modules, including isolated modules and
/// intra-module references. Facade reexports are separate crate-level consumers.
pub(crate) type ModuleDependencies = HashMap<EmissionSite, HashSet<EmissionSite>>;

pub(crate) fn module_dependencies<'a>(
    items: &ItemCatalog,
    requests: impl IntoIterator<Item = (ComponentId, &'a [RequestModel])>,
    layout: &ItemLayout,
) -> anyhow::Result<ModuleDependencies> {
    let mut graph = ModuleDependencies::new();
    for (item, record) in items.iter() {
        let source = layout.placement(item).emission_site();
        graph.entry(source).or_default();
        record.visit_type_uses(|typ| {
            add_type_dependencies(&mut graph, source, typ, layout).with_context(|| {
                format!("invalid dependencies for item {} ({item:?})", record.metadata.ident)
            })
        })?;
    }
    for (component, requests) in requests {
        let source = EmissionSite::ComponentRequests(component);
        for request in requests {
            graph.entry(source).or_default();
            // The request builder sets optional fields directly, so moving a
            // parameter declaration cannot silently widen its visibility.
            if let Some(params_root) = &request.params
                && let Some(RustObject::Struct(params)) = items.object_for_type(&params_root.typ)
            {
                let param_site = layout
                    .placement(params_root.typ.item_id().expect("parameter struct has no item"))
                    .emission_site();
                for field in &params.fields {
                    anyhow::ensure!(
                        param_site == source || field.required || field.vis.is_public(),
                        "request {} cannot set private parameter field {} in another emission site",
                        request.ident,
                        field.field_name
                    );
                }
            }
            request
                .visit_type_uses(items, |typ| {
                    add_type_dependencies(&mut graph, source, typ, layout)
                })
                .with_context(|| format!("invalid dependencies for request {}", request.ident))?;
        }
    }
    Ok(graph)
}

fn add_type_dependencies(
    graph: &mut ModuleDependencies,
    source: EmissionSite,
    typ: &RustType,
    layout: &ItemLayout,
) -> anyhow::Result<()> {
    match typ {
        RustType::Item(item) => {
            let target = layout.placement(*item);
            anyhow::ensure!(
                target.address() != ItemAddress::Local || target.emission_site() == source,
                "local item {item:?} is not visible from {source:?}; emitted in {:?}",
                target.emission_site(),
            );
            graph.entry(source).or_default().insert(target.emission_site());
            graph.entry(target.emission_site()).or_default();
        }
        RustType::Container(container) => {
            add_type_dependencies(graph, source, container.value_typ(), layout)?;
        }
        RustType::Reference(inner) => add_type_dependencies(graph, source, inner, layout)?,
        RustType::Primitive(_) | RustType::External(_) => {}
        RustType::Object(_, _) | RustType::Path(_) => {
            anyhow::bail!("unlowered type in module dependencies: {typ:?}");
        }
    }
    Ok(())
}
