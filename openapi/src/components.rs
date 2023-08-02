use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use anyhow::Context;
use indexmap::{IndexMap, IndexSet};
use petgraph::Direction;
use tracing::info;

use crate::crate_inference::{maybe_specified_crate, validate_crate_map, Crate};
use crate::printable::{PrintableContainer, PrintableType};
use crate::requests::parse_requests;
use crate::rust_object::RustObject;
use crate::rust_type::{Container, PathToType, RustType};
use crate::spec::{as_object_properties, get_request_form_parameters, Spec};
use crate::spec_inference::{infer_id_name, Inference};
use crate::stripe_object::{
    parse_stripe_schema_as_rust_object, OperationType, StripeObject, StripeOperation,
    StripeResource,
};
use crate::types::{ComponentPath, RustIdent};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct CrateInfo {
    krate: Crate,
    type_defs_in_stripe_types: bool,
}

impl CrateInfo {
    pub fn new(krate: Crate) -> Self {
        Self { krate, type_defs_in_stripe_types: false }
    }

    pub fn set_type_defs_in_types_crate(&mut self) {
        self.type_defs_in_stripe_types = true;
    }

    pub fn are_type_defs_types_crate(&self) -> bool {
        self.type_defs_in_stripe_types || self.krate == Crate::Types
    }

    pub fn for_types(&self) -> Crate {
        if self.type_defs_in_stripe_types {
            Crate::Types
        } else {
            self.krate
        }
    }

    pub fn base(&self) -> Crate {
        self.krate
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Module {
    Package { name: String, members: Vec<ComponentPath>, krate: Option<CrateInfo> },
    Component { path: ComponentPath, krate: Option<CrateInfo> },
}

impl Module {
    pub fn krate(&self) -> Option<CrateInfo> {
        match self {
            Module::Package { krate, .. } => *krate,
            Module::Component { krate, .. } => *krate,
        }
    }

    #[track_caller]
    pub fn krate_unwrapped(&self) -> CrateInfo {
        let Some(krate) = self.krate() else {
            panic!("Module has no crate assigned: \n{:?}", self);
        };
        krate
    }

    #[track_caller]
    pub fn krate_unwrapped_mut(&mut self) -> &mut CrateInfo {
        let Some(krate) = match self {
            Module::Package { krate, .. } => krate,
            Module::Component { krate, .. } => krate,
        }
        .as_mut() else {
            panic!("No krate assigned");
        };
        krate
    }

    pub fn assign_crate(&mut self, new_krate: Crate) {
        match self {
            Module::Package { krate, .. } => {
                *krate = Some(CrateInfo::new(new_krate));
            }
            Module::Component { krate, .. } => {
                *krate = Some(CrateInfo::new(new_krate));
            }
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ModuleName(String);

impl Debug for ModuleName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ModuleName {
    pub fn from_package_name(name: String) -> Self {
        Self(name)
    }

    pub fn from_comp_path(path: &ComponentPath) -> Self {
        Self(path.to_string())
    }
}

impl AsRef<str> for ModuleName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ModuleName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Deref for ModuleName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ModuleName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

pub struct Components {
    pub modules: IndexMap<ModuleName, Module>,
    components: IndexMap<ComponentPath, StripeObject>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PathInfo {
    pub krate: Option<Crate>,
    pub path: Option<String>,
}

impl Components {
    #[track_caller]
    pub fn get(&self, path: &ComponentPath) -> &StripeObject {
        &self.components[path.as_ref()]
    }

    pub fn parent_for_deleted(&self, path: &ComponentPath) -> &StripeObject {
        assert!(path.is_deleted(), "Expected path for deleted object");
        let parent =
            self.components.get(path.trim_start_matches("deleted_")).expect("Component not found");
        parent
    }

    #[track_caller]
    pub fn get_module(&self, name: &ModuleName) -> &Module {
        self.modules.get(name).unwrap_or_else(|| panic!("Module {} not found!", name))
    }

    #[track_caller]
    pub fn get_module_mut(&mut self, name: &ModuleName) -> &mut Module {
        self.modules.get_mut(name).unwrap_or_else(|| panic!("Module {} not found!", name))
    }

    pub fn get_crate_members(&self, krate: Crate) -> IndexSet<&ModuleName> {
        let mut members = IndexSet::new();
        for (mod_name, module) in &self.modules {
            let Some(mod_krate) = module.krate() else {
                continue;
            };
            if mod_krate.base() == krate || mod_krate.for_types() == krate {
                members.insert(mod_name);
            }
        }
        members
    }

    pub fn maybe_get(&self, path: &str) -> Option<&StripeObject> {
        self.components.get(path)
    }

    #[track_caller]
    pub fn containing_crate(&self, path: &ComponentPath) -> CrateInfo {
        let mod_path = self.containing_module(path);
        let module = self.get_module(mod_path);
        module.krate_unwrapped()
    }

    pub fn containing_module(&self, path: &ComponentPath) -> &ModuleName {
        &self.get(path).module
    }

    #[track_caller]
    pub fn resolve_path(&self, path: &ComponentPath) -> PathInfo {
        let component = self.get(path);
        let mut pieces = vec![];
        let parent = if path.is_deleted() { self.parent_for_deleted(path) } else { component };
        pieces.push(parent.mod_path());
        if let Some(package) = &parent.resource.in_package {
            pieces.push(package.to_string());
        }
        let krate = self.containing_crate(path).for_types();
        let full_path = pieces.into_iter().rev().collect::<Vec<_>>().join("::");
        PathInfo { krate: Some(krate), path: Some(full_path) }
    }

    pub fn construct_printable_type(&self, typ: &RustType) -> PrintableType {
        match typ {
            RustType::Object(obj, metadata) => PrintableType::QualifiedPath {
                path: None,
                has_ref: obj.has_reference(),
                is_ref: false,
                ident: metadata.ident.clone(),
            },

            RustType::Path { path, has_reference, is_ref, .. } => {
                let (path, ident) = match path {
                    PathToType::Component(path) => {
                        let comp = self.get(path);
                        (Some(self.resolve_path(path)), comp.ident().clone())
                    }
                    PathToType::IntraFile(ident) => (None, ident.clone()),
                    PathToType::ObjectId(path) => {
                        let ident = infer_id_name(path);
                        let path_info = self.resolve_path(path);
                        (Some(path_info), ident)
                    }
                    PathToType::Types(ident) => {
                        (Some(PathInfo { krate: Some(Crate::Types), path: None }), ident.clone())
                    }
                };
                PrintableType::QualifiedPath {
                    path,
                    has_ref: *has_reference,
                    is_ref: *is_ref,
                    ident,
                }
            }
            RustType::Simple(typ) => PrintableType::Simple(*typ),
            RustType::Container(typ) => {
                let inner = Box::new(self.construct_printable_type(typ.value_typ()));
                let printable = match typ {
                    Container::List(_) => PrintableContainer::List(inner),
                    Container::Vec(_) => PrintableContainer::Vec(inner),
                    Container::Slice(_) => PrintableContainer::Slice(inner),
                    Container::Expandable(_) => PrintableContainer::Expandable(inner),
                    Container::Option(_) => PrintableContainer::Option(inner),
                    Container::Box(_) => PrintableContainer::Box(inner),
                    Container::Map { key, is_ref, .. } => {
                        PrintableContainer::Map { key: *key, is_ref: *is_ref, value: inner }
                    }
                };
                PrintableType::Container(printable)
            }
        }
    }

    pub fn add_component_deps<'a>(
        &'a self,
        deps: &mut IndexSet<&'a ComponentPath>,
        path: &'a ComponentPath,
    ) {
        let component = self.get(path);
        let base_type = component.rust_obj();
        self.add_deps_from_obj(deps, base_type);
        let reqs = &component.requests;
        let mut typs = vec![];
        for req in reqs {
            if let Some(params) = &req.params {
                typs.push(params);
            }
            typs.push(&req.returned);
            for path_param in &req.path_params {
                typs.push(&path_param.rust_type);
            }
        }
        for typ in typs {
            self.add_deps_from_typ(deps, typ);
        }
    }

    pub fn add_deps_from_obj<'a>(
        &'a self,
        deps: &mut IndexSet<&'a ComponentPath>,
        obj: &'a RustObject,
    ) {
        match obj {
            RustObject::Struct(fields) => {
                for field in fields {
                    self.add_deps_from_typ(deps, &field.rust_type);
                }
            }
            RustObject::Enum(variants) => {
                for variant in variants {
                    if let Some(typ) = &variant.rust_type {
                        self.add_deps_from_typ(deps, typ);
                    }
                }
            }
            RustObject::FieldlessEnum(_) => {}
        }
    }

    pub fn add_deps_from_typ<'a>(
        &'a self,
        deps: &mut IndexSet<&'a ComponentPath>,
        typ: &'a RustType,
    ) {
        match typ {
            RustType::Object(obj, _) => self.add_deps_from_obj(deps, obj),
            RustType::Path { path: PathToType::Component(path), .. } => {
                deps.insert(path);
            }
            RustType::Container(inner) => self.add_deps_from_typ(deps, inner.value_typ()),
            _ => {}
        }
    }

    pub fn deps_for_module<'a>(&'a self, module: &'a Module) -> IndexSet<&'a ComponentPath> {
        let mut deps = IndexSet::new();
        match module {
            Module::Package { members, .. } => {
                for path in members {
                    self.add_component_deps(&mut deps, path);
                }
            }
            Module::Component { path, .. } => {
                self.add_component_deps(&mut deps, path);
            }
        }
        deps
    }

    pub fn filter_unused_modules(&mut self) {
        let mut unused = vec![];
        loop {
            let mod_graph = self.gen_module_dep_graph();
            for (mod_name, module) in &self.modules {
                // This will be a false positive here since we don't include `ApiError` in
                // dependency calculation even though it is really the error type in all
                // requests.
                if mod_name.as_ref() == "error" {
                    continue;
                }
                match module {
                    Module::Package { .. } => {}
                    Module::Component { path, .. } => {
                        let obj = self.get(path);
                        if obj.requests.is_empty() {
                            if mod_graph.neighbors_directed(mod_name, Direction::Incoming).count()
                                == 0
                            {
                                unused.push(mod_name.clone());
                            }
                        }
                    }
                }
            }
            let done_filtering = unused.is_empty();
            if done_filtering {
                break;
            }

            info!("Filtering unused modules: {unused:#?}");
            for unused_mod in unused.drain(..) {
                self.modules.remove(&unused_mod);
                self.components.remove(unused_mod.as_ref());
            }
        }
    }
}

pub fn get_components(spec: &Spec) -> anyhow::Result<Components> {
    let mut components = IndexMap::with_capacity(spec.component_schemas().len());

    let mut resource_map = HashMap::new();
    let mut obj_map = IndexMap::new();
    let mut id_map = HashMap::new();
    for path in spec.component_schemas().keys() {
        let path = ComponentPath::new(path.clone());
        let schema = spec.get_component_schema(&path);
        let stripe_resource = StripeResource::from_schema(schema, path.clone())?;
        let data = parse_stripe_schema_as_rust_object(schema, &path, &stripe_resource.base_ident);
        if let Some(obj_name) = &data.object_name {
            if let Some(id_typ) = data.id_type.as_ref().and_then(|t| t.as_id_path()) {
                id_map.insert(obj_name.clone(), id_typ.clone());
            }
        }
        resource_map.insert(path.clone(), stripe_resource);
        obj_map.insert(path, data);
    }

    for (path, data) in obj_map {
        let resource = resource_map.get(&path).unwrap();
        let schema = spec.get_component_schema(&path);
        let stripe_reqs: Vec<StripeOperation> =
            if let Some(val) = schema.schema_data.extensions.get("x-stripeOperations") {
                serde_json::from_value(val.clone())?
            } else {
                vec![]
            };
        let reqs = parse_requests(stripe_reqs, spec, &resource.base_ident, &path, &id_map)?;
        components.insert(
            path.clone(),
            StripeObject {
                requests: reqs,
                module: find_parent_module(&path, &resource_map),
                resource: resource.clone(),
                data,
            },
        );
    }

    let mut modules = IndexMap::new();
    let mut packages = IndexSet::new();
    for (path, obj) in &components {
        if obj.resource.in_package.is_none() && !obj.is_deleted_item() {
            modules.insert(
                ModuleName::from_comp_path(obj.path()),
                Module::Component {
                    path: path.clone(),
                    krate: maybe_specified_crate(path.as_ref()).map(CrateInfo::new),
                },
            );
        }
        if let Some(package) = &obj.resource.in_package {
            packages.insert(package.to_string());
        }
    }
    for package in packages {
        let mut members = vec![];
        for obj in components.values() {
            if obj.resource.in_package.as_deref() == Some(&package) && !obj.is_deleted_item() {
                members.push(obj.path().clone());
            }
        }
        let krate = maybe_specified_crate(&package).map(CrateInfo::new).unwrap_or_else(|| {
            panic!("Package {} required a crate mapping", package);
        });
        modules.insert(
            ModuleName::from_package_name(package.clone()),
            Module::Package { name: package, members, krate: Some(krate) },
        );
    }
    let mut components = Components { modules, components };
    components.filter_unused_modules();

    validate_crate_map(&components)?;
    components.infer_all_crate_assignments()?;
    Ok(components)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OverrideMetadata {
    pub ident: RustIdent,
    pub doc: String,
    pub mod_path: String,
}

pub struct Overrides {
    pub overrides: IndexMap<RustObject, OverrideMetadata>,
}

#[derive(Debug, Copy, Clone)]
pub struct OverrideData {
    pub doc: &'static str,
    pub mod_path: &'static str,
    pub ident: &'static str,
    pub source: OverrideSource,
}

#[derive(Debug, Copy, Clone)]
pub struct RequestOverrideSource {
    path: &'static str,
    op: OperationType,
    field_name: &'static str,
}

#[derive(Debug, Copy, Clone)]
pub enum OverrideSource {
    Request(RequestOverrideSource),
}

const OVERRIDES: &[OverrideData] = &[OverrideData {
    doc: "",
    mod_path: "api_version",
    ident: "ApiVersion",
    source: OverrideSource::Request(RequestOverrideSource {
        path: "/v1/webhook_endpoints",
        op: OperationType::Post,
        field_name: "api_version",
    }),
}];

fn get_override_object(
    spec: &Spec,
    data: &OverrideData,
) -> anyhow::Result<(RustObject, OverrideMetadata)> {
    match data.source {
        OverrideSource::Request(req_src) => {
            let op = spec
                .get_request_operation(req_src.path, req_src.op)
                .context("Request not found")?;
            let form_params = get_request_form_parameters(op)
                .context("No form params")?
                .as_item()
                .context("Was a ref")?;
            let typ = as_object_properties(form_params).context("Not an object")?;
            let schema = typ.get(req_src.field_name).context("Field not found")?;
            let ident = RustIdent::create(data.ident);
            let (obj, _) = Inference::new(&ident)
                .infer_schema_or_ref_type(schema)
                .into_object()
                .context("Expected object type to be inferred")?;
            Ok((
                obj,
                OverrideMetadata {
                    ident,
                    doc: data.doc.to_string(),
                    mod_path: data.mod_path.to_string(),
                },
            ))
        }
    }
}

pub fn build_field_overrides(spec: &Spec) -> anyhow::Result<Overrides> {
    let mut overrides = IndexMap::new();
    for override_ in OVERRIDES {
        let (obj, meta) = get_override_object(spec, override_)
            .with_context(|| format!("Failed to construct override for source {override_:?}"))?;
        overrides.insert(obj, meta);
    }
    Ok(Overrides { overrides })
}

impl Overrides {
    pub fn replace_typ(&self, typ: &mut RustType) {
        match typ {
            RustType::Object(obj, _) => {
                if let Some(meta) = self.overrides.get(obj) {
                    *typ = RustType::Path {
                        path: PathToType::Types(meta.ident.clone()),
                        is_ref: false,
                        has_reference: obj.has_reference(),
                        is_copy: obj.is_copy(),
                    }
                } else {
                    self.replace_obj(obj);
                }
            }
            RustType::Simple(_) => {}
            RustType::Path { .. } => {}
            RustType::Container(inner) => {
                self.replace_typ(inner.value_typ_mut());
            }
        }
    }

    fn replace_obj(&self, obj: &mut RustObject) {
        match obj {
            RustObject::Struct(fields) => {
                for field in fields {
                    self.replace_typ(&mut field.rust_type);
                }
            }
            RustObject::FieldlessEnum(_) => {}
            RustObject::Enum(variants) => {
                for var in variants {
                    if let Some(typ) = &mut var.rust_type {
                        self.replace_typ(typ);
                    }
                }
            }
        }
    }
}

fn find_parent_module(
    path: &ComponentPath,
    components: &HashMap<ComponentPath, StripeResource>,
) -> ModuleName {
    let start_path = if path.is_deleted() { path.as_not_deleted() } else { path.clone() };
    let parent = components.get(&start_path).unwrap();
    if let Some(package_name) = &parent.in_package {
        ModuleName::from_package_name(package_name.clone())
    } else {
        ModuleName::from_comp_path(&parent.path)
    }
}

impl Components {
    pub fn apply_overrides(&mut self, overrides: &Overrides) {
        for obj in self.components.values_mut() {
            let obj_typ = &mut obj.data.obj;
            overrides.replace_obj(obj_typ);

            for req in &mut obj.requests {
                if let Some(typ) = &mut req.params {
                    overrides.replace_typ(typ);
                }
                overrides.replace_typ(&mut req.returned);
            }
        }
    }
}
