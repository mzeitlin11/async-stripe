use std::collections::{BTreeSet, HashMap, HashSet};

use anyhow::{Context as _, ensure};

use crate::component_catalog::ComponentId;
use crate::crates::{Crate, CrateConfig};
use crate::item_catalog::{ItemCatalog, ItemId};
use crate::item_layout::{EmissionSite, ItemLayout};
use crate::layout::ComponentLayout;
use crate::request_model::RequestModel;

/// A useful public feature and the model/request roots it makes available.
#[derive(Debug, Clone)]
pub(crate) struct ModelFeature {
    pub name: String,
    pub items: Vec<ItemId>,
    pub requests: Vec<ComponentId>,
    pub consumer_crate: Crate,
}

/// Internal flags enable model declarations without their HTTP request builders.
fn model_feature_name(group: &str, has_requests_here: bool) -> String {
    if has_requests_here { format!("__model_{group}") } else { group.to_string() }
}

/// The condition under which a generated source module is compiled.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Availability {
    Always,
    Features(BTreeSet<String>),
    Disabled,
}

impl Availability {
    pub(crate) fn for_models(&self, has_requests_here: impl Fn(&str) -> bool) -> Self {
        match self {
            Self::Features(features) => Self::Features(
                features
                    .iter()
                    .map(|feature| model_feature_name(feature, has_requests_here(feature)))
                    .collect(),
            ),
            Self::Always | Self::Disabled => self.clone(),
        }
    }

    pub(crate) fn union(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Always, _) | (_, Self::Always) => Self::Always,
            (Self::Disabled, other) | (other, Self::Disabled) => other.clone(),
            (Self::Features(left), Self::Features(right)) => {
                Self::Features(left.union(right).cloned().collect())
            }
        }
    }

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

/// Availability closes actual uses at emitted-module granularity. Model flags
/// close model roots only. Public API flags additionally expose the models
/// returned/accepted by requests, without enabling those models' own endpoints.
#[derive(Debug, Clone)]
pub(crate) struct ModelAvailability {
    sites: HashMap<EmissionSite, Availability>,
    public_sites: HashMap<EmissionSite, Availability>,
    features: BTreeSet<String>,
    request_components: HashMap<String, Vec<ComponentId>>,
    request_models: HashMap<ComponentId, Vec<(EmissionSite, String)>>,
    required_request_models: BTreeSet<ComponentId>,
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
        let graph = crate::module_dependencies::module_dependencies(items, requests, layout)?;
        let always = closure(
            &graph,
            unconditional.iter().map(|item| layout.placement(*item).emission_site()),
        )?;
        let mut sites =
            graph.keys().map(|&site| (site, Availability::Disabled)).collect::<HashMap<_, _>>();
        for &site in &always {
            sites.insert(site, Availability::Always);
        }
        let mut public_sites = sites.clone();
        let mut names = BTreeSet::new();
        let mut request_components = HashMap::new();
        let mut model_closures = HashMap::new();
        let mut roots: HashMap<EmissionSite, BTreeSet<String>> = HashMap::new();
        for feature in features {
            ensure!(!feature.name.is_empty(), "model feature name cannot be empty");
            ensure!(
                names.insert(feature.name.clone()),
                "duplicate model feature: {}",
                feature.name
            );
            request_components.insert(feature.name.clone(), feature.requests.clone());
            let models = closure(
                &graph,
                feature.items.iter().map(|item| layout.placement(*item).emission_site()),
            )?;
            for &site in &models {
                enable(&mut sites, site, &feature.name);
            }
            let api = closure(
                &graph,
                models
                    .iter()
                    .copied()
                    .chain(feature.requests.iter().copied().map(EmissionSite::ComponentRequests)),
            )?;
            // Request source files compile under the public flag, never the
            // internal model flag. Their DTO requirements are forwarded below.
            for &component in &feature.requests {
                enable(&mut sites, EmissionSite::ComponentRequests(component), &feature.name);
            }
            for site in api {
                enable(&mut public_sites, site, &feature.name);
            }
            model_closures.insert(feature.name.clone(), models);
            // A single root owns exactly this source module's dependency
            // closure. A batched decoder feature could introduce unrelated DTOs.
            if let [root] = feature.items.as_slice() {
                roots
                    .entry(layout.placement(*root).emission_site())
                    .or_default()
                    .insert(feature.name.clone());
            }
        }

        let mut request_models: HashMap<ComponentId, Vec<(EmissionSite, String)>> = HashMap::new();
        let mut required_request_models = BTreeSet::new();
        for feature in features {
            let models = &model_closures[&feature.name];
            for &component in &feature.requests {
                let source = EmissionSite::ComponentRequests(component);
                for &target in graph
                    .get(&source)
                    .with_context(|| format!("request group has no emitted source: {source:?}"))?
                {
                    if target == source || always.contains(&target) || models.contains(&target) {
                        continue;
                    }
                    if let Some(group) = roots.get(&target).and_then(|names| names.first()) {
                        request_models.entry(component).or_default().push((target, group.clone()));
                    } else if let EmissionSite::ComponentTypes(component) = target {
                        // Group assembly registers these component roots as
                        // model-only features and rebuilds the availability plan.
                        required_request_models.insert(component);
                    } else {
                        anyhow::bail!(
                            "request {component:?} needs an independently selectable model for {target:?}"
                        );
                    }
                }
            }
        }

        Ok(Self {
            sites,
            public_sites,
            features: names,
            request_components,
            request_models,
            required_request_models,
            dependencies: graph,
        })
    }

    /// Conditions for model declarations; request sites retain public API flags.
    pub(crate) fn site(&self, site: EmissionSite) -> &Availability {
        self.sites.get(&site).unwrap_or(&Availability::Disabled)
    }

    /// Facade exports include model types exposed by an enabled HTTP API.
    pub(crate) fn public_site(&self, site: EmissionSite) -> &Availability {
        self.public_sites.get(&site).unwrap_or(&Availability::Disabled)
    }

    pub(crate) fn features(&self) -> &BTreeSet<String> {
        &self.features
    }
    pub(crate) fn request_components(&self, feature: &str) -> &[ComponentId] {
        self.request_components.get(feature).map(Vec::as_slice).unwrap_or_default()
    }
    pub(crate) fn required_request_models(&self) -> &BTreeSet<ComponentId> {
        &self.required_request_models
    }

    pub(crate) fn has_requests_in(
        &self,
        feature: &str,
        krate: Crate,
        layout: &ComponentLayout,
    ) -> bool {
        self.request_components(feature)
            .iter()
            .any(|component| layout.placement(*component).request_crate() == krate)
    }

    pub(crate) fn model_flag(
        &self,
        feature: &str,
        krate: Crate,
        layout: &ComponentLayout,
    ) -> String {
        model_feature_name(feature, self.has_requests_in(feature, krate, layout))
    }

    pub(crate) fn model_condition(
        &self,
        site: EmissionSite,
        krate: Crate,
        layout: &ComponentLayout,
    ) -> Availability {
        self.site(site).for_models(|feature| self.has_requests_in(feature, krate, layout))
    }

    /// Extra DTO flags needed by this crate's endpoint implementations. Each
    /// target uses its model flag, so returning B never enables B's endpoints.
    pub(crate) fn request_model_flags(
        &self,
        feature: &str,
        krate: Crate,
        layout: &ComponentLayout,
        config: &CrateConfig,
    ) -> anyhow::Result<Vec<String>> {
        ensure!(
            self.required_request_models.is_empty(),
            "request model roots must be registered before emitting Cargo features: {:?}",
            self.required_request_models
        );
        let mut flags = BTreeSet::new();
        for &component in self.request_components(feature) {
            if layout.placement(component).request_crate() != krate {
                continue;
            }
            for (site, group) in self.request_models.get(&component).into_iter().flatten() {
                let target = site.krate(layout).context("request model has no generated crate")?;
                let flag = self.model_flag(group, target, layout);
                flags.insert(if target == krate {
                    flag
                } else {
                    format!("{}/{flag}", target.crate_name(config))
                });
            }
        }
        Ok(flags.into_iter().collect())
    }

    pub(crate) fn dependencies(&self) -> &HashMap<EmissionSite, HashSet<EmissionSite>> {
        &self.dependencies
    }
}

fn enable(sites: &mut HashMap<EmissionSite, Availability>, site: EmissionSite, feature: &str) {
    match sites.get_mut(&site).expect("closed site was not registered") {
        Availability::Always => {}
        availability @ Availability::Disabled => {
            *availability = Availability::Features(BTreeSet::from([feature.to_string()]));
        }
        Availability::Features(groups) => {
            groups.insert(feature.to_string());
        }
    }
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
    use crate::crates::Crate;
    use crate::item_catalog::{CanonicalTypeRoot, ItemCatalog, ItemId, ItemMetadata, RustItem};
    use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
    use crate::layout::ComponentLayout;
    use crate::operations::{OperationCatalog, OperationType, StripeOperation};
    use crate::request_model::{RequestModel, RequestPathParam};
    use crate::rust_type::RustType;
    use crate::stripe_object::{StripeObject, StripeResource};
    use crate::types::{ComponentName, RustIdent};

    #[test]
    fn model_conditions_can_share_an_outer_module_with_public_request_flags() {
        let public = Availability::Features(["charge".into()].into_iter().collect());
        assert_eq!(
            public.for_models(|_| true).cfg_attribute(),
            "#[cfg(feature = \"__model_charge\")]\n"
        );
        assert_eq!(
            public.union(&public.for_models(|_| true)).cfg_attribute(),
            "#[cfg(any(feature = \"__model_charge\", feature = \"charge\"))]\n"
        );
        assert_eq!(public.for_models(|_| false), public);
        let mixed =
            Availability::Features(["charge".into(), "customer".into()].into_iter().collect());
        assert_eq!(
            mixed.for_models(|feature| feature == "charge"),
            Availability::Features(
                ["__model_charge".into(), "customer".into()].into_iter().collect()
            )
        );
        assert_eq!(Availability::Always.for_models(|_| true), Availability::Always);
        assert_eq!(Availability::Disabled.for_models(|_| true), Availability::Disabled);
    }

    struct Fixture {
        items: ItemCatalog,
        layout: ItemLayout,
        component_layout: ComponentLayout,
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
            let core = crate::crates::test_config()
                .all()
                .find(|krate| krate.base_name(&crate::crates::test_config()) == "core")
                .unwrap();
            let component_layout = ComponentLayout::for_test(&[
                (charge, core, core),
                (customer, core, core),
                (other, core, core),
            ]);
            let default = ItemLayout::from_component_layout(
                &components,
                &items,
                &component_layout,
                &crate::crates::test_config(),
            )
            .unwrap();
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
            layout.validate(&items, &component_layout, &crate::crates::test_config()).unwrap();
            Self { items, layout, component_layout, charge, customer, other }
        }

        fn feature(&self, name: &str, component: ComponentId) -> ModelFeature {
            ModelFeature {
                consumer_crate: crate::crates::Crate::SHARED,
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
        let group = ModelFeature {
            consumer_crate: crate::crates::Crate::SHARED,
            name: "details".to_string(),
            items: vec![details],
            requests: vec![],
        };
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
    fn request_requirements_are_public_without_broadening_model_conditions() {
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
            consumer_crate: crate::crates::Crate::SHARED,
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
        assert_eq!(plan.request_components("retrieve"), &[fixture.charge]);
        assert!(plan.request_components("unknown").is_empty());
        assert_eq!(
            plan.site(EmissionSite::ComponentRequests(fixture.charge)),
            &enabled_by(&["retrieve"])
        );
        for component in [fixture.charge, fixture.customer, fixture.other] {
            let site = EmissionSite::ComponentTypes(component);
            assert_eq!(plan.site(site), &Availability::Disabled);
            assert_eq!(plan.public_site(site), &enabled_by(&["retrieve"]));
        }
        assert_eq!(
            plan.required_request_models(),
            &[fixture.charge, fixture.customer, fixture.other,].into_iter().collect()
        );
        assert!(
            plan.request_model_flags(
                "retrieve",
                fixture.component_layout.placement(fixture.charge).request_crate(),
                &fixture.component_layout,
                &crate::crates::test_config()
            )
            .is_err()
        );
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
    #[test]
    fn returning_another_model_enables_its_data_without_its_http_api() {
        let fixture = Fixture::new(false);
        let request = |component| RequestModel {
            ident: RustIdent::create("retrieve"),
            path_params: vec![],
            params: None,
            method_type: OperationType::Get,
            returned: CanonicalTypeRoot {
                typ: RustType::Item(fixture.items.component_root(fixture.other)),
                declaration: None,
            },
            doc_comment: None,
            req_path: format!("/{component:?}"),
        };
        let request_a = request(fixture.customer);
        let request_b = request(fixture.other);
        let groups = [
            ModelFeature {
                name: "customer".into(),
                items: vec![fixture.items.component_root(fixture.customer)],
                requests: vec![fixture.customer],
                consumer_crate: fixture
                    .component_layout
                    .placement(fixture.customer)
                    .request_crate(),
            },
            ModelFeature {
                name: "other".into(),
                items: vec![fixture.items.component_root(fixture.other)],
                requests: vec![fixture.other],
                consumer_crate: fixture.component_layout.placement(fixture.other).request_crate(),
            },
        ];
        let plan = ModelAvailability::build(
            &fixture.items,
            [
                (fixture.customer, std::slice::from_ref(&request_a)),
                (fixture.other, std::slice::from_ref(&request_b)),
            ],
            &fixture.layout,
            &[],
            &groups,
        )
        .unwrap();
        let b = EmissionSite::ComponentTypes(fixture.other);
        assert_eq!(plan.site(b), &enabled_by(&["other"]));
        assert_eq!(plan.public_site(b), &enabled_by(&["customer", "other"]));
        assert_eq!(
            plan.site(EmissionSite::ComponentRequests(fixture.other)),
            &enabled_by(&["other"])
        );
        assert!(plan.required_request_models().is_empty());
        let krate = fixture.component_layout.placement(fixture.customer).request_crate();
        assert_eq!(
            plan.model_flag("customer", krate, &fixture.component_layout),
            "__model_customer"
        );
        assert_eq!(
            plan.request_model_flags(
                "customer",
                krate,
                &fixture.component_layout,
                &crate::crates::test_config()
            )
            .unwrap(),
            ["__model_other"]
        );
        assert!(
            !plan
                .model_condition(b, krate, &fixture.component_layout)
                .cfg_attribute()
                .contains("__model_customer")
        );
    }
}
