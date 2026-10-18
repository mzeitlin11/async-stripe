use anyhow::{Context, ensure};
use indexmap::IndexMap;

use crate::components::{Components, RequestSource};
use crate::operations::OperationSelector;
use crate::operations::OperationType;
use crate::rust_object::{ObjectMetadata, ObjectUsage, RustObject};
use crate::rust_type::{DeclarationRoot, RustType};
use crate::types::RustIdent;
use crate::visitor::VisitMut;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OverrideMetadata {
    pub metadata: ObjectMetadata,
    pub mod_path: String,
}

pub struct Overrides {
    pub overrides: IndexMap<RustObject, OverrideMetadata>,
}

impl Overrides {
    pub fn new(components: &Components) -> anyhow::Result<Self> {
        let mut overrides = IndexMap::new();
        let mut identifiers = IndexMap::new();
        let mut modules = IndexMap::new();
        for override_ in OVERRIDES {
            if let Some(previous) = identifiers.insert(override_.ident, override_.reason) {
                anyhow::bail!(
                    "override output identifier `{}` is duplicated (reasons: `{previous}` and `{}`)",
                    override_.ident,
                    override_.reason
                );
            }
            if let Some(previous) = modules.insert(override_.mod_path, override_.reason) {
                anyhow::bail!(
                    "override output module `{}` is duplicated (reasons: `{previous}` and `{}`)",
                    override_.mod_path,
                    override_.reason
                );
            }
            let (obj, meta) = get_override_object(components, override_).with_context(|| {
                format!(
                    "failed to construct override for source {override_:?}; reason: {}",
                    override_.reason
                )
            })?;
            ensure!(
                overrides.insert(obj, meta).is_none(),
                "multiple overrides extract the same Rust object shape"
            );
        }
        Ok(Self { overrides })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OverrideData {
    pub doc: &'static str,
    pub mod_path: &'static str,
    pub ident: &'static str,
    pub source: RequestSource,
    pub reason: &'static str,
}

const OVERRIDES: &[OverrideData] = &[OverrideData {
    doc: "",
    mod_path: "api_version",
    ident: "ApiVersion",
    source: RequestSource {
        operation: OperationSelector { path: "/webhook_endpoints", method: OperationType::Post },
        field_name: "api_version",
    },
    reason: "API versions form a shared public type rather than an anonymous request enum",
}];

fn get_override_object(
    components: &Components,
    data: &OverrideData,
) -> anyhow::Result<(RustObject, OverrideMetadata)> {
    let operation = components.resolve_operation(data.source.operation)?;
    let req = components
        .get_request_spec(operation)
        .context("resolved operation did not have a generated request")?;
    let available_fields = req
        .param_struct_fields()
        .context("selected request does not have struct parameters")?
        .iter()
        .map(|field| field.field_name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let (obj, _) = req
        .get_param_field(data.source.field_name)
        .with_context(|| {
            format!(
                "request parameter field `{}` was not found; available fields: {available_fields}",
                data.source.field_name
            )
        })?
        .extract_object()
        .with_context(|| {
            format!("request parameter field `{}` is not an object", data.source.field_name)
        })?;
    let metadata = ObjectMetadata::new(RustIdent::unchanged(data.ident)).doc(data.doc.to_string());

    Ok((obj.clone(), OverrideMetadata { metadata, mod_path: data.mod_path.to_string() }))
}

impl VisitMut for Overrides {
    fn visit_typ_mut(&mut self, typ: &mut RustType, usage: ObjectUsage) {
        if let Some((obj, _)) = typ.as_object_mut()
            && let Some(meta) = self.overrides.get(obj)
        {
            *typ = RustType::path(DeclarationRoot::Shared(meta.metadata.ident.clone()));
        }
        typ.visit_mut(self, usage);
    }
}
