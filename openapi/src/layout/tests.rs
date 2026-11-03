use std::collections::{BTreeMap, HashMap};

use indexmap::IndexMap;

use super::ComponentLayout;
use crate::component_catalog::{ComponentCatalog, ComponentId};
use crate::components::{Components, TypeSpec};
use crate::crate_dependencies::{crate_dependencies, validate_crate_dependencies};
use crate::crates::{ALL_CRATES, Crate};
use crate::item_catalog::{ItemCatalog, ItemMetadata};
use crate::item_layout::{ItemAddress, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::model_availability::{ModelAvailability, ModelFeature};
use crate::operations::{OperationCatalog, StripeOperation};
use crate::rust_object::{ObjectUsage, RustObject, Struct};
use crate::rust_type::DeclarationRoot;
use crate::stripe_object::{StripeObject, StripeResource};
use crate::types::{ComponentName, RustIdent};

fn krate(name: &str) -> Crate {
    *ALL_CRATES.iter().find(|krate| krate.base_name() == name).unwrap()
}

struct Fixture {
    components: Components,
    items: ItemCatalog,
    initial: ComponentLayout,
    item_layout: ItemLayout,
    availability: ModelAvailability,
    feature_crates: HashMap<String, Crate>,
    ids: HashMap<&'static str, ComponentId>,
}

impl Fixture {
    fn new(reversed: bool) -> Self {
        let mut names = [
            "charge",
            "payment_method_details",
            "payment_method_details_card",
            "card_wallet",
            "shared_data",
            "only_core",
            "always_data",
            "disabled",
        ];
        if reversed {
            names.reverse();
        }
        let catalog = ComponentCatalog::from_names(
            names.iter().map(|name| ComponentName::new((*name).to_string())),
        )
        .unwrap();
        let ids = names
            .iter()
            .map(|&name| (name, catalog.id_str(name).unwrap()))
            .collect::<HashMap<_, _>>();
        let mut objects = IndexMap::new();
        let mut placements = Vec::new();
        for (component, name) in catalog.iter() {
            let schema = match name.as_ref() {
                "charge" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique identifier for the object."},
                        "details": {"$ref": "#/components/schemas/payment_method_details"},
                        "shared": {"$ref": "#/components/schemas/shared_data"},
                        "local": {"$ref": "#/components/schemas/only_core"}
                    },
                    "required": ["id"]
                }),
                "payment_method_details" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "type": {"type": "string"},
                        "card": {"$ref": "#/components/schemas/payment_method_details_card"}
                    },
                    "required": ["type"]
                }),
                "payment_method_details_card" => serde_json::json!({
                    "type": "object", "properties": {
                        "wallet": {"$ref": "#/components/schemas/card_wallet"}
                    }
                }),
                _ => {
                    serde_json::json!({"type": "object", "properties": {"value": {"type": "string"}}})
                }
            };
            let schema = serde_json::from_value(schema).unwrap();
            let resource = StripeResource::from_schema(&schema, name.clone()).unwrap();
            objects.insert(component, StripeObject::from_schema(&schema, &resource, &catalog));
            let owner = match name.as_ref() {
                "charge" | "only_core" | "always_data" => krate("core"),
                "disabled" => krate("fraud"),
                _ => krate("payment"),
            };
            placements.push((component, owner, Crate::SHARED));
        }
        let mut operations = OperationCatalog::from_operations(
            catalog.iter().map(|(component, _)| (component, &[] as &[StripeOperation])),
        )
        .unwrap();
        for (component, _) in catalog.iter() {
            operations.attach_requests(component, vec![]).unwrap();
        }
        let mut components = Components::for_test(catalog, objects, operations);
        let shared = RustIdent::create("shared_override");
        components.extra_types.insert(
            shared,
            TypeSpec {
                obj: RustObject::Struct(Struct::new(vec![])),
                doc: None,
                mod_path: "shared_override".into(),
                usage: ObjectUsage::type_def(),
            },
        );
        let mut items = ItemCatalog::build(&components, |component| {
            ItemMetadata::new(components.get_by_id(component).ident().clone())
        })
        .unwrap();
        items.use_opaque_relationships(&components).unwrap();
        let initial = ComponentLayout::for_test(&placements);
        let mut item_layout =
            ItemLayout::from_component_layout(&components, &items, &initial).unwrap();
        item_layout.separate_reference_types(&items);
        let unconditional = items
            .iter()
            .filter_map(|(item, record)| {
                (record.is_reference_foundation()
                    || matches!(
                        record.origin.type_root,
                        TypeRoot::Declaration(DeclarationRoot::Shared(_))
                    )
                    || item == items.component_root(ids["always_data"]))
                .then_some(item)
            })
            .collect::<Vec<_>>();
        let mut features = vec![
            ModelFeature {
                consumer_crate: crate::crates::Crate::SHARED,
                name: "charge".into(),
                items: vec![items.component_root(ids["charge"])],
                requests: vec![],
            },
            ModelFeature {
                consumer_crate: crate::crates::Crate::SHARED,
                name: "card".into(),
                items: vec![items.component_root(ids["payment_method_details_card"])],
                requests: vec![],
            },
        ];
        if reversed {
            features.reverse();
        }
        let availability =
            ModelAvailability::build(&items, [], &item_layout, &unconditional, &features).unwrap();
        let feature_crates = features
            .iter()
            .map(|feature| {
                (
                    feature.name.clone(),
                    if feature.name == "charge" { krate("core") } else { krate("payment") },
                )
            })
            .collect();
        Self { components, items, initial, item_layout, availability, feature_crates, ids }
    }

    fn place(&self) -> ComponentLayout {
        self.initial
            .with_consumer_model_crates(
                &self.components,
                &self.items,
                &self.item_layout,
                &self.availability,
                &self.feature_crates,
            )
            .unwrap()
    }

    fn types(&self, placed: &ComponentLayout, name: &str) -> Crate {
        placed.placement(self.ids[name]).types_crate()
    }

    fn by_name(&self, placed: &ComponentLayout) -> BTreeMap<String, (Crate, Crate)> {
        self.components
            .components
            .keys()
            .map(|&component| {
                let placement = placed.placement(component);
                (
                    self.components.component_name(component).to_string(),
                    (placement.request_crate(), placement.types_crate()),
                )
            })
            .collect()
    }
}

#[test]
fn decoder_models_keep_their_own_consumers_without_moving_the_raw_record() {
    let fixture = Fixture::new(false);
    let placed = fixture.place();
    assert_eq!(fixture.types(&placed, "payment_method_details"), Crate::SHARED);
    assert_eq!(fixture.types(&placed, "payment_method_details_card"), krate("payment"));
    assert_eq!(fixture.types(&placed, "card_wallet"), krate("payment"));
    let mut layout =
        ItemLayout::from_component_layout(&fixture.components, &fixture.items, &placed).unwrap();
    layout.separate_reference_types(&fixture.items);
    let dependencies = crate_dependencies(&fixture.items, [], &placed, &layout).unwrap();
    validate_crate_dependencies(&dependencies).unwrap();
    assert!(!dependencies.contains_edge(Crate::SHARED, krate("payment")));
}

#[test]
fn existing_component_facades_are_consumers_even_without_a_field_reference() {
    let fixture = Fixture::new(false);
    let placed = fixture.place();
    // SharedData is used only by Charge, but Payment keeps its public alias.
    assert_eq!(fixture.types(&placed, "shared_data"), Crate::SHARED);
    assert_eq!(fixture.types(&placed, "charge"), krate("core"));
    assert_eq!(fixture.types(&placed, "only_core"), krate("core"));
}

#[test]
fn fixed_support_and_disabled_models_stay_shared_and_service_owners_do_not_change() {
    let fixture = Fixture::new(false);
    let placed = fixture.place();
    assert_eq!(fixture.types(&placed, "always_data"), Crate::SHARED);
    assert_eq!(fixture.types(&placed, "disabled"), Crate::SHARED);
    for &component in fixture.components.components.keys() {
        assert_eq!(
            placed.placement(component).request_crate(),
            fixture.initial.placement(component).request_crate()
        );
    }
    let mut layout =
        ItemLayout::from_component_layout(&fixture.components, &fixture.items, &placed).unwrap();
    layout.separate_reference_types(&fixture.items);
    let id = fixture.items.component_id_item(fixture.ids["charge"]).unwrap();
    assert_eq!(layout.placement(id).address(), ItemAddress::CrateRoot(Crate::SHARED));
    let shared = fixture
        .items
        .root(&TypeRoot::Declaration(DeclarationRoot::Shared(RustIdent::create("shared_override"))))
        .unwrap()
        .declaration
        .unwrap();
    assert_eq!(layout.placement(shared).address(), ItemAddress::CrateRoot(Crate::SHARED));
}

#[test]
fn component_and_feature_insertion_order_do_not_change_placement() {
    let forward = Fixture::new(false);
    let reversed = Fixture::new(true);
    assert_ne!(forward.ids["charge"], reversed.ids["charge"]);
    assert_eq!(forward.by_name(&forward.place()), reversed.by_name(&reversed.place()));
}

#[test]
fn a_feature_without_a_service_owner_is_rejected() {
    let mut fixture = Fixture::new(false);
    fixture.feature_crates.remove("card");
    let error = fixture
        .initial
        .with_consumer_model_crates(
            &fixture.components,
            &fixture.items,
            &fixture.item_layout,
            &fixture.availability,
            &fixture.feature_crates,
        )
        .unwrap_err();
    assert!(error.to_string().contains("card"));
}
