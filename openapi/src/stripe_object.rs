use std::collections::HashMap;

use heck::ToSnakeCase;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use openapiv3::Schema;
use serde::Deserialize;

use crate::component_catalog::ComponentCatalog;
use crate::deduplication::DeduppedObject;
use crate::operations::StripeOperation;
use crate::rust_object::{ObjectIdentity, ObjectUsage, RustObject};
use crate::spec_inference::Inference;
use crate::types::{ComponentName, RustIdent};
use crate::visitor::{Visit, VisitMut};

#[derive(Debug, Clone)]
pub struct StripeObject {
    path: ComponentName,
    ident: RustIdent,
    obj: RustObject,
    object_name: Option<String>,
    identity: Option<ObjectIdentity>,
    pub stripe_doc_url: Option<String>,
    pub deduplicated_objects: IndexMap<RustIdent, DeduppedObject>,
}

impl StripeObject {
    pub(crate) fn from_schema(
        schema: &Schema,
        resource: &StripeResource,
        catalog: &ComponentCatalog,
    ) -> Self {
        let ParsedStripeObject { obj, object_name, identity } =
            parse_stripe_schema_as_rust_object(schema, &resource.path, resource.ident(), catalog);
        Self {
            path: resource.path.clone(),
            ident: resource.ident().clone(),
            obj,
            object_name,
            identity,
            stripe_doc_url: None,
            deduplicated_objects: IndexMap::default(),
        }
    }

    pub fn mod_path(&self) -> String {
        self.ident.to_snake_case()
    }

    pub fn object_name(&self) -> Option<&str> {
        self.object_name.as_deref()
    }

    pub fn path(&self) -> &ComponentName {
        &self.path
    }

    pub fn ident(&self) -> &RustIdent {
        &self.ident
    }

    pub fn id_type_ident(&self) -> RustIdent {
        RustIdent::create(format!("{}Id", self.ident()))
    }

    pub fn identity(&self) -> Option<&ObjectIdentity> {
        self.identity.as_ref()
    }

    pub fn rust_obj(&self) -> &RustObject {
        &self.obj
    }

    pub fn visit<'a, V: Visit<'a>>(&'a self, visitor: &mut V) {
        visitor.visit_obj(&self.obj, None, ObjectUsage::type_def());
    }

    pub fn visit_mut<V: VisitMut>(&mut self, visitor: &mut V) {
        visitor.visit_obj_mut(&mut self.obj, None, ObjectUsage::type_def());
    }
}

struct ParsedStripeObject {
    obj: RustObject,
    object_name: Option<String>,
    identity: Option<ObjectIdentity>,
}

fn parse_stripe_schema_as_rust_object(
    schema: &Schema,
    path: &ComponentName,
    ident: &RustIdent,
    catalog: &ComponentCatalog,
) -> ParsedStripeObject {
    let not_deleted_path = path.as_not_deleted();
    let id_component = catalog
        .id(&not_deleted_path)
        .unwrap_or_else(|| panic!("id component was missing from catalog: {not_deleted_path}"));
    let infer_ctx = Inference::new(ident, catalog).id_component(id_component).required(true);
    let typ = infer_ctx.infer_schema_type(schema);
    let Some((mut rust_obj, _)) = typ.into_object() else {
        panic!("Unexpected top level schema type for {path}");
    };
    match &mut rust_obj {
        RustObject::Struct(struct_) => {
            let mut id_type = None;
            let mut object_name = None;
            struct_.fields.retain(|field| {
                if field.field_name == "id" && field.rust_type.as_id_or_opt_id_path().is_some() {
                    id_type = Some(field.rust_type.clone());
                }
                if field.field_name == "object"
                    && let Some(RustObject::FieldlessEnum(variants)) =
                        field.rust_type.as_rust_object()
                    && variants.len() == 1
                {
                    let first = variants.first().unwrap();
                    object_name = Some(first.wire_name.clone());

                    // This constant field just helps serialize the constant "object"
                    // key - we don't want it as part of the public API needed to
                    // construct these types
                    return false;
                }
                true
            });
            if let Some(obj_name) = &object_name {
                struct_.object_field = Some(obj_name.clone());
            }
            ParsedStripeObject {
                obj: rust_obj,
                object_name,
                identity: id_type.map(ObjectIdentity::IdType),
            }
        }
        RustObject::Enum(_) => {
            // TODO: could validate that this enum holds what we expect from a top-level component,
            // namely a union of references to other components. We also are implicitly assuming
            // these components have ids
            ParsedStripeObject {
                obj: rust_obj,
                object_name: None,
                identity: Some(ObjectIdentity::ObjectUnion),
            }
        }
        RustObject::FieldlessEnum(_) => panic!("Unexpected top level schema type"),
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
struct BaseResource {
    pub in_package: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StripeResource {
    pub path: ComponentName,
    base_ident: RustIdent,
    pub in_package: Option<String>,
    pub operations: Vec<StripeOperation>,
}

impl StripeResource {
    pub fn ident(&self) -> &RustIdent {
        &self.base_ident
    }

    pub fn from_schema(schema: &Schema, path: ComponentName) -> anyhow::Result<Self> {
        let resource =
            if let Some(stripe_resource) = schema.schema_data.extensions.get("x-stripeResource") {
                serde_json::from_value(stripe_resource.clone())?
            } else {
                BaseResource::default()
            };
        let mut in_package = None;
        if let Some(package) = resource.in_package
            && !package.is_empty()
        {
            in_package = Some(package.to_snake_case());
        }

        let ident = infer_object_ident(&path);
        let operations = if let Some(val) = schema.schema_data.extensions.get("x-stripeOperations")
        {
            serde_json::from_value(val.clone())?
        } else {
            vec![]
        };
        Ok(Self { base_ident: ident, in_package, operations, path })
    }
}

fn infer_object_ident(path: &ComponentName) -> RustIdent {
    lazy_static! {
        static ref OBJECT_RENAMES: HashMap<&'static str, &'static str> = HashMap::from([
            ("invoiceitem", "invoice_item"),
            ("item", "checkout_session_item"),
            ("line_item", "invoice_line_item"),
            ("fee_refund", "application_fee_refund"),
        ]);
    }
    if let Some(renamed) = OBJECT_RENAMES.get(path.as_ref()) {
        RustIdent::create(renamed)
    } else {
        RustIdent::create(path)
    }
}
