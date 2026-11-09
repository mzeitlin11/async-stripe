use super::*;
use crate::component_catalog::ComponentCatalog;
use crate::components::Components;
use crate::item_catalog::{ItemCatalog, ItemMetadata};
use crate::operations::{OperationCatalog, StripeOperation};
use crate::rust_object::{ObjectUsage, StructField};
use crate::stripe_object::{StripeObject, StripeResource};
use crate::types::ComponentName;

fn fixture() -> (Components, ItemCatalogDraft, [ItemId; 4]) {
    let catalog = ComponentCatalog::from_names(
        ["api_errors", "customer", "other", "family"].map(|name| ComponentName::new(name.into())),
    )
    .unwrap();
    let objects = catalog
        .iter()
        .map(|(component, name)| {
            let schema = if name.as_ref() == "family" {
                serde_json::json!({"oneOf":[
                    {"$ref":"#/components/schemas/customer"},
                    {"$ref":"#/components/schemas/other"}
                ]})
            } else {
                serde_json::json!({"type":"object","properties":{
                "id":{"type":"string","description":"Unique identifier for the object."},
                "object":{"type":"string","enum":[name.as_ref()]}
            },"required":["id","object"]})
            };
            let schema = serde_json::from_value(schema).unwrap();
            let resource = StripeResource::from_schema(&schema, name.clone()).unwrap();
            (component, StripeObject::from_schema(&schema, &resource, &catalog))
        })
        .collect();
    let mut operations = OperationCatalog::from_operations(
        catalog.iter().map(|(component, _)| (component, &[] as &[StripeOperation])),
    )
    .unwrap();
    for (component, _) in catalog.iter() {
        operations.attach_requests(component, vec![]).unwrap();
    }
    let components = Components::for_test(catalog, objects, operations);
    let mut draft = ItemCatalogDraft::build(&components, |component| {
        ItemMetadata::new(components.get_by_id(component).ident().clone())
    })
    .unwrap();
    let roots = ["api_errors", "customer", "other", "family"].map(|name| {
        let component =
            components.components.iter().find(|(_, object)| object.mod_path() == name).unwrap().0;
        draft.catalog.component_root(*component)
    });
    let [owner, customer, other, family] = roots;
    let RustItem::Object(object) = &mut draft.catalog.items[owner.index()].item else { panic!() };
    let RustObject::Struct(record) = &mut object.definition else { panic!() };
    record.fields.extend([
        StructField::new(
            "expanded",
            RustType::vec(RustType::option(RustType::expandable(RustType::Item(customer)))),
        ),
        StructField::new("expanded_family", RustType::expandable(RustType::Item(family))),
        StructField::new("source", RustType::Item(family)),
        StructField::new("card", RustType::option(RustType::Item(other))),
        StructField::new(
            "payment_intent",
            RustType::option(RustType::boxed(RustType::Item(other))),
        ),
    ]);
    object.usage = ObjectUsage::type_def();
    (components, draft, roots)
}

fn fields(items: &ItemCatalog, owner: ItemId) -> Vec<StructField> {
    let RustObject::Struct(record) = &items.get(owner).object().unwrap().definition else {
        panic!()
    };
    record.fields.clone()
}

#[test]
fn only_expandable_storage_changes_and_canonical_identity_is_preserved() {
    let (components, mut draft, [owner, customer, other, family]) = fixture();
    let before_fields = fields(&draft.catalog, owner);
    let before = draft
        .catalog
        .iter()
        .map(|(item, record)| {
            (
                item,
                record.origin.clone(),
                record.metadata.clone(),
                record.children.clone(),
                record.occurrences.clone(),
            )
        })
        .collect::<Vec<_>>();
    let union = draft.catalog.get(family).object().unwrap().definition.clone();
    let customer_id = draft
        .catalog
        .iter()
        .find(|(_, record)| record.metadata.ident.to_string() == "CustomerId")
        .unwrap()
        .0;
    draft.use_lazy_expandables().unwrap();
    let items = draft.finish(&components).unwrap();
    let after = items
        .iter()
        .map(|(item, record)| {
            (
                item,
                record.origin.clone(),
                record.metadata.clone(),
                record.children.clone(),
                record.occurrences.clone(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(before, after);
    let after_fields = fields(&items, owner);
    assert_eq!(
        after_fields.iter().find(|field| field.field_name == "expanded").unwrap().rust_type,
        RustType::vec(RustType::option(RustType::Container(Container::LazyExpandable(Box::new(
            RustType::Item(customer_id)
        )))))
    );
    assert_eq!(
        after_fields.iter().find(|field| field.field_name == "expanded_family").unwrap().rust_type,
        RustType::Container(Container::LazyExpandable(Box::new(RustType::External(
            ExternalType::SmolStr
        ))))
    );
    for name in ["id", "source", "card", "payment_intent"] {
        assert_eq!(
            after_fields.iter().find(|field| field.field_name == name),
            before_fields.iter().find(|field| field.field_name == name)
        );
    }
    assert_eq!(items.get(family).object().unwrap().definition, union);
    assert!(matches!(
        items.get(family).object().unwrap().enum_of_objects,
        Some(EnumOfObjects::ObjectUnion(_))
    ));
    let mut uses = Vec::new();
    items
        .get(owner)
        .visit_type_uses(|typ| {
            if let Some(item) = typ.item_id() {
                uses.push(item);
            }
            Ok(())
        })
        .unwrap();
    assert!(uses.contains(&customer_id));
    assert!(!uses.contains(&customer));
    assert!(uses.contains(&other));
    assert!(uses.contains(&family));
}

#[test]
fn optional_target_identity_survives_lowering() {
    let (components, mut draft, [_, customer, _, _]) = fixture();
    let RustItem::Object(object) = &mut draft.catalog.items[customer.index()].item else {
        panic!()
    };
    let ObjectIdentity::IdType(id) = object.identity.as_mut().unwrap() else { panic!() };
    *id = RustType::option(id.clone());
    let RustObject::Struct(record) = &mut object.definition else { panic!() };
    record.fields.iter_mut().find(|field| field.field_name == "id").unwrap().rust_type = id.clone();
    // This member's optional ID makes the fixture's ordinary family invalid;
    // examine the transform directly without finalizing that unrelated union.
    draft.use_lazy_expandables().unwrap();
    let owner = draft
        .catalog
        .iter()
        .find(|(_, record)| record.metadata.ident.to_string() == "ApiErrors")
        .unwrap()
        .0;
    let owner_fields = fields(&draft.catalog, owner);
    let represented =
        &owner_fields.iter().find(|field| field.field_name == "expanded").unwrap().rust_type;
    let RustType::Container(Container::Vec(outer)) = represented else { panic!() };
    let RustType::Container(Container::Option(inner)) = &**outer else { panic!() };
    let RustType::Container(Container::LazyExpandable(id)) = &**inner else { panic!() };
    assert!(id.is_option());
    // Parsed schema metadata is unchanged by the representation transform.
    let (&customer_component, _) =
        components.components.iter().find(|(_, object)| object.mod_path() == "customer").unwrap();
    assert!(
        !components
            .get_by_id(customer_component)
            .identity()
            .unwrap()
            .id_type()
            .unwrap()
            .is_option()
    );
}
