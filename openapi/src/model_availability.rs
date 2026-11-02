use std::collections::{BTreeSet, HashMap, HashSet};

use anyhow::{Context as _, ensure};

use crate::component_catalog::ComponentId;
use crate::item_catalog::{ItemCatalog, ItemId};
use crate::item_layout::{EmissionSite, ItemLayout};
use crate::request_model::RequestModel;
use crate::rust_type::RustType;

/// A useful public feature and the model/request roots it makes available.
/// Dependencies follow the selected Rust representation, rather than the
/// original semantic relationships recorded during inference.
#[derive(Debug, Clone)]
pub(crate) struct ModelFeature {
    pub name: String,
    pub items: Vec<ItemId>,
    pub requests: Vec<ComponentId>,
}

/// The condition under which a generated source module is compiled.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Availability {
    Always,
    Features(BTreeSet<String>),
    Disabled,
}

impl Availability {
    /// A component's outer module can contain independently enabled model and
    /// request modules. It must exist whenever either child is compiled.
    pub(crate) fn union(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Always, _) | (_, Self::Always) => Self::Always,
            (Self::Disabled, other) | (other, Self::Disabled) => other.clone(),
            (Self::Features(left), Self::Features(right)) => {
                Self::Features(left.union(right).cloned().collect())
            }
        }
    }

    /// Prefix a module or public reexport with its compilation condition.
    pub(crate) fn cfg_attribute(&self) -> String {
        match self {
            Self::Always => String::new(),
            Self::Disabled => "#[cfg(any())]\n".to_string(),
            Self::Features(features) => {
                let predicates = features
                    .iter()
                    .map(|feature| format!("feature = {feature:?}"))
                    .collect::<Vec<_>>();
                if predicates.len() == 1 {
                    format!("#[cfg({})]\n", predicates[0])
                } else {
                    format!("#[cfg(any({}))]\n", predicates.join(", "))
                }
            }
        }
    }
}

/// Compile-time availability at the granularity of emitted source modules.
///
/// Every declaration in a module is compiled together. Closing dependencies at
/// item granularity alone would miss references in siblings that share a file.
/// This plan is built after representation and placement have been selected.
#[derive(Debug, Clone)]
pub(crate) struct ModelAvailability {
    sites: HashMap<EmissionSite, Availability>,
    features: BTreeSet<String>,
    dependencies: HashMap<EmissionSite, HashSet<EmissionSite>>,
}

impl ModelAvailability {
    pub(crate) fn build<'a>(
        items: &ItemCatalog,
        requests: impl IntoIterator<Item = (ComponentId, &'a [RequestModel])>,
        layout: &ItemLayout,
        unconditional: &[ItemId],
        features: &[ModelFeature],
    ) -> anyhow::Result<Self> {
        let mut graph: HashMap<EmissionSite, HashSet<EmissionSite>> = HashMap::new();
        for (item, _) in items.iter() {
            graph.entry(layout.placement(item).emission_site()).or_default();
        }
        for (item, record) in items.iter() {
            let source = layout.placement(item).emission_site();
            let dependencies = graph.get_mut(&source).expect("item site was not registered");
            record.visit_type_uses(|typ| collect_type_sites(typ, layout, dependencies))?;
        }
        for (component, requests) in requests {
            if requests.is_empty() {
                continue;
            }
            let dependencies = graph.entry(EmissionSite::ComponentRequests(component)).or_default();
            for request in requests {
                request
                    .visit_type_uses(items, |typ| collect_type_sites(typ, layout, dependencies))
                    .with_context(|| format!("invalid availability for {}", request.ident))?;
            }
        }

        let always = closure(
            &graph,
            unconditional.iter().map(|item| layout.placement(*item).emission_site()),
        )?;
        let mut sites =
            graph.keys().map(|&site| (site, Availability::Disabled)).collect::<HashMap<_, _>>();
        for site in always {
            sites.insert(site, Availability::Always);
        }
        let mut names = BTreeSet::new();
        for feature in features {
            ensure!(!feature.name.is_empty(), "model feature name cannot be empty");
            ensure!(
                names.insert(feature.name.clone()),
                "duplicate model feature: {}",
                feature.name
            );
            let roots = feature
                .items
                .iter()
                .map(|item| layout.placement(*item).emission_site())
                .chain(feature.requests.iter().copied().map(EmissionSite::ComponentRequests));
            for site in closure(&graph, roots)? {
                match sites.get_mut(&site).expect("closed site was not registered") {
                    Availability::Always => {}
                    availability @ Availability::Disabled => {
                        *availability =
                            Availability::Features(BTreeSet::from([feature.name.clone()]));
                    }
                    Availability::Features(groups) => {
                        groups.insert(feature.name.clone());
                    }
                }
            }
        }
        Ok(Self { sites, features: names, dependencies: graph })
    }

    pub(crate) fn site(&self, site: EmissionSite) -> &Availability {
        self.sites.get(&site).unwrap_or(&Availability::Disabled)
    }

    pub(crate) fn features(&self) -> &BTreeSet<String> {
        &self.features
    }

    /// The represented module dependencies used to compute these conditions.
    pub(crate) fn dependencies(&self) -> &HashMap<EmissionSite, HashSet<EmissionSite>> {
        &self.dependencies
    }
}

fn collect_type_sites(
    typ: &RustType,
    layout: &ItemLayout,
    sites: &mut HashSet<EmissionSite>,
) -> anyhow::Result<()> {
    match typ {
        RustType::Item(item) => {
            sites.insert(layout.placement(*item).emission_site());
        }
        RustType::Container(container) => collect_type_sites(container.value_typ(), layout, sites)?,
        RustType::Reference(inner) => collect_type_sites(inner, layout, sites)?,
        RustType::Primitive(_) | RustType::External(_) => {}
        RustType::Object(_, _) | RustType::Path(_) => {
            anyhow::bail!("unlowered type in model availability: {typ:?}")
        }
    }
    Ok(())
}

fn closure(
    graph: &HashMap<EmissionSite, HashSet<EmissionSite>>,
    roots: impl IntoIterator<Item = EmissionSite>,
) -> anyhow::Result<HashSet<EmissionSite>> {
    let mut seen = HashSet::new();
    let mut pending = roots.into_iter().collect::<Vec<_>>();
    while let Some(site) = pending.pop() {
        if !seen.insert(site) {
            continue;
        }
        let dependencies = graph.get(&site).with_context(|| {
            format!("availability root or dependency has no emitted declaration: {site:?}")
        })?;
        pending.extend(dependencies.iter().copied());
    }
    Ok(seen)
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use super::{Availability, ModelAvailability, ModelFeature};
    use crate::component_catalog::{ComponentCatalog, ComponentId};
    use crate::components::Components;
    use crate::crates::{ALL_CRATES, Crate};
    use crate::item_catalog::{CanonicalTypeRoot, ItemCatalog, ItemId, ItemMetadata, RustItem};
    use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
    use crate::layout::ComponentLayout;
    use crate::operations::{OperationCatalog, OperationType, StripeOperation};
    use crate::request_model::{RequestModel, RequestPathParam};
    use crate::rust_type::RustType;
    use crate::stripe_object::{StripeObject, StripeResource};
    use crate::types::{ComponentName, RustIdent};

    struct Fixture {
        items: ItemCatalog,
        layout: ItemLayout,
        charge: ComponentId,
        customer: ComponentId,
        other: ComponentId,
    }

    impl Fixture {
        fn new(with_ids: bool) -> Self {
            let catalog = ComponentCatalog::from_names(
                ["charge", "customer", "other"].map(|name| ComponentName::new(name.to_string())),
            )
            .unwrap();
            let charge = catalog.id_str("charge").unwrap();
            let customer = catalog.id_str("customer").unwrap();
            let other = catalog.id_str("other").unwrap();
            let mut objects = IndexMap::new();
            for (component, _) in catalog.iter() {
                let mut schema = if component == charge {
                    serde_json::json!({
                        "type": "object", "properties": {
                            "details": {"type": "object", "properties": {"note": {"type": "string"}}},
                            "customer": {"$ref": "#/components/schemas/customer"}
                        }
                    })
                } else if component == other {
                    serde_json::json!({"type": "object", "properties": {"customer": {"$ref": "#/components/schemas/customer"}}})
                } else {
                    serde_json::json!({"type": "object", "properties": {"email": {"type": "string"}}})
                };
                if with_ids {
                    schema["properties"]["id"] = serde_json::json!({
                        "type": "string", "description": "Unique identifier for the object."
                    });
                    schema["required"] = serde_json::json!(["id"]);
                }
                let schema = serde_json::from_value(schema).unwrap();
                let resource =
                    StripeResource::from_schema(&schema, catalog.name(component).clone()).unwrap();
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
            let core = *ALL_CRATES.iter().find(|krate| krate.base_name() == "core").unwrap();
            let component_layout = ComponentLayout::for_test(&[
                (charge, core, core),
                (customer, core, core),
                (other, core, core),
            ]);
            let default =
                ItemLayout::from_component_layout(&components, &items, &component_layout).unwrap();
            // IDs are independently available; sharing their model's file would
            // necessarily make the model available with them.
            let layout = ItemLayout::for_test(
                items
                    .iter()
                    .map(|(item, record)| {
                        if matches!(record.item, RustItem::IdNewtype) {
                            (
                                EmissionSite::Standalone { item, krate: Crate::SHARED },
                                ItemAddress::CrateRoot(Crate::SHARED),
                            )
                        } else {
                            let placement = default.placement(item);
                            (placement.emission_site(), placement.address())
                        }
                    })
                    .collect(),
            );
            layout.validate(&items, &component_layout).unwrap();
            Self { items, layout, charge, customer, other }
        }

        fn feature(&self, name: &str, component: ComponentId) -> ModelFeature {
            ModelFeature {
                name: name.to_string(),
                items: vec![self.items.component_root(component)],
                requests: vec![],
            }
        }

        fn build(&self, unconditional: &[ItemId], groups: &[ModelFeature]) -> ModelAvailability {
            ModelAvailability::build(&self.items, [], &self.layout, unconditional, groups).unwrap()
        }
    }

    fn enabled_by(names: &[&str]) -> Availability {
        Availability::Features(names.iter().map(|name| name.to_string()).collect())
    }

    #[test]
    fn independent_groups_add_their_conditions_to_shared_dependencies() {
        let fixture = Fixture::new(false);
        let plan = fixture.build(
            &[],
            &[fixture.feature("charge", fixture.charge), fixture.feature("other", fixture.other)],
        );
        assert_eq!(
            plan.site(EmissionSite::ComponentTypes(fixture.charge)),
            &enabled_by(&["charge"])
        );
        assert_eq!(
            plan.site(EmissionSite::ComponentTypes(fixture.customer)),
            &enabled_by(&["charge", "other"])
        );
        assert_eq!(plan.site(EmissionSite::ComponentTypes(fixture.other)), &enabled_by(&["other"]));
        assert_eq!(
            plan.features().iter().map(String::as_str).collect::<Vec<_>>(),
            ["charge", "other"]
        );
    }

    #[test]
    fn the_retained_graph_includes_disabled_modules_and_same_module_references() {
        let fixture = Fixture::new(false);
        let plan = fixture.build(&[], &[fixture.feature("charge", fixture.charge)]);
        let charge = EmissionSite::ComponentTypes(fixture.charge);
        let customer = EmissionSite::ComponentTypes(fixture.customer);
        let other = EmissionSite::ComponentTypes(fixture.other);
        assert!(plan.dependencies()[&charge].contains(&charge));
        assert!(plan.dependencies()[&charge].contains(&customer));
        assert!(plan.dependencies()[&other].contains(&customer));
        assert!(plan.dependencies()[&customer].is_empty());
        assert_eq!(plan.site(other), &Availability::Disabled);
    }

    #[test]
    fn unconditional_dependencies_win_without_enabling_unrelated_models() {
        let fixture = Fixture::new(false);
        let plan = fixture.build(
            &[fixture.items.component_root(fixture.other)],
            &[fixture.feature("charge", fixture.charge)],
        );
        assert_eq!(
            plan.site(EmissionSite::ComponentTypes(fixture.customer)),
            &Availability::Always
        );
        assert_eq!(plan.site(EmissionSite::ComponentTypes(fixture.other)), &Availability::Always);
        assert_eq!(
            plan.site(EmissionSite::ComponentTypes(fixture.charge)),
            &enabled_by(&["charge"])
        );
        let empty = fixture.build(&[], &[]);
        assert_eq!(
            empty.site(EmissionSite::ComponentTypes(fixture.charge)),
            &Availability::Disabled
        );
    }

    #[test]
    fn selecting_a_nested_declaration_includes_its_compiled_siblings_dependencies() {
        let fixture = Fixture::new(false);
        let charge = fixture.items.component_root(fixture.charge);
        let details = fixture.items.get(charge).children[0];
        let group =
            ModelFeature { name: "details".to_string(), items: vec![details], requests: vec![] };
        let plan = fixture.build(&[], &[group]);
        // The selected nested details struct has no Customer field, but its
        // enclosing source file contains Charge, which does reference Customer.
        assert_eq!(
            plan.site(EmissionSite::ComponentTypes(fixture.customer)),
            &enabled_by(&["details"])
        );
        assert_eq!(plan.site(EmissionSite::ComponentTypes(fixture.other)), &Availability::Disabled);
    }

    #[test]
    fn request_features_follow_path_parameter_and_return_types() {
        let fixture = Fixture::new(false);
        let request = RequestModel {
            ident: RustIdent::create("retrieve_charge"),
            path_params: vec![RequestPathParam {
                name: "id".to_string(),
                typ: RustType::Item(fixture.items.component_root(fixture.customer)),
            }],
            params: Some(CanonicalTypeRoot {
                typ: RustType::Item(fixture.items.component_root(fixture.other)),
                declaration: None,
            }),
            method_type: OperationType::Get,
            returned: CanonicalTypeRoot {
                typ: RustType::Item(fixture.items.component_root(fixture.charge)),
                declaration: None,
            },
            doc_comment: None,
            req_path: "/charges".to_string(),
        };
        let group = ModelFeature {
            name: "retrieve".to_string(),
            items: vec![],
            requests: vec![fixture.charge],
        };
        let plan = ModelAvailability::build(
            &fixture.items,
            [(fixture.charge, std::slice::from_ref(&request))],
            &fixture.layout,
            &[],
            &[group],
        )
        .unwrap();
        for site in [
            EmissionSite::ComponentRequests(fixture.charge),
            EmissionSite::ComponentTypes(fixture.charge),
            EmissionSite::ComponentTypes(fixture.customer),
            EmissionSite::ComponentTypes(fixture.other),
        ] {
            assert_eq!(plan.site(site), &enabled_by(&["retrieve"]));
        }
    }

    #[test]
    fn unconditional_ids_do_not_enable_their_models() {
        let fixture = Fixture::new(true);
        let ids = fixture
            .items
            .iter()
            .filter_map(|(item, record)| matches!(record.item, RustItem::IdNewtype).then_some(item))
            .collect::<Vec<_>>();
        assert_eq!(ids.len(), 3);
        let plan = fixture.build(&ids, &[]);
        for id in ids {
            assert_eq!(
                plan.site(fixture.layout.placement(id).emission_site()),
                &Availability::Always
            );
        }
        for component in [fixture.charge, fixture.customer, fixture.other] {
            assert_eq!(plan.site(EmissionSite::ComponentTypes(component)), &Availability::Disabled);
        }
    }
}
