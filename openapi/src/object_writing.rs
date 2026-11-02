use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;

use anyhow::{bail, ensure};
use indoc::formatdoc;

use crate::STRIPE_TYPES;
use crate::component_catalog::ComponentId;
use crate::crates::Crate;
use crate::item_catalog::{ItemCatalog, ItemId, ItemMetadata, ItemProperties, RustItem};
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::model_availability::{Availability, ModelAvailability};
use crate::output::GeneratedWorkspace;
use crate::output_locations::OutputLocations;
use crate::render::{Lifetime, RenderContext};
use crate::request_model::RequestModel;
use crate::rust_object::EnumOfObjects::{MaybeDeleted, ObjectUnion};
use crate::rust_object::{ObjectIdentity, ObjectUsage, RustObject};
use crate::rust_type::RustType;
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
                let writer = ObjectWriter::new(*self, &record.metadata.ident, object.usage);
                writer.write_polymorphic_struct(out, strukt);
            }
            RustItem::Object(object) => self.write_object_with_properties(
                &object.definition,
                &record.metadata,
                object.usage,
                self.item_properties(item),
                object.enum_of_objects.as_ref(),
                out,
            ),
            RustItem::IdNewtype => {
                let _ = writeln!(out, "{STRIPE_TYPES}::def_id!({});", record.metadata.ident);
            }
            RustItem::OpaqueObject(tag) => {
                let _ =
                    writeln!(out, "{STRIPE_TYPES}::def_object_value!({});", record.metadata.ident);
                if let Some(tag) = tag {
                    let _ = writeln!(
                        out,
                        "impl {STRIPE_TYPES}::ObjectType for {} {{ const OBJECT_TYPE: &'static str = {tag:?}; }}",
                        record.metadata.ident
                    );
                }
            }
            RustItem::OpaqueRecord(_) => {
                if let Some(doc) = &record.metadata.doc {
                    let _ = write!(out, "{}", write_doc_comment(doc, 0));
                }
                let _ =
                    writeln!(out, "{STRIPE_TYPES}::def_record_value!({});", record.metadata.ident);
            }
        }
    }

    fn write_object_with_properties(
        &self,
        obj: &RustObject,
        metadata: &ItemMetadata,
        usage: ObjectUsage,
        properties: ItemProperties,
        enum_of_objects: Option<&crate::rust_object::EnumOfObjects>,
        out: &mut String,
    ) {
        if let Some(doc) = &metadata.doc {
            let comment = write_doc_comment(doc, 0);
            let _ = write!(out, "{comment}");
        }

        // If the object contains any references, we'll need to print with a lifetime
        let lifetime = if properties.has_lifetime { Some(Lifetime) } else { None };
        let ident = &metadata.ident;

        let mut writer = ObjectWriter::new(*self, ident, usage);
        writer.lifetime(lifetime).derive_copy(properties.is_copy).derive_eq(properties.is_eq);

        match obj {
            RustObject::Struct(struct_) => {
                writer.write_struct_definition(out, struct_);
                if usage.used_as_request_param {
                    writer.write_struct_constructor(out, struct_);
                }
            }
            RustObject::FieldlessEnum(variants) => {
                writer.provide_unknown_variant(obj.provide_unknown_variant());
                writer.use_underscore_unknown(obj.use_underscore_unknown());
                writer.write_fieldless_enum_variants(out, variants);
            }
            RustObject::Enum(variants) => {
                if let Some(objects) = enum_of_objects {
                    writer.write_enum_of_objects(out, objects);
                } else {
                    writer.write_arbitrary_enum_variants(out, variants);
                }
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
        if let Some(tag) = object_tag {
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
        let context = RenderContext::new(self.layout, self.items, site);
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
        let context = RenderContext::new(self.layout, self.items, site);
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

    /// Decoder methods compile with their payload declaration. The raw record's
    /// module can therefore be enabled without compiling every optional detail.
    pub(crate) fn record_accessors(
        &mut self,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        for accessor in self.items.record_accessors() {
            let payload = self.layout.placement(accessor.payload);
            let owner = self.layout.placement(accessor.owner);
            ensure!(
                matches!(owner.address(), ItemAddress::CrateRoot(_))
                    && owner.address() == payload.address(),
                "lazy record accessor {} requires its owner and payload in the same public crate",
                accessor.field.field_name
            );
            ensure!(self.emitted[accessor.payload.index()], "record payload has not been emitted");
            let site = payload.emission_site();
            let path = self.locations.emission_path(site, self.items);
            ensure!(
                self.files.get(&path) == Some(&site),
                "record payload file has not been opened"
            );
            let context = RenderContext::new(self.layout, self.items, site);
            let owner_type = RustType::Item(accessor.owner);
            let payload_type = RustType::Item(accessor.payload);
            let method = accessor.field.variable_name();
            let mut text = String::new();
            let _ = writeln!(text, "impl {} {{", context.display_type(&owner_type));
            if let Some(doc) = &accessor.field.doc_comment {
                let _ = write!(text, "{}", write_doc_comment(doc, 4));
            }
            let _ = write!(
                text,
                "{}",
                write_doc_comment(
                    "Decode this optional field without consuming the record.\n\n\
                 Each call clones and decodes only this field's retained JSON subtree.\n\n\
                 # Errors\n\
                 Returns an error if a present, non-null payload does not satisfy the selected model.",
                    4,
                )
            );
            let _ = writeln!(
                text,
                "pub fn {method}(&self) -> Result<{}, {STRIPE_TYPES}::RecordDecodeError> {{ self.field_as::<{}>({:?}) }}\n}}",
                context.display_type(&accessor.field.rust_type),
                context.display_type(&payload_type),
                accessor.field.wire_name(),
            );
            output.append(path, text)?;
        }
        Ok(())
    }

    pub(crate) fn component_reexports(
        &self,
        component: ComponentId,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        let facade = self.locations.request_crate(component);
        let model = self.items.component_root(component);
        let placement = self.layout.placement(model);
        let mut wildcard_site = None;
        // Keep the existing public component modules and their wildcard
        // reexports when the model still occupies its component types module.
        if let ItemAddress::CrateRoot(krate) = placement.address()
            && krate != facade
            && placement.emission_site() == EmissionSite::ComponentTypes(component)
        {
            let module = self.locations.components.get_by_id(component).mod_path();
            output.append(
                facade.get_path_to_mod(),
                format!(
                    "{}pub use {}::{module}::*;",
                    self.cfg(placement.emission_site()),
                    krate.lib_name()
                ),
            )?;
            wildcard_site = Some(placement.emission_site());
        }
        for item in std::iter::once(model).chain(self.items.component_id_item(component)) {
            let placement = self.layout.placement(item);
            ensure!(
                matches!(placement.address(), ItemAddress::CrateRoot(_)),
                "component public declaration is not exported: {item:?}"
            );
            if Some(placement.emission_site()) != wildcard_site {
                self.reexport(item, facade, output)?;
            }
        }
        Ok(())
    }

    pub(crate) fn reexport(
        &self,
        item: ItemId,
        facade: Crate,
        output: &mut GeneratedWorkspace,
    ) -> anyhow::Result<()> {
        let ItemAddress::CrateRoot(krate) = self.layout.placement(item).address() else {
            bail!("public declaration is not exported: {item:?}")
        };
        if krate != facade {
            output.append(
                facade.get_path_to_mod(),
                format!(
                    "{}pub use {}::{};",
                    self.cfg(self.layout.placement(item).emission_site()),
                    krate.lib_name(),
                    self.items.get(item).metadata.ident
                ),
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
                    krate.lib_name()
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
                        krate.get_path_to_mod(),
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
                        krate.get_path_to_mod(),
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
                    self.locations.request_crate(component).get_path_to_mod(),
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
                    krate.get_path_to_mod(),
                    format!("{cfg}pub mod {module}; {cfg}pub use {module}::{ident};"),
                )?;
            }
            EmissionSite::Webhooks => {}
        }
        Ok(path)
    }
}
