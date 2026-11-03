use std::collections::HashMap;

use anyhow::Context as _;

use crate::components::Components;
use crate::item_catalog::{CanonicalTypeRoot, ItemCatalog};
use crate::item_origin::TypeRoot;
use crate::operations::{BodyEncoding, OperationType, RequestSpec};
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
    pub body_encoding: BodyEncoding,
    pub api_version: Option<String>,
    pub pagination_item: Option<RustType>,
}

#[derive(Debug, Clone, Default)]
pub struct RequestCatalog {
    by_component: HashMap<crate::component_catalog::ComponentId, Vec<RequestModel>>,
}

impl RequestCatalog {
    pub(crate) fn iter(
        &self,
    ) -> impl Iterator<Item = (crate::component_catalog::ComponentId, &[RequestModel])> {
        self.by_component.iter().map(|(&component, requests)| (component, requests.as_slice()))
    }

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
    /// Visit signature types and the parameter fields named directly by the
    /// generated constructor/setters, even when their declaration is elsewhere.
    pub(crate) fn visit_type_uses(
        &self,
        items: &ItemCatalog,
        mut visit: impl FnMut(&RustType) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        for typ in self
            .path_params
            .iter()
            .map(|param| &param.typ)
            .chain(self.params.iter().map(|params| &params.typ))
            .chain(std::iter::once(&self.returned.typ))
            .chain(self.pagination_item.iter())
        {
            visit(typ)?;
        }
        if let Some(params) = &self.params
            && let Some(crate::rust_object::RustObject::Struct(strukt)) =
                items.object_for_type(&params.typ)
        {
            for field in &strukt.fields {
                visit(&field.rust_type).with_context(|| {
                    format!("invalid constructor dependencies for request {}", self.ident)
                })?;
            }
        }
        Ok(())
    }

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

        let pagination_item = if request.url_pagination {
            let object = items
                .object_for_type(&returned.typ)
                .context("v2 list response must be a struct")?;
            let data = object.get_struct_field("data").context("v2 list response has no data")?;
            for name in ["next_page_url", "previous_page_url"] {
                anyhow::ensure!(
                    object.get_struct_field(name) == Some(&RustType::option(RustType::string())),
                    "v2 list {name} must be a nullable string"
                );
            }
            let RustType::Container(crate::rust_type::Container::Vec(item)) = data else {
                anyhow::bail!("v2 list data must be an array");
            };
            Some((**item).clone())
        } else {
            None
        };
        Ok(Self {
            ident: request.ident.clone(),
            path_params,
            params,
            method_type: request.method_type,
            returned,
            doc_comment: request.doc_comment.clone(),
            req_path: request.req_path.clone(),
            body_encoding: request.body_encoding,
            api_version: request.api_version.clone(),
            pagination_item,
        })
    }
}

fn lower_root(items: &ItemCatalog, root: TypeRoot) -> anyhow::Result<CanonicalTypeRoot> {
    items
        .root(&root)
        .with_context(|| format!("request type root was not lowered: {root:?}"))
        .cloned()
}
