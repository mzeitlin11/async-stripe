use indexmap::IndexMap;

use super::{CatalogBuilder, ItemCatalog, ItemMetadata, RustItem};
use crate::component_catalog::{ComponentCatalog, ComponentId};
use crate::components::Components;
use crate::crate_dependencies::{crate_dependencies, validate_crate_dependencies};
use crate::crates::{ALL_CRATES, Crate};
use crate::item_layout::{EmissionSite, ItemLayout};
use crate::item_origin::{ItemOccurrence, ItemOrigin, TypeRoot};
use crate::layout::ComponentLayout;
use crate::model_availability::{Availability, ModelAvailability, ModelFeature};
use crate::object_writing::ItemEmitter;
use crate::operations::{OperationCatalog, StripeOperation};
use crate::output::GeneratedWorkspace;
use crate::output_locations::OutputLocations;
use crate::rust_object::{ObjectMetadata, ObjectUsage, RustObject, Struct, StructField};
use crate::rust_type::{Container, DeclarationRoot, ExternalType, RustType};
use crate::stripe_object::{StripeObject, StripeResource};
use crate::types::{ComponentName, RustIdent};

fn krate(name: &str) -> Crate {
    *ALL_CRATES.iter().find(|krate| krate.base_name() == name).unwrap()
}

struct Fixture {
    components: Components,
    items: ItemCatalog,
    placement: ComponentLayout,
    charge: ComponentId,
    customer: ComponentId,
    source: ComponentId,
    errors: ComponentId,
}

impl Fixture {
    fn new() -> Self {
        let catalog = ComponentCatalog::from_names(
            ["charge", "customer", "source_family", "api_errors"]
                .map(|name| ComponentName::new(name.to_string())),
        )
        .unwrap();
        let charge = catalog.id_str("charge").unwrap();
        let customer = catalog.id_str("customer").unwrap();
        let source = catalog.id_str("source_family").unwrap();
        let errors = catalog.id_str("api_errors").unwrap();
        let mut objects = IndexMap::new();
        for (component, name) in catalog.iter() {
            let schema = if component == source {
                serde_json::json!({"oneOf": [
                    {"$ref": "#/components/schemas/charge"},
                    {"$ref": "#/components/schemas/customer"}
                ]})
            } else if component == errors {
                serde_json::json!({"type": "object", "properties": {"message": {"type": "string"}}})
            } else {
                serde_json::json!({
                    "type": "object", "properties": {
                        "id": {"type": "string", "description": "Unique identifier for the object."},
                        "object": {"type": "string", "enum": [name.as_ref()]}
                    }, "required": ["id", "object"]
                })
            };
            let schema = serde_json::from_value(schema).unwrap();
            let resource = StripeResource::from_schema(&schema, name.clone()).unwrap();
            objects.insert(component, StripeObject::from_schema(&schema, &resource, &catalog));
        }
        let mut operations = OperationCatalog::from_operations(
            catalog.iter().map(|(component, _)| (component, &[] as &[StripeOperation])),
        )
        .unwrap();
        for (component, _) in catalog.iter() {
            operations.attach_requests(component, vec![]).unwrap();
        }
        let components = Components::for_test(catalog, objects, operations);
        let mut items = ItemCatalog::build(&components, |component| {
            ItemMetadata::new(components.get_by_id(component).ident().clone())
        })
        .unwrap();
        let customer_item = items.component_root(customer);
        let source_item = items.component_root(source);
        let charge_item = items.component_root(charge);
        let error_item = items.component_root(errors);
        let RustItem::Object(object) = &mut items.items[charge_item.index()].item else { panic!() };
        let RustObject::Struct(strukt) = &mut object.definition else { panic!() };
        strukt.fields.push(StructField::new(
            "customer",
            RustType::option(RustType::expandable(RustType::Item(customer_item))),
        ));
        strukt
            .fields
            .push(StructField::new("source", RustType::option(RustType::Item(source_item))));
        let RustItem::Object(object) = &mut items.items[error_item.index()].item else { panic!() };
        let RustObject::Struct(strukt) = &mut object.definition else { panic!() };
        strukt.fields.push(StructField::new(
            "customer",
            RustType::option(RustType::boxed(RustType::Item(customer_item))),
        ));
        strukt
            .fields
            .push(StructField::new("charge", RustType::option(RustType::Item(charge_item))));
        // Multiple contributing origins are deliberately distinct from the
        // canonical declaration's origin, and must survive representation choice.
        items.items[source_item.index()].occurrences.push(ItemOccurrence {
            origin: ItemOrigin {
                type_root: TypeRoot::Declaration(DeclarationRoot::Component(charge)),
                path: vec!["source".to_string()],
            },
            usage: ObjectUsage::type_def(),
        });
        items.analyze_properties().unwrap();
        let placement = ComponentLayout::for_test(&[
            (charge, krate("core"), krate("core")),
            (customer, krate("billing"), krate("billing")),
            (source, krate("payment"), krate("payment")),
            (errors, Crate::SHARED, Crate::SHARED),
        ]);
        Self { components, items, placement, charge, customer, source, errors }
    }

    fn layout(&self, independent_references: bool) -> ItemLayout {
        let mut layout =
            ItemLayout::from_component_layout(&self.components, &self.items, &self.placement)
                .unwrap();
        if independent_references {
            layout.separate_reference_types(&self.items);
            layout.validate(&self.items, &self.placement).unwrap();
        }
        layout
    }
}

#[test]
fn representation_preserves_item_ids_names_and_occurrence_history() {
    let mut fixture = Fixture::new();
    let before = fixture
        .items
        .iter()
        .map(|(item, record)| {
            (
                item,
                record.origin.clone(),
                record.metadata.clone(),
                record.occurrences.clone(),
                record.sharing_rule,
            )
        })
        .collect::<Vec<_>>();
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let after = fixture
        .items
        .iter()
        .map(|(item, record)| {
            (
                item,
                record.origin.clone(),
                record.metadata.clone(),
                record.occurrences.clone(),
                record.sharing_rule,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(before, after);
    assert!(matches!(fixture.components.get_by_id(fixture.source).rust_obj(), RustObject::Enum(_)));
    assert!(matches!(
        fixture.items.get(fixture.items.component_root(fixture.source)).item,
        RustItem::OpaqueObject
    ));
}

#[test]
fn opaque_relationships_remove_full_model_dependencies_but_keep_typed_ids() {
    let mut fixture = Fixture::new();
    let initial =
        crate_dependencies(&fixture.items, [], &fixture.placement, &fixture.layout(false)).unwrap();
    assert!(initial.contains_edge(krate("core"), krate("billing")));
    assert!(initial.contains_edge(krate("core"), krate("payment")));
    assert!(initial.contains_edge(Crate::SHARED, krate("billing")));
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let layout = fixture.layout(true);
    let graph = crate_dependencies(&fixture.items, [], &fixture.placement, &layout).unwrap();
    assert!(!graph.contains_edge(krate("core"), krate("billing")));
    assert!(!graph.contains_edge(krate("core"), krate("payment")));
    assert!(!graph.contains_edge(Crate::SHARED, krate("billing")));
    assert!(!graph.contains_edge(Crate::SHARED, krate("core")));
    validate_crate_dependencies(&graph).unwrap();
    let charge = fixture.items.get(fixture.items.component_root(fixture.charge)).object().unwrap();
    let customer_id = fixture.items.component_id_item(fixture.customer).unwrap();
    assert_eq!(
        charge.definition.get_struct_field("customer"),
        Some(&RustType::option(RustType::Container(Container::OpaqueExpandable(Box::new(
            RustType::Item(customer_id)
        )))))
    );
}

#[test]
fn diagnostic_context_becomes_opaque_without_changing_scalar_fields() {
    let mut fixture = Fixture::new();
    let errors = fixture.items.component_root(fixture.errors);
    let message = fixture
        .items
        .get(errors)
        .object()
        .unwrap()
        .definition
        .get_struct_field("message")
        .unwrap()
        .clone();
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let object = fixture.items.get(errors).object().unwrap();
    assert_eq!(object.definition.get_struct_field("message"), Some(&message));
    assert_eq!(
        object.definition.get_struct_field("customer"),
        Some(&RustType::option(RustType::boxed(RustType::External(ExternalType::ObjectValue))))
    );
    assert_eq!(
        object.definition.get_struct_field("charge"),
        Some(&RustType::option(RustType::External(ExternalType::ObjectValue)))
    );
}

#[test]
fn tagless_expandables_defer_models_while_diagnostic_objects_keep_tagged_contract() {
    let mut fixture = Fixture::new();
    let customer = fixture.items.component_root(fixture.customer);
    let RustItem::Object(object) = &mut fixture.items.items[customer.index()].item else {
        panic!()
    };
    let RustObject::Struct(strukt) = &mut object.definition else { panic!() };
    // The guard must use the canonical model, which may have been rewritten,
    // rather than the immutable inference-side component.
    strukt.object_field = None;
    assert!(fixture.components.get_by_id(fixture.customer).object_name().is_some());
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let charge = fixture.items.get(fixture.items.component_root(fixture.charge)).object().unwrap();
    assert_eq!(
        charge.definition.get_struct_field("customer"),
        Some(&RustType::option(RustType::Container(Container::UntaggedExpandable(Box::new(
            RustType::Item(fixture.items.component_id_item(fixture.customer).unwrap())
        )))))
    );
    let errors = fixture.items.get(fixture.items.component_root(fixture.errors)).object().unwrap();
    assert_eq!(
        errors.definition.get_struct_field("customer"),
        Some(&RustType::option(RustType::boxed(RustType::Item(customer))))
    );
    // A tagged resource in the same diagnostic record still becomes raw.
    assert_eq!(
        errors.definition.get_struct_field("charge"),
        Some(&RustType::option(RustType::External(ExternalType::ObjectValue)))
    );
    let layout = fixture.layout(true);
    let groups = [ModelFeature {
        consumer_crate: crate::crates::Crate::SHARED,
        name: "charge".into(),
        items: vec![fixture.items.component_root(fixture.charge)],
        requests: vec![],
    }];
    let plan = ModelAvailability::build(&fixture.items, [], &layout, &[], &groups).unwrap();
    // Charge no longer requires its expandable target model. The diagnostic
    // record is not enabled by this feature either.
    assert_eq!(plan.site(layout.placement(customer).emission_site()), &Availability::Disabled);
}

#[test]
fn nominal_object_families_emit_without_importing_their_concrete_variants() {
    let mut fixture = Fixture::new();
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let layout = fixture.layout(true);
    let source = fixture.items.component_root(fixture.source);
    let mut emitter = ItemEmitter::new(
        &fixture.items,
        &layout,
        OutputLocations::new(&fixture.components, &fixture.placement),
    );
    let mut output = GeneratedWorkspace::default();
    emitter.emit(source, &mut output).unwrap();
    let emitted = std::str::from_utf8(
        output.contents("crates/async-stripe-shared/src/source_family.rs").unwrap(),
    )
    .unwrap();
    assert!(emitted.contains("def_object_value!(SourceFamily)"));
    assert!(!emitted.contains("Charge"));
    assert!(!emitted.contains("Customer"));
}

#[test]
fn availability_uses_representations_instead_of_semantic_relationships() {
    let mut fixture = Fixture::new();
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let layout = fixture.layout(true);
    let always = fixture
        .items
        .iter()
        .filter_map(|(item, record)| {
            (matches!(record.item, RustItem::IdNewtype | RustItem::OpaqueObject)
                || item == fixture.items.component_root(fixture.errors))
            .then_some(item)
        })
        .collect::<Vec<_>>();
    let groups = [ModelFeature {
        consumer_crate: crate::crates::Crate::SHARED,
        name: "charge".to_string(),
        items: vec![fixture.items.component_root(fixture.charge)],
        requests: vec![],
    }];
    let plan = ModelAvailability::build(&fixture.items, [], &layout, &always, &groups).unwrap();
    assert_eq!(plan.site(EmissionSite::ComponentTypes(fixture.customer)), &Availability::Disabled);
    assert_eq!(
        plan.site(EmissionSite::ComponentTypes(fixture.charge)),
        &Availability::Features(["charge".to_string()].into_iter().collect())
    );
}

#[test]
fn erased_reference_does_not_enable_a_structural_child_moved_to_another_module() {
    let catalog = ComponentCatalog::from_names(
        ["charge", "customer"].map(|name| ComponentName::new(name.to_string())),
    )
    .unwrap();
    let charge = catalog.id_str("charge").unwrap();
    let customer = catalog.id_str("customer").unwrap();
    let mut builder = CatalogBuilder::default();
    let root = builder
        .collect_root_object(
            TypeRoot::Declaration(DeclarationRoot::Component(charge)),
            &RustObject::Struct(Struct::new(vec![StructField::new(
                "details",
                RustType::Object(
                    RustObject::Struct(Struct::new(vec![StructField::new(
                        "customer",
                        RustType::component(customer),
                    )])),
                    ObjectMetadata::new(RustIdent::create("charge_details")),
                ),
            )])),
            ItemMetadata::new(RustIdent::create("charge")),
            ObjectUsage::type_def(),
        )
        .unwrap();
    builder
        .collect_root_object(
            TypeRoot::Declaration(DeclarationRoot::Component(customer)),
            &RustObject::Struct(Struct::new(vec![])),
            ItemMetadata::new(RustIdent::create("customer")),
            ObjectUsage::type_def(),
        )
        .unwrap();
    let mut items = builder.finish().unwrap();
    items.lower_named_references().unwrap();
    let child = items.get(root).children[0];
    let RustItem::Object(object) = &mut items.items[root.index()].item else { panic!() };
    let RustObject::Struct(strukt) = &mut object.definition else { panic!() };
    strukt.fields[0].rust_type = RustType::External(ExternalType::ObjectValue);
    let component_layout = ComponentLayout::for_test(&[
        (charge, krate("core"), krate("core")),
        (customer, krate("billing"), krate("billing")),
    ]);
    let layout = ItemLayout::for_test(
        items
            .iter()
            .map(|(item, record)| {
                if item == child {
                    (
                        EmissionSite::Standalone { item, krate: krate("core") },
                        crate::item_layout::ItemAddress::CrateRoot(krate("core")),
                    )
                } else {
                    let TypeRoot::Declaration(DeclarationRoot::Component(component)) =
                        record.origin.type_root
                    else {
                        panic!()
                    };
                    (
                        EmissionSite::ComponentTypes(component),
                        crate::item_layout::ItemAddress::CrateRoot(if component == charge {
                            krate("core")
                        } else {
                            krate("billing")
                        }),
                    )
                }
            })
            .collect(),
    );
    layout.validate(&items, &component_layout).unwrap();
    let group = ModelFeature {
        consumer_crate: crate::crates::Crate::SHARED,
        name: "charge".to_string(),
        items: vec![root],
        requests: vec![],
    };
    let plan = ModelAvailability::build(&items, [], &layout, &[], &[group]).unwrap();
    assert_eq!(plan.site(layout.placement(child).emission_site()), &Availability::Disabled);
    assert_eq!(plan.site(EmissionSite::ComponentTypes(customer)), &Availability::Disabled);
}

struct RecordFixture {
    components: Components,
    items: ItemCatalog,
    owner: ComponentId,
    card: ComponentId,
    other: ComponentId,
}

impl RecordFixture {
    fn new() -> Self {
        let catalog = ComponentCatalog::from_names(
            [
                "payment_method_details",
                "payment_method_details_card",
                "payment_method_details_other",
            ]
            .map(|name| ComponentName::new(name.into())),
        )
        .unwrap();
        let owner = catalog.id_str("payment_method_details").unwrap();
        let card = catalog.id_str("payment_method_details_card").unwrap();
        let other = catalog.id_str("payment_method_details_other").unwrap();
        let mut objects = IndexMap::new();
        for (component, name) in catalog.iter() {
            let schema = if component == owner {
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "type": {"type": "string"},
                        "card": {"$ref": "#/components/schemas/payment_method_details_card"},
                        "other": {"$ref": "#/components/schemas/payment_method_details_other"}
                    },
                    "required": ["type"]
                })
            } else {
                serde_json::json!({"type": "object", "properties": {"value": {"type": "string"}}})
            };
            let schema = serde_json::from_value(schema).unwrap();
            let resource = StripeResource::from_schema(&schema, name.clone()).unwrap();
            objects.insert(component, StripeObject::from_schema(&schema, &resource, &catalog));
        }
        let mut operations = OperationCatalog::from_operations(
            catalog.iter().map(|(component, _)| (component, &[] as &[StripeOperation])),
        )
        .unwrap();
        for (component, _) in catalog.iter() {
            operations.attach_requests(component, vec![]).unwrap();
        }
        let components = Components::for_test(catalog, objects, operations);
        let items = ItemCatalog::build(&components, |component| {
            ItemMetadata::new(components.get_by_id(component).ident().clone())
        })
        .unwrap();
        Self { components, items, owner, card, other }
    }

    fn layout(&self, card_crate: Crate) -> (ComponentLayout, ItemLayout) {
        let placement = ComponentLayout::for_test(&[
            (self.owner, Crate::SHARED, Crate::SHARED),
            (self.card, card_crate, card_crate),
            (self.other, Crate::SHARED, Crate::SHARED),
        ]);
        let layout =
            ItemLayout::from_component_layout(&self.components, &self.items, &placement).unwrap();
        (placement, layout)
    }
}

#[test]
fn lazy_record_preserves_identity_and_source_without_storage_dependencies() {
    let mut fixture = RecordFixture::new();
    let owner = fixture.items.component_root(fixture.owner);
    let before = fixture.items.get(owner).clone();
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let record = fixture.items.get(owner);
    assert_eq!(record.origin, before.origin);
    assert_eq!(record.occurrences, before.occurrences);
    assert_eq!(record.metadata, before.metadata);
    let RustItem::OpaqueRecord(fields) = &record.item else { panic!("record was not lowered") };
    assert_eq!(fields.len(), 2);
    let mut uses = 0;
    record
        .visit_type_uses(|_| {
            uses += 1;
            Ok(())
        })
        .unwrap();
    assert_eq!(uses, 0);
    assert!(matches!(
        fixture.components.get_by_id(fixture.owner).rust_obj(),
        RustObject::Struct(_)
    ));
    assert!(!fixture.items.properties(owner).is_copy);
    assert!(!fixture.items.properties(owner).is_eq);
    assert!(!fixture.items.properties(owner).has_lifetime);
}

#[test]
fn decoder_metadata_does_not_link_generated_model_availability() {
    let mut fixture = RecordFixture::new();
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let (_, layout) = fixture.layout(krate("core"));
    let owner = fixture.items.component_root(fixture.owner);
    let card = fixture.items.component_root(fixture.card);
    let other = fixture.items.component_root(fixture.other);
    let features = [
        ModelFeature {
            consumer_crate: Crate::SHARED,
            name: "details".into(),
            items: vec![owner],
            requests: vec![],
        },
        ModelFeature {
            consumer_crate: krate("core"),
            name: "card".into(),
            items: vec![card],
            requests: vec![],
        },
    ];
    let plan = ModelAvailability::build(&fixture.items, [], &layout, &[], &features).unwrap();
    let owner_site = layout.placement(owner).emission_site();
    let card_site = layout.placement(card).emission_site();
    assert_eq!(
        plan.site(owner_site),
        &Availability::Features(["details".into()].into_iter().collect())
    );
    assert_eq!(
        plan.site(card_site),
        &Availability::Features(["card".into()].into_iter().collect())
    );
    assert_eq!(plan.site(layout.placement(other).emission_site()), &Availability::Disabled);
    assert!(!plan.dependencies()[&owner_site].contains(&card_site));
    assert!(!plan.dependencies()[&card_site].contains(&owner_site));
}

#[test]
fn facade_extension_methods_span_crates_and_follow_separate_model_conditions() {
    let mut fixture = RecordFixture::new();
    let owner = fixture.items.component_root(fixture.owner);
    // This common field makes the receiver an ordinary mixed record, whose
    // availability is independent of either optional payload model.
    let RustItem::Object(object) = &mut fixture.items.items[owner.index()].item else { panic!() };
    let RustObject::Struct(record) = &mut object.definition else { panic!() };
    record.fields.push(StructField::new("common", RustType::string()).required(true));
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let (placement, layout) = fixture.layout(krate("core"));
    let card = fixture.items.component_root(fixture.card);
    let other = fixture.items.component_root(fixture.other);
    let features = [
        ModelFeature {
            consumer_crate: Crate::SHARED,
            name: "details".into(),
            items: vec![owner],
            requests: vec![],
        },
        ModelFeature {
            consumer_crate: krate("core"),
            name: "card".into(),
            items: vec![card],
            requests: vec![],
        },
        ModelFeature {
            consumer_crate: Crate::SHARED,
            name: "other".into(),
            items: vec![other],
            requests: vec![],
        },
    ];
    let plan = ModelAvailability::build(&fixture.items, [], &layout, &[], &features).unwrap();
    let owner_site = layout.placement(owner).emission_site();
    let card_site = layout.placement(card).emission_site();
    assert_eq!(plan.site(owner_site), &Availability::Features(["details".into()].into()));
    assert_eq!(plan.site(card_site), &Availability::Features(["card".into()].into()));
    assert!(!plan.dependencies()[&owner_site].contains(&card_site));
    assert!(!plan.dependencies()[&card_site].contains(&owner_site));
    let dependencies = crate_dependencies(&fixture.items, [], &placement, &layout).unwrap();
    validate_crate_dependencies(&dependencies).unwrap();
    assert!(!dependencies.contains_edge(Crate::SHARED, krate("core")));

    let mut output = GeneratedWorkspace::default();
    let mut emitter = ItemEmitter::new(
        &fixture.items,
        &layout,
        OutputLocations::new(&fixture.components, &placement),
    )
    .with_availability(&plan);
    for (item, _) in fixture.items.iter() {
        emitter.emit(item, &mut output).unwrap();
    }
    emitter.finish().unwrap();
    for path in [
        "crates/async-stripe-shared/src/payment_method_details.rs",
        "crates/async-stripe-core/src/payment_method_details_card.rs",
    ] {
        let model = std::str::from_utf8(output.contents(path).unwrap()).unwrap();
        assert!(!model.contains("pub fn card("));
        assert!(!model.contains("trait PaymentMethodDetailsExt"));
    }
    crate::facade::write_facade(
        OutputLocations::new(&fixture.components, &placement),
        &fixture.items,
        &layout,
        &plan,
        &[
            ("details".into(), fixture.owner),
            ("card".into(), fixture.card),
            ("other".into(), fixture.other),
        ],
        "1.0.0",
        &mut output,
    )
    .unwrap();
    let exports =
        std::str::from_utf8(output.contents("async-stripe/src/generated/mod.rs").unwrap()).unwrap();
    assert!(exports.contains("#[cfg(feature = \"details\")]\npub trait PaymentMethodDetailsExt"));
    let methods = exports.split("pub trait PaymentMethodDetailsExt").nth(1).unwrap();
    assert!(methods.contains("#[cfg(feature = \"card\")]"));
    assert!(methods.contains("fn card("));
    assert!(methods.contains("#[cfg(feature = \"other\")]"));
    assert!(methods.contains("fn other("));
    assert!(exports.contains("impl PaymentMethodDetailsExt for"));
    assert!(exports.contains("pub mod prelude"));
    let manifest: toml::Value = toml::from_str(
        std::str::from_utf8(output.contents("async-stripe/Cargo.toml").unwrap()).unwrap(),
    )
    .unwrap();
    let forwarded = manifest["features"]["card"].as_array().unwrap();
    assert!(forwarded.contains(&toml::Value::String("async-stripe-core/card".into())));
    assert!(!forwarded.contains(&toml::Value::String("async-stripe-shared/card".into())));
}

#[test]
fn pure_record_foundation_keeps_standalone_decoder_extension_available() {
    let mut fixture = RecordFixture::new();
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let (placement, mut layout) = fixture.layout(krate("core"));
    layout.separate_reference_types(&fixture.items);
    layout.validate(&fixture.items, &placement).unwrap();
    let owner = fixture.items.component_root(fixture.owner);
    let card = fixture.items.component_root(fixture.card);
    let group = ModelFeature {
        consumer_crate: krate("core"),
        name: "card".into(),
        items: vec![card],
        requests: vec![],
    };
    let plan = ModelAvailability::build(&fixture.items, [], &layout, &[owner], &[group]).unwrap();
    assert_eq!(plan.site(layout.placement(owner).emission_site()), &Availability::Always);
    assert!(
        !plan.dependencies()[&layout.placement(card).emission_site()]
            .contains(&layout.placement(owner).emission_site())
    );
    let mut output = GeneratedWorkspace::default();
    crate::facade::write_facade(
        OutputLocations::new(&fixture.components, &placement),
        &fixture.items,
        &layout,
        &plan,
        &[("card".into(), fixture.card)],
        "1.0.0",
        &mut output,
    )
    .unwrap();
    let exports =
        std::str::from_utf8(output.contents("async-stripe/src/generated/mod.rs").unwrap()).unwrap();
    assert!(exports.contains("pub trait PaymentMethodDetailsExt"));
    assert!(exports.contains("#[cfg(feature = \"card\")]"));
    assert!(exports.contains("fn card("));
    assert!(!exports.contains("feature = \"details\""));
}

#[test]
fn decoder_metadata_does_not_create_generated_crate_dependencies() {
    let mut fixture = RecordFixture::new();
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let (placement, layout) = fixture.layout(krate("core"));
    let dependencies = crate_dependencies(&fixture.items, [], &placement, &layout).unwrap();
    validate_crate_dependencies(&dependencies).unwrap();
    assert!(!dependencies.contains_edge(Crate::SHARED, krate("core")));
}

#[test]
fn ambiguous_open_records_keep_their_fields_typed() {
    let mut fixture = RecordFixture::new();
    let owner = fixture.items.component_root(fixture.owner);
    let RustItem::Object(object) = &mut fixture.items.items[owner.index()].item else { panic!() };
    let RustObject::Struct(strukt) = &mut object.definition else { panic!() };
    let field = strukt.fields.iter_mut().find(|field| field.field_name == "card").unwrap();
    field.rust_type = RustType::string();
    fixture.items.use_opaque_relationships(&fixture.components).unwrap();
    let object = fixture.items.get(owner).object().unwrap();
    assert!(object.deferred_fields.is_empty());
    assert_eq!(object.definition.get_struct_field("card"), Some(&RustType::string()));
}
