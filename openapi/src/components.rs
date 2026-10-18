use std::collections::HashMap;
use std::fmt::Debug;

use anyhow::bail;
use indexmap::{IndexMap, IndexSet};
use petgraph::Direction;
use tracing::debug;

use crate::component_catalog::{ComponentCatalog, ComponentId};
use crate::crates::ResolvedCrateConfig;
use crate::deduplication::deduplicate_types;
use crate::dependencies::{Dependency, DependencyIndex, EdgePolicy};
use crate::layout::ComponentLayoutDraft;
use crate::operations::{
    OperationCatalog, OperationId, OperationRecord, OperationSelector, RequestSpec, StripeOperation,
};
use crate::overrides::Overrides;
use crate::requests::parse_requests;
use crate::rust_object::{ObjectIdentity, ObjectKind, ObjectUsage, RustObject};
use crate::rust_type::RustType;
use crate::spec::Spec;
use crate::stripe_object::{StripeObject, StripeResource};
use crate::types::{ComponentName, RustIdent};
use crate::webhook::WebhookObject;

#[derive(Clone, Debug)]
pub struct TypeSpec {
    pub obj: RustObject,
    pub doc: Option<String>,
    pub mod_path: String,
    pub usage: ObjectUsage,
}

pub struct Components {
    catalog: ComponentCatalog,
    operation_catalog: OperationCatalog,
    dependency_index: DependencyIndex,
    pub components: IndexMap<ComponentId, StripeObject>,
    pub webhook_objs: Vec<WebhookObject>,
    pub extra_types: IndexMap<RustIdent, TypeSpec>,
}

struct ParsedComponent {
    object: StripeObject,
    in_package: Option<String>,
    operations: Vec<StripeOperation>,
}

impl Components {
    pub fn parsed_component_count(&self) -> usize {
        self.catalog.len()
    }

    pub fn parsed_dependency_count(&self) -> usize {
        self.dependency_index.facts().len()
    }

    pub fn declared_operation_count(&self) -> usize {
        self.operation_catalog.len()
    }

    pub(crate) fn dependencies(&self) -> &[Dependency] {
        self.dependency_index.facts()
    }

    pub(crate) fn operation_record(&self, operation: OperationId) -> &OperationRecord {
        self.operation_catalog.record(operation)
    }

    pub(crate) fn requests_for_component(&self, component: ComponentId) -> &[RequestSpec] {
        self.operation_catalog.requests_for_component(component)
    }

    pub(crate) fn has_requests(&self, component: ComponentId) -> bool {
        !self.requests_for_component(component).is_empty()
    }

    pub(crate) fn request_count(&self) -> usize {
        self.operation_catalog.request_count()
    }

    pub(crate) fn request_component_count(&self) -> usize {
        self.components.keys().filter(|component| self.has_requests(**component)).count()
    }

    pub(crate) fn validate_catalog(&self) -> anyhow::Result<()> {
        self.catalog.validate_retained(self.components.keys().copied())
    }

    #[track_caller]
    pub(crate) fn component_name(&self, id: ComponentId) -> &ComponentName {
        self.catalog.name(id)
    }

    pub(crate) fn is_retained(&self, id: ComponentId) -> bool {
        self.components.contains_key(&id)
    }

    #[track_caller]
    pub(crate) fn get_by_id(&self, id: ComponentId) -> &StripeObject {
        &self.components[&id]
    }

    pub(crate) fn deps_for_component(
        &self,
        component: ComponentId,
        policy: EdgePolicy,
    ) -> IndexSet<ComponentId> {
        self.dependency_index.component_targets(component, policy)
    }

    pub(crate) fn deps_for_component_type(
        &self,
        component: ComponentId,
        policy: EdgePolicy,
    ) -> IndexSet<ComponentId> {
        self.dependency_index.component_model_targets(component, policy)
    }

    pub fn deps_for_webhooks(&self) -> IndexSet<ComponentId> {
        self.dependency_index.webhook_targets()
    }

    pub fn filter_unused_components(&mut self) {
        loop {
            let mut unused = vec![];
            let webhook_deps = self.deps_for_webhooks();
            // Expandable targets still need to be retained even when crate-placement
            // analysis treats the corresponding type edges as removable.
            let graph = self.gen_component_dep_graph(EdgePolicy::current_layout());
            for &component_id in self.components.keys() {
                let path = self.component_name(component_id);
                // The `error` component is a false positive since we don't include the error
                // type in dependency calculations, but every request depends on `error`
                if path.as_ref() == "error" || webhook_deps.contains(&component_id) {
                    continue;
                }

                if !self.has_requests(component_id)
                    && graph.neighbors_directed(component_id, Direction::Incoming).count() == 0
                {
                    unused.push(component_id);
                }
            }
            let done_filtering = unused.is_empty();
            if done_filtering {
                break;
            }

            debug!("Filtering unused components: {unused:#?}");
            for unused_component in unused {
                self.components.shift_remove(&unused_component);
            }
        }
    }

    pub fn get_request_spec(&self, operation: OperationId) -> Option<&RequestSpec> {
        self.operation_catalog.request(operation)
    }

    pub(crate) fn is_nested_resource_of(&self, component: ComponentId, other: ComponentId) -> bool {
        let requests = self.requests_for_component(component);
        if requests.is_empty() {
            return false;
        }
        let other = self.get_by_id(other);
        requests.iter().all(|request| {
            let start = format!("/{}", other.path());
            request.req_path.starts_with(&start)
        })
    }

    pub fn resolve_operation(&self, selector: OperationSelector) -> anyhow::Result<OperationId> {
        let declared = self.operation_catalog.candidates(selector);
        let generated = declared
            .iter()
            .copied()
            .filter(|operation| self.get_request_spec(*operation).is_some())
            .collect::<Vec<_>>();
        if generated.len() != 1 {
            let candidates = declared
                .iter()
                .map(|operation| {
                    let record = self.operation_catalog.record(*operation);
                    let component = self.component_name(record.component);
                    let status =
                        if generated.contains(operation) { "generated" } else { "not generated" };
                    format!("{component} ({:?} {}, {status})", record.method, record.path)
                })
                .collect::<Vec<_>>();
            let candidates =
                if candidates.is_empty() { "none".to_string() } else { candidates.join(", ") };
            bail!(
                "operation selector {selector:?} matched {} generated requests; declared candidates: {candidates}",
                generated.len()
            );
        }
        Ok(generated[0])
    }

    pub(crate) fn run_deduplication_pass(&mut self) {
        let Components { operation_catalog, components, .. } = self;
        for (&component_id, comp) in components {
            let requests = operation_catalog.requests_for_component_mut(component_id);
            let extra_typs = deduplicate_types(comp, requests, component_id);
            for (ident, typ) in extra_typs {
                comp.deduplicated_objects.insert(ident, typ);
            }
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) fn apply_overrides(&mut self) -> anyhow::Result<()> {
        let mut overrides = Overrides::new(self)?;
        for comp in self.components.values_mut() {
            comp.visit_mut(&mut overrides);
        }
        for request in self.operation_catalog.requests_mut() {
            request.visit_mut(&mut overrides);
        }
        for (obj, override_meta) in overrides.overrides {
            self.extra_types.insert(
                override_meta.metadata.ident.clone(),
                TypeSpec {
                    obj,
                    doc: override_meta.metadata.doc,
                    mod_path: override_meta.mod_path,
                    usage: ObjectUsage { kind: ObjectKind::Type, used_as_request_param: true },
                },
            );
        }

        Ok(())
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn infer_components(spec: &Spec) -> anyhow::Result<(Components, ComponentLayoutDraft)> {
    let mut webhook_objs = vec![];
    let mut components = IndexMap::with_capacity(spec.component_schemas().len());

    let mut parsed_components = IndexMap::new();
    let mut id_map = HashMap::new();
    let mut component_layout = ComponentLayoutDraft::default();
    let mut catalog_names = Vec::new();
    for path in spec.component_schemas().keys() {
        let path = ComponentName::new(path.clone());
        let schema = spec.get_component_schema(&path);
        if !WebhookObject::is_webhook_schema(schema) {
            catalog_names.push(path);
        }
    }
    let catalog = ComponentCatalog::from_names(catalog_names)?;
    let crate_config = ResolvedCrateConfig::from_catalog(&catalog)?;

    for path in spec.component_schemas().keys() {
        let path = ComponentName::new(path.clone());
        let schema = spec.get_component_schema(&path);

        if let Some(webhook_object) = WebhookObject::from_schema(schema, &catalog)? {
            webhook_objs.push(webhook_object);
            continue;
        };

        let resource = StripeResource::from_schema(schema, path.clone())?;
        let object = StripeObject::from_schema(schema, &resource, &catalog);
        if let Some(obj_name) = object.object_name()
            && let Some(id_typ) = object
                .identity()
                .and_then(ObjectIdentity::id_type)
                .and_then(RustType::as_id_or_opt_id_path)
        {
            // We will hit duplicate object names from e.g. `account` and `deleted_account`, but sanity check nothing
            // worse can happen
            if let Some(orig) = id_map.insert(obj_name.to_string(), id_typ)
                && orig != id_typ
            {
                bail!("Mismatched object name and id type for {obj_name}");
            }
        }
        let component = catalog.id(&path).expect("inferred component was missing from catalog");
        parsed_components.insert(
            component,
            ParsedComponent {
                object,
                in_package: resource.in_package,
                operations: resource.operations,
            },
        );
    }

    debug_assert_eq!(
        catalog.iter().map(|(component, _)| component).collect::<Vec<_>>(),
        parsed_components.keys().copied().collect::<Vec<_>>()
    );
    let mut operation_catalog = OperationCatalog::from_operations(
        parsed_components
            .iter()
            .map(|(component, parsed)| (*component, parsed.operations.as_slice())),
    )?;

    for (component_id, parsed) in parsed_components {
        let path = parsed.object.path();

        if parsed.in_package.is_none() {
            tracing::debug!("No package for {path}. Using path name for inference instead");
        }

        let inferred_crate = parsed
            .in_package
            .as_ref()
            .map(|package| crate_config.crate_for_package(package))
            .transpose()?
            .or_else(|| crate_config.crate_for_component(component_id));
        component_layout.insert_initial(component_id, inferred_crate);

        let reqs = parse_requests(
            parsed.operations,
            operation_catalog.for_component(component_id),
            spec,
            parsed.object.ident(),
            &id_map,
            &catalog,
        )?;
        operation_catalog.attach_requests(component_id, reqs)?;
        components.insert(component_id, parsed.object);
    }

    let mut inferred = Components {
        catalog,
        operation_catalog,
        dependency_index: DependencyIndex::default(),
        components,
        webhook_objs,
        extra_types: Default::default(),
    };
    inferred.dependency_index = DependencyIndex::build(&inferred);
    Ok((inferred, component_layout))
}

#[derive(Debug, Copy, Clone)]
pub struct RequestSource {
    pub operation: OperationSelector,
    pub field_name: &'static str,
}
