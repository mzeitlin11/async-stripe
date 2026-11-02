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
    let group = ModelFeature { name: "charge".to_string(), items: vec![root], requests: vec![] };
    let plan = ModelAvailability::build(&items, [], &layout, &[], &[group]).unwrap();
    assert_eq!(plan.site(layout.placement(child).emission_site()), &Availability::Disabled);
    assert_eq!(plan.site(EmissionSite::ComponentTypes(customer)), &Availability::Disabled);
}
