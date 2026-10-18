use std::fmt::Write as _;
use std::path::PathBuf;

use anyhow::Context;
use indoc::formatdoc;

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crate_inference::CrateInferenceWarning;
use crate::crate_table::generate_crate_table;
use crate::crates::{ALL_CRATES, Crate, get_crate_doc_comment};
pub use crate::dependencies::EdgePolicy;
use crate::graph::gen_crate_dep_graph;
use crate::item_catalog::{ItemId, ItemMetadata};
use crate::item_layout::{EmissionSite, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::layout::ComponentLayout;
use crate::object_writing::{gen_obj, gen_requests};
use crate::output::GeneratedWorkspace;
use crate::output_locations::OutputLocations;
use crate::pipeline::run_pipeline;
use crate::render::RenderContext;
use crate::rust_model::RustModel;
use crate::rust_type::DeclarationRoot;
use crate::spec::Spec;
use crate::spec_inference::infer_doc_comment;
use crate::stripe_object::StripeObject;
use crate::templates::cargo_toml::gen_crate_toml;
use crate::templates::utils::write_top_level_doc_comment;
use crate::url_finder::UrlFinder;
use crate::webhook::write_generated_for_webhooks;

pub struct CodeGen {
    pub components: Components,
    pub(crate) model: RustModel,
    component_layout: ComponentLayout,
    item_layout: ItemLayout,
    pub spec: Spec,
    pub version: String,
    pub crate_inference_warnings: Vec<CrateInferenceWarning>,
    edge_policy: EdgePolicy,
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

        Ok(Self {
            components,
            model,
            component_layout,
            item_layout,
            spec,
            version,
            crate_inference_warnings,
            edge_policy: options.edge_policy,
        })
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

    fn render_context_for_item(&self, item: ItemId) -> RenderContext<'_> {
        self.render_context(self.item_layout.placement(item).emission_site())
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

    fn write_components(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        for (&component, object) in &self.components.components {
            self.write_component(component, object, output)?;
        }

        let crate_path = Crate::SHARED.get_path();
        let crate_mod_path = crate_path.join("mod.rs");
        for (ident, typ_info) in &self.components.extra_types {
            let mut out = String::new();
            let item = self
                .model
                .items()
                .root(&TypeRoot::Declaration(DeclarationRoot::Shared(ident.clone())))
                .and_then(|root| root.declaration)
                .expect("shared item declaration was not cataloged");
            let context = self.render_context_for_item(item);
            context.write_item(item, &mut out);
            output.write(crate_path.join(format!("{}.rs", typ_info.mod_path)), out)?;
            output.append(
                &crate_mod_path,
                format!("pub mod {0}; pub use {0}::{1};", typ_info.mod_path, ident),
            )?;
        }
        Ok(())
    }

    pub fn render_workspace(&self) -> anyhow::Result<GeneratedWorkspace> {
        let mut output = GeneratedWorkspace::default();
        self.write_crate_base(&mut output)?;
        self.write_components(&mut output)?;
        self.write_api_version_file(&mut output)?;
        let context = self.render_context(EmissionSite::Webhooks);
        write_generated_for_webhooks(context, &self.components.webhook_objs, &mut output)
            .context("Could not write webhook generated code")?;
        output.write("crate_info.md", generate_crate_table(self.output_locations()))?;
        self.write_object_info_for_testing(&mut output)?;
        Ok(output)
    }

    fn write_crate_base(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let locations = self.output_locations();
        let crate_graph = gen_crate_dep_graph(&self.components, self.edge_policy, |component| {
            locations.placement(component)
        });

        for krate in &*ALL_CRATES {
            let neighbors = crate_graph.neighbors(*krate);
            let base_path = PathBuf::from(krate.generated_out_path());
            let request_features = locations
                .crate_members(*krate)
                .into_iter()
                .filter(|c| locations.components.has_requests(*c) && *krate != Crate::SHARED)
                .map(|c| locations.components.get_by_id(c).mod_path())
                .collect();

            let toml = gen_crate_toml(*krate, neighbors.collect(), request_features, &self.version);
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

    fn write_component_requests(
        &self,
        component: ComponentId,
        comp: &StripeObject,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        let locations = self.output_locations();
        let context = self.render_context(EmissionSite::ComponentRequests(component));
        let req_content = gen_requests(self.model.requests_for_component(component), context);

        let req_file_content = formatdoc! {
            r#"
            use stripe_client_core::{{StripeClient, StripeBlockingClient, StripeRequest, RequestBuilder, StripeMethod}};

            {req_content}
            "#
        };
        output.write(locations.requests_content_path(component), req_file_content)?;
        let feature_gate = comp.mod_path();
        let new_mod_file_content = formatdoc! {
            r#"
            #[cfg(feature = "{feature_gate}")]
            mod requests;
            #[cfg(feature = "{feature_gate}")]
            pub use requests::*;
            "#
        };
        output.append(locations.requests_module_path(component), new_mod_file_content)
    }

    #[tracing::instrument(level = "debug", skip_all, fields(path = %comp.path()))]
    fn write_component(
        &self,
        component: ComponentId,
        comp: &StripeObject,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        let locations = self.output_locations();
        let struct_defs = self.gen_struct_definitions_for_component(component);
        if !self.components.has_requests(component)
            || locations.types_split_from_requests(component)
        {
            output.write(locations.types_content_path(component), struct_defs)?;
            output.append(
                locations.types_crate(component).get_path_to_mod(),
                format!(
                    // NB: we add doc(hidden) and doc(inline) to hide the implementation details
                    // of these being public modules so that other generated crates can import them. It
                    // just keeps the already giant rustdoc page a bit cleaner
                    "#[doc(hidden)]\npub mod {0};#[doc(inline)]\npub use {0}::*;",
                    comp.mod_path()
                ),
            )?;
        } else {
            output.write(locations.types_content_path(component), struct_defs)?;
            output.append(locations.requests_module_path(component), "pub(crate) mod types;")?;
            output.append(
                locations.request_crate(component).get_path_to_mod(),
                format!("pub use {0}::types::*;", comp.mod_path()),
            )?;
        }

        if self.components.has_requests(component) {
            output.append(
                locations.request_crate(component).get_path_to_mod(),
                format!("pub mod {};", comp.mod_path()),
            )?;
            self.write_component_requests(component, comp, output)?;
        }

        // Reexport in this crate if we wrote types to `stripe_shared` instead of the
        // component's base crate.
        if locations.types_split_from_requests(component) {
            output.append(
                locations.request_crate(component).get_path_to_mod(),
                format!("pub use {}::{}::*;", Crate::SHARED.lib_name(), comp.mod_path()),
            )?;
        }

        for ident in comp.deduplicated_objects.keys() {
            let mut out = String::new();
            let item = self
                .model
                .items()
                .root(&TypeRoot::Declaration(DeclarationRoot::Deduplicated {
                    component,
                    ident: ident.clone(),
                }))
                .and_then(|root| root.declaration)
                .expect("deduplicated item declaration was not cataloged");
            let context = self.render_context_for_item(item);
            context.write_item(item, &mut out);
            let dst_file = match self.item_layout.placement(item).emission_site() {
                EmissionSite::ComponentTypes(owner) => locations.types_content_path(owner),
                EmissionSite::ComponentRequests(owner) => locations.requests_content_path(owner),
                EmissionSite::SharedType(_) | EmissionSite::Webhooks => {
                    panic!("deduplicated item had an invalid emission site: {item:?}")
                }
            };
            output.append(dst_file, out)?;
        }

        Ok(())
    }

    fn gen_struct_definitions_for_component(&self, component: ComponentId) -> String {
        let base_item = self.model.items().component_root(component);
        let context = self.render_context_for_item(base_item);
        gen_obj(base_item, context)
    }

    fn write_object_info_for_testing(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let mut checks = String::new();
        for (&component, obj) in &self.components.components {
            if obj.object_name().is_none() {
                continue;
            }
            let path = self.components.component_name(component);
            let krate = self.output_locations().request_crate(component);
            let import_path = format!("{}::{}", krate.lib_name(), obj.ident());
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
