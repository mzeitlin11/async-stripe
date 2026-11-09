use std::fmt::Write as _;
use std::path::PathBuf;

use anyhow::Context;
use indoc::formatdoc;
use petgraph::prelude::DiGraphMap;
use tracing::{debug, info};

use crate::components::{Components, infer_components};
use crate::crate_dependencies::{crate_dependencies, validate_crate_dependencies};
use crate::crate_inference::CrateInferenceWarning;
use crate::crate_table::generate_crate_table;
use crate::crates::{Crate, CrateConfig};
pub use crate::dependencies::EdgePolicy;
use crate::exports::legacy_facade_dependencies;
use crate::item_catalog::{ItemCatalog, ItemCatalogDraft, ItemMetadata};
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::layout::ComponentLayout;
use crate::object_writing::ItemEmitter;
use crate::output::GeneratedWorkspace;
use crate::output_locations::OutputLocations;
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
    pub(crate) config: CrateConfig,
    pub(crate) components: Components,
    pub(crate) model: RustModel,
    component_layout: ComponentLayout,
    pub(crate) item_layout: ItemLayout,
    pub(crate) crate_dependencies: DiGraphMap<Crate, ()>,
    pub(crate) spec: Spec,
    pub(crate) version: String,
    pub(crate) crate_inference_warnings: Vec<CrateInferenceWarning>,
    edge_policy: EdgePolicy,
}

impl CodeGen {
    pub fn new(spec: Spec, url_finder: UrlFinder, version: String) -> anyhow::Result<Self> {
        Self::with_config(
            spec,
            url_finder,
            version,
            EdgePolicy::default(),
            CrateConfig::load(std::path::Path::new("gen_crates.toml"))?,
        )
    }

    pub fn with_config(
        spec: Spec,
        url_finder: UrlFinder,
        version: String,
        edge_policy: EdgePolicy,
        config: CrateConfig,
    ) -> anyhow::Result<Self> {
        Self::build(spec, url_finder, version, edge_policy, config, false)
    }

    /// Generate the normal SDK with lazy storage only for expandable references.
    /// Crate placement, feature policy, and ordinary object unions are unchanged.
    pub fn with_lazy_expandable_config(
        spec: Spec,
        url_finder: UrlFinder,
        version: String,
        edge_policy: EdgePolicy,
        config: CrateConfig,
    ) -> anyhow::Result<Self> {
        Self::build(spec, url_finder, version, edge_policy, config, true)
    }

    fn build(
        spec: Spec,
        url_finder: UrlFinder,
        version: String,
        edge_policy: EdgePolicy,
        config: CrateConfig,
        lazy_expandables: bool,
    ) -> anyhow::Result<Self> {
        let (mut components, mut request_owners) = infer_components(&spec, &config)?;
        components.filter_unused_components();
        components.validate_catalog()?;

        // Service ownership is inferred before rewrites for compatibility. Final
        // model placement is selected separately, after Rust-model construction.
        let crate_inference_warnings =
            components.infer_request_owners(&mut request_owners, edge_policy, &config)?;
        info!("Finished inferring request owners");

        components.apply_overrides()?;
        debug!("Finished applying overrides");
        components.run_deduplication_pass();
        info!("Finished deduplication pass");

        // Attach doc urls for top-level components
        for comp in components.components.values_mut() {
            if let Some(doc_url) = url_finder.url_for_object(comp.path()) {
                comp.stripe_doc_url = Some(doc_url);
            }
        }

        let mut draft = ItemCatalogDraft::build(&components, |component| {
            let object = components.get_by_id(component);
            let schema = spec.get_component_schema(object.path());
            let doc_comment = infer_doc_comment(schema, object.stripe_doc_url.as_deref());
            ItemMetadata::new(object.ident().clone()).doc(doc_comment)
        })?;
        if lazy_expandables {
            draft.use_lazy_expandables()?;
        }
        let model = RustModel::from_draft(&components, draft)?;
        let component_layout =
            components.legacy_model_layout(request_owners, edge_policy, &config)?;
        let item_layout = ItemLayout::from_component_layout(
            &components,
            model.items(),
            &component_layout,
            &config,
        )?;
        let mut crate_dependencies = crate_dependencies(
            model.items(),
            model.requests(),
            &component_layout,
            &item_layout,
            &config,
        )?;
        for (source, target) in legacy_facade_dependencies(config.all()) {
            crate_dependencies.add_edge(source, target, ());
        }

        Ok(Self {
            config,
            components,
            model,
            component_layout,
            item_layout,
            crate_dependencies,
            spec,
            version,
            crate_inference_warnings,
            edge_policy,
        })
    }

    pub fn crate_config(&self) -> &CrateConfig {
        &self.config
    }

    pub fn components(&self) -> &Components {
        &self.components
    }

    pub fn crate_inference_warnings(&self) -> &[CrateInferenceWarning] {
        &self.crate_inference_warnings
    }

    pub fn request_component_count(&self) -> usize {
        self.components.request_component_count()
    }

    pub(crate) const fn edge_policy(&self) -> EdgePolicy {
        self.edge_policy
    }

    pub(crate) fn render_context(&self, emission_site: EmissionSite) -> RenderContext<'_> {
        RenderContext::new(&self.item_layout, self.model.items(), emission_site, &self.config)
    }

    pub(crate) fn output_locations(&self) -> OutputLocations<'_> {
        OutputLocations::new(&self.components, &self.component_layout, &self.config)
    }

    fn write_api_version_file(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let config = &self.config;
        let base_path = Crate::SHARED.get_path(config);
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
        write_generated_for_webhooks(context, &self.components.webhook_objs, &mut output)
            .context("Could not write webhook generated code")?;
        output.write("crate_info.md", generate_crate_table(self.output_locations()))?;
        Self::write_object_info_for_testing(
            self.model.items(),
            &self.item_layout,
            self.output_locations(),
            &mut output,
        )?;
        Ok(output)
    }

    fn write_crate_base(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let config = &self.config;
        let locations = self.output_locations();
        for krate in config.all() {
            let mut dependencies = self.crate_dependencies.neighbors(krate).collect::<Vec<_>>();
            dependencies.sort_by_key(|c| c.base_name(config));
            let base_path = PathBuf::from(krate.generated_out_path(config));
            let request_features = locations
                .crate_members(krate)
                .filter(|c| locations.components.has_requests(*c) && krate != Crate::SHARED)
                .map(|c| locations.components.get_by_id(c).mod_path())
                .collect();

            let toml = gen_crate_toml(krate, dependencies, request_features, &self.version, config);
            output.write(base_path.join("Cargo.toml"), toml)?;

            let lib_name = krate.lib_name(config);
            let doc_comment = write_top_level_doc_comment(config.doc_comment(krate));

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

    pub(crate) fn write_object_info_for_testing(
        items: &ItemCatalog,
        item_layout: &ItemLayout,
        locations: OutputLocations<'_>,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        let config = locations.config;
        let components = locations.components;
        let mut checks = String::new();
        for (&component, obj) in &components.components {
            if obj.object_name().is_none() {
                continue;
            }
            let path = components.component_name(component);
            let model = items.component_root(component);
            anyhow::ensure!(
                matches!(item_layout.placement(model).address(), ItemAddress::CrateRoot(_)),
                "fixture model is not publicly addressable: {path}"
            );
            // Component exports keep the model available from its service
            // facade even when its canonical declaration is placed elsewhere.
            // Preserve that path spelling, using the actual declaration name.
            let krate = locations.request_crate(component);
            let import_path =
                format!("{}::{}", krate.lib_name(config), items.get(model).metadata.ident);
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
