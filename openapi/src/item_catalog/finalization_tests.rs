use super::{CatalogBuilder, ItemCatalog, ItemCatalogDraft, ItemId, ItemMetadata};
use crate::component_catalog::{ComponentCatalog, ComponentId};
use crate::components::Components;
use crate::item_origin::TypeRoot;
use crate::operations::{OperationCatalog, StripeOperation};
use crate::rust_object::{EnumOfObjects, EnumVariant, ObjectMetadata, ObjectUsage, RustObject};
use crate::rust_type::{DeclarationRoot, RustType};
use crate::stripe_object::{StripeObject, StripeResource};
use crate::types::{ComponentName, RustIdent};
use crate::webhook::WebhookObject;

fn empty_components() -> Components {
    Components::for_test(
        ComponentCatalog::from_names(std::iter::empty::<ComponentName>()).unwrap(),
        Default::default(),
        OperationCatalog::from_operations(std::iter::empty::<(ComponentId, &[StripeOperation])>())
            .unwrap(),
    )
}

fn finalize_object(
    object: RustObject,
    usage: ObjectUsage,
) -> anyhow::Result<(ItemCatalog, ItemId)> {
    let components = empty_components();
    let mut builder = CatalogBuilder::default();
    let item = builder.collect_root_object(
        TypeRoot::Declaration(DeclarationRoot::Shared(RustIdent::create("probe"))),
        &object,
        ItemMetadata::new(RustIdent::create("probe")),
        usage,
    )?;
    let mut catalog = builder.finish()?;
    catalog.lower_named_references()?;
    catalog.classify_enums(&components);
    let items = ItemCatalogDraft { catalog }.finish(&components)?;
    Ok((items, item))
}

#[test]
fn unsupported_response_enums_fail_during_finalization() {
    let object =
        RustObject::Enum(vec![EnumVariant::new(RustIdent::create("raw"), RustType::string())]);
    let error = finalize_object(object.clone(), ObjectUsage::return_type()).unwrap_err();
    assert!(error.to_string().contains("not a supported object union or live/deleted pair"));
    finalize_object(object, ObjectUsage::request_param()).unwrap();
}

fn object_union_draft() -> (Components, ItemCatalogDraft, ItemId, ComponentId) {
    let catalog = ComponentCatalog::from_names(
        ["customer", "charge"].map(|name| ComponentName::new(name.to_owned())),
    )
    .unwrap();
    let customer = catalog.id_str("customer").unwrap();
    let charge = catalog.id_str("charge").unwrap();
    let objects = [customer, charge]
        .into_iter()
        .map(|component| {
            let name = catalog.name(component);
            let schema = serde_json::from_value(serde_json::json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "Unique identifier for the object."},
                    "object": {"type": "string", "enum": [name.as_ref()]}
                },
                "required": ["id", "object"]
            }))
            .unwrap();
            let resource = StripeResource::from_schema(&schema, name.clone()).unwrap();
            (component, StripeObject::from_schema(&schema, &resource, &catalog))
        })
        .collect();
    let mut operations = OperationCatalog::from_operations(
        [customer, charge].map(|component| (component, &[] as &[StripeOperation])),
    )
    .unwrap();
    for component in [customer, charge] {
        operations.attach_requests(component, vec![]).unwrap();
    }
    let mut components = Components::for_test(catalog, objects, operations);
    components.webhook_objs.push(WebhookObject {
        event_type: "test.created".to_owned(),
        doc: String::new(),
        typ: RustType::Object(
            RustObject::Enum(vec![
                EnumVariant::new(RustIdent::create("customer"), RustType::component(customer)),
                EnumVariant::new(RustIdent::create("charge"), RustType::component(charge)),
            ]),
            ObjectMetadata::new(RustIdent::create("test_created")),
        ),
    });
    let draft = ItemCatalogDraft::build(&components, |component| {
        ItemMetadata::new(components.get_by_id(component).ident().clone())
    })
    .unwrap();
    let union = draft
        .catalog
        .root(&TypeRoot::Webhook("test.created".to_owned()))
        .unwrap()
        .declaration
        .unwrap();
    (components, draft, union, customer)
}

#[test]
fn object_enums_reject_request_usage_and_borrowed_payloads() {
    for borrowed in [false, true] {
        let (components, mut draft, union, _) = object_union_draft();
        let super::RustItem::Object(object) = &mut draft.catalog.items[union.index()].item else {
            panic!("expected object enum");
        };
        if borrowed {
            let RustObject::Enum(variants) = &mut object.definition else {
                panic!("expected enum");
            };
            variants[0].rust_type =
                Some(RustType::reference(variants[0].rust_type.take().unwrap()));
        } else {
            object.usage = ObjectUsage::request_param();
        }
        let error = draft.finish(&components).unwrap_err();
        assert!(error.to_string().contains(if borrowed {
            "borrowed fields"
        } else {
            "request parameters"
        }));
    }
}

#[test]
fn final_union_tags_follow_represented_struct_discriminators() {
    let (components, mut draft, union, customer) = object_union_draft();
    let customer_item = draft.catalog.component_root(customer);
    let super::RustItem::Object(object) = &mut draft.catalog.items[customer_item.index()].item
    else {
        panic!("expected customer object");
    };
    let RustObject::Struct(strukt) = &mut object.definition else {
        panic!("expected struct");
    };
    strukt.object_field = Some("changed_customer".to_owned());
    let items = draft.finish(&components).unwrap();
    let Some(EnumOfObjects::ObjectUnion(members)) =
        &items.get(union).object().unwrap().enum_of_objects
    else {
        panic!("expected object union");
    };
    assert!(members.contains_key("changed_customer"));
    assert!(!members.contains_key("customer"));
    assert_eq!(components.get_by_id(customer).object_name(), Some("customer"));
}

#[test]
fn removing_a_represented_discriminator_cannot_leave_a_tagged_union() {
    let (components, mut draft, _, customer) = object_union_draft();
    let customer_item = draft.catalog.component_root(customer);
    let super::RustItem::Object(object) = &mut draft.catalog.items[customer_item.index()].item
    else {
        panic!("expected customer object");
    };
    let RustObject::Struct(strukt) = &mut object.definition else {
        panic!("expected struct");
    };
    strukt.object_field = None;
    let error = draft.finish(&components).unwrap_err();
    assert!(error.to_string().contains("not a supported object union or live/deleted pair"));
}
