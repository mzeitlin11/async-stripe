use super::{ItemCatalog, ItemId, ItemMetadata, RustItem};
use crate::component_catalog::{ComponentCatalog, ComponentId};
use crate::components::Components;
use crate::crate_dependencies::{crate_dependencies, validate_crate_dependencies};
use crate::crates::{ALL_CRATES, Crate};
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::layout::ComponentLayout;
use crate::object_writing::ItemEmitter;
use crate::operations::{
    MethodType, OperationCatalog, OperationType, RequestParam, RequestSpec, StripeOperation,
};
use crate::output::GeneratedWorkspace;
use crate::output_locations::OutputLocations;
use crate::render::RenderContext;
use crate::request_model::{RequestCatalog, RequestModel};
use crate::rust_object::{
    EnumVariant, FieldlessVariant, ObjectMetadata, RustObject, Struct, StructField, Visibility,
};
use crate::rust_type::RustType;
use crate::stripe_object::{StripeObject, StripeResource};
use crate::types::{ComponentName, RustIdent};
use crate::webhook::{WebhookObject, write_generated_for_webhooks};

fn krate(name: &str) -> Crate {
    *ALL_CRATES.iter().find(|krate| krate.base_name() == name).unwrap()
}

struct Fixture {
    components: Components,
    component_layout: ComponentLayout,
    items: ItemCatalog,
    customer: ComponentId,
    charge: ComponentId,
    request: RequestModel,
}

impl Fixture {
    fn new() -> Self {
        let catalog = ComponentCatalog::from_names(
            ["customer", "charge"].map(|name| ComponentName::new(name.to_string())),
        )
        .unwrap();
        let customer = catalog.id_str("customer").unwrap();
        let charge = catalog.id_str("charge").unwrap();
        let schema = serde_json::from_value(serde_json::json!({
            "type": "object", "properties": {"id": {"type": "string", "description": "Unique identifier for the object."}}, "required": ["id"]
        }))
        .unwrap();
        let objects = [customer, charge]
            .map(|component| {
                let resource =
                    StripeResource::from_schema(&schema, catalog.name(component).clone()).unwrap();
                (component, StripeObject::from_schema(&schema, &resource, &catalog))
            })
            .into_iter()
            .collect();
        let operation = StripeOperation {
            method_name: "retrieve".to_string(),
            method_type: MethodType::Retrieve,
            method_on: "service".to_string(),
            operation_type: OperationType::Get,
            path: "/v1/customers".to_string(),
        };
        let mut operations = OperationCatalog::from_operations([
            (customer, std::slice::from_ref(&operation)),
            (charge, &[]),
        ])
        .unwrap();
        let mode = RustType::Object(
            RustObject::FieldlessEnum(vec![FieldlessVariant::new(
                "simple".to_string(),
                RustIdent::create("simple"),
            )]),
            ObjectMetadata::new(RustIdent::create("customer_params_mode")),
        );
        let mut charge_field = StructField::new("charge", RustType::component(charge));
        charge_field.required = true;
        let params = RustType::Object(
            RustObject::Struct(Struct::new(vec![
                charge_field,
                StructField::new("mode", RustType::option(mode)),
            ])),
            ObjectMetadata::new(RustIdent::create("customer_params")),
        );
        let returned = RustType::Object(
            RustObject::Struct(Struct::new(vec![])),
            ObjectMetadata::new(RustIdent::create("customer_response")),
        );
        operations
            .attach_requests(
                customer,
                vec![RequestSpec {
                    operation_id: operations.for_component(customer)[0],
                    ident: RustIdent::create("retrieve_customer"),
                    path_params: vec![],
                    params: Some(RequestParam { typ: params }),
                    method_type: OperationType::Get,
                    returned,
                    doc_comment: None,
                    req_path: "/customers".to_string(),
                }],
            )
            .unwrap();
        operations.attach_requests(charge, vec![]).unwrap();
        let components = Components::for_test(catalog, objects, operations);
        let items = ItemCatalog::build(&components, |component| {
            ItemMetadata::new(components.get_by_id(component).ident().clone())
        })
        .unwrap();
        let requests = RequestCatalog::build(&components, &items).unwrap();
        let request = requests.for_component(customer)[0].clone();
        let component_layout = ComponentLayout::for_test(&[
            (customer, krate("core"), krate("core")),
            (charge, krate("billing"), krate("billing")),
        ]);
        Self { components, component_layout, items, customer, charge, request }
    }

    fn layout(&self, moved: &[(ItemId, Crate)]) -> ItemLayout {
        let default = ItemLayout::from_component_layout(
            &self.components,
            &self.items,
            &self.component_layout,
        )
        .unwrap();
        let layout = ItemLayout::for_test(
            self.items
                .iter()
                .map(|(item, _)| {
                    if let Some(&(_, krate)) = moved.iter().find(|(moved, _)| *moved == item) {
                        (EmissionSite::Standalone { item, krate }, ItemAddress::CrateRoot(krate))
                    } else {
                        let placement = default.placement(item);
                        (placement.emission_site(), placement.address())
                    }
                })
                .collect(),
        );
        layout.validate(&self.items, &self.component_layout).unwrap();
        layout
    }

    fn emit(&self, layout: &ItemLayout) -> anyhow::Result<GeneratedWorkspace> {
        let mut output = GeneratedWorkspace::default();
        let mut emitter = ItemEmitter::new(
            &self.items,
            layout,
            OutputLocations::new(&self.components, &self.component_layout),
        );
        for component in [self.customer, self.charge] {
            emitter.emit(self.items.component_root(component), &mut output)?;
            emitter.emit(self.items.component_id_item(component).unwrap(), &mut output)?;
            if component == self.customer {
                emitter.requests(component, std::slice::from_ref(&self.request), &mut output)?;
            }
            emitter.component_reexports(component, &mut output)?;
        }
        emitter.finish()?;
        Ok(output)
    }
}

fn contents<'a>(output: &'a GeneratedWorkspace, path: &str) -> &'a str {
    std::str::from_utf8(output.contents(path).unwrap()).unwrap()
}

#[test]
fn webhook_union_references_and_feature_gates_follow_its_placement() {
    let mut fixture = Fixture::new();
    fixture.components.webhook_objs.push(WebhookObject {
        event_type: "test.created".to_string(),
        doc: "An event.".to_string(),
        typ: RustType::Object(
            RustObject::Enum(vec![
                EnumVariant::new(
                    RustIdent::create("customer"),
                    RustType::component(fixture.customer),
                ),
                EnumVariant::new(RustIdent::create("charge"), RustType::component(fixture.charge)),
            ]),
            ObjectMetadata::new(RustIdent::create("test_created")),
        ),
    });
    fixture.items = ItemCatalog::build(&fixture.components, |component| {
        ItemMetadata::new(fixture.components.get_by_id(component).ident().clone())
    })
    .unwrap();
    let item = fixture
        .items
        .root(&TypeRoot::Webhook("test.created".to_string()))
        .unwrap()
        .declaration
        .unwrap();
    for target in [None, Some(Crate::SHARED), Some(krate("payment"))] {
        let moved = target.map(|krate| (item, krate)).into_iter().collect::<Vec<_>>();
        let layout = fixture.layout(&moved);
        let mut output = GeneratedWorkspace::default();
        write_generated_for_webhooks(
            RenderContext::new(&layout, &fixture.items, EmissionSite::Webhooks),
            &fixture.components.webhook_objs,
            &mut output,
        )
        .unwrap();
        let webhook = contents(&output, "async-stripe-webhook/mod.rs");
        let name = target.map_or_else(
            || "TestCreated".to_string(),
            |krate| format!("{}::TestCreated", krate.lib_name()),
        );
        assert!(webhook.contains(&format!("TestCreated(Box<{name}>),")));
        assert_eq!(
            webhook.matches("#[cfg(feature = \"async-stripe-payment\")]").count(),
            if target == Some(krate("payment")) { 3 } else { 0 }
        );
    }
}

#[test]
fn moving_an_id_changes_its_file_references_and_facade_export() {
    let fixture = Fixture::new();
    let id = fixture.items.component_id_item(fixture.customer).unwrap();
    let layout = fixture.layout(&[(id, Crate::SHARED)]);
    let graph = crate_dependencies(
        &fixture.items,
        [(fixture.customer, std::slice::from_ref(&fixture.request))],
        &fixture.component_layout,
        &layout,
    )
    .unwrap();
    validate_crate_dependencies(&graph).unwrap();
    let output = fixture.emit(&layout).unwrap();
    let model = contents(&output, "crates/async-stripe-core/src/customer/types.rs");
    assert!(model.contains("stripe_shared::CustomerId"));
    assert!(!model.contains("def_id!(CustomerId)"));
    assert_eq!(
        contents(&output, "crates/async-stripe-shared/src/customer_id.rs")
            .matches("def_id!(CustomerId)")
            .count(),
        1
    );
    assert!(
        contents(&output, "crates/async-stripe-shared/src/mod.rs")
            .contains("pub use customer_id::CustomerId;")
    );
    assert!(
        contents(&output, "crates/async-stripe-core/src/mod.rs")
            .contains("pub use stripe_shared::CustomerId;")
    );
}

#[test]
fn moved_request_declarations_and_nested_types_follow_their_own_placements() {
    let fixture = Fixture::new();
    let params = fixture.request.params.as_ref().unwrap().declaration.unwrap();
    let child = fixture.items.get(params).children[0];
    let returned = fixture.request.returned.declaration.unwrap();
    let layout = fixture.layout(&[
        (params, krate("payment")),
        (child, Crate::SHARED),
        (returned, Crate::SHARED),
    ]);
    let graph = crate_dependencies(
        &fixture.items,
        [(fixture.customer, std::slice::from_ref(&fixture.request))],
        &fixture.component_layout,
        &layout,
    )
    .unwrap();
    validate_crate_dependencies(&graph).unwrap();
    // The request constructor names Charge directly, in addition to referring
    // to the parameter declaration in payment. Both direct edges are required.
    assert!(graph.contains_edge(krate("core"), krate("billing")));
    assert!(graph.contains_edge(krate("core"), krate("payment")));
    let output = fixture.emit(&layout).unwrap();
    let requests = contents(&output, "crates/async-stripe-core/src/customer/requests.rs");
    assert!(requests.contains("stripe_payment::CustomerParams"));
    assert!(requests.contains("stripe_shared::CustomerResponse"));
    assert!(requests.contains("stripe_shared::CustomerParamsMode"));
    assert!(!requests.contains("pub struct CustomerParams"));
    assert!(!requests.contains("pub struct CustomerResponse"));
    assert!(!requests.contains("pub enum CustomerParamsMode"));
    assert!(
        contents(&output, "crates/async-stripe-payment/src/customer_params.rs")
            .contains("pub struct CustomerParams")
    );
    assert!(
        contents(&output, "crates/async-stripe-shared/src/customer_response.rs")
            .contains("pub struct CustomerResponse")
    );
    assert!(
        contents(&output, "crates/async-stripe-shared/src/customer_params_mode.rs")
            .contains("pub enum CustomerParamsMode")
    );
}

#[test]
fn moving_a_parent_does_not_make_its_local_children_visible_elsewhere() {
    let fixture = Fixture::new();
    let params = fixture.request.params.as_ref().unwrap().declaration.unwrap();
    let layout = fixture.layout(&[(params, krate("payment"))]);
    let error = crate_dependencies(
        &fixture.items,
        [(fixture.customer, std::slice::from_ref(&fixture.request))],
        &fixture.component_layout,
        &layout,
    )
    .unwrap_err();
    assert!(format!("{error:#}").contains("is not visible"));
}

#[test]
fn declarations_must_be_emitted_exactly_once() {
    let fixture = Fixture::new();
    let layout = fixture.layout(&[]);
    let mut output = GeneratedWorkspace::default();
    let mut emitter = ItemEmitter::new(
        &fixture.items,
        &layout,
        OutputLocations::new(&fixture.components, &fixture.component_layout),
    );
    let model = fixture.items.component_root(fixture.customer);
    emitter.emit(model, &mut output).unwrap();
    assert!(emitter.emit(model, &mut output).unwrap_err().to_string().contains("emitted twice"));
    // Emitting a model no longer implicitly emits its ID.
    assert!(emitter.finish().unwrap_err().to_string().contains("CustomerId"));
}

#[test]
fn different_standalone_modules_cannot_silently_share_a_file() {
    let mut fixture = Fixture::new();
    let id = fixture.items.component_id_item(fixture.customer).unwrap();
    let returned = fixture.request.returned.declaration.unwrap();
    fixture.items.items[returned.index()].metadata.ident = RustIdent::unchanged("CustomerID");
    let layout = fixture.layout(&[(id, Crate::SHARED), (returned, Crate::SHARED)]);
    assert!(fixture.emit(&layout).unwrap_err().to_string().contains("different emission sites"));
}

#[test]
fn placing_a_private_declaration_in_a_public_module_does_not_change_its_visibility() {
    let mut fixture = Fixture::new();
    let params = fixture.request.params.as_ref().unwrap().declaration.unwrap();
    let child = fixture.items.get(params).children[0];
    let layout = fixture.layout(&[(params, krate("payment")), (child, Crate::SHARED)]);
    let RustItem::Object(object) = &mut fixture.items.items[params.index()].item else { panic!() };
    let RustObject::Struct(strukt) = &mut object.definition else { panic!() };
    strukt.vis = Visibility::Private;
    assert!(
        layout
            .validate(&fixture.items, &fixture.component_layout)
            .unwrap_err()
            .to_string()
            .contains("private declaration")
    );
}

#[test]
fn request_setters_cannot_access_private_fields_in_another_module() {
    let mut fixture = Fixture::new();
    let params = fixture.request.params.as_ref().unwrap().declaration.unwrap();
    let child = fixture.items.get(params).children[0];
    let layout = fixture.layout(&[(params, krate("payment")), (child, Crate::SHARED)]);
    // Required values go through the parameter type's public constructor.
    let RustItem::Object(object) = &mut fixture.items.items[params.index()].item else { panic!() };
    let RustObject::Struct(strukt) = &mut object.definition else { panic!() };
    strukt.fields[0].vis = Visibility::Private;
    crate_dependencies(
        &fixture.items,
        [(fixture.customer, std::slice::from_ref(&fixture.request))],
        &fixture.component_layout,
        &layout,
    )
    .unwrap();
    // Optional values are set directly on the inner struct by the wrapper.
    let RustItem::Object(object) = &mut fixture.items.items[params.index()].item else { panic!() };
    let RustObject::Struct(strukt) = &mut object.definition else { panic!() };
    strukt.fields[1].vis = Visibility::Private;
    let error = crate_dependencies(
        &fixture.items,
        [(fixture.customer, std::slice::from_ref(&fixture.request))],
        &fixture.component_layout,
        &layout,
    )
    .unwrap_err();
    assert!(format!("{error:#}").contains("cannot set private parameter field mode"));
}

#[test]
fn moving_a_model_and_its_id_keeps_the_request_module_and_public_facade() {
    let fixture = Fixture::new();
    let model = fixture.items.component_root(fixture.customer);
    let id = fixture.items.component_id_item(fixture.customer).unwrap();
    let layout = fixture.layout(&[(model, Crate::SHARED), (id, Crate::SHARED)]);
    let graph = crate_dependencies(
        &fixture.items,
        [(fixture.customer, std::slice::from_ref(&fixture.request))],
        &fixture.component_layout,
        &layout,
    )
    .unwrap();
    validate_crate_dependencies(&graph).unwrap();
    let output = fixture.emit(&layout).unwrap();
    assert!(
        contents(&output, "crates/async-stripe-shared/src/customer.rs")
            .contains("pub struct Customer")
    );
    assert!(output.contents("crates/async-stripe-core/src/customer/types.rs").is_none());
    let facade = contents(&output, "crates/async-stripe-core/src/mod.rs");
    assert!(facade.contains("pub mod customer;"));
    assert!(facade.contains("pub use stripe_shared::Customer;"));
    assert!(facade.contains("pub use stripe_shared::CustomerId;"));
    assert!(
        !contents(&output, "crates/async-stripe-core/src/customer/mod.rs").contains("mod types;")
    );
}

#[test]
fn a_standalone_model_cannot_shadow_its_request_directory() {
    let fixture = Fixture::new();
    let model = fixture.items.component_root(fixture.customer);
    let layout = fixture.layout(&[(model, krate("core"))]);
    assert!(fixture.emit(&layout).unwrap_err().to_string().contains("conflicting owners"));
}
