use std::collections::{HashMap, HashSet};

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::deduplication::SharingRule;
use crate::item_origin::{ItemOccurrence, ItemOrigin, TypeRoot};
use crate::rust_object::{
    EnumOfObjects, ObjectIdentity, ObjectMetadata, ObjectUsage, RustObject, as_enum_of_objects,
};
use crate::rust_type::{DeclarationRoot, RustType};
use crate::types::RustIdent;

/// Dense, run-local identity for a canonical Rust declaration.
///
/// IDs are allocated after overrides and deduplication, so one ID corresponds
/// to one type declaration that will be generated.
// TODO: macro for the id newtypes
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ItemId(u32);

impl ItemId {
    fn from_index(index: usize) -> anyhow::Result<Self> {
        anyhow::ensure!(index <= u32::MAX as usize, "too many generated Rust items");
        Ok(Self(index as u32))
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The canonical type at one inference root and the declaration structurally
/// introduced there, if any.
///
/// A root may reference an existing item without declaring it. For example, a
/// request returning `Customer` has an item type but no request-local declaration.
#[derive(Debug, Clone)]
pub struct CanonicalTypeRoot {
    pub typ: RustType,
    pub declaration: Option<ItemId>,
}

/// Presentation metadata retained for a lowered Rust declaration.
///
/// Inference-only provenance such as schema titles, field names, and parents
/// remains on `ObjectMetadata` and is deliberately discarded at allocation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ItemMetadata {
    pub ident: RustIdent,
    pub doc: Option<String>,
}

impl ItemMetadata {
    pub fn new(ident: RustIdent) -> Self {
        Self { ident, doc: None }
    }

    pub fn doc(mut self, doc: String) -> Self {
        self.doc = Some(doc);
        self
    }
}

impl From<&ObjectMetadata> for ItemMetadata {
    fn from(metadata: &ObjectMetadata) -> Self {
        Self { ident: metadata.ident.clone(), doc: metadata.doc.clone() }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ItemKind {
    Struct,
    Enum,
    FieldlessEnum,
    IdNewtype,
}

impl ItemKind {
    pub fn of(item: &RustItem) -> Self {
        match item {
            RustItem::Object(object) => match &object.definition {
                RustObject::Struct(_) => Self::Struct,
                RustObject::Enum(_) => Self::Enum,
                RustObject::FieldlessEnum(_) => Self::FieldlessEnum,
            },
            RustItem::IdNewtype => Self::IdNewtype,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectItem {
    pub definition: RustObject,
    pub usage: ObjectUsage,
    pub enum_of_objects: Option<EnumOfObjects>,
    /// Canonical identity semantics used by a top-level component's `Object` impl.
    pub identity: Option<ObjectIdentity>,
}

#[derive(Debug, Clone)]
pub enum RustItem {
    Object(Box<ObjectItem>),
    IdNewtype,
}

#[derive(Debug, Clone)]
pub struct ItemRecord {
    /// Canonical declaration origin, used for emission ownership. Original
    /// inferred occurrences remain separate when a declaration is shared.
    pub origin: ItemOrigin,
    pub occurrences: Vec<ItemOccurrence>,
    /// The rule selecting this declaration for sharing. Nested declarations
    /// can inherit several occurrences from a shared parent without having
    /// been selected independently.
    pub sharing_rule: Option<SharingRule>,
    pub metadata: ItemMetadata,
    /// Declarations structurally introduced by this item, in existing
    /// recursive emission order. Cross-item path references are not children.
    pub children: Vec<ItemId>,
    /// Canonical declaration with nested declarations replaced by `ItemId`
    /// references into this catalog.
    pub item: RustItem,
}

impl ItemRecord {
    pub fn object(&self) -> Option<&ObjectItem> {
        match &self.item {
            RustItem::Object(object) => Some(object),
            RustItem::IdNewtype => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct ItemProperties {
    pub has_lifetime: bool,
    pub is_copy: bool,
    pub is_eq: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ItemCatalog {
    items: Vec<ItemRecord>,
    roots: HashMap<TypeRoot, CanonicalTypeRoot>,
    properties: Vec<ItemProperties>,
    property_analysis_rounds: usize,
}

impl ItemCatalog {
    pub fn build(
        components: &Components,
        component_metadata: impl Fn(ComponentId) -> ItemMetadata,
    ) -> anyhow::Result<Self> {
        let mut builder = CatalogBuilder::default();

        // Match the current declaration-emission order: each component's
        // model, request-local objects, and deduplicated objects, followed by
        // shared overrides and webhook-local unions.
        for (&component, object) in &components.components {
            let root = TypeRoot::Declaration(DeclarationRoot::Component(component));
            let metadata = component_metadata(component);
            let item = builder.collect_root_object(
                root,
                object.rust_obj(),
                metadata,
                ObjectUsage::type_def(),
            )?;
            let RustItem::Object(item) = &mut builder.item_mut(item).item else {
                unreachable!("component root was allocated as an object")
            };
            item.identity = object.identity().cloned();
            if object
                .identity()
                .and_then(ObjectIdentity::id_type)
                .and_then(RustType::as_id_or_opt_id_path)
                == Some(component)
            {
                builder.collect_id_newtype(component, object.id_type_ident())?;
            }
            for request in components.requests_for_component(component) {
                for (index, path_param) in request.path_params.iter().enumerate() {
                    let _ = builder.collect_root_type(
                        TypeRoot::OperationPathParam { operation: request.operation_id, index },
                        &path_param.rust_type,
                        ObjectUsage::request_param(),
                    )?;
                }
                if let Some(params) = &request.params {
                    let _ = builder.collect_root_type(
                        TypeRoot::OperationParam(request.operation_id),
                        &params.typ,
                        ObjectUsage::request_param(),
                    )?;
                }
                let _ = builder.collect_root_type(
                    TypeRoot::OperationReturn(request.operation_id),
                    &request.returned,
                    ObjectUsage::return_type(),
                )?;
            }

            for (ident, deduplicated) in &object.deduplicated_objects {
                let metadata = ItemMetadata::new(ident.clone());
                let item = builder.collect_root_object(
                    TypeRoot::Declaration(DeclarationRoot::Deduplicated {
                        component,
                        ident: ident.clone(),
                    }),
                    &deduplicated.object,
                    metadata,
                    deduplicated.usage,
                )?;
                builder.set_shared_occurrences(item, &deduplicated.occurrences);
                builder.item_mut(item).sharing_rule = Some(deduplicated.sharing_rule);
            }
        }

        for (ident, shared) in &components.extra_types {
            let metadata = ItemMetadata { ident: ident.clone(), doc: shared.doc.clone() };
            builder.collect_root_object(
                TypeRoot::Declaration(DeclarationRoot::Shared(ident.clone())),
                &shared.obj,
                metadata,
                shared.usage,
            )?;
        }

        for webhook in &components.webhook_objs {
            let root = TypeRoot::Webhook(webhook.event_type.clone());
            if let Some(item) =
                builder.collect_root_type(root, &webhook.typ, ObjectUsage::type_def())?
            {
                // Webhook-local object unions are declarations named for the
                // event, not for the spec's enclosing `object` field.
                builder.item_mut(item).metadata.ident = RustIdent::create(&webhook.event_type);
            }
        }

        let mut catalog = builder.finish()?;
        catalog.lower_named_references()?;
        catalog.validate_lowered_structure()?;
        catalog.classify_enums(components);
        catalog.analyze_properties()?;
        catalog.validate_component_lifetimes(components)?;

        for &component in components.components.keys() {
            let root = catalog.component_root(component);
            anyhow::ensure!(
                catalog.get(root).origin
                    == ItemOrigin {
                        type_root: TypeRoot::Declaration(DeclarationRoot::Component(component)),
                        path: Vec::new(),
                    },
                "component root item did not retain its canonical origin: {}",
                components.component_name(component)
            );
        }
        Ok(catalog)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, id: ItemId) -> &ItemRecord {
        &self.items[id.index()]
    }

    pub fn component_root(&self, component: ComponentId) -> ItemId {
        self.root(&TypeRoot::Declaration(DeclarationRoot::Component(component)))
            .and_then(|root| root.declaration)
            .unwrap_or_else(|| panic!("component root was not cataloged: {component:?}"))
    }

    pub fn root(&self, root: &TypeRoot) -> Option<&CanonicalTypeRoot> {
        self.roots.get(root)
    }

    pub fn component_for_type(&self, typ: &RustType) -> Option<ComponentId> {
        component_for_type(&self.items, typ)
    }

    pub fn properties_for_type(&self, typ: &RustType) -> anyhow::Result<ItemProperties> {
        crate::item_properties::resolve(&self.items, &self.properties, typ)
    }

    pub fn properties(&self, item: ItemId) -> ItemProperties {
        self.properties[item.index()]
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = (ItemId, &ItemRecord)> {
        self.items.iter().enumerate().map(|(index, item)| {
            let id = ItemId::from_index(index).expect("catalog length was already validated");
            (id, item)
        })
    }

    pub const fn property_analysis_rounds(&self) -> usize {
        self.property_analysis_rounds
    }

    fn lower_named_references(&mut self) -> anyhow::Result<()> {
        let declarations = self
            .roots
            .iter()
            .filter_map(|(root, record)| record.declaration.map(|item| (root.clone(), item)))
            .collect::<HashMap<_, _>>();

        for item in &mut self.items {
            if let RustItem::Object(object) = &mut item.item {
                lower_object_references(&mut object.definition, &declarations)?;
                if let Some(ObjectIdentity::IdType(id_type)) = &mut object.identity {
                    lower_type_references(id_type, &declarations)?;
                }
            }
        }
        for root in self.roots.values_mut() {
            lower_type_references(&mut root.typ, &declarations)?;
        }
        Ok(())
    }

    fn analyze_properties(&mut self) -> anyhow::Result<()> {
        let (inferred, rounds) = crate::item_properties::infer(&self.items)?;
        self.properties = inferred;
        self.property_analysis_rounds = rounds;
        Ok(())
    }

    fn validate_lowered_structure(&self) -> anyhow::Result<()> {
        for (index, item) in self.items.iter().enumerate() {
            let id = ItemId::from_index(index)?;
            if let RustItem::Object(object) = &item.item {
                validate_lowered_object(&object.definition, self.items.len())
                    .map_err(|error| anyhow::anyhow!("invalid canonical item {id:?}: {error}"))?;
                if let Some(ObjectIdentity::IdType(id_type)) = &object.identity {
                    validate_lowered_type(id_type, self.items.len()).map_err(|error| {
                        anyhow::anyhow!("invalid canonical ID type for item {id:?}: {error}")
                    })?;
                }
            }
            for child in &item.children {
                anyhow::ensure!(
                    child.index() < self.items.len(),
                    "canonical item {id:?} references missing child {child:?}"
                );
            }
        }
        for (root, record) in &self.roots {
            validate_lowered_type(&record.typ, self.items.len())
                .map_err(|error| anyhow::anyhow!("invalid canonical root {root:?}: {error}"))?;
            if let Some(item) = record.declaration {
                let expected = ItemOrigin { type_root: root.clone(), path: Vec::new() };
                anyhow::ensure!(
                    self.get(item).origin == expected,
                    "canonical root declaration did not match its origin: {root:?}"
                );
            }
        }
        Ok(())
    }

    fn validate_component_lifetimes(&self, components: &Components) -> anyhow::Result<()> {
        for &component in components.components.keys() {
            let item = self.component_root(component);
            anyhow::ensure!(
                !self.properties(item).has_lifetime,
                "top-level component unexpectedly requires a lifetime: {}",
                components.component_name(component)
            );
        }
        Ok(())
    }

    fn classify_enums(&mut self, components: &Components) {
        let classifications = self
            .items
            .iter()
            .map(|item| match &item.item {
                RustItem::Object(object) => match &object.definition {
                    RustObject::Enum(variants) => as_enum_of_objects(components, variants, |typ| {
                        component_for_type(&self.items, typ)
                    }),
                    RustObject::Struct(_) | RustObject::FieldlessEnum(_) => None,
                },
                RustItem::IdNewtype => None,
            })
            .collect::<Vec<_>>();
        for (item, classification) in self.items.iter_mut().zip(classifications) {
            if let RustItem::Object(object) = &mut item.item {
                object.enum_of_objects = classification;
            }
        }
    }
}

fn validate_lowered_object(object: &RustObject, item_count: usize) -> anyhow::Result<()> {
    match object {
        RustObject::Struct(struct_) => {
            for field in &struct_.fields {
                validate_lowered_type(&field.rust_type, item_count)?;
            }
        }
        RustObject::Enum(variants) => {
            for variant in variants {
                if let Some(typ) = &variant.rust_type {
                    validate_lowered_type(typ, item_count)?;
                }
            }
        }
        RustObject::FieldlessEnum(_) => {}
    }
    Ok(())
}

fn validate_lowered_type(typ: &RustType, item_count: usize) -> anyhow::Result<()> {
    match typ {
        RustType::Object(_, _) => anyhow::bail!("inline object remained after item allocation"),
        RustType::Item(item) => {
            anyhow::ensure!(
                item.index() < item_count,
                "type references missing canonical item {item:?}"
            );
            Ok(())
        }
        RustType::Primitive(_) | RustType::External(_) => Ok(()),
        RustType::Path(path) => anyhow::bail!("named path remained after item lowering: {path:?}"),
        RustType::Container(container) => validate_lowered_type(container.value_typ(), item_count),
        RustType::Reference(inner) => validate_lowered_type(inner, item_count),
    }
}

fn component_for_type(items: &[ItemRecord], typ: &RustType) -> Option<ComponentId> {
    match typ {
        RustType::Item(item) => {
            let target = items.get(item.index())?;
            if target.origin.path.is_empty()
                && let TypeRoot::Declaration(DeclarationRoot::Component(component)) =
                    target.origin.type_root
            {
                Some(component)
            } else {
                None
            }
        }
        RustType::Reference(inner) => component_for_type(items, inner),
        _ => None,
    }
}

fn lower_object_references(
    object: &mut RustObject,
    roots: &HashMap<TypeRoot, ItemId>,
) -> anyhow::Result<()> {
    match object {
        RustObject::Struct(struct_) => {
            for field in &mut struct_.fields {
                lower_type_references(&mut field.rust_type, roots)?;
            }
        }
        RustObject::Enum(variants) => {
            for variant in variants {
                if let Some(typ) = &mut variant.rust_type {
                    lower_type_references(typ, roots)?;
                }
            }
        }
        RustObject::FieldlessEnum(_) => {}
    }
    Ok(())
}

fn lower_type_references(
    typ: &mut RustType,
    roots: &HashMap<TypeRoot, ItemId>,
) -> anyhow::Result<()> {
    match typ {
        RustType::Object(object, _) => lower_object_references(object, roots),
        RustType::Path(path) => {
            let root = TypeRoot::Declaration(path.clone());
            let item = roots.get(&root).copied().ok_or_else(|| {
                anyhow::anyhow!("named type reference has no canonical item: {root:?}")
            })?;
            *typ = RustType::Item(item);
            Ok(())
        }
        RustType::Container(container) => lower_type_references(container.value_typ_mut(), roots),
        RustType::Reference(inner) => lower_type_references(inner, roots),
        RustType::Primitive(_) | RustType::External(_) | RustType::Item(_) => Ok(()),
    }
}

#[derive(Default)]
struct CatalogBuilder {
    catalog: ItemCatalog,
    origins: HashSet<ItemOrigin>,
    /// Parent-first IDs are reserved before recursively collecting children.
    /// Incomplete records never escape into the finished `ItemCatalog`.
    items: Vec<Option<ItemRecord>>,
    path: Vec<String>,
}

impl CatalogBuilder {
    fn set_shared_occurrences(&mut self, item: ItemId, occurrences: &[ItemOccurrence]) {
        let record = self.item_mut(item);
        record.occurrences = occurrences
            .iter()
            .map(|source| {
                let mut origin = source.origin.clone();
                origin.path.extend(record.origin.path.iter().cloned());
                ItemOccurrence { origin, usage: source.usage }
            })
            .collect();
        let children = record.children.clone();
        for child in children {
            self.set_shared_occurrences(child, occurrences);
        }
    }

    fn reserve_item(&mut self, origin: ItemOrigin) -> anyhow::Result<ItemId> {
        let id = ItemId::from_index(self.items.len())?;
        anyhow::ensure!(
            self.origins.insert(origin.clone()),
            "multiple generated items had the same origin: {origin:?}"
        );
        self.items.push(None);
        Ok(id)
    }

    fn complete_item(&mut self, id: ItemId, item: ItemRecord) -> anyhow::Result<()> {
        let slot = self
            .items
            .get_mut(id.index())
            .ok_or_else(|| anyhow::anyhow!("canonical item ID was not reserved: {id:?}"))?;
        anyhow::ensure!(slot.is_none(), "canonical item was completed twice: {id:?}");
        *slot = Some(item);
        Ok(())
    }

    fn item_mut(&mut self, id: ItemId) -> &mut ItemRecord {
        self.items[id.index()]
            .as_mut()
            .unwrap_or_else(|| panic!("canonical item was not completed: {id:?}"))
    }

    fn finish(mut self) -> anyhow::Result<ItemCatalog> {
        anyhow::ensure!(
            self.origins.len() == self.items.len(),
            "canonical item origins and declarations differed in length"
        );
        self.catalog.items = self
            .items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                item.ok_or_else(|| anyhow::anyhow!("canonical item {index} was never completed"))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(self.catalog)
    }

    fn collect_id_newtype(
        &mut self,
        component: ComponentId,
        ident: RustIdent,
    ) -> anyhow::Result<ItemId> {
        let root = TypeRoot::Declaration(DeclarationRoot::ObjectId(component));
        let origin = ItemOrigin { type_root: root.clone(), path: Vec::new() };
        let id = self.reserve_item(origin.clone())?;
        self.complete_item(
            id,
            ItemRecord {
                origin,
                occurrences: Vec::new(),
                sharing_rule: None,
                metadata: ItemMetadata::new(ident),
                children: Vec::new(),
                item: RustItem::IdNewtype,
            },
        )?;
        anyhow::ensure!(
            self.catalog
                .roots
                .insert(
                    root.clone(),
                    CanonicalTypeRoot { typ: RustType::Item(id), declaration: Some(id) },
                )
                .is_none(),
            "generated type root was cataloged twice: {root:?}"
        );
        Ok(id)
    }

    fn collect_root_object(
        &mut self,
        root: TypeRoot,
        object: &RustObject,
        metadata: ItemMetadata,
        usage: ObjectUsage,
    ) -> anyhow::Result<ItemId> {
        let item = self.collect_object(root.clone(), object, metadata, usage)?;
        anyhow::ensure!(
            self.catalog
                .roots
                .insert(
                    root.clone(),
                    CanonicalTypeRoot { typ: RustType::Item(item), declaration: Some(item) },
                )
                .is_none(),
            "generated type root was cataloged twice: {root:?}"
        );
        Ok(item)
    }

    fn collect_object(
        &mut self,
        root: TypeRoot,
        object: &RustObject,
        metadata: ItemMetadata,
        usage: ObjectUsage,
    ) -> anyhow::Result<ItemId> {
        let origin = ItemOrigin { type_root: root.clone(), path: self.path.clone() };
        let id = self.reserve_item(origin.clone())?;
        let mut lowered = object.clone();
        let mut children = Vec::new();
        match &mut lowered {
            RustObject::Struct(struct_) => {
                for field in &mut struct_.fields {
                    self.path.push(field.field_name.clone());
                    if let Some(child) =
                        self.lower_nested_type(&root, &mut field.rust_type, usage)?
                    {
                        children.push(child);
                    }
                    self.path.pop();
                }
            }
            RustObject::Enum(variants) => {
                for variant in variants {
                    let Some(typ) = &mut variant.rust_type else {
                        continue;
                    };
                    self.path.push(variant.variant.to_string());
                    if let Some(child) = self.lower_nested_type(&root, typ, usage)? {
                        children.push(child);
                    }
                    self.path.pop();
                }
            }
            RustObject::FieldlessEnum(_) => {}
        }
        self.complete_item(
            id,
            ItemRecord {
                occurrences: vec![ItemOccurrence { origin: origin.clone(), usage }],
                origin,
                sharing_rule: None,
                metadata,
                children,
                item: RustItem::Object(Box::new(ObjectItem {
                    definition: lowered,
                    usage,
                    enum_of_objects: None,
                    identity: None,
                })),
            },
        )?;
        Ok(id)
    }

    fn collect_root_type(
        &mut self,
        root: TypeRoot,
        typ: &RustType,
        usage: ObjectUsage,
    ) -> anyhow::Result<Option<ItemId>> {
        debug_assert!(self.path.is_empty());
        let mut lowered = typ.clone();
        let item = self.lower_nested_type(&root, &mut lowered, usage)?;
        anyhow::ensure!(
            self.catalog
                .roots
                .insert(root.clone(), CanonicalTypeRoot { typ: lowered, declaration: item })
                .is_none(),
            "generated type root was cataloged twice: {root:?}"
        );
        Ok(item)
    }

    fn lower_nested_type(
        &mut self,
        root: &TypeRoot,
        typ: &mut RustType,
        usage: ObjectUsage,
    ) -> anyhow::Result<Option<ItemId>> {
        match typ {
            RustType::Object(object, metadata) => {
                let metadata = ItemMetadata::from(&*metadata);
                let item = self.collect_object(root.clone(), object, metadata, usage)?;
                *typ = RustType::Item(item);
                return Ok(Some(item));
            }
            RustType::Container(container) => {
                return self.lower_nested_type(root, container.value_typ_mut(), usage);
            }
            RustType::Reference(inner) => return self.lower_nested_type(root, inner, usage),
            RustType::Primitive(_)
            | RustType::External(_)
            | RustType::Item(_)
            | RustType::Path(_) => {}
        }
        Ok(None)
    }
}

#[cfg(test)]
mod dependency_tests;

#[cfg(test)]
mod tests {
    use super::{CatalogBuilder, ItemMetadata, RustItem};
    use crate::component_catalog::ComponentCatalog;
    use crate::deduplication::SharingRule;
    use crate::item_origin::{ItemOccurrence, ItemOrigin, TypeRoot};
    use crate::rust_object::{
        ObjectIdentity, ObjectMetadata, ObjectUsage, RustObject, Struct, StructField,
    };
    use crate::rust_type::{DeclarationRoot, RustType};
    use crate::types::{ComponentName, RustIdent};

    #[test]
    fn allocates_deterministic_ids_for_nested_items() {
        let components =
            ComponentCatalog::from_names([ComponentName::new("charge".to_string())]).unwrap();
        let component = components.id_str("charge").unwrap();
        let nested = RustType::Object(
            RustObject::Struct(Struct::new(vec![])),
            ObjectMetadata::new(RustIdent::create("details")),
        );
        let root = RustObject::Struct(Struct::new(vec![StructField::new("details", nested)]));
        let mut builder = CatalogBuilder::default();

        let root_id = builder
            .collect_object(
                TypeRoot::Declaration(DeclarationRoot::Component(component)),
                &root,
                ItemMetadata::new(RustIdent::create("charge")),
                ObjectUsage::type_def(),
            )
            .unwrap();
        let catalog = builder.finish().unwrap();

        assert_eq!(root_id.index(), 0);
        assert_eq!(catalog.len(), 2);
        assert_eq!(catalog.iter().nth(1).unwrap().0.index(), 1);
        assert_eq!(catalog.iter().nth(1).unwrap().1.origin.path, ["details"]);
        let Some(object) = catalog.get(root_id).object() else { panic!("expected root object") };
        let RustObject::Struct(root) = &object.definition else { panic!("expected root struct") };
        assert!(matches!(root.fields[0].rust_type, RustType::Item(item) if item.index() == 1));
    }

    #[test]
    fn shared_parents_preserve_original_locations_for_nested_declarations() {
        let components =
            ComponentCatalog::from_names([ComponentName::new("customer".to_string())]).unwrap();
        let component = components.id_str("customer").unwrap();
        let root = TypeRoot::Declaration(DeclarationRoot::Deduplicated {
            component,
            ident: RustIdent::create("details"),
        });
        let nested = RustType::Object(
            RustObject::Struct(Struct::new(vec![StructField::new(
                "value",
                RustType::Object(
                    RustObject::Struct(Struct::new(vec![])),
                    ObjectMetadata::new(RustIdent::create("value")),
                ),
            )])),
            ObjectMetadata::new(RustIdent::create("entries")),
        );
        let object = RustObject::Struct(Struct::new(vec![StructField::new(
            "entries",
            RustType::vec(nested),
        )]));
        let occurrences = ["first", "second"].map(|field| ItemOccurrence {
            origin: ItemOrigin {
                type_root: TypeRoot::Declaration(DeclarationRoot::Component(component)),
                path: vec![field.to_string()],
            },
            usage: ObjectUsage::type_def(),
        });
        let mut builder = CatalogBuilder::default();
        let shared = builder
            .collect_root_object(
                root.clone(),
                &object,
                ItemMetadata::new(RustIdent::create("details")),
                ObjectUsage::type_def(),
            )
            .unwrap();
        builder.set_shared_occurrences(shared, &occurrences);
        builder.item_mut(shared).sharing_rule = Some(SharingRule::CommonTitle);
        let mut catalog = builder.finish().unwrap();
        catalog.lower_named_references().unwrap();
        catalog.validate_lowered_structure().unwrap();

        assert_eq!(catalog.len(), 3);
        assert_eq!(catalog.get(shared).occurrences, occurrences);
        assert_eq!(catalog.get(shared).sharing_rule, Some(SharingRule::CommonTitle));
        let grandchild = catalog.get(catalog.get(catalog.get(shared).children[0]).children[0]);
        assert_eq!(grandchild.origin.type_root, root);
        assert_eq!(grandchild.origin.path, ["entries", "value"]);
        assert_eq!(
            grandchild.occurrences,
            ["first", "second"].map(|field| ItemOccurrence {
                origin: ItemOrigin {
                    type_root: TypeRoot::Declaration(DeclarationRoot::Component(component)),
                    path: vec![field.to_string(), "entries".to_string(), "value".to_string()],
                },
                usage: ObjectUsage::type_def(),
            })
        );
        assert!(grandchild.sharing_rule.is_none());
    }

    #[test]
    fn lowers_named_references_after_all_items_are_allocated() {
        let components = ComponentCatalog::from_names([
            ComponentName::new("charge".to_string()),
            ComponentName::new("customer".to_string()),
        ])
        .unwrap();
        let charge = components.id_str("charge").unwrap();
        let customer = components.id_str("customer").unwrap();
        let charge_object = RustObject::Struct(Struct::new(vec![StructField::new(
            "customer",
            RustType::component(customer),
        )]));
        let mut builder = CatalogBuilder::default();

        let charge_item = builder
            .collect_root_object(
                TypeRoot::Declaration(DeclarationRoot::Component(charge)),
                &charge_object,
                ItemMetadata::new(RustIdent::create("charge")),
                ObjectUsage::type_def(),
            )
            .unwrap();
        let customer_item = builder
            .collect_root_object(
                TypeRoot::Declaration(DeclarationRoot::Component(customer)),
                &RustObject::Struct(Struct::new(vec![])),
                ItemMetadata::new(RustIdent::create("customer")),
                ObjectUsage::type_def(),
            )
            .unwrap();
        let mut catalog = builder.finish().unwrap();

        catalog.lower_named_references().unwrap();

        let Some(object) = catalog.get(charge_item).object() else {
            panic!("expected charge object")
        };
        let RustObject::Struct(charge_struct) = &object.definition else {
            panic!("expected charge struct")
        };
        assert_eq!(charge_struct.fields[0].rust_type, RustType::Item(customer_item));
        assert_eq!(
            catalog
                .root(&TypeRoot::Declaration(DeclarationRoot::Component(charge)))
                .map(|root| &root.typ),
            Some(&RustType::Item(charge_item))
        );
    }

    #[test]
    fn allocates_and_lowers_component_id_items() {
        let components =
            ComponentCatalog::from_names([ComponentName::new("customer".to_string())]).unwrap();
        let customer = components.id_str("customer").unwrap();
        let mut builder = CatalogBuilder::default();

        let customer_item = builder
            .collect_root_object(
                TypeRoot::Declaration(DeclarationRoot::Component(customer)),
                &RustObject::Struct(Struct::new(vec![])),
                ItemMetadata::new(RustIdent::create("customer")),
                ObjectUsage::type_def(),
            )
            .unwrap();
        let customer_id =
            builder.collect_id_newtype(customer, RustIdent::create("customer_id")).unwrap();
        let RustItem::Object(customer_object) = &mut builder.item_mut(customer_item).item else {
            panic!("expected customer object")
        };
        customer_object.identity = Some(ObjectIdentity::IdType(RustType::object_id(customer)));
        let mut catalog = builder.finish().unwrap();

        catalog.lower_named_references().unwrap();
        catalog.validate_lowered_structure().unwrap();
        catalog.analyze_properties().unwrap();

        assert_eq!(
            catalog
                .get(customer_item)
                .object()
                .unwrap()
                .identity
                .as_ref()
                .and_then(ObjectIdentity::id_type),
            Some(&RustType::Item(customer_id))
        );
        assert_eq!(
            catalog
                .root(&TypeRoot::Declaration(DeclarationRoot::ObjectId(customer)))
                .map(|root| &root.typ),
            Some(&RustType::Item(customer_id))
        );
        assert!(matches!(catalog.get(customer_id).item, RustItem::IdNewtype));
        assert!(!catalog.properties(customer_id).is_copy);
        assert!(catalog.properties(customer_id).is_eq);
    }

    #[test]
    fn analyzes_properties_across_item_cycles() {
        let components = ComponentCatalog::from_names([
            ComponentName::new("charge".to_string()),
            ComponentName::new("customer".to_string()),
        ])
        .unwrap();
        let charge = components.id_str("charge").unwrap();
        let customer = components.id_str("customer").unwrap();
        let mut builder = CatalogBuilder::default();

        let charge_item = builder
            .collect_root_object(
                TypeRoot::Declaration(DeclarationRoot::Component(charge)),
                &RustObject::Struct(Struct::new(vec![StructField::new(
                    "customer",
                    RustType::component(customer),
                )])),
                ItemMetadata::new(RustIdent::create("charge")),
                ObjectUsage::type_def(),
            )
            .unwrap();
        let customer_item = builder
            .collect_root_object(
                TypeRoot::Declaration(DeclarationRoot::Component(customer)),
                &RustObject::Struct(Struct::new(vec![StructField::new(
                    "charges",
                    RustType::vec(RustType::component(charge)),
                )])),
                ItemMetadata::new(RustIdent::create("customer")),
                ObjectUsage::type_def(),
            )
            .unwrap();
        let mut catalog = builder.finish().unwrap();

        catalog.lower_named_references().unwrap();
        catalog.validate_lowered_structure().unwrap();
        catalog.analyze_properties().unwrap();

        assert!(!catalog.properties(charge_item).is_copy);
        assert!(!catalog.properties(customer_item).is_copy);
        assert!(catalog.properties(charge_item).is_eq);
        assert!(catalog.properties(customer_item).is_eq);
        assert!(catalog.property_analysis_rounds() >= 3);
    }

    #[test]
    fn rejects_inline_objects_after_lowering() {
        let components =
            ComponentCatalog::from_names([ComponentName::new("charge".to_string())]).unwrap();
        let charge = components.id_str("charge").unwrap();
        let mut builder = CatalogBuilder::default();
        let charge_item = builder
            .collect_root_object(
                TypeRoot::Declaration(DeclarationRoot::Component(charge)),
                &RustObject::Struct(Struct::new(vec![])),
                ItemMetadata::new(RustIdent::create("charge")),
                ObjectUsage::type_def(),
            )
            .unwrap();
        let mut catalog = builder.finish().unwrap();
        let RustItem::Object(object) = &mut catalog.items[charge_item.index()].item else {
            panic!("expected charge object")
        };
        let RustObject::Struct(charge) = &mut object.definition else {
            panic!("expected charge struct")
        };
        charge.fields.push(StructField::new(
            "details",
            RustType::Object(
                RustObject::Struct(Struct::new(vec![])),
                ObjectMetadata::new(RustIdent::create("details")),
            ),
        ));

        let error = catalog.validate_lowered_structure().unwrap_err();
        assert!(error.to_string().contains("inline object remained"));
    }
}
