use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use petgraph::prelude::DiGraphMap;

use crate::codegen::CodeGen;
use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crates::{ALL_CRATES, Crate};
use crate::dependencies::{Dependency, DependencySource, EdgePolicy};
use crate::graph::{ComponentGraph, gen_crate_dep_graph};
use crate::item_catalog::{ItemKind, RustItem};
use crate::item_origin::{ItemOrigin, TypeRoot};
use crate::output::ArtifactSet;
use crate::rust_object::{ObjectKind, RustObject};
use crate::rust_type::{DeclarationRoot, RustType};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GraphDetail {
    None,
    Default,
    All,
}

#[derive(Debug, Default)]
pub struct AnalysisBundle {
    files: ArtifactSet,
}

impl AnalysisBundle {
    pub fn write_to(&self, root: &Path) -> anyhow::Result<()> {
        self.files.materialize_owned(root, ".async-stripe-analysis-files.json")
    }

    fn insert(&mut self, path: impl Into<PathBuf>, contents: String) {
        self.files.write(path.into(), contents).expect("analysis file was emitted twice");
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct ShapeStats {
    objects: usize,
    fields: usize,
    variants: usize,
}

impl ShapeStats {
    fn add(&mut self, other: Self) {
        self.objects += other.objects;
        self.fields += other.fields;
        self.variants += other.variants;
    }
}

#[derive(Debug, Clone, Default)]
struct CrateStats {
    model_components: usize,
    request_components: usize,
    requests: usize,
    shape: ShapeStats,
}

#[derive(Debug)]
struct ComponentStats {
    id: ComponentId,
    model: ShapeStats,
    requests: ShapeStats,
}

impl CodeGen {
    pub fn analysis_bundle(&self, graph_detail: GraphDetail) -> AnalysisBundle {
        let locations = self.output_locations();
        let component_stats = component_stats(&self.components);
        let crate_stats = crate_stats(self, &component_stats);
        let mut bundle = AnalysisBundle::default();
        bundle.insert("summary.md", summary(self, &component_stats, &crate_stats));
        bundle.insert("components.csv", components_csv(self, &component_stats));
        bundle.insert("items.csv", items_csv(self));
        bundle.insert("item-occurrences.csv", item_occurrences_csv(self));
        bundle.insert("dependencies.csv", dependencies_csv(&self.components));
        bundle.insert("crate-inference-warnings.csv", crate_warnings_csv(self));

        if graph_detail != GraphDetail::None {
            bundle.insert(
                "graphs/crate-dependencies.dot",
                crate_graph_dot(&gen_crate_dep_graph(
                    &self.components,
                    self.edge_policy(),
                    |component| locations.placement(component),
                )),
            );
            bundle.insert(
                "graphs/component-dependencies.dot",
                component_graph_dot(
                    &self.components,
                    &self.components.gen_component_dep_graph(self.edge_policy()),
                    "all",
                    self.edge_policy(),
                ),
            );
            bundle.insert(
                "graphs/component-model-dependencies.dot",
                component_graph_dot(
                    &self.components,
                    &self.components.gen_component_type_dep_graph(self.edge_policy()),
                    "model",
                    self.edge_policy(),
                ),
            );
        }
        if graph_detail == GraphDetail::All {
            for (suffix, policy) in [
                ("with-expandables", EdgePolicy::current_layout()),
                ("without-expandables", EdgePolicy::without_expandables()),
                ("with-id-only", EdgePolicy::all_dependencies()),
            ] {
                bundle.insert(
                    format!("graphs/component-dependencies-{suffix}.dot"),
                    component_graph_dot(
                        &self.components,
                        &self.components.gen_component_dep_graph(policy),
                        "all",
                        policy,
                    ),
                );
                bundle.insert(
                    format!("graphs/component-model-dependencies-{suffix}.dot"),
                    component_graph_dot(
                        &self.components,
                        &self.components.gen_component_type_dep_graph(policy),
                        "model",
                        policy,
                    ),
                );
            }
        }
        bundle
    }
}

fn component_stats(components: &Components) -> Vec<ComponentStats> {
    let mut stats = Vec::with_capacity(components.components.len());
    for (&id, component) in &components.components {
        let mut model = shape_for_object(component.rust_obj());
        let mut requests = ShapeStats::default();
        for object in component.deduplicated_objects.values() {
            let shape = shape_for_object(&object.object);
            match object.usage.kind {
                ObjectKind::Type => model.add(shape),
                ObjectKind::RequestParam | ObjectKind::RequestReturned => requests.add(shape),
            }
        }
        for request in components.requests_for_component(id) {
            for path_param in &request.path_params {
                requests.add(shape_for_type(&path_param.rust_type));
            }
            if let Some(params) = &request.params {
                requests.add(shape_for_type(&params.typ));
            }
            requests.add(shape_for_type(&request.returned));
        }
        stats.push(ComponentStats { id, model, requests });
    }
    stats.sort_by_key(|stats| components.component_name(stats.id).to_string());
    stats
}

fn crate_stats(
    codegen: &CodeGen,
    component_stats: &[ComponentStats],
) -> BTreeMap<String, CrateStats> {
    let locations = codegen.output_locations();
    let mut crates = BTreeMap::new();
    for krate in &*ALL_CRATES {
        let name = krate.crate_name();
        crates.insert(name, CrateStats::default());
    }
    for stats in component_stats {
        let placement = locations.placement(stats.id);
        let types = crates.get_mut(&placement.types_crate().crate_name()).unwrap();
        types.model_components += 1;
        types.shape.add(stats.model);
        if codegen.components.has_requests(stats.id) {
            let requests = crates.get_mut(&placement.request_crate().crate_name()).unwrap();
            requests.request_components += 1;
            requests.requests += codegen.components.requests_for_component(stats.id).len();
            requests.shape.add(stats.requests);
        }
    }
    let shared = crates.get_mut(&Crate::SHARED.crate_name()).unwrap();
    for extra in codegen.components.extra_types.values() {
        shared.shape.add(shape_for_object(&extra.obj));
    }
    let webhook_name = "async-stripe-webhook".to_string();
    let mut webhook = CrateStats::default();
    for event in &codegen.components.webhook_objs {
        webhook.shape.add(shape_for_type(&event.typ));
    }
    crates.insert(webhook_name, webhook);
    crates
}

fn summary(
    codegen: &CodeGen,
    component_stats: &[ComponentStats],
    crate_stats: &BTreeMap<String, CrateStats>,
) -> String {
    let generated_requests = codegen.components.request_count();
    let mut shape = ShapeStats::default();
    for stats in component_stats {
        shape.add(stats.model);
        shape.add(stats.requests);
    }
    for extra in codegen.components.extra_types.values() {
        shape.add(shape_for_object(&extra.obj));
    }
    for webhook in &codegen.components.webhook_objs {
        shape.add(shape_for_type(&webhook.typ));
    }
    let items = codegen.model.items();
    let canonical_objects =
        items.iter().filter(|(_, item)| matches!(item.item, RustItem::Object(_))).count();
    assert_eq!(
        shape.objects, canonical_objects,
        "canonical item catalog did not match the independently counted Rust objects"
    );

    let retained_dependencies = codegen
        .components
        .dependencies()
        .iter()
        .filter(|dependency| {
            let (source_retained, target_retained) =
                dependency_retention(&codegen.components, dependency);
            source_retained && target_retained
        })
        .collect::<Vec<_>>();
    let mut forms = BTreeMap::<String, usize>::new();
    let mut usages = BTreeMap::<String, usize>::new();
    for dependency in &retained_dependencies {
        *forms.entry(format!("{:?}", dependency.form)).or_default() += 1;
        *usages.entry(format!("{:?}", dependency.usage)).or_default() += 1;
    }

    let mut out = String::from("# Code Generation Analysis\n\n## Pipeline\n\n");
    let _ = writeln!(out, "- Parsed components: {}", codegen.components.parsed_component_count());
    let _ = writeln!(out, "- Retained components: {}", codegen.components.components.len());
    let _ =
        writeln!(out, "- Declared operations: {}", codegen.components.declared_operation_count());
    let _ = writeln!(out, "- Generated requests: {generated_requests}");
    let _ = writeln!(out, "- Webhook event payloads: {}", codegen.components.webhook_objs.len());
    let _ = writeln!(out, "- Objects: {}", shape.objects);
    let _ = writeln!(out, "- Canonical Rust objects: {canonical_objects}");
    let _ = writeln!(out, "- Canonical Rust items: {}", codegen.canonical_item_count());
    let lifetime_items = items.iter().filter(|(id, _)| items.properties(*id).has_lifetime).count();
    let copy_items = items.iter().filter(|(id, _)| items.properties(*id).is_copy).count();
    let eq_items = items.iter().filter(|(id, _)| items.properties(*id).is_eq).count();
    let _ =
        writeln!(out, "- Canonical property analysis rounds: {}", items.property_analysis_rounds());
    let _ = writeln!(out, "- Canonical items with lifetimes: {lifetime_items}");
    let _ = writeln!(out, "- Canonical items deriving Copy: {copy_items}");
    let _ = writeln!(out, "- Canonical items deriving Eq: {eq_items}");
    let _ = writeln!(out, "- Fields: {}", shape.fields);
    let _ = writeln!(out, "- Enum variants: {}", shape.variants);
    let _ = writeln!(
        out,
        "- Parsed dependency facts: {}",
        codegen.components.parsed_dependency_count()
    );
    let _ = writeln!(out, "- Retained dependency facts: {}", retained_dependencies.len());
    let _ = writeln!(
        out,
        "- Expandable edges used for layout: {}",
        codegen.edge_policy().keeps_expandables()
    );
    let _ =
        writeln!(out, "- ID-only edges used for layout: {}", codegen.edge_policy().keeps_id_only());
    let _ = writeln!(out, "- Crate inference warnings: {}", codegen.crate_inference_warnings.len());
    out.push_str("\n## Dependency facts\n\n### By representation\n\n");
    for (form, count) in forms {
        let _ = writeln!(out, "- {form}: {count}");
    }
    out.push_str("\n### By usage\n\n");
    for (usage, count) in usages {
        let _ = writeln!(out, "- {usage}: {count}");
    }

    out.push_str(
        "\n## Crates\n\n| Crate | Model components | Request components | Requests | Objects | Fields |\n|---|---:|---:|---:|---:|---:|\n",
    );
    for (name, stats) in crate_stats {
        let _ = writeln!(
            out,
            "| {name} | {} | {} | {} | {} | {} |",
            stats.model_components,
            stats.request_components,
            stats.requests,
            stats.shape.objects,
            stats.shape.fields
        );
    }
    out
}

fn items_csv(codegen: &CodeGen) -> String {
    let mut out =
        String::from("item_id,origin,ident,kind,usage,has_lifetime,is_copy,is_eq,sharing_rule\n");
    let items = codegen.model.items();
    for (id, item) in items.iter() {
        let properties = items.properties(id);
        let row = [
            id.index().to_string(),
            item_origin(codegen, &item.origin),
            item.metadata.ident.to_string(),
            format!("{:?}", ItemKind::of(&item.item)),
            item.object().map(|object| format!("{:?}", object.usage)).unwrap_or_default(),
            properties.has_lifetime.to_string(),
            properties.is_copy.to_string(),
            properties.is_eq.to_string(),
            item.sharing_rule.map(|rule| format!("{rule:?}")).unwrap_or_default(),
        ];
        let _ =
            writeln!(out, "{}", row.iter().map(|value| csv(value)).collect::<Vec<_>>().join(","));
    }
    out
}

fn item_occurrences_csv(codegen: &CodeGen) -> String {
    let mut out = String::from("item_id,origin,usage\n");
    for (id, item) in codegen.model.items().iter() {
        for occurrence in &item.occurrences {
            let _ = writeln!(
                out,
                "{},{},{}",
                id.index(),
                csv(&item_origin(codegen, &occurrence.origin)),
                csv(&format!("{:?}", occurrence.usage)),
            );
        }
    }
    out
}

fn item_origin(codegen: &CodeGen, origin: &ItemOrigin) -> String {
    let root = match &origin.type_root {
        TypeRoot::Declaration(DeclarationRoot::Component(component)) => {
            format!("component:{}", codegen.components.component_name(*component))
        }
        TypeRoot::Declaration(DeclarationRoot::ObjectId(component)) => {
            format!("object-id:{}", codegen.components.component_name(*component))
        }
        TypeRoot::OperationParam(operation) => {
            let record = codegen.components.operation_record(*operation);
            format!("operation:{} {:?} params", record.path, record.method)
        }
        TypeRoot::OperationPathParam { operation, index } => {
            let record = codegen.components.operation_record(*operation);
            let request = codegen
                .components
                .get_request_spec(*operation)
                .expect("operation path parameter had no retained request");
            let param = &request.path_params[*index];
            format!("operation:{} {:?} path-param:{}", record.path, record.method, param.name)
        }
        TypeRoot::OperationReturn(operation) => {
            let record = codegen.components.operation_record(*operation);
            format!("operation:{} {:?} return", record.path, record.method)
        }
        TypeRoot::Declaration(DeclarationRoot::Deduplicated { component, ident }) => {
            format!("deduplicated:{}:{ident}", codegen.components.component_name(*component))
        }
        TypeRoot::Declaration(DeclarationRoot::Shared(ident)) => format!("shared:{ident}"),
        TypeRoot::Webhook(event) => format!("webhook:{event}"),
    };
    if origin.path.is_empty() { root } else { format!("{root}.{}", origin.path.join(".")) }
}

fn components_csv(codegen: &CodeGen, stats: &[ComponentStats]) -> String {
    let locations = codegen.output_locations();
    let mut out = String::from(
        "component,ident,request_crate,types_crate,requests,model_objects,model_fields,request_objects,request_fields,dependencies,dependencies_without_expandables,dependencies_with_id_only,model_dependencies,model_dependencies_without_expandables,model_dependencies_with_id_only\n",
    );
    for stats in stats {
        let component = codegen.components.get_by_id(stats.id);
        let placement = locations.placement(stats.id);
        let values = [
            codegen.components.component_name(stats.id).to_string(),
            component.ident().to_string(),
            placement.request_crate().crate_name(),
            placement.types_crate().crate_name(),
            codegen.components.requests_for_component(stats.id).len().to_string(),
            stats.model.objects.to_string(),
            stats.model.fields.to_string(),
            stats.requests.objects.to_string(),
            stats.requests.fields.to_string(),
            codegen
                .components
                .deps_for_component(stats.id, EdgePolicy::current_layout())
                .len()
                .to_string(),
            codegen
                .components
                .deps_for_component(stats.id, EdgePolicy::without_expandables())
                .len()
                .to_string(),
            codegen
                .components
                .deps_for_component(stats.id, EdgePolicy::all_dependencies())
                .len()
                .to_string(),
            codegen
                .components
                .deps_for_component_type(stats.id, EdgePolicy::current_layout())
                .len()
                .to_string(),
            codegen
                .components
                .deps_for_component_type(stats.id, EdgePolicy::without_expandables())
                .len()
                .to_string(),
            codegen
                .components
                .deps_for_component_type(stats.id, EdgePolicy::all_dependencies())
                .len()
                .to_string(),
        ];
        let _ = writeln!(out, "{}", values.map(|value| csv(&value)).join(","));
    }
    out
}

fn dependencies_csv(components: &Components) -> String {
    let mut rows = components
        .dependencies()
        .iter()
        .map(|dependency| {
            let (source_retained, target_retained) = dependency_retention(components, dependency);
            let source = match &dependency.origin.source {
                DependencySource::Component(component) => {
                    format!("component:{}", components.component_name(*component))
                }
                DependencySource::Operation(operation) => {
                    let record = components.operation_record(*operation);
                    format!(
                        "operation:{:?} {} [{}]",
                        record.method,
                        record.path,
                        components.component_name(record.component)
                    )
                }
                DependencySource::WebhookEvent(event) => format!("webhook:{event}"),
            };
            vec![
                source,
                source_retained.to_string(),
                format!("{:?}", dependency.usage),
                format!("{:?}", dependency.form),
                dependency.origin.path.join("."),
                components.component_name(dependency.target).to_string(),
                target_retained.to_string(),
            ]
        })
        .collect::<Vec<_>>();
    rows.sort();

    let mut out =
        String::from("source,source_retained,usage,representation,path,target,target_retained\n");
    for row in rows {
        let _ =
            writeln!(out, "{}", row.iter().map(|value| csv(value)).collect::<Vec<_>>().join(","));
    }
    out
}

fn dependency_retention(components: &Components, dependency: &Dependency) -> (bool, bool) {
    let source_retained = match dependency.origin.source {
        DependencySource::Component(component) => components.is_retained(component),
        DependencySource::Operation(operation) => {
            components.is_retained(components.operation_record(operation).component)
        }
        DependencySource::WebhookEvent(_) => true,
    };
    (source_retained, components.is_retained(dependency.target))
}

fn crate_warnings_csv(codegen: &CodeGen) -> String {
    let mut rows = codegen
        .crate_inference_warnings
        .iter()
        .map(|warning| {
            vec![
                warning.component.to_string(),
                warning.chosen_crate.crate_name(),
                warning
                    .depended_on_by
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(";"),
                warning
                    .candidate_crates
                    .iter()
                    .map(|krate| krate.crate_name())
                    .collect::<Vec<_>>()
                    .join(";"),
            ]
        })
        .collect::<Vec<_>>();
    rows.sort();
    let mut out = String::from("component,assigned_crate,depended_on_by,candidate_crates\n");
    for row in rows {
        let _ =
            writeln!(out, "{}", row.iter().map(|value| csv(value)).collect::<Vec<_>>().join(","));
    }
    out
}

fn shape_for_type(typ: &RustType) -> ShapeStats {
    match typ {
        RustType::Object(object, _) => shape_for_object(object),
        RustType::Container(container) => shape_for_type(container.value_typ()),
        RustType::Reference(inner) => shape_for_type(inner),
        RustType::Path(_) | RustType::Primitive(_) | RustType::External(_) | RustType::Item(_) => {
            ShapeStats::default()
        }
    }
}

fn shape_for_object(object: &RustObject) -> ShapeStats {
    let mut stats = ShapeStats { objects: 1, ..ShapeStats::default() };
    match object {
        RustObject::Struct(struct_) => {
            stats.fields += struct_.fields.len();
            for field in &struct_.fields {
                stats.add(shape_for_type(&field.rust_type));
            }
        }
        RustObject::Enum(variants) => {
            stats.variants += variants.len();
            for variant in variants {
                if let Some(typ) = &variant.rust_type {
                    stats.add(shape_for_type(typ));
                }
            }
        }
        RustObject::FieldlessEnum(variants) => stats.variants += variants.len(),
    }
    stats
}

fn component_graph_dot(
    components: &Components,
    graph: &ComponentGraph,
    usage: &str,
    policy: EdgePolicy,
) -> String {
    let mut nodes = graph
        .nodes()
        .map(|component| components.component_name(component).to_string())
        .collect::<Vec<_>>();
    nodes.sort();
    let mut edges = graph
        .all_edges()
        .map(|(source, target, _)| {
            (
                components.component_name(source).to_string(),
                components.component_name(target).to_string(),
            )
        })
        .collect::<Vec<_>>();
    edges.sort();

    let mut out = format!(
        "// usage = {usage}\n// expandable_edges = {}\n// id_only_edges = {}\ndigraph component_dependencies {{\n",
        if policy.keeps_expandables() { "keep" } else { "opaque" },
        if policy.keeps_id_only() { "keep" } else { "opaque" }
    );
    for node in nodes {
        let _ = writeln!(out, "  \"{}\";", dot(&node));
    }
    for (source, target) in edges {
        let _ = writeln!(out, "  \"{}\" -> \"{}\";", dot(&source), dot(&target));
    }
    out.push_str("}\n");
    out
}

fn crate_graph_dot(graph: &DiGraphMap<Crate, ()>) -> String {
    let mut nodes = graph.nodes().map(Crate::crate_name).collect::<Vec<_>>();
    nodes.sort();
    let mut edges = graph
        .all_edges()
        .map(|(source, target, _)| (source.crate_name(), target.crate_name()))
        .collect::<Vec<_>>();
    edges.sort();

    let mut out = String::from(
        "// edge direction: source crate depends on target crate\ndigraph crate_dependencies {\n",
    );
    for node in nodes {
        let _ = writeln!(out, "  \"{}\";", dot(&node));
    }
    for (source, target) in edges {
        let _ = writeln!(out, "  \"{}\" -> \"{}\";", dot(&source), dot(&target));
    }
    out.push_str("}\n");
    out
}

fn csv(value: &str) -> String {
    if value.contains([',', '"', '\n']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn dot(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
