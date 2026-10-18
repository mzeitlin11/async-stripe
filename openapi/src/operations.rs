use std::collections::{HashMap, HashSet};

use anyhow::ensure;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::component_catalog::ComponentId;
use crate::rust_object::{ObjectUsage, RustObject, StructField};
use crate::rust_type::RustType;
use crate::types::RustIdent;
use crate::visitor::{Visit, VisitMut};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StripeOperation {
    pub method_name: String,
    pub method_type: MethodType,
    pub method_on: String,
    #[serde(rename = "operation")]
    pub operation_type: OperationType,
    pub path: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MethodType {
    Retrieve,
    List,
    Create,
    Update,
    Delete,
    Custom,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Get,
    Post,
    Delete,
}

/// Stable identity for one operation declaration within a codegen run.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OperationId(u32);

impl OperationId {
    fn from_index(index: usize) -> anyhow::Result<Self> {
        ensure!(index <= u32::MAX as usize, "too many operation declarations");
        Ok(Self(index as u32))
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone)]
pub struct RequestSpec {
    pub operation_id: OperationId,
    pub ident: RustIdent,
    pub path_params: Vec<PathParam>,
    pub params: Option<RequestParam>,
    pub method_type: OperationType,
    pub returned: RustType,
    pub doc_comment: Option<String>,
    pub req_path: String,
}

#[derive(Debug, Clone)]
pub struct RequestParam {
    pub typ: RustType,
}

impl RequestSpec {
    pub fn visit<'a, V: Visit<'a>>(&'a self, visitor: &mut V) {
        visitor.visit_typ(&self.returned, ObjectUsage::return_type());
        if let Some(params) = &self.params {
            visitor.visit_typ(&params.typ, ObjectUsage::request_param());
        }
    }

    pub fn visit_mut<V: VisitMut>(&mut self, visitor: &mut V) {
        visitor.visit_typ_mut(&mut self.returned, ObjectUsage::return_type());
        if let Some(params) = &mut self.params {
            visitor.visit_typ_mut(&mut params.typ, ObjectUsage::request_param());
        }
    }

    pub fn param_struct_fields(&self) -> Option<&[StructField]> {
        let params = self.params.as_ref()?;
        match &params.typ.as_rust_object()? {
            RustObject::Struct(struct_) => Some(&struct_.fields),
            _ => None,
        }
    }

    pub fn get_param_field(&self, field: &str) -> Option<&RustType> {
        self.params.as_ref()?.typ.as_rust_object()?.get_struct_field(field)
    }
}

#[derive(Debug, Clone)]
pub struct PathParam {
    pub name: String,
    pub rust_type: RustType,
}

/// A human-authored operation lookup key used by configuration.
///
/// Selectors are intentionally not identities: multiple components may
/// declare operations for the same method and path.
#[derive(Debug, Copy, Clone)]
pub struct OperationSelector {
    pub path: &'static str,
    pub method: OperationType,
}

#[derive(Debug, Clone)]
pub struct OperationRecord {
    pub component: ComponentId,
    pub path: String,
    pub method: OperationType,
}

#[derive(Debug, Clone, Default)]
struct ComponentOperations {
    declarations: Vec<OperationId>,
    requests: Option<Vec<RequestSpec>>,
}

impl ComponentOperations {
    fn requests(&self) -> &[RequestSpec] {
        self.requests.as_deref().expect("component requests were not attached")
    }

    fn requests_mut(&mut self) -> &mut [RequestSpec] {
        self.requests.as_deref_mut().expect("component requests were not attached")
    }
}

#[derive(Debug, Clone)]
pub struct OperationCatalog {
    records: Vec<OperationRecord>,
    by_component: IndexMap<ComponentId, ComponentOperations>,
    by_method_and_path: HashMap<OperationType, HashMap<String, Vec<OperationId>>>,
}

impl OperationCatalog {
    pub fn from_operations<'a>(
        components: impl IntoIterator<Item = (ComponentId, &'a [StripeOperation])>,
    ) -> anyhow::Result<Self> {
        let mut catalog = Self {
            records: Vec::new(),
            by_component: IndexMap::new(),
            by_method_and_path: HashMap::new(),
        };
        for (component, operations) in components {
            ensure!(
                catalog.by_component.insert(component, ComponentOperations::default()).is_none(),
                "component operations were cataloged twice: {component:?}"
            );
            for operation in operations {
                let id = OperationId::from_index(catalog.records.len())?;
                let path = normalize_path(&operation.path).to_string();
                catalog.records.push(OperationRecord {
                    component,
                    path: path.clone(),
                    method: operation.operation_type,
                });
                catalog
                    .by_component
                    .get_mut(&component)
                    .expect("component was inserted before its operations")
                    .declarations
                    .push(id);
                catalog
                    .by_method_and_path
                    .entry(operation.operation_type)
                    .or_default()
                    .entry(path)
                    .or_default()
                    .push(id);
            }
        }
        Ok(catalog)
    }

    #[track_caller]
    pub fn record(&self, id: OperationId) -> &OperationRecord {
        &self.records[id.index()]
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn for_component(&self, component: ComponentId) -> &[OperationId] {
        self.by_component
            .get(&component)
            .unwrap_or_else(|| panic!("component operations were not cataloged: {component:?}"))
            .declarations
            .as_slice()
    }

    pub fn attach_requests(
        &mut self,
        component: ComponentId,
        requests: Vec<RequestSpec>,
    ) -> anyhow::Result<()> {
        let component_operations = self.by_component.get_mut(&component).ok_or_else(|| {
            anyhow::anyhow!("generated requests referenced an unknown component: {component:?}")
        })?;
        let mut seen = HashSet::with_capacity(requests.len());
        for request in &requests {
            let operation = request.operation_id;
            let record = self.records.get(operation.index()).ok_or_else(|| {
                anyhow::anyhow!("generated request referenced unknown operation: {operation:?}")
            })?;
            ensure!(
                record.component == component,
                "generated request owner differed from its operation: {operation:?}"
            );
            ensure!(
                seen.insert(operation),
                "operation generated more than one request: {operation:?}"
            );
        }
        ensure!(
            component_operations.requests.replace(requests).is_none(),
            "component requests were attached twice: {component:?}"
        );
        Ok(())
    }

    pub fn request(&self, operation: OperationId) -> Option<&RequestSpec> {
        let owner = self.record(operation).component;
        self.requests_for_component(owner).iter().find(|request| request.operation_id == operation)
    }

    pub fn requests_for_component(&self, component: ComponentId) -> &[RequestSpec] {
        self.by_component
            .get(&component)
            .map(ComponentOperations::requests)
            .unwrap_or_else(|| panic!("component requests were not attached: {component:?}"))
    }

    pub fn requests_for_component_mut(&mut self, component: ComponentId) -> &mut [RequestSpec] {
        self.by_component
            .get_mut(&component)
            .map(ComponentOperations::requests_mut)
            .unwrap_or_else(|| panic!("component requests were not attached: {component:?}"))
    }

    pub fn requests_mut(&mut self) -> impl Iterator<Item = &mut RequestSpec> {
        self.by_component.values_mut().flat_map(|operations| operations.requests_mut().iter_mut())
    }

    pub fn request_count(&self) -> usize {
        self.by_component.values().map(|operations| operations.requests().len()).sum()
    }

    pub fn candidates(&self, selector: OperationSelector) -> &[OperationId] {
        let candidates: &[OperationId] = self
            .by_method_and_path
            .get(&selector.method)
            .and_then(|paths| paths.get(selector.path))
            .map_or(&[], Vec::as_slice);
        debug_assert!(candidates.iter().all(|id| {
            let record = self.record(*id);
            record.method == selector.method && record.path == selector.path
        }));
        candidates
    }
}

fn normalize_path(path: &str) -> &str {
    path.strip_prefix("/v1").unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::{
        MethodType, OperationCatalog, OperationSelector, OperationType, RequestSpec,
        StripeOperation,
    };
    use crate::component_catalog::{ComponentCatalog, ComponentId};
    use crate::rust_type::RustType;
    use crate::types::{ComponentName, RustIdent};

    fn operation(path: &str) -> StripeOperation {
        StripeOperation {
            method_name: "retrieve".to_string(),
            method_type: MethodType::Retrieve,
            method_on: "service".to_string(),
            operation_type: OperationType::Get,
            path: path.to_string(),
        }
    }

    fn component_id(catalog: &ComponentCatalog, name: &str) -> ComponentId {
        catalog.id(&ComponentName::new(name.to_string())).unwrap()
    }

    fn request(operation_id: super::OperationId, ident: &str) -> RequestSpec {
        RequestSpec {
            operation_id,
            ident: RustIdent::create(ident),
            path_params: Vec::new(),
            params: None,
            method_type: OperationType::Get,
            returned: RustType::string(),
            doc_comment: None,
            req_path: "/accounts/{account}".to_string(),
        }
    }

    #[test]
    fn preserves_declaration_identity_for_duplicate_endpoint_keys() {
        let components = ComponentCatalog::from_names([
            ComponentName::new("account".to_string()),
            ComponentName::new("person".to_string()),
        ])
        .unwrap();
        let account = component_id(&components, "account");
        let person = component_id(&components, "person");
        let account_ops = [operation("/v1/accounts/{account}")];
        let person_ops = [operation("/v1/accounts/{account}")];
        let operations = OperationCatalog::from_operations([
            (account, &account_ops[..]),
            (person, &person_ops[..]),
        ])
        .unwrap();
        let candidates = operations.candidates(OperationSelector {
            path: "/accounts/{account}",
            method: OperationType::Get,
        });

        assert_eq!(candidates.len(), 2);
        assert_ne!(candidates[0], candidates[1]);
        assert_eq!(operations.record(candidates[0]).component, account);
        assert_eq!(operations.record(candidates[1]).component, person);
    }

    #[test]
    fn owns_generated_requests_by_operation_and_component() {
        let components = ComponentCatalog::from_names([
            ComponentName::new("account".to_string()),
            ComponentName::new("person".to_string()),
        ])
        .unwrap();
        let account = component_id(&components, "account");
        let person = component_id(&components, "person");
        let account_ops = [operation("/v1/accounts/{account}")];
        let person_ops = [operation("/v1/accounts/{account}/persons/{person}")];
        let mut operations = OperationCatalog::from_operations([
            (account, &account_ops[..]),
            (person, &person_ops[..]),
        ])
        .unwrap();
        let account_operation = operations.for_component(account)[0];

        operations
            .attach_requests(account, vec![request(account_operation, "RetrieveAccount")])
            .unwrap();
        operations.attach_requests(person, Vec::new()).unwrap();

        assert_eq!(operations.request_count(), 1);
        assert_eq!(operations.requests_for_component(account).len(), 1);
        assert!(operations.requests_for_component(person).is_empty());
        assert_eq!(
            operations.request(account_operation).unwrap().ident.as_ref(),
            "RetrieveAccount"
        );
    }
}
