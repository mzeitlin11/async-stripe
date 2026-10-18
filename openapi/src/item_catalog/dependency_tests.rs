use super::{CanonicalTypeRoot, CatalogBuilder, ItemCatalog, ItemId, ItemMetadata, RustItem};
use crate::component_catalog::{ComponentCatalog, ComponentId};
use crate::crate_dependencies::{crate_dependencies, validate_crate_dependencies};
use crate::crates::{ALL_CRATES, Crate};
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::layout::ComponentLayout;
use crate::operations::OperationType;
use crate::request_model::{RequestModel, RequestPathParam};
use crate::rust_object::{
    EnumVariant, ObjectIdentity, ObjectUsage, RustObject, Struct, StructField,
};
use crate::rust_type::{DeclarationRoot, RustType};
use crate::types::{ComponentName, RustIdent};

fn krate(name: &str) -> Crate {
    *ALL_CRATES.iter().find(|krate| krate.base_name() == name).unwrap()
}

fn components() -> (ComponentId, ComponentId, ComponentId) {
    let catalog = ComponentCatalog::from_names(
        ["charge", "customer", "other"].map(|name| ComponentName::new(name.to_string())),
    )
    .unwrap();
    (
        catalog.id_str("charge").unwrap(),
        catalog.id_str("customer").unwrap(),
        catalog.id_str("other").unwrap(),
    )
}

fn collect(builder: &mut CatalogBuilder, component: ComponentId, object: RustObject) -> ItemId {
    builder
        .collect_root_object(
            TypeRoot::Declaration(DeclarationRoot::Component(component)),
            &object,
            ItemMetadata::new(RustIdent::create("test_object")),
            ObjectUsage::type_def(),
        )
        .unwrap()
}

fn finish(builder: CatalogBuilder) -> ItemCatalog {
    let mut items = builder.finish().unwrap();
    items.lower_named_references().unwrap();
    items.validate_lowered_structure().unwrap();
    items
}

fn placement(component: ComponentId, krate: Crate) -> (EmissionSite, ItemAddress) {
    (EmissionSite::ComponentTypes(component), ItemAddress::CrateRoot(krate))
}

#[test]
fn id_references_follow_the_id_item_instead_of_the_model() {
    let (charge, customer, other) = components();
    let (core, billing, payment) = (krate("core"), krate("billing"), krate("payment"));
    let mut builder = CatalogBuilder::default();
    collect(
        &mut builder,
        charge,
        RustObject::Struct(Struct::new(vec![StructField::new(
            "customer",
            RustType::option(RustType::vec(RustType::reference(RustType::object_id(customer)))),
        )])),
    );
    collect(&mut builder, customer, RustObject::Struct(Struct::new(vec![])));
    builder.collect_id_newtype(customer, RustIdent::create("customer_id")).unwrap();
    let items = finish(builder);
    let component_layout = ComponentLayout::for_test(&[
        (charge, core, core),
        (customer, billing, billing),
        (other, payment, payment),
    ]);
    // A counterfactual item layout: CustomerId lives with payment types while
    // Customer remains in billing. Component identity must not decide the edge.
    let layout = ItemLayout::for_test(vec![
        placement(charge, core),
        placement(customer, billing),
        placement(other, payment),
    ]);
    let graph = crate_dependencies(&items, [], &component_layout, &layout).unwrap();
    assert!(graph.contains_edge(core, payment));
    assert!(!graph.contains_edge(core, billing));
    assert!(!graph.contains_edge(core, core));
    validate_crate_dependencies(&graph).unwrap();
}

#[test]
fn model_and_request_dependencies_use_their_own_source_crates() {
    let (charge, customer, other) = components();
    let (core, billing, payment) = (krate("core"), krate("billing"), krate("payment"));
    let treasury = krate("treasury");
    let mut builder = CatalogBuilder::default();
    collect(
        &mut builder,
        charge,
        RustObject::Enum(vec![
            EnumVariant::new(
                RustIdent::create("customer"),
                RustType::boxed(RustType::component(customer)),
            ),
            EnumVariant::fieldless(RustIdent::create("unknown")),
        ]),
    );
    collect(&mut builder, customer, RustObject::Struct(Struct::new(vec![])));
    let other_item = collect(&mut builder, other, RustObject::Struct(Struct::new(vec![])));
    let items = finish(builder);
    let component_layout = ComponentLayout::for_test(&[
        (charge, core, billing),
        (customer, payment, payment),
        (other, treasury, treasury),
    ]);
    let layout = ItemLayout::for_test(vec![
        placement(charge, billing),
        placement(customer, payment),
        placement(other, treasury),
    ]);
    // Each signature position is independently required, including roots that
    // reference an existing item without introducing a request declaration.
    for position in ["path", "params", "return"] {
        let root = CanonicalTypeRoot { typ: RustType::Item(other_item), declaration: None };
        let request = RequestModel {
            ident: RustIdent::create("get_charge"),
            path_params: if position == "path" {
                vec![RequestPathParam { name: "id".to_string(), typ: root.typ.clone() }]
            } else {
                vec![]
            },
            params: (position == "params").then(|| root.clone()),
            method_type: OperationType::Get,
            returned: if position == "return" {
                root
            } else {
                CanonicalTypeRoot { typ: RustType::bool(), declaration: None }
            },
            doc_comment: None,
            req_path: "/charges".to_string(),
        };
        let requests = [request];
        let graph =
            crate_dependencies(&items, [(charge, requests.as_slice())], &component_layout, &layout)
                .unwrap();
        assert!(graph.contains_edge(billing, payment), "{position}");
        assert!(graph.contains_edge(core, treasury), "{position}");
        assert!(!graph.contains_edge(core, payment), "{position}");
        assert!(!graph.contains_edge(billing, treasury), "{position}");
        validate_crate_dependencies(&graph).unwrap();
    }
}

#[test]
fn object_impl_associated_id_type_is_a_dependency() {
    let (charge, customer, _) = components();
    let (core, billing) = (krate("core"), krate("billing"));
    let mut builder = CatalogBuilder::default();
    let charge_item = collect(&mut builder, charge, RustObject::Struct(Struct::new(vec![])));
    builder.collect_id_newtype(customer, RustIdent::create("customer_id")).unwrap();
    let RustItem::Object(object) = &mut builder.item_mut(charge_item).item else { panic!() };
    // Deliberately absent from the fields, so the impl must be visited itself.
    object.identity = Some(ObjectIdentity::IdType(RustType::object_id(customer)));
    let items = finish(builder);
    let component_layout =
        ComponentLayout::for_test(&[(charge, core, core), (customer, billing, billing)]);
    let layout = ItemLayout::for_test(vec![placement(charge, core), placement(customer, billing)]);
    let graph = crate_dependencies(&items, [], &component_layout, &layout).unwrap();
    assert!(graph.contains_edge(core, billing));
}

#[test]
fn local_items_must_be_visible_from_the_emission_site_even_within_one_crate() {
    let (charge, customer, _) = components();
    let core = krate("core");
    let mut builder = CatalogBuilder::default();
    collect(
        &mut builder,
        charge,
        RustObject::Struct(Struct::new(vec![StructField::new(
            "customer",
            RustType::component(customer),
        )])),
    );
    collect(&mut builder, customer, RustObject::Struct(Struct::new(vec![])));
    let items = finish(builder);
    let component_layout =
        ComponentLayout::for_test(&[(charge, core, core), (customer, core, core)]);
    let same_module = ItemLayout::for_test(vec![
        placement(charge, core),
        (EmissionSite::ComponentTypes(charge), ItemAddress::Local),
    ]);
    let graph = crate_dependencies(&items, [], &component_layout, &same_module).unwrap();
    assert!(!graph.contains_edge(core, core));
    for site in [EmissionSite::ComponentTypes(customer), EmissionSite::ComponentRequests(charge)] {
        let different_module =
            ItemLayout::for_test(vec![placement(charge, core), (site, ItemAddress::Local)]);
        let error =
            crate_dependencies(&items, [], &component_layout, &different_module).unwrap_err();
        assert!(format!("{error:#}").contains("is not visible"));
    }
}

#[test]
fn counterfactual_cycles_are_reportable_but_rejected_before_generation() {
    let (charge, customer, _) = components();
    let core = krate("core");
    let mut builder = CatalogBuilder::default();
    collect(
        &mut builder,
        charge,
        RustObject::Struct(Struct::new(vec![StructField::new(
            "customer",
            RustType::expandable(RustType::component(customer)),
        )])),
    );
    collect(&mut builder, customer, RustObject::Struct(Struct::new(vec![])));
    let items = finish(builder);
    let component_layout =
        ComponentLayout::for_test(&[(charge, core, Crate::SHARED), (customer, core, core)]);
    let layout =
        ItemLayout::for_test(vec![placement(charge, Crate::SHARED), placement(customer, core)]);
    let graph = crate_dependencies(&items, [], &component_layout, &layout).unwrap();
    assert!(graph.contains_edge(Crate::SHARED, core));
    assert!(graph.contains_edge(core, Crate::SHARED));
    assert!(validate_crate_dependencies(&graph).unwrap_err().to_string().contains("cycle"));
}
