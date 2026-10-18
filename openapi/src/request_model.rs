use std::collections::HashMap;

use anyhow::Context as _;

use crate::components::Components;
use crate::item_catalog::{CanonicalTypeRoot, ItemCatalog};
use crate::item_origin::TypeRoot;
use crate::operations::{OperationType, RequestSpec};
use crate::rust_type::RustType;
use crate::types::RustIdent;

#[derive(Debug, Clone)]
pub struct RequestPathParam {
    pub name: String,
    pub typ: RustType,
}

/// Rendering-ready request semantics. All types have passed through item
/// allocation and named-reference lowering; no inference-side type is retained.
#[derive(Debug, Clone)]
pub struct RequestModel {
    pub ident: RustIdent,
    pub path_params: Vec<RequestPathParam>,
    pub params: Option<CanonicalTypeRoot>,
    pub method_type: OperationType,
    pub returned: CanonicalTypeRoot,
    pub doc_comment: Option<String>,
    pub req_path: String,
}

#[derive(Debug, Clone, Default)]
pub struct RequestCatalog {
    by_component: HashMap<crate::component_catalog::ComponentId, Vec<RequestModel>>,
}

impl RequestCatalog {
    pub fn build(components: &Components, items: &ItemCatalog) -> anyhow::Result<Self> {
        let mut catalog = Self::default();
        for &component_id in components.components.keys() {
            let models = components
                .requests_for_component(component_id)
                .iter()
                .map(|request| RequestModel::lower(request, items))
                .collect::<anyhow::Result<Vec<_>>>()?;
            anyhow::ensure!(
                catalog.by_component.insert(component_id, models).is_none(),
                "component requests were lowered twice: {component_id:?}"
            );
        }
        Ok(catalog)
    }

    #[track_caller]
    pub fn for_component(
        &self,
        component: crate::component_catalog::ComponentId,
    ) -> &[RequestModel] {
        self.by_component
            .get(&component)
            .unwrap_or_else(|| panic!("component requests were not lowered: {component:?}"))
    }
}

impl RequestModel {
    fn lower(request: &RequestSpec, items: &ItemCatalog) -> anyhow::Result<Self> {
        let path_params = request
            .path_params
            .iter()
            .enumerate()
            .map(|(index, param)| {
                let type_root =
                    TypeRoot::OperationPathParam { operation: request.operation_id, index };
                let root = items.root(&type_root).with_context(|| {
                    format!("request path parameter root was not lowered: {type_root:?}")
                })?;
                anyhow::ensure!(
                    root.declaration.is_none(),
                    "request path parameter unexpectedly introduced a declaration: {type_root:?}"
                );
                Ok(RequestPathParam { name: param.name.clone(), typ: root.typ.clone() })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let params = request
            .params
            .as_ref()
            .map(|_| lower_root(items, TypeRoot::OperationParam(request.operation_id)))
            .transpose()?;
        let returned = lower_root(items, TypeRoot::OperationReturn(request.operation_id))?;

        Ok(Self {
            ident: request.ident.clone(),
            path_params,
            params,
            method_type: request.method_type,
            returned,
            doc_comment: request.doc_comment.clone(),
            req_path: request.req_path.clone(),
        })
    }
}

fn lower_root(items: &ItemCatalog, root: TypeRoot) -> anyhow::Result<CanonicalTypeRoot> {
    items
        .root(&root)
        .with_context(|| format!("request type root was not lowered: {root:?}"))
        .cloned()
}
