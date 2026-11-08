use std::collections::HashSet;

use super::{ItemCatalog, ItemId, RustItem};
use crate::item_origin::TypeRoot;
use crate::rust_object::{ObjectKind, RustObject, StructField};
use crate::rust_type::{Container, DeclarationRoot, RustType};

/// Identify response record fields selected by a sibling string discriminator.
/// Common fields keep their ordinary types; request construction stays eager.
pub(super) fn deferred_fields(items: &ItemCatalog, item: ItemId) -> Vec<StructField> {
    let Some(object) = items.get(item).object() else { return Vec::new() };
    if object.usage.used_as_request_param || object.usage.kind == ObjectKind::RequestParam {
        return Vec::new();
    }
    let RustObject::Struct(record) = &object.definition else { return Vec::new() };
    let mut selected = HashSet::new();
    for discriminator in &record.fields {
        let typ = discriminator.rust_type.with_option_stripped();
        // These schemas list their discriminator states only in prose.
        // Keep the relationships explicit rather than guessing free strings.
        let known_open_payloads: &[&str] =
            match (items.get(item).metadata.ident.as_ref(), discriminator.wire_name()) {
                ("Source", "flow") => &["redirect", "receiver", "code_verification"],
                ("TaxProductResourceTaxAssociationTransactionAttempts", "status") => {
                    &["committed", "errored"]
                }
                _ => &[],
            };
        if let Some(RustObject::FieldlessEnum(variants)) = items.object_for_type(typ) {
            if variants.is_empty() {
                continue;
            }
            for field in &record.fields {
                if field.wire_name() != discriminator.wire_name()
                    && optional_record_payload(items, field)
                    && variants.iter().any(|variant| variant.wire_name == field.wire_name())
                {
                    selected.insert(field.wire_name());
                }
            }
        } else if !known_open_payloads.is_empty() && *typ == RustType::string() {
            selected.extend(
                record
                    .fields
                    .iter()
                    .filter(|field| {
                        !field.required
                            && known_open_payloads.contains(&field.wire_name())
                            && optional_record_payload(items, field)
                    })
                    .map(|field| field.wire_name()),
            );
        } else if discriminator.wire_name() == "type" && *typ == RustType::string() {
            let candidates = record
                .fields
                .iter()
                .filter(|field| !field.required && optional_record_payload(items, field))
                .collect::<Vec<_>>();
            // With open string types, a lone optional object among common
            // fields need not be a variant. Require a family of payloads, or
            // the unambiguous type-plus-one-payload shape.
            if candidates.len() >= 2 || (candidates.len() == 1 && record.fields.len() == 2) {
                selected.extend(candidates.into_iter().map(|field| field.wire_name()));
            }
        }
    }
    record.fields.iter().filter(|field| selected.contains(field.wire_name())).cloned().collect()
}

fn optional_record_payload(items: &ItemCatalog, field: &StructField) -> bool {
    if !field.vis.is_public() {
        return false;
    }
    let RustType::Container(Container::Option(inner)) = &field.rust_type else { return false };
    let RustType::Item(payload) = **inner else { return false };
    let payload = items.get(payload);
    // Inline and deduplicated declarations compile in their owner's module.
    // Deferring their field would not remove those compilation dependencies,
    // and they have no independently addressable decoder capability.
    if !payload.origin.path.is_empty()
        || !matches!(payload.origin.type_root, TypeRoot::Declaration(DeclarationRoot::Component(_)))
    {
        return false;
    }
    match &payload.item {
        RustItem::Object(object) => {
            !object.usage.used_as_request_param
                && object.usage.kind != ObjectKind::RequestParam
                && matches!(object.definition, RustObject::Struct(_) | RustObject::Enum(_))
        }
        RustItem::OpaqueObject | RustItem::OpaqueRecord(_) => true,
        RustItem::IdNewtype => false,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::deferred_fields;
    use crate::component_catalog::ComponentCatalog;
    use crate::components::Components;
    use crate::item_catalog::{ItemCatalog, ItemId, ItemMetadata, RustItem};
    use crate::operations::{OperationCatalog, StripeOperation};
    use crate::rust_object::{ObjectUsage, RustObject, Visibility};
    use crate::rust_type::RustType;
    use crate::stripe_object::{StripeObject, StripeResource};
    use crate::types::ComponentName;

    fn catalog(schema: serde_json::Value) -> (ItemCatalog, ItemId) {
        let names = ComponentCatalog::from_names(
            ["record", "payload_a", "payload_b"].map(|name| ComponentName::new(name.to_string())),
        )
        .unwrap();
        let record = names.id_str("record").unwrap();
        let objects = names
            .iter()
            .map(|(component, name)| {
                let schema = if component == record {
                    schema.clone()
                } else {
                    serde_json::json!({"type":"object", "properties":{"value":{"type":"string"}}})
                };
                let schema = serde_json::from_value(schema).unwrap();
                let resource = StripeResource::from_schema(&schema, name.clone()).unwrap();
                (component, StripeObject::from_schema(&schema, &resource, &names))
            })
            .collect();
        let mut operations = OperationCatalog::from_operations(
            names.iter().map(|(component, _)| (component, &[] as &[StripeOperation])),
        )
        .unwrap();
        for (component, _) in names.iter() {
            operations.attach_requests(component, vec![]).unwrap();
        }
        let components = Components::for_test(names, objects, operations);
        let items = ItemCatalog::build(&components, |component| {
            ItemMetadata::new(components.get_by_id(component).ident().clone())
        })
        .unwrap();
        let owner = items.component_root(record);
        (items, owner)
    }

    fn fields(items: &ItemCatalog, owner: ItemId) -> BTreeSet<String> {
        deferred_fields(items, owner).iter().map(|field| field.wire_name().to_string()).collect()
    }

    fn schema(discriminator: &str, enum_values: Option<&[&str]>) -> serde_json::Value {
        let mut schema = serde_json::json!({
            "type":"object", "properties":{
                "card":{"$ref":"#/components/schemas/payload_a"},
                "bank":{"$ref":"#/components/schemas/payload_b"},
                "billing_details":{"$ref":"#/components/schemas/payload_a"},
                "created":{"type":"integer"}
            }, "required":["billing_details", discriminator]
        });
        let mut typ = serde_json::json!({"type":"string"});
        if let Some(values) = enum_values {
            typ["enum"] = serde_json::json!(values);
        }
        schema["properties"][discriminator] = typ;
        schema
    }

    #[test]
    fn closed_discriminators_select_payloads_and_preserve_common_fields() {
        for discriminator in ["type", "business_type", "reason", "status", "rate_type"] {
            let (items, owner) = catalog(schema(discriminator, Some(&["card", "bank", "none"])));
            assert_eq!(fields(&items, owner), ["bank".to_string(), "card".to_string()].into());
        }
    }

    #[test]
    fn singleton_enum_discriminators_select_their_named_payload() {
        let (items, owner) = catalog(schema("type", Some(&["card"])));
        assert_eq!(fields(&items, owner), ["card".to_string()].into());
    }

    #[test]
    fn nullable_discriminators_and_required_nullable_payloads_remain_supported() {
        let mut schema = schema("business_type", Some(&["card", "bank"]));
        schema["required"] = serde_json::json!(["card", "billing_details"]);
        schema["properties"]["business_type"]["nullable"] = serde_json::json!(true);
        schema["properties"]["card"] = serde_json::json!({
            "anyOf":[{"$ref":"#/components/schemas/payload_a"}], "nullable":true
        });
        let (items, owner) = catalog(schema);
        assert_eq!(fields(&items, owner), ["bank".to_string(), "card".to_string()].into());
    }

    #[test]
    fn open_types_defer_optional_families_without_defering_required_common_objects() {
        let (items, owner) = catalog(schema("type", None));
        assert_eq!(fields(&items, owner), ["bank".to_string(), "card".to_string()].into());
    }

    #[test]
    fn open_single_payloads_need_the_pure_record_shape() {
        let (items, owner) = catalog(serde_json::json!({
            "type":"object", "properties": {
                "type":{"type":"string"},
                "card":{"$ref":"#/components/schemas/payload_a"}
            }, "required":["type"]
        }));
        assert_eq!(fields(&items, owner), ["card".to_string()].into());

        let (items, owner) = catalog(serde_json::json!({
            "type":"object", "properties": {
                "type":{"type":"string"}, "display_name":{"type":"string"},
                "logo":{"$ref":"#/components/schemas/payload_a"}
            }, "required":["type","display_name","logo"]
        }));
        assert!(fields(&items, owner).is_empty());

        let (items, owner) = catalog(serde_json::json!({
            "type":"object", "properties": {
                "type":{"type":"string"}, "created":{"type":"integer"},
                "logo":{"$ref":"#/components/schemas/payload_a"}
            }, "required":["type","created"]
        }));
        assert!(fields(&items, owner).is_empty());
    }

    #[test]
    fn inline_payloads_stay_typed_while_named_payloads_can_be_decoded_independently() {
        let (items, owner) = catalog(serde_json::json!({
            "type":"object", "properties": {
                "type":{"type":"string", "enum":["card","bank"]},
                "card":{
                    "type":"object", "properties":{"last4":{"type":"string"}},
                    "required":["last4"]
                },
                "bank":{"$ref":"#/components/schemas/payload_b"}
            }, "required":["type"]
        }));
        assert_eq!(fields(&items, owner), ["bank".to_string()].into());
    }

    #[test]
    fn request_inputs_and_types_shared_with_request_inputs_stay_eager() {
        let (mut items, owner) = catalog(schema("type", Some(&["card", "bank"])));
        let RustItem::Object(object) = &mut items.items[owner.index()].item else { panic!() };
        object.usage = ObjectUsage::request_param();
        assert!(fields(&items, owner).is_empty());
        let RustItem::Object(object) = &mut items.items[owner.index()].item else { panic!() };
        object.usage = ObjectUsage::type_def();
        object.usage.used_as_request_param = true;
        assert!(fields(&items, owner).is_empty());
    }

    #[test]
    fn source_flow_defers_only_its_three_documented_payloads() {
        let (mut items, owner) = catalog(serde_json::json!({
            "type":"object", "properties": {
                "flow":{"type":"string"},
                "redirect":{"$ref":"#/components/schemas/payload_a"},
                "receiver":{"$ref":"#/components/schemas/payload_b"},
                "code_verification":{"$ref":"#/components/schemas/payload_a"},
                "owner":{"$ref":"#/components/schemas/payload_b"}
            }, "required":["flow"]
        }));
        // The same field names on an unrelated record do not establish a
        // discriminator relationship for an arbitrary string property.
        assert!(fields(&items, owner).is_empty());
        items.items[owner.index()].metadata.ident = crate::types::RustIdent::create("source");
        assert_eq!(
            fields(&items, owner),
            ["code_verification".to_string(), "receiver".to_string(), "redirect".to_string()]
                .into()
        );
    }

    #[test]
    fn tax_transaction_status_defers_only_its_two_documented_payloads() {
        let (mut items, owner) = catalog(serde_json::json!({
            "type":"object", "properties": {
                "status":{"type":"string"},
                "source":{"type":"string"},
                "committed":{"$ref":"#/components/schemas/payload_a"},
                "errored":{"$ref":"#/components/schemas/payload_b"},
                "context":{"$ref":"#/components/schemas/payload_a"}
            }, "required":["status","source"]
        }));
        assert!(fields(&items, owner).is_empty());
        items.items[owner.index()].metadata.ident = crate::types::RustIdent::create(
            "tax_product_resource_tax_association_transaction_attempts",
        );
        assert_eq!(fields(&items, owner), ["committed".to_string(), "errored".to_string()].into());
    }

    #[test]
    fn option_bags_without_discriminators_keep_their_typed_fields() {
        let (mut items, owner) = catalog(serde_json::json!({
            "type":"object", "properties": {
                "card":{"anyOf":[{"$ref":"#/components/schemas/payload_a"}],"nullable":true},
                "bank":{"$ref":"#/components/schemas/payload_b"},
                "common":{"$ref":"#/components/schemas/payload_a"},
                "nullable_common":{"type":"string", "nullable":true},
                "name":{"type":"string"}
            }, "required":["card","common","nullable_common","name"]
        }));
        for name in [
            "CheckoutSessionPaymentMethodOptions",
            "PaymentIntentPaymentMethodOptions",
            "SetupIntentPaymentMethodOptions",
            "InvoicesPaymentMethodOptions",
            "SubscriptionsResourcePaymentMethodOptions",
            "PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions",
        ] {
            items.items[owner.index()].metadata.ident = crate::types::RustIdent::unchanged(name);
            assert!(fields(&items, owner).is_empty());
        }
        items.items[owner.index()].metadata.ident =
            crate::types::RustIdent::unchanged("OtherPaymentMethodOptions");
        assert!(fields(&items, owner).is_empty());

        items.items[owner.index()].metadata.ident =
            crate::types::RustIdent::unchanged("PaymentIntentPaymentMethodOptions");
        let RustItem::Object(object) = &mut items.items[owner.index()].item else { panic!() };
        object.usage = ObjectUsage::request_param();
        assert!(fields(&items, owner).is_empty());
        let RustItem::Object(object) = &mut items.items[owner.index()].item else { panic!() };
        object.usage = ObjectUsage::type_def();
        object.usage.used_as_request_param = true;
        assert!(fields(&items, owner).is_empty());
    }

    #[test]
    fn scalar_id_enum_expandable_and_private_fields_are_not_record_payloads() {
        let (mut items, owner) = catalog(serde_json::json!({
            "type":"object", "properties": {
                "type":{"type":"string", "enum":["id","status","source","card","bank"]},
                "id":{"type":"string","description":"Unique identifier for the object."},
                "status":{"type":"string","enum":["enabled","disabled"]},
                "source":{"$ref":"#/components/schemas/payload_a"},
                "card":{"$ref":"#/components/schemas/payload_a"},
                "bank":{"$ref":"#/components/schemas/payload_b"}
            }, "required":["type"]
        }));
        let RustItem::Object(object) = &mut items.items[owner.index()].item else { panic!() };
        let RustObject::Struct(record) = &mut object.definition else { panic!() };
        let source = record.fields.iter_mut().find(|field| field.wire_name() == "source").unwrap();
        let target = source.rust_type.item_id().unwrap();
        source.rust_type = RustType::option(RustType::expandable(RustType::Item(target)));
        record.fields.iter_mut().find(|field| field.wire_name() == "bank").unwrap().vis =
            Visibility::Private;
        assert_eq!(fields(&items, owner), ["card".to_string()].into());
    }
}
