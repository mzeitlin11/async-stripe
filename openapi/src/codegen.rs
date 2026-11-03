use std::fmt::Write as _;
use std::path::PathBuf;

use anyhow::Context;
use indoc::formatdoc;
use petgraph::prelude::DiGraphMap;

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
        let crate_dependencies = crate_dependencies(
            model.items(),
            model.requests(),
            &component_layout,
            &item_layout,
            !spec.is_v2_experiment(),
        )?;

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

    pub(crate) fn output_locations(&self) -> OutputLocations<'_> {
        OutputLocations::new(&self.components, &self.component_layout)
    }

    fn write_api_version_file(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        if self.spec.is_v2_experiment() {
            return Ok(());
        }
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
        let mut emitter = ItemEmitter::new(items, &self.item_layout, self.output_locations())
            .with_request_features(!self.spec.is_v2_experiment());
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
        self.write_namespace_facades(&mut output)?;
        self.write_api_version_file(&mut output)?;
        let context = self.render_context(EmissionSite::Webhooks);
        write_generated_for_webhooks(context, &self.components.webhook_objs, &mut output)
            .context("Could not write webhook generated code")?;
        output.write("crate_info.md", generate_crate_table(self.output_locations()))?;
        self.write_object_info_for_testing(&mut output)?;
        Ok(output)
    }

    fn write_namespace_facades(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        if !self.spec.is_v2_experiment() {
            return Ok(());
        }
        let mut groups = std::collections::BTreeMap::<(Crate, String), String>::new();
        for (&component, object) in &self.components.components {
            let Some((namespace, name)) =
                object.path().strip_prefix("v2.").and_then(|path| path.rsplit_once('.'))
            else {
                continue;
            };
            anyhow::ensure!(
                !namespace.contains('.'),
                "unsupported nested public namespace: {namespace}"
            );
            let krate = self.component_layout.placement(component).request_crate();
            let group = groups.entry((krate, namespace.to_owned())).or_default();
            let alias = crate::types::RustIdent::create(name);
            let ident = object.ident();
            let _ = writeln!(group, "pub use crate::{ident} as {alias};");
            if self.model.items().component_id_item(component).is_some() {
                let _ = writeln!(group, "pub use crate::{ident}Id as {alias}Id;");
            }
            if self.components.has_requests(component) {
                let module = object.mod_path();
                let _ = writeln!(group, "pub mod {name} {{ pub use crate::{module}::*;");
                for request in self.model.requests_for_component(component) {
                    let short = request
                        .ident
                        .strip_suffix(ident.as_ref())
                        .context("request name has no resource suffix")?;
                    let _ =
                        writeln!(group, "pub use crate::{module}::{} as {short};", request.ident);
                }
                group.push_str("}\n");
            }
        }
        for ((krate, namespace), content) in groups {
            output
                .append(krate.get_path_to_mod(), format!("pub mod {namespace} {{ {content} }}"))?;
        }
        Ok(())
    }

    fn write_crate_base(&self, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
        let locations = self.output_locations();
        for krate in &*ALL_CRATES {
            let mut dependencies = self.crate_dependencies.neighbors(*krate).collect::<Vec<_>>();
            dependencies.sort_unstable();
            let base_path = PathBuf::from(krate.generated_out_path());
            let request_features = locations
                .crate_members(*krate)
                .into_iter()
                .filter(|c| locations.components.has_requests(*c) && *krate != Crate::SHARED)
                .map(|c| locations.components.get_by_id(c).mod_path())
                .collect();

            let toml = gen_crate_toml(
                *krate,
                dependencies,
                request_features,
                &self.version,
                self.spec.is_v2_experiment(),
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
