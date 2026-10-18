use indexmap::IndexMap;
use tracing::debug;

use crate::component_catalog::ComponentId;
use crate::item_origin::{ItemOccurrence, ItemOrigin, TypeRoot};
use crate::operations::RequestSpec;
use crate::rust_object::{
    EnumVariant, ObjectKind, ObjectMetadata, ObjectUsage, RustObject, StructField,
};
use crate::rust_type::{DeclarationRoot, RustType};
use crate::stripe_object::StripeObject;
use crate::types::RustIdent;
use crate::visitor::{Visit, VisitMut};

#[derive(Debug)]
struct ObjectOccurrence {
    metadata: ObjectMetadata,
    source: ItemOccurrence,
}

#[derive(Debug)]
struct CollectDuplicateObjects {
    // For fieldless enums this compares only the variants, independently of
    // names, documentation, and usage. For composite objects, retain the
    // existing conservative equality (including nested names and field docs)
    // until their presentation and sharing rules are specified separately.
    objs: IndexMap<RustObject, Vec<ObjectOccurrence>>,
    origin: ItemOrigin,
}

impl CollectDuplicateObjects {
    fn new(component: ComponentId) -> Self {
        Self {
            objs: IndexMap::new(),
            origin: ItemOrigin {
                type_root: TypeRoot::Declaration(DeclarationRoot::Component(component)),
                path: Vec::new(),
            },
        }
    }

    fn collect_request(&mut self, request: &RequestSpec) {
        debug_assert!(self.origin.path.is_empty());
        self.origin.type_root = TypeRoot::OperationReturn(request.operation_id);
        self.visit_typ(&request.returned, ObjectUsage::return_type());
        if let Some(params) = &request.params {
            self.origin.type_root = TypeRoot::OperationParam(request.operation_id);
            self.visit_typ(&params.typ, ObjectUsage::request_param());
        }
    }
}

impl Visit<'_> for CollectDuplicateObjects {
    fn visit_obj(&mut self, obj: &RustObject, meta: Option<&ObjectMetadata>, usage: ObjectUsage) {
        if let Some(meta) = meta {
            self.objs.entry(obj.clone()).or_default().push(ObjectOccurrence {
                metadata: meta.clone(),
                source: ItemOccurrence { origin: self.origin.clone(), usage },
            });
        };
        obj.visit(self, usage);
    }

    fn visit_field(&mut self, field: &StructField, usage: ObjectUsage) {
        self.origin.path.push(field.field_name.clone());
        self.visit_typ(&field.rust_type, usage);
        self.origin.path.pop();
    }

    fn visit_variant(&mut self, variant: &EnumVariant, usage: ObjectUsage) {
        if let Some(typ) = &variant.rust_type {
            self.origin.path.push(variant.variant.to_string());
            self.visit_typ(typ, usage);
            self.origin.path.pop();
        }
    }
}

/// The existing component-local naming cues that permit declaration sharing.
/// Structural equality alone never establishes shared public identity.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SharingRule {
    ModelFieldEnum,
    CommonTitle,
    CommonDescription,
}

#[derive(Debug)]
struct SharingDecision {
    ident: RustIdent,
    usage: ObjectUsage,
    rule: SharingRule,
    occurrences: Vec<ItemOccurrence>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DeduppedObject {
    pub usage: ObjectUsage,
    pub object: RustObject,
    pub occurrences: Vec<ItemOccurrence>,
    pub sharing_rule: SharingRule,
}

#[derive(Debug)]
struct DeduplicateObjects {
    objs: IndexMap<RustObject, SharingDecision>,
    component: ComponentId,
}

impl DeduplicateObjects {
    pub fn new(component: ComponentId) -> Self {
        Self { objs: Default::default(), component }
    }
}

impl VisitMut for DeduplicateObjects {
    fn visit_typ_mut(&mut self, typ: &mut RustType, usage: ObjectUsage)
    where
        Self: Sized,
    {
        if let Some((obj, _)) = typ.as_object_mut()
            && let Some(dedup_spec) = self.objs.get(obj)
        {
            *typ = RustType::Path(DeclarationRoot::Deduplicated {
                component: self.component,
                ident: dedup_spec.ident.clone(),
            })
        }
        typ.visit_mut(self, usage);
    }
}

fn implied_name_from_meta_doc(metadata: &ObjectMetadata) -> Option<&str> {
    let doc = metadata.doc.as_ref()?;
    implied_name_from_doc(doc)
}

fn implied_name_from_doc(doc: &str) -> Option<&str> {
    let mut words = doc.split_ascii_whitespace();
    if words.next() != Some("The") {
        return None;
    }
    let second_word = words.next();
    if words.next() != Some("of") {
        return None;
    }
    second_word
}

fn model_field_enum_name(occurrences: &[ObjectOccurrence]) -> Option<RustIdent> {
    let type_data = &occurrences
        .iter()
        .find(|occurrence| occurrence.source.usage.kind == ObjectKind::Type)?
        .metadata;
    let parent = type_data.parent.as_ref()?;
    let field_name = type_data.field_name.as_ref()?;

    if occurrences.iter().any(|o| o.metadata.field_name.as_ref() != Some(field_name)) {
        return None;
    }
    Some(RustIdent::joined(parent, field_name))
}

/// Select a shared public name using the existing component-local identity rules.
fn shared_name(
    object: &RustObject,
    occurrences: &[ObjectOccurrence],
) -> Option<(RustIdent, SharingRule)> {
    if matches!(object, RustObject::FieldlessEnum(_))
        && let Some(ident) = model_field_enum_name(occurrences)
    {
        return Some((ident, SharingRule::ModelFieldEnum));
    }
    let first_meta = &occurrences.first()?.metadata;
    if let Some(title) = &first_meta.title {
        // `param` is used very generally and will not be a helpful name to infer
        if title != "param" && occurrences.iter().all(|o| o.metadata.title.as_ref() == Some(title))
        {
            return Some((RustIdent::create(title), SharingRule::CommonTitle));
        }
    }

    if let Some(doc_name) = implied_name_from_meta_doc(first_meta)
        && let Some(parent) = &first_meta.parent
        && occurrences.iter().all(|o| {
            implied_name_from_meta_doc(&o.metadata) == Some(doc_name)
                && o.metadata.parent.as_ref() == Some(parent)
        })
    {
        return Some((RustIdent::joined(parent, doc_name), SharingRule::CommonDescription));
    }
    None
}

fn shared_usage(occurrences: &[ObjectOccurrence], rule: SharingRule) -> Option<ObjectUsage> {
    if rule == SharingRule::ModelFieldEnum {
        // Fieldless enums have no borrowing or field-level serde behavior to
        // reconcile. A model enum can also serialize as a request parameter.
        return Some(ObjectUsage {
            kind: ObjectKind::Type,
            used_as_request_param: occurrences.iter().any(|o| o.source.usage.used_as_request_param),
        });
    }

    // For composite objects, context affects field serialization, constructors,
    // and borrowing. Keep those declarations separate unless usage is identical.
    let first_usage = occurrences.first()?.source.usage;
    occurrences.iter().all(|o| o.source.usage == first_usage).then_some(first_usage)
}

fn choose_sharing(
    object: &RustObject,
    occurrences: &[ObjectOccurrence],
) -> Option<SharingDecision> {
    if occurrences.len() < 2 {
        return None;
    }
    let (ident, rule) = shared_name(object, occurrences)?;
    let usage = shared_usage(occurrences, rule)?;
    Some(SharingDecision {
        ident,
        usage,
        rule,
        occurrences: occurrences.iter().map(|o| o.source.clone()).collect(),
    })
}

#[tracing::instrument(level = "debug", skip(comp), fields(path = %comp.path()))]
pub fn deduplicate_types(
    comp: &mut StripeObject,
    requests: &mut [RequestSpec],
    component: ComponentId,
) -> IndexMap<RustIdent, DeduppedObject> {
    let mut objs = IndexMap::new();

    // We run deduplication passes until there are no further changes since one round
    // of deduplicating can enable another
    loop {
        let mut collector = CollectDuplicateObjects::new(component);
        comp.visit(&mut collector);
        for request in requests.iter() {
            collector.collect_request(request);
        }

        let mut dedupper = DeduplicateObjects::new(component);
        for (obj, occurrences) in collector.objs {
            if let Some(inferred) = choose_sharing(&obj, &occurrences) {
                let ident = &inferred.ident;
                // Don't add another deduplicated type with the same name as an existing one
                if dedupper.objs.values().all(|o| &o.ident != ident) && !objs.contains_key(ident) {
                    dedupper.objs.insert(obj, inferred);
                }
            }
        }
        // If we weren't able to deduplicate anything new, we're done
        if dedupper.objs.is_empty() {
            break;
        }

        comp.visit_mut(&mut dedupper);
        for request in requests.iter_mut() {
            request.visit_mut(&mut dedupper);
        }
        for (obj, info) in dedupper.objs {
            let ident = info.ident.clone();
            let usage = info.usage;
            let shared = DeduppedObject {
                usage,
                object: obj,
                occurrences: info.occurrences,
                sharing_rule: info.rule,
            };
            if objs.insert(ident.clone(), shared).is_some() {
                panic!("Tried to add duplicate ident {ident}");
            }
        }
    }
    if !objs.is_empty() {
        debug!("Deduplicated {} types", objs.len());
    }
    objs
}

#[cfg(test)]
mod tests {
    use super::{
        CollectDuplicateObjects, SharingRule, choose_sharing, deduplicate_types, shared_name,
    };
    use crate::component_catalog::{ComponentCatalog, ComponentId};
    use crate::item_origin::{ItemOrigin, TypeRoot};
    use crate::operations::{
        MethodType, OperationCatalog, OperationId, OperationType, RequestParam, RequestSpec,
        StripeOperation,
    };
    use crate::rust_object::{
        EnumVariant, FieldlessVariant, ObjectKind, ObjectMetadata, ObjectUsage, RustObject, Struct,
        StructField,
    };
    use crate::rust_type::{DeclarationRoot, RustType};
    use crate::stripe_object::{StripeObject, StripeResource};
    use crate::types::{ComponentName, RustIdent};
    use crate::visitor::Visit;

    fn catalog() -> ComponentCatalog {
        ComponentCatalog::from_names([
            ComponentName::new("customer".to_string()),
            ComponentName::new("account".to_string()),
        ])
        .unwrap()
    }

    fn operation(component: ComponentId) -> OperationId {
        let operation = StripeOperation {
            method_name: "update".to_string(),
            method_type: MethodType::Update,
            method_on: "service".to_string(),
            operation_type: OperationType::Post,
            path: "/v1/customers/{customer}".to_string(),
        };
        let operations =
            OperationCatalog::from_operations([(component, std::slice::from_ref(&operation))])
                .unwrap();
        operations.for_component(component)[0]
    }

    fn status() -> RustObject {
        RustObject::FieldlessEnum(
            ["active", "inactive"]
                .into_iter()
                .map(|name| FieldlessVariant::new(name.to_string(), RustIdent::create(name)))
                .collect(),
        )
    }

    fn metadata(ident: &str, parent: &str, field: &str) -> ObjectMetadata {
        let mut metadata = ObjectMetadata::new(RustIdent::create(ident));
        metadata.parent = Some(RustIdent::create(parent));
        metadata.field_name = Some(field.to_string());
        metadata
    }

    #[test]
    fn shares_model_and_request_enums_and_retains_every_usage() {
        let catalog = catalog();
        let component = catalog.id_str("customer").unwrap();
        let operation = operation(component);
        let schema = serde_json::from_value(serde_json::json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string", "enum": ["active", "inactive"],
                    "description": "Status on the stored model."
                }
            },
            "required": ["status"]
        }))
        .unwrap();
        let resource =
            StripeResource::from_schema(&schema, ComponentName::new("customer".to_string()))
                .unwrap();
        let mut model = StripeObject::from_schema(&schema, &resource, &catalog);
        let request_status = RustType::Object(
            status(),
            metadata("update_customer_status", "update_customer", "status")
                .doc("Status supplied to the update endpoint.".to_string()),
        );
        let returned_status = RustType::Object(
            status(),
            metadata("returned_status", "retrieve_customer", "status")
                .doc("Status returned by the endpoint.".to_string()),
        );
        let mut requests = [RequestSpec {
            operation_id: operation,
            ident: RustIdent::create("update_customer"),
            path_params: Vec::new(),
            params: Some(RequestParam {
                typ: RustType::Object(
                    RustObject::Struct(Struct::new(vec![StructField::new(
                        "status",
                        RustType::option(request_status),
                    )])),
                    ObjectMetadata::new(RustIdent::create("update_customer")),
                ),
            }),
            returned: returned_status,
            method_type: OperationType::Post,
            doc_comment: None,
            req_path: "/customers/{customer}".to_string(),
        }];

        let declarations = deduplicate_types(&mut model, &mut requests, component);

        assert_eq!(declarations.len(), 1);
        let (ident, shared) = declarations.first().unwrap();
        assert_eq!(ident, &RustIdent::create("customer_status"));
        assert_eq!(shared.sharing_rule, SharingRule::ModelFieldEnum);
        assert_eq!(shared.usage.kind, ObjectKind::Type);
        assert!(shared.usage.should_impl_serialize());
        assert!(shared.usage.should_impl_deserialize());
        let reference =
            RustType::path(DeclarationRoot::Deduplicated { component, ident: ident.clone() });
        assert_eq!(model.rust_obj().get_struct_field("status"), Some(&reference));
        assert_eq!(requests[0].returned, reference);
        assert_eq!(
            requests[0].get_param_field("status").unwrap().with_option_stripped(),
            &reference
        );
        let sources = shared
            .occurrences
            .iter()
            .map(|source| (source.origin.clone(), source.usage))
            .collect::<Vec<_>>();
        assert_eq!(
            sources,
            vec![
                (
                    ItemOrigin {
                        type_root: TypeRoot::Declaration(DeclarationRoot::Component(component)),
                        path: vec!["status".to_string()],
                    },
                    ObjectUsage::type_def()
                ),
                (
                    ItemOrigin {
                        type_root: TypeRoot::OperationReturn(operation),
                        path: Vec::new(),
                    },
                    ObjectUsage::return_type()
                ),
                (
                    ItemOrigin {
                        type_root: TypeRoot::OperationParam(operation),
                        path: vec!["status".to_string()],
                    },
                    ObjectUsage::request_param()
                ),
            ]
        );
    }

    #[test]
    fn keeps_structs_with_different_usage_separate_even_with_a_common_name() {
        let catalog = catalog();
        let component = catalog.id_str("customer").unwrap();
        let mut collector = CollectDuplicateObjects::new(component);
        let object = RustObject::Struct(Struct::new(vec![StructField::new(
            "value",
            RustType::option(RustType::string()),
        )]));
        let mut meta = ObjectMetadata::new(RustIdent::create("details"));
        meta.title = Some("details".to_string());
        collector.visit_obj(&object, Some(&meta), ObjectUsage::type_def());
        collector.origin.type_root = TypeRoot::OperationParam(operation(component));
        collector.visit_obj(&object, Some(&meta), ObjectUsage::request_param());
        let (object, occurrences) = collector.objs.first().unwrap();

        assert_eq!(shared_name(object, occurrences).unwrap().1, SharingRule::CommonTitle);
        assert!(choose_sharing(object, occurrences).is_none());
    }

    #[test]
    fn shares_composite_objects_only_with_matching_usage_and_an_identity_cue() {
        let catalog = catalog();
        let component = catalog.id_str("customer").unwrap();
        let object = RustObject::Struct(Struct::new(vec![StructField::new(
            "value",
            RustType::option(RustType::string()),
        )]));
        let mut meta = ObjectMetadata::new(RustIdent::create("details"));
        meta.title = Some("details".to_string());
        let mut collector = CollectDuplicateObjects::new(component);
        for path in ["first", "second"] {
            collector.origin.path = vec![path.to_string()];
            collector.visit_obj(&object, Some(&meta), ObjectUsage::request_param());
        }
        let (object, occurrences) = collector.objs.first().unwrap();
        let decision = choose_sharing(object, occurrences).unwrap();

        assert_eq!(decision.rule, SharingRule::CommonTitle);
        assert_eq!(decision.usage, ObjectUsage::request_param());
        assert_eq!(decision.occurrences.len(), 2);
    }

    #[test]
    fn equal_enum_values_do_not_imply_shared_public_identity() {
        let catalog = catalog();
        let component = catalog.id_str("customer").unwrap();
        let mut collector = CollectDuplicateObjects::new(component);
        let object = status();
        for field in ["status", "phase"] {
            let meta = metadata(field, "customer", field);
            collector.visit_obj(&object, Some(&meta), ObjectUsage::type_def());
        }
        let (object, occurrences) = collector.objs.first().unwrap();

        assert!(choose_sharing(object, occurrences).is_none());
    }

    #[test]
    fn enum_usage_union_requires_the_model_field_rule() {
        let catalog = catalog();
        let component = catalog.id_str("customer").unwrap();
        let mut collector = CollectDuplicateObjects::new(component);
        let object = status();
        let mut meta = ObjectMetadata::new(RustIdent::create("status"));
        meta.title = Some("status".to_string());
        for usage in [ObjectUsage::request_param(), ObjectUsage::return_type()] {
            collector.visit_obj(&object, Some(&meta), usage);
        }
        let (object, occurrences) = collector.objs.first().unwrap();

        assert!(choose_sharing(object, occurrences).is_none());
    }

    #[test]
    fn nominal_references_remain_part_of_definition_equality() {
        let catalog = catalog();
        let component = catalog.id_str("customer").unwrap();
        let mut collector = CollectDuplicateObjects::new(component);
        let mut meta = ObjectMetadata::new(RustIdent::create("owner"));
        meta.title = Some("owner".to_string());
        for name in ["customer", "account"] {
            let object = RustObject::Struct(Struct::new(vec![StructField::new(
                "id",
                RustType::object_id(catalog.id_str(name).unwrap()),
            )]));
            collector.visit_obj(&object, Some(&meta), ObjectUsage::type_def());
        }

        assert_eq!(collector.objs.len(), 2);
        assert!(
            collector
                .objs
                .iter()
                .all(|(object, occurrences)| { choose_sharing(object, occurrences).is_none() })
        );
    }

    #[test]
    fn records_variant_and_field_paths_through_containers() {
        let catalog = catalog();
        let component = catalog.id_str("customer").unwrap();
        let object = RustObject::Enum(vec![EnumVariant::new(
            RustIdent::create("payload"),
            RustType::Object(
                RustObject::Struct(Struct::new(vec![StructField::new(
                    "statuses",
                    RustType::option(RustType::vec(RustType::Object(
                        status(),
                        metadata("status", "customer", "statuses"),
                    ))),
                )])),
                ObjectMetadata::new(RustIdent::create("payload")),
            ),
        )]);
        let mut collector = CollectDuplicateObjects::new(component);
        collector.visit_obj(&object, None, ObjectUsage::type_def());

        assert_eq!(collector.objs[&status()][0].source.origin.path, ["Payload", "statuses"]);
        assert!(collector.origin.path.is_empty());
    }

    #[test]
    fn retains_conservative_matching_for_composite_field_documentation() {
        let catalog = catalog();
        let component = catalog.id_str("customer").unwrap();
        let mut collector = CollectDuplicateObjects::new(component);
        let mut meta = ObjectMetadata::new(RustIdent::create("details"));
        meta.title = Some("details".to_string());
        for doc in ["Description from the model.", "Description from the request."] {
            let object = RustObject::Struct(Struct::new(vec![
                StructField::new("value", RustType::string()).doc(doc),
            ]));
            collector.visit_obj(&object, Some(&meta), ObjectUsage::type_def());
        }

        assert_eq!(collector.objs.len(), 2);
    }
}
