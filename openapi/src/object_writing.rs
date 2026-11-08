use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;

use anyhow::ensure;
use indoc::formatdoc;

use crate::STRIPE_TYPES;
use crate::component_catalog::ComponentId;
use crate::crates::Crate;
use crate::exports::{PublicExport, component_exports};
use crate::item_catalog::{ItemCatalog, ItemId, RustItem};
use crate::item_layout::{EmissionSite, ItemLayout};
use crate::model_availability::{Availability, ModelAvailability};
use crate::output::GeneratedWorkspace;
use crate::output_locations::OutputLocations;
use crate::render::{Lifetime, RenderContext};
use crate::request_model::RequestModel;
use crate::rust_object::EnumOfObjects::{MaybeDeleted, ObjectUnion};
use crate::rust_object::{ObjectIdentity, RustObject};
use crate::templates::ObjectWriter;
use crate::templates::object_trait::{write_object_trait, write_object_trait_for_enum};
use crate::templates::utils::write_doc_comment;

impl RenderContext<'_> {
    fn write_item_definition(&self, item: ItemId, out: &mut String) {
        self.assert_emits(item);
        let record = self.item(item);
        match &record.item {
            RustItem::Object(object) if !object.deferred_fields.is_empty() => {
                if let Some(doc) = &record.metadata.doc {
                    out.push_str(&write_doc_comment(doc, 0));
                }
                let RustObject::Struct(strukt) = &object.definition else { unreachable!() };
                ObjectWriter::new(*self, &record.metadata.ident, object.usage)
                    .write_polymorphic_struct(out, strukt);
            }
            RustItem::OpaqueObject => {
                let _ =
                    writeln!(out, "{STRIPE_TYPES}::def_object_value!({});", record.metadata.ident);
            }
            RustItem::OpaqueRecord(_) => {
                if let Some(doc) = &record.metadata.doc {
                    out.push_str(&write_doc_comment(doc, 0));
                }
                let _ =
                    writeln!(out, "{STRIPE_TYPES}::def_record_value!({});", record.metadata.ident);
            }
            RustItem::Object(object) => {
                if let Some(doc) = &record.metadata.doc {
                    let comment = write_doc_comment(doc, 0);
                    let _ = write!(out, "{comment}");
                }

                let properties = self.item_properties(item);
                let lifetime = properties.has_lifetime.then_some(Lifetime);
                let mut writer = ObjectWriter::new(*self, &record.metadata.ident, object.usage);
                writer
                    .lifetime(lifetime)
                    .derive_copy(properties.is_copy)
                    .derive_eq(properties.is_eq);

                match &object.definition {
                    RustObject::Struct(struct_) => {
                        writer.write_struct_definition(out, struct_);
                        if object.usage.used_as_request_param {
                            writer.write_struct_constructor(out, struct_);
                        }
                    }
                    RustObject::FieldlessEnum(variants) => {
                        writer.provide_unknown_variant(object.definition.provide_unknown_variant());
                        writer.use_underscore_unknown(object.definition.use_underscore_unknown());
                        writer.write_fieldless_enum_variants(out, variants);
                    }
                    RustObject::Enum(variants) => {
                        if let Some(objects) = &object.enum_of_objects {
                            writer.write_enum_of_objects(out, objects);
                        } else {
                            writer.write_arbitrary_enum_variants(out, variants);
                        }
                    }
                }
            }
            RustItem::IdNewtype => {
                let _ = writeln!(out, "{STRIPE_TYPES}::def_id!({});", record.metadata.ident);
            }
        }
    }

    fn write_item_impls(&self, item: ItemId, out: &mut String) {
        let record = self.item(item);
        let Some(object) = record.object() else { return };

        let ident = &record.metadata.ident;
        let obj = &object.definition;
        let object_tag = match obj {
            RustObject::Struct(strukt) => strukt.object_field.as_deref(),
            RustObject::Enum(variants)
                if matches!(object.enum_of_objects, Some(MaybeDeleted(_))) =>
            {
                let mut tags = variants.iter().map(|variant| {
                    let RustObject::Struct(model) =
                        self.item_object_for_type(variant.rust_type.as_ref()?)?
                    else {
                        return None;
                    };
                    model.object_field.as_deref()
                });
                tags.next().flatten().filter(|tag| tags.all(|other| other == Some(*tag)))
            }
            _ => None,
        };
        if let Some(tag) = object_tag.filter(|_| self.uses_opaque_relationships()) {
            let _ = writeln!(
                out,
                "impl {STRIPE_TYPES}::ObjectType for {ident} {{ const OBJECT_TYPE: &'static str = {tag:?}; }}"
            );
        }
        match (obj, &object.identity) {
            (_, None) => {}
            (RustObject::Struct(_), Some(ObjectIdentity::IdType(id_typ))) => {
                write_object_trait(out, ident, self.display_type(id_typ));
            }
            (RustObject::Enum(_), Some(ObjectIdentity::ObjectUnion)) => {
                let Some(object_names) = &object.enum_of_objects else {
                    panic!("Object {ident} is an enum that is not a union of stripe objects");
                };
                let ObjectUnion(objects) = object_names else {
                    panic!("Object {ident} is an enum that is not a union of stripe objects");
                };
                write_object_trait_for_enum(out, ident, objects, *self)
            }
            (kind, identity) => {
                panic!(
                    "invalid object identity combination for {ident}: {kind:?} with {identity:?}"
                )
            }
        }
    }
}

/// Emits each canonical declaration once, using its selected placement. The
/// caller controls source order; this writer controls files and module exports.
/// Children are visited in declaration order, but each has its own placement.
pub(crate) struct ItemEmitter<'a> {
    items: &'a ItemCatalog,
    layout: &'a ItemLayout,
    locations: OutputLocations<'a>,
    emitted: Vec<bool>,
    files: HashMap<PathBuf, EmissionSite>,
    modules: HashMap<(Crate, String), EmissionSite>,
    availability: Option<&'a ModelAvailability>,
}

impl<'a> ItemEmitter<'a> {
    pub(crate) fn new(
        items: &'a ItemCatalog,
        layout: &'a ItemLayout,
        locations: OutputLocations<'a>,
    ) -> Self {
        Self {
            items,
            layout,
            locations,
            emitted: vec![false; items.len()],
            files: HashMap::new(),
            modules: HashMap::new(),
            availability: None,
        }
    }

    pub(crate) fn with_availability(mut self, availability: &'a ModelAvailability) -> Self {
        self.availability = Some(availability);
        self
    }

    fn cfg(&self, site: EmissionSite) -> String {
        self.availability.map(|plan| self.condition(plan, site).cfg_attribute()).unwrap_or_default()
    }

    fn condition(&self, plan: &ModelAvailability, site: EmissionSite) -> Availability {
        let krate = match site {
            EmissionSite::ComponentTypes(component) => self.locations.types_crate(component),
            EmissionSite::Standalone { krate, .. } => krate,
            EmissionSite::ComponentRequests(_) | EmissionSite::Webhooks => {
                return plan.site(site).clone();
            }
        };
        plan.site(site).for_models(|feature| {
            plan.request_components(feature)
                .iter()
                .any(|component| self.locations.request_crate(*component) == krate)
        })
    }

    pub(crate) fn emit(
        &mut self,
        item: ItemId,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        ensure!(!self.emitted[item.index()], "declaration emitted twice: {item:?}");
        self.emitted[item.index()] = true;
        let site = self.layout.placement(item).emission_site();
        let path = self.open_site(site, output)?;
        let context = RenderContext::new(self.layout, self.items, site, self.locations.config);
        let mut text = String::new();
        context.write_item_definition(item, &mut text);
        output.append(&path, text)?;
        for &child in &self.items.get(item).children {
            self.emit(child, output)?;
        }
        let mut text = String::new();
        context.write_item_impls(item, &mut text);
        output.append(path, text)
    }

    pub(crate) fn requests(
        &mut self,
        component: ComponentId,
        requests: &[RequestModel],
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        if requests.is_empty() {
            return Ok(());
        }
        let site = EmissionSite::ComponentRequests(component);
        let path = self.open_site(site, output)?;
        let context = RenderContext::new(self.layout, self.items, site, self.locations.config);
        for request in requests {
            if let Some(item) = request.params.as_ref().and_then(|params| params.declaration) {
                self.emit(item, output)?;
            }
            output.append(&path, request.generate(context))?;
            if let Some(item) = request.returned.declaration {
                self.emit(item, output)?;
            }
        }
        output.append(path, "\n")
    }

    pub(crate) fn component_reexports(
        &self,
        component: ComponentId,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        for export in
            component_exports(component, self.items, self.layout, self.locations.component_layout)?
        {
            self.write_export(export, output)?;
        }
        Ok(())
    }

    pub(crate) fn reexport(
        &self,
        item: ItemId,
        facade: Crate,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        self.write_export(PublicExport::item(item, facade, self.layout)?, output)
    }

    fn write_export(
        &self,
        export: PublicExport,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        if let Some((facade, text)) = export.render(self.items, self.locations) {
            let site = match export {
                PublicExport::Item { item, .. } => self.layout.placement(item).emission_site(),
                PublicExport::ComponentModule { component, .. } => {
                    EmissionSite::ComponentTypes(component)
                }
            };
            output.append(
                facade.get_path_to_mod(self.locations.config),
                format!("{}{text}", self.cfg(site)),
            )?;
        }
        Ok(())
    }

    pub(crate) fn finish(self) -> anyhow::Result<()> {
        let missing = self
            .items
            .iter()
            .filter(|(item, _)| !self.emitted[item.index()])
            .map(|(item, record)| format!("{} ({item:?})", record.metadata.ident))
            .collect::<Vec<_>>();
        ensure!(missing.is_empty(), "declarations were not emitted: {}", missing.join(", "));
        Ok(())
    }

    fn open_site(
        &mut self,
        site: EmissionSite,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<PathBuf> {
        let path = self.locations.emission_path(site, self.items);
        if let Some(previous) = self.files.get(&path) {
            ensure!(
                *previous == site,
                "different emission sites use {}: {previous:?} and {site:?}",
                path.display()
            );
            return Ok(path);
        }
        self.files.insert(path.clone(), site);
        let cfg = self.cfg(site);
        let module = match site {
            EmissionSite::ComponentTypes(component)
            | EmissionSite::ComponentRequests(component) => {
                let krate = if matches!(site, EmissionSite::ComponentTypes(_)) {
                    self.locations.types_crate(component)
                } else {
                    self.locations.request_crate(component)
                };
                // Types and requests may intentionally share the component's
                // outer module, even though their content files differ.
                Some((
                    krate,
                    self.locations.components.get_by_id(component).mod_path(),
                    EmissionSite::ComponentTypes(component),
                ))
            }
            EmissionSite::Standalone { item, krate } => {
                Some((krate, self.locations.standalone_module(item, self.items), site))
            }
            EmissionSite::Webhooks => None,
        };
        if let Some((krate, module, owner)) = module {
            ensure!(module != "mod", "standalone module would replace the crate root");
            if let Some(previous) = self.modules.insert((krate, module.clone()), owner) {
                ensure!(
                    previous == owner,
                    "output module {}::{module} has conflicting owners: {previous:?} and {owner:?}",
                    krate.lib_name(self.locations.config)
                );
            }
        }
        match site {
            EmissionSite::ComponentTypes(component) => {
                let object = self.locations.components.get_by_id(component);
                let module = object.mod_path();
                let krate = self.locations.types_crate(component);
                if !self.locations.components.has_requests(component)
                    || self.locations.types_split_from_requests(component)
                {
                    output.append(
                        krate.get_path_to_mod(self.locations.config),
                        format!(
                            "{cfg}#[doc(hidden)]\npub mod {module};{cfg}#[doc(inline)]\npub use {module}::*;"
                        ),
                    )?;
                } else {
                    output.append(
                        self.locations.requests_module_path(component),
                        format!("{cfg}pub(crate) mod types;"),
                    )?;
                    output.append(
                        krate.get_path_to_mod(self.locations.config),
                        format!("{cfg}pub use {module}::types::*;"),
                    )?;
                }
            }
            EmissionSite::ComponentRequests(component) => {
                let module = self.locations.components.get_by_id(component).mod_path();
                let outer_cfg = self
                    .availability
                    .map(|plan| {
                        if self.locations.types_split_from_requests(component) {
                            plan.site(site).cfg_attribute()
                        } else {
                            let models =
                                self.condition(plan, EmissionSite::ComponentTypes(component));
                            plan.site(site).union(&models).cfg_attribute()
                        }
                    })
                    .unwrap_or_default();
                output.append(
                    self.locations.request_crate(component).get_path_to_mod(self.locations.config),
                    format!("{outer_cfg}pub mod {module};"),
                )?;
                let request_cfg = if self.availability.is_some() {
                    cfg
                } else {
                    format!("#[cfg(feature = {module:?})]\n")
                };
                output.append(
                    self.locations.requests_module_path(component),
                    formatdoc! {
                        r#"
                    {request_cfg}mod requests;
                    {request_cfg}pub use requests::*;
                    "#
                    },
                )?;
                output.append(&path, "use stripe_client_core::{StripeClient, StripeBlockingClient, StripeRequest, RequestBuilder, StripeMethod};\n\n")?;
            }
            EmissionSite::Standalone { item, krate } => {
                let module = self.locations.standalone_module(item, self.items);
                let ident = &self.items.get(item).metadata.ident;
                output.append(
                    krate.get_path_to_mod(self.locations.config),
                    format!("{cfg}pub mod {module}; {cfg}pub use {module}::{ident};"),
                )?;
            }
            EmissionSite::Webhooks => {}
        }
        Ok(path)
    }
}
