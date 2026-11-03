use std::fmt::Write as _;
use std::path::PathBuf;

use anyhow::Context;
use indoc::formatdoc;
use petgraph::prelude::DiGraphMap;

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crate_dependencies::{crate_dependencies, validate_crate_dependencies};
use crate::crate_inference::CrateInferenceWarning;
use crate::crate_table::generate_crate_table;
use crate::crates::{ALL_CRATES, Crate, get_crate_doc_comment};
pub use crate::dependencies::EdgePolicy;
use crate::item_catalog::ItemMetadata;
use crate::item_layout::{EmissionSite, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::layout::ComponentLayout;
use crate::model_availability::{ModelAvailability, ModelFeature, model_feature_name};
use crate::object_writing::ItemEmitter;
use crate::output::GeneratedWorkspace;
use crate::output_locations::OutputLocations;
use crate::pipeline::run_pipeline;
use crate::render::RenderContext;
use crate::rust_model::RustModel;
use crate::rust_type::DeclarationRoot;
use crate::spec::Spec;
use crate::spec_inference::infer_doc_comment;
use crate::templates::cargo_toml::gen_crate_toml;
use crate::templates::utils::write_top_level_doc_comment;
use crate::url_finder::UrlFinder;
use crate::webhook::write_generated_for_webhooks;

pub struct CodeGen {
    pub components: Components,
    pub(crate) model: RustModel,
    component_layout: ComponentLayout,
    pub(crate) item_layout: ItemLayout,
    pub(crate) crate_dependencies: DiGraphMap<Crate, ()>,
    pub spec: Spec,
    pub version: String,
    pub crate_inference_warnings: Vec<CrateInferenceWarning>,
    edge_policy: EdgePolicy,
    pub(crate) availability: Option<ModelAvailability>,
    model_groups: Vec<ModelFeature>,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct CodeGenOptions {
    pub edge_policy: EdgePolicy,
}

impl CodeGen {
    pub fn new(spec: Spec, url_finder: UrlFinder, version: String) -> anyhow::Result<Self> {
        Self::new_with_options(spec, url_finder, version, CodeGenOptions::default())
    }

    pub fn new_with_options(
        spec: Spec,
        url_finder: UrlFinder,
        version: String,
        options: CodeGenOptions,
    ) -> anyhow::Result<Self> {
        let pipeline = run_pipeline(&spec, options.edge_policy)?;
        let mut components = pipeline.components;
        let component_layout = pipeline.component_layout;
        let crate_inference_warnings = pipeline.crate_inference_warnings;

        // Attach doc urls for top-level components
        for comp in components.components.values_mut() {
            if let Some(doc_url) = url_finder.url_for_object(comp.path()) {
                comp.stripe_doc_url = Some(doc_url);
            }
        }

        let model = RustModel::build(&components, |component| {
            let object = components.get_by_id(component);
            let schema = spec.get_component_schema(object.path());
            let doc_comment = infer_doc_comment(schema, object.stripe_doc_url.as_deref());
            ItemMetadata::new(object.ident().clone()).doc(doc_comment)
        })?;
        let item_layout =
            ItemLayout::from_component_layout(&components, model.items(), &component_layout)?;
        let crate_dependencies =
            crate_dependencies(model.items(), model.requests(), &component_layout, &item_layout)?;

        Ok(Self {
            components,
            model,
            component_layout,
            item_layout,
            crate_dependencies,
            spec,
            version,
            crate_inference_warnings,
            edge_policy: options.edge_policy,
            availability: None,
            model_groups: Vec::new(),
        })
    }

    /// Build an experimental set of independently compiled v1 resource groups.
    /// Normal generation retains its existing concrete representations.
    pub fn new_opaque_model_groups(
        spec: Spec,
        url_finder: UrlFinder,
        version: String,
        groups: &[&str],
    ) -> anyhow::Result<Self> {
        Self::new(spec, url_finder, version)?.with_opaque_model_groups(groups)
    }

    /// Build the experimental opaque representation for every v1 resource and
    /// endpoint group. Supporting schemas follow their actual dependencies.
    pub fn new_opaque_v1(
        spec: Spec,
        url_finder: UrlFinder,
        version: String,
    ) -> anyhow::Result<Self> {
        let generated = Self::new(spec, url_finder, version)?;
        let groups = generated.resource_model_groups();
        let names = groups.iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>();
        generated.with_opaque_model_groups(&names)
    }

    fn resource_model_groups(&self) -> Vec<(String, ComponentId)> {
        self.components
            .components
            .iter()
            .filter(|(component, object)| {
                object.object_name().is_some()
                    || object.identity().is_some()
                    || self.components.has_requests(**component)
            })
            .map(|(&component, object)| (object.mod_path(), component))
            .collect()
    }

    fn with_opaque_model_groups(mut self, groups: &[&str]) -> anyhow::Result<Self> {
        let generated = &mut self;
        anyhow::ensure!(!groups.is_empty(), "at least one model group is required");
        generated.model.use_opaque_relationships(&generated.components)?;
        let items = generated.model.items();
        generated.item_layout.separate_reference_types(items);
        generated.item_layout.validate(items, &generated.component_layout)?;
        for name in groups {
            let (&component, object) = generated
                .components
                .components
                .iter()
                .find(|(_, object)| object.mod_path() == *name)
                .with_context(|| format!("unknown model group {name}"))?;
            generated.model_groups.push(ModelFeature {
                name: object.mod_path(),
                items: vec![items.component_root(component)],
                requests: generated
                    .components
                    .has_requests(component)
                    .then_some(component)
                    .into_iter()
                    .collect(),
                consumer_crate: generated.component_layout.placement(component).request_crate(),
            });
        }
        let mut availability = generated.model_availability()?;
        let mut registered_decoders = std::collections::HashSet::new();
        loop {
            let before = registered_decoders.len();
            for accessor in items.record_accessors() {
                if registered_decoders.contains(&accessor.payload)
                    || *availability
                        .site(generated.item_layout.placement(accessor.owner).emission_site())
                        == crate::model_availability::Availability::Disabled
                {
                    continue;
                }
                let TypeRoot::Declaration(DeclarationRoot::Component(component)) =
                    items.get(accessor.payload).origin.type_root
                else {
                    anyhow::bail!("record decoder has no component root: {:?}", accessor.payload);
                };
                let consumer_crate =
                    generated.component_layout.placement(component).request_crate();
                let owner = items.get(accessor.owner);
                let individual = owner.origin.path.is_empty()
                    && matches!(owner.origin.type_root,
                        TypeRoot::Declaration(DeclarationRoot::Component(owner))
                        if generated.components.component_name(owner).as_ref() == "payment_method_details");
                let name = if individual {
                    generated.components.get_by_id(component).mod_path()
                } else {
                    format!("{}_decoders", consumer_crate.base_name())
                };
                if let Some(group) =
                    generated.model_groups.iter_mut().find(|group| group.name == name)
                {
                    anyhow::ensure!(
                        group.consumer_crate == consumer_crate && group.requests.is_empty(),
                        "decoder group conflicts with an existing resource group: {name}"
                    );
                    if !group.items.contains(&accessor.payload) {
                        group.items.push(accessor.payload);
                    }
                } else {
                    generated.model_groups.push(ModelFeature {
                        name,
                        items: vec![accessor.payload],
                        requests: vec![],
                        consumer_crate,
                    });
                }
                registered_decoders.insert(accessor.payload);
            }
            if registered_decoders.len() == before {
                break;
            }
            // Enabling one payload can reveal another lazy record whose own
            // optional decoders were unavailable in the previous closure.
            availability = generated.model_availability()?;
        }
        generated.availability = Some(availability);
        generated.crate_dependencies = crate_dependencies(
            items,
            generated.model.requests(),
            &generated.component_layout,
            &generated.item_layout,
        )?;
        Ok(self)
    }

    fn model_availability(&self) -> anyhow::Result<ModelAvailability> {
        let items = self.model.items();
        let unconditional = items.iter().filter_map(|(item, record)| {
            let support = record.is_reference_foundation()
                || matches!(&record.origin.type_root, TypeRoot::Declaration(DeclarationRoot::Shared(_)))
                || matches!(record.origin.type_root, TypeRoot::Webhook(_))
                || (record.origin.path.is_empty() && matches!(record.origin.type_root,
                    TypeRoot::Declaration(DeclarationRoot::Component(component))
                    if matches!(self.components.component_name(component).as_ref(), "error" | "event")));
            support.then_some(item)
        }).collect::<Vec<_>>();
        ModelAvailability::build(
            items,
            self.model.requests(),
            &self.item_layout,
            &unconditional,
            &self.model_groups,
        )
    }

    /// Place complete v1 model modules using their represented consumers and
    /// retained domain exports, preserving existing package names.
    ///
    /// This experiment requires the complete resource surface so placement does
    /// not change according to a consumer's subset of enabled features.
    pub fn with_consumer_model_layout(mut self) -> anyhow::Result<Self> {
        let availability = self
            .availability
            .as_ref()
            .context("consumer model layout requires opaque v1 generation")?;
        anyhow::ensure!(
            self.resource_model_groups().iter().all(|(name, component)| {
                self.model_groups.iter().any(|group| {
                    group.name == *name
                        && group.items.contains(&self.model.items().component_root(*component))
                })
            }),
            "consumer model layout requires every v1 resource group; use --all"
        );
        let feature_crates = self
            .model_groups
            .iter()
            .map(|group| (group.name.clone(), group.consumer_crate))
            .collect();
        self.component_layout = self.component_layout.with_consumer_model_crates(
            &self.components,
            self.model.items(),
            &self.item_layout,
            availability,
            &feature_crates,
        )?;
        self.item_layout = ItemLayout::from_component_layout(
            &self.components,
            self.model.items(),
            &self.component_layout,
        )?;
        self.item_layout.separate_reference_types(self.model.items());
        self.item_layout.validate(self.model.items(), &self.component_layout)?;
        self.availability = Some(self.model_availability()?);
        self.crate_dependencies = crate_dependencies(
            self.model.items(),
            self.model.requests(),
            &self.component_layout,
            &self.item_layout,
        )?;
        validate_crate_dependencies(&self.crate_dependencies)?;
        Ok(self)
    }

    pub fn canonical_item_count(&self) -> usize {
        self.model.items().len()
    }

    // TODO: too many useless helpers
    pub fn request_component_count(&self) -> usize {
        self.components.request_component_count()
    }

    pub const fn edge_policy(&self) -> EdgePolicy {
        self.edge_policy
    }

    pub(crate) fn render_context(&self, emission_site: EmissionSite) -> RenderContext<'_> {
        RenderContext::new(&self.item_layout, self.model.items(), emission_site)
    }

    pub(crate) fn output_locations(&self) -> OutputLocations<'_> {
        OutputLocations::new(&self.components, &self.component_layout)
    }

    fn write_api_version_file(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let base_path = Crate::SHARED.get_path();
        let mut mod_rs_contents = String::new();
        let mod_rs_path = base_path.join("mod.rs");

        // Write the current API version
        let version_file_content = format!(
            "pub const VERSION: crate::ApiVersion = crate::ApiVersion::V{};",
            self.spec.version().replace(['-', '.'], "_")
        );
        output.write(base_path.join("version.rs"), version_file_content)?;
        let _ = writeln!(mod_rs_contents, "pub mod version;");

        output.append(mod_rs_path, mod_rs_contents)
    }

    fn write_declarations(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let items = self.model.items();
        let mut emitter = ItemEmitter::new(items, &self.item_layout, self.output_locations());
        if let Some(plan) = &self.availability {
            emitter = emitter.with_availability(plan);
        }
        // This traversal preserves source order. Every declaration, including
        // recursively introduced children, is routed by its own item placement.
        for (&component, object) in &self.components.components {
            emitter.emit(items.component_root(component), output)?;
            if let Some(item) = items.component_id_item(component) {
                emitter.emit(item, output)?;
            }
            emitter.requests(component, self.model.requests_for_component(component), output)?;
            emitter.component_reexports(component, output)?;
            for ident in object.deduplicated_objects.keys() {
                let root = TypeRoot::Declaration(DeclarationRoot::Deduplicated {
                    component,
                    ident: ident.clone(),
                });
                let item = items
                    .root(&root)
                    .and_then(|root| root.declaration)
                    .expect("deduplicated item declaration was not cataloged");
                emitter.emit(item, output)?;
            }
        }
        for ident in self.components.extra_types.keys() {
            let root = TypeRoot::Declaration(DeclarationRoot::Shared(ident.clone()));
            let item = items
                .root(&root)
                .and_then(|root| root.declaration)
                .expect("shared item declaration was not cataloged");
            emitter.emit(item, output)?;
            emitter.reexport(item, Crate::SHARED, output)?;
        }
        for webhook in &self.components.webhook_objs {
            let root = TypeRoot::Webhook(webhook.event_type.clone());
            if let Some(item) = items.root(&root).and_then(|root| root.declaration) {
                emitter.emit(item, output)?;
            }
        }
        emitter.finish()
    }

    pub fn render_workspace(&self) -> anyhow::Result<GeneratedWorkspace> {
        validate_crate_dependencies(&self.crate_dependencies)?;
        let mut output = GeneratedWorkspace::default();
        self.write_crate_base(&mut output)?;
        self.write_declarations(&mut output)?;
        self.write_api_version_file(&mut output)?;
        let context = self.render_context(EmissionSite::Webhooks);
        write_generated_for_webhooks(
            context,
            &self.components.webhook_objs,
            self.availability.as_ref(),
            &mut output,
        )
        .context("Could not write webhook generated code")?;
        output.write("crate_info.md", generate_crate_table(self.output_locations()))?;
        self.write_object_info_for_testing(&mut output)?;
        if self.availability.is_some() {
            let request_groups = self
                .model_groups
                .iter()
                .flat_map(|group| {
                    group.requests.iter().map(|&component| (group.name.clone(), component))
                })
                .collect::<Vec<_>>();
            crate::facade::write_facade(
                self.output_locations(),
                self.model.items(),
                &self.item_layout,
                self.availability.as_ref().expect("opaque model availability"),
                &request_groups,
                &self.version,
                &mut output,
            )?;
            crate::webhook::write_manifest(
                context,
                self.output_locations(),
                &self.components.webhook_objs,
                self.availability.as_ref().expect("opaque model availability"),
                &self.version,
                &mut output,
            )?;
            self.write_fixture_manifest(&mut output)?;
        }
        Ok(output)
    }

    fn write_crate_base(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let locations = self.output_locations();
        for krate in &*ALL_CRATES {
            let mut dependencies = self.crate_dependencies.neighbors(*krate).collect::<Vec<_>>();
            dependencies.sort_unstable();
            let base_path = PathBuf::from(krate.generated_out_path());
            let features = if let Some(plan) = &self.availability {
                let mut features = std::collections::BTreeMap::new();
                for feature in plan.features() {
                    let requests_here = plan
                        .request_components(feature)
                        .iter()
                        .any(|component| locations.request_crate(*component) == *krate);
                    let model_feature = model_feature_name(feature, requests_here);
                    let forwarded = dependencies
                        .iter()
                        .map(|dep| {
                            let requests_there = plan
                                .request_components(feature)
                                .iter()
                                .any(|component| locations.request_crate(*component) == *dep);
                            format!(
                                "{}/{}",
                                dep.crate_name(),
                                model_feature_name(feature, requests_there)
                            )
                        })
                        .collect();
                    features.insert(model_feature.clone(), forwarded);
                    if requests_here {
                        features
                            .insert(feature.clone(), vec!["__requests".to_string(), model_feature]);
                    }
                }
                if *krate != Crate::SHARED {
                    features
                        .insert("__requests".into(), vec!["dep:async-stripe-client-core".into()]);
                }
                features.insert("full".into(), plan.features().iter().cloned().collect());
                Some(features)
            } else if *krate == Crate::SHARED {
                None
            } else {
                let mut features = locations
                    .crate_members(*krate)
                    .into_iter()
                    .filter(|c| locations.components.has_requests(*c))
                    .map(|c| (locations.components.get_by_id(c).mod_path(), Vec::new()))
                    .collect::<std::collections::BTreeMap<_, _>>();
                features.insert("full".into(), features.keys().cloned().collect());
                Some(features)
            };
            let toml = gen_crate_toml(
                *krate,
                dependencies,
                features,
                self.availability.is_some(),
                &self.version,
            );
            output.write(base_path.join("Cargo.toml"), toml)?;

            let lib_name = krate.lib_name();
            let doc_comment = write_top_level_doc_comment(get_crate_doc_comment(*krate));

            // We set up some things in the base `mod.rs` file:
            // 1. Without this recursion limit increase, `cargo doc` fails
            // 2. The `extern` allows generated code to use absolute paths starting with the crate name instead of `crate`
            // 3. Allow some warnings that are not currently fixed, but could be.
            let mod_rs = formatdoc! {
                r#"
            #![recursion_limit = "256"]
            #![deny(clippy::large_stack_frames)]
            #![allow(clippy::large_enum_variant)]
            #![allow(rustdoc::broken_intra_doc_links)]
            #![allow(rustdoc::invalid_html_tags)]
            #![allow(non_camel_case_types)]

            {doc_comment}
            extern crate self as {lib_name};

            miniserde::make_place!(Place);
            "#
            };

            let mod_path = base_path.join("src/mod.rs");
            output.write(&mod_path, mod_rs)?;

            // NB: a hack to avoid the insanely long lines generated because of _very_ long
            // type names causing `rustfmt` errors (https://github.com/rust-lang/rustfmt/issues/5315)
        }
        Ok(())
    }

    fn write_fixture_manifest(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let template = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/Cargo.toml"));
        let mut manifest: toml::Value = toml::from_str(template)?;
        let dependencies = manifest
            .get_mut("dev-dependencies")
            .and_then(toml::Value::as_table_mut)
            .context("fixture manifest has no dev-dependencies table")?;
        for krate in &*ALL_CRATES {
            let package = krate.crate_name();
            let features = dependencies
                .get_mut(&package)
                .and_then(toml::Value::as_table_mut)
                .with_context(|| format!("fixture manifest has no generated dependency {package}"))?
                .entry("features")
                .or_insert_with(|| toml::Value::Array(vec![]))
                .as_array_mut()
                .with_context(|| {
                    format!("fixture dependency features are not an array: {package}")
                })?;
            let full = toml::Value::String("full".into());
            features.retain(|feature| {
                matches!(
                    feature.as_str(),
                    Some("serialize" | "deserialize" | "redact-generated-debug")
                )
            });
            features.push(full);
        }
        let features = manifest
            .get_mut("features")
            .and_then(toml::Value::as_table_mut)
            .context("fixture manifest has no feature table")?;
        features.insert("opaque-models".into(), toml::Value::Array(vec![]));
        let defaults = features
            .get_mut("default")
            .and_then(toml::Value::as_array_mut)
            .context("fixture manifest has no default feature list")?;
        let opaque = toml::Value::String("opaque-models".into());
        if !defaults.contains(&opaque) {
            defaults.push(opaque);
        }
        output.write("tests/Cargo.toml", toml::to_string_pretty(&manifest)?)
    }

    fn write_object_info_for_testing(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let mut checks = String::new();
        for (&component, obj) in &self.components.components {
            if obj.object_name().is_none() {
                continue;
            }
            let path = self.components.component_name(component);
            let item = self.model.items().component_root(component);
            let crate::item_layout::ItemAddress::CrateRoot(krate) =
                self.item_layout.placement(item).address()
            else {
                anyhow::bail!("fixture model is not publicly addressable: {component:?}");
            };
            let import_path =
                format!("{}::{}", krate.lib_name(), self.model.items().get(item).metadata.ident);
            let _ = writeln!(checks, r#"check_object::<{import_path}>(resources, "{path}");"#);
        }
        let content = formatdoc! {
            r#"
            use crate::deserialization_fixture::check_object;

            pub fn check_fixtures(resources: &serde_json::Value) {{
                {checks}
            }}
            "#
        };

        output.write("tests/mod.rs", content)
    }
}
