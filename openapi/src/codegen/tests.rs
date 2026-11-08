use indexmap::IndexMap;

use super::CodeGen;
use crate::component_catalog::ComponentCatalog;
use crate::components::Components;
use crate::crate_dependencies::crate_dependencies;
use crate::crates::Crate;
use crate::dependencies::EdgePolicy;
use crate::item_catalog::{ItemCatalogDraft, ItemMetadata};
use crate::item_layout::ItemLayout;
use crate::layout::ComponentLayout;
use crate::operations::{OperationCatalog, StripeOperation};
use crate::rust_model::RustModel;
use crate::spec::Spec;
use crate::stripe_object::{StripeObject, StripeResource};
use crate::types::ComponentName;

fn decoder_fixture(reverse: bool) -> CodeGen {
    let catalog = ComponentCatalog::from_names(
        [
            "card_generated_from_payment_method_details",
            "payment_method_details",
            "payment_method_details_card_present",
            "resource_a",
            "resource_b",
            "family",
        ]
        .map(|name| ComponentName::new(name.to_string())),
    )
    .unwrap();
    let mut entries = catalog.iter().map(|(id, name)| (id, name.clone())).collect::<Vec<_>>();
    if reverse {
        entries.reverse();
    }
    let objects = entries
        .into_iter()
        .map(|(id, name)| {
            let schema = if name.as_ref() == "family" {
                serde_json::json!({"oneOf": [
                    {"$ref":"#/components/schemas/resource_a"},
                    {"$ref":"#/components/schemas/resource_b"}
                ]})
            } else if matches!(name.as_ref(), "resource_a" | "resource_b") {
                serde_json::json!({
                    "type":"object", "properties":{
                        "id":{"type":"string", "description":"Unique identifier for the object."},
                        "object":{"type":"string", "enum":[name.as_ref()]}
                    }, "required":["id", "object"]
                })
            } else if name.as_ref() == "payment_method_details_card_present" {
                serde_json::json!({
                    "type":"object", "properties":{"last4":{"type":"string"}},
                    "required":["last4"]
                })
            } else {
                serde_json::json!({
                    "type":"object", "properties":{
                        "type":{"type":"string"},
                        "card_present":{"$ref":"#/components/schemas/payment_method_details_card_present"}
                    }, "required":["type"]
                })
            };
            let schema = serde_json::from_value(schema).unwrap();
            let resource = StripeResource::from_schema(&schema, name).unwrap();
            (id, StripeObject::from_schema(&schema, &resource, &catalog))
        })
        .collect::<IndexMap<_, _>>();
    let mut operations = OperationCatalog::from_operations(
        catalog.iter().map(|(id, _)| (id, &[] as &[StripeOperation])),
    )
    .unwrap();
    for (id, _) in catalog.iter() {
        operations.attach_requests(id, vec![]).unwrap();
    }
    let components = Components::for_test(catalog, objects, operations);
    let config = crate::crates::test_config();
    let payment = config.all().find(|krate| krate.base_name(&config) == "payment").unwrap();
    let placements =
        components.components.keys().map(|&id| (id, payment, Crate::SHARED)).collect::<Vec<_>>();
    let component_layout = ComponentLayout::for_test(&placements);
    let mut draft = ItemCatalogDraft::build(&components, |id| {
        ItemMetadata::new(components.get_by_id(id).ident().clone())
    })
    .unwrap();
    draft.use_opaque_relationships(&components).unwrap();
    let model = RustModel::from_draft(&components, draft).unwrap();
    let item_layout =
        ItemLayout::from_component_layout(&components, model.items(), &component_layout, &config)
            .unwrap()
            .with_reference_types(model.items(), &component_layout, &config)
            .unwrap();
    let crate_dependencies = crate_dependencies(
        model.items(),
        model.requests(),
        &component_layout,
        &item_layout,
        &config,
    )
    .unwrap();
    let spec = Spec::new(
        serde_json::from_value(serde_json::json!({
            "openapi":"3.0.0", "info":{"title":"decoder fixture","version":"test"}, "paths":{}
        }))
        .unwrap(),
    );
    CodeGen {
        config,
        components,
        model,
        component_layout,
        item_layout,
        crate_dependencies,
        spec,
        version: "1.0.0".into(),
        crate_inference_warnings: vec![],
        edge_policy: EdgePolicy::default(),
        availability: None,
        model_groups: vec![],
    }
}

#[test]
fn shared_record_payloads_keep_their_individual_decoder_feature_in_either_owner_order() {
    for reverse in [false, true] {
        let generated = decoder_fixture(reverse)
            .with_opaque_model_groups(&["card_generated_from_payment_method_details"])
            .unwrap();
        let component = generated
            .components
            .components
            .keys()
            .copied()
            .find(|id| {
                generated.components.component_name(*id).as_ref()
                    == "payment_method_details_card_present"
            })
            .unwrap();
        let payload = generated.model.items().component_root(component);
        let groups = generated
            .model_groups
            .iter()
            .filter(|group| group.items.contains(&payload))
            .map(|group| group.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(groups, ["payment_method_details_card_present"]);
    }
}

#[test]
fn requestless_raw_families_are_available_without_registering_a_feature() {
    let generated = decoder_fixture(false);
    let (&family, _) = generated
        .components
        .components
        .iter()
        .find(|(id, _)| generated.components.component_name(**id).as_ref() == "family")
        .unwrap();
    assert!(!generated.resource_model_groups().iter().any(|(_, component)| *component == family));
    let generated = generated.with_opaque_model_groups(&["family"]).unwrap();
    assert!(!generated.model_groups.iter().any(|group| group.name == "family"));
    let plan = generated.availability.as_ref().unwrap();
    let site = generated
        .item_layout
        .placement(generated.model.items().component_root(family))
        .emission_site();
    assert_eq!(plan.site(site), &crate::model_availability::Availability::Always);
    assert!(!plan.features().contains("family"));
    let mut output = crate::output::GeneratedWorkspace::default();
    crate::facade::write_facade(
        generated.output_locations(),
        generated.model.items(),
        &generated.item_layout,
        plan,
        &[],
        &generated.version,
        &mut output,
    )
    .unwrap();
    let exports =
        std::str::from_utf8(output.contents("async-stripe/src/generated/mod.rs").unwrap()).unwrap();
    assert!(exports.contains("Family"));
    assert!(!exports.contains("feature = \"family\""));
    let manifest: toml::Value = toml::from_str(
        std::str::from_utf8(output.contents("async-stripe/Cargo.toml").unwrap()).unwrap(),
    )
    .unwrap();
    assert!(manifest["features"].get("family").is_none());
}
