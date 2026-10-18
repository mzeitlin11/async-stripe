use std::fmt::{Debug, Display, Formatter};

use crate::component_catalog::ComponentId;
use crate::item_catalog::ItemId;
use crate::rust_object::{ObjectMetadata, ObjectUsage, RustObject};
use crate::types::RustIdent;
use crate::visitor::{Visit, VisitMut};

/// A path to a type defined elsewhere.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DeclarationRoot {
    /// A top-level component.
    Component(ComponentId),
    /// The id for a top-level component.
    ObjectId(ComponentId),
    Shared(RustIdent),
    Deduplicated {
        component: ComponentId,
        ident: RustIdent,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustType {
    Object(RustObject, ObjectMetadata),
    /// A Rust primitive or standard-library scalar.
    Primitive(PrimitiveType),
    /// A symbolic type supplied by one of the generated crates' dependencies.
    External(ExternalType),
    /// A declaration stored in the post-dedup Rust item arena.
    Item(ItemId),
    Path(DeclarationRoot),
    /// Type containing an inner type.
    Container(Container),
    /// A borrowed view of another type.
    ///
    /// Keeping the reference explicit avoids encoding borrowing independently
    /// in strings, paths, slices, and maps.
    Reference(Box<RustType>),
}

impl RustType {
    pub const fn bool() -> Self {
        Self::Primitive(PrimitiveType::Bool)
    }

    pub const fn float() -> Self {
        Self::Primitive(PrimitiveType::Float)
    }

    pub const fn string() -> Self {
        Self::Primitive(PrimitiveType::String)
    }

    pub const fn int(typ: IntType) -> Self {
        Self::Primitive(PrimitiveType::Int(typ))
    }

    pub const fn external(external: ExternalType) -> Self {
        Self::External(external)
    }

    pub fn option(typ: Self) -> Self {
        Self::Container(Container::Option(Box::new(typ)))
    }

    pub fn boxed(typ: Self) -> Self {
        Self::Container(Container::Box(Box::new(typ)))
    }

    pub fn list(typ: Self) -> Self {
        Self::Container(Container::List(Box::new(typ)))
    }

    pub fn search_list(typ: Self) -> Self {
        Self::Container(Container::SearchList(Box::new(typ)))
    }

    /// Construct an `Expandable<{typ}>`.
    pub fn expandable(typ: Self) -> Self {
        Self::Container(Container::Expandable(Box::new(typ)))
    }

    /// Construct a `HashMap<String, {typ}>`.
    pub fn str_map(value: Self) -> Self {
        Self::Container(Container::Map { key: MapKey::String, value: Box::new(value) })
    }

    /// Construct a `HashMap<Currency, {typ}>`.
    pub fn currency_map(typ: Self) -> Self {
        Self::Container(Container::Map { key: MapKey::Currency, value: Box::new(typ) })
    }

    /// Construct a `Vec<{typ}>`.
    pub fn vec(typ: Self) -> Self {
        Self::Container(Container::Vec(Box::new(typ)))
    }

    /// Construct a `[{typ}]`. Borrowed slices are wrapped in
    /// `RustType::Reference`.
    pub fn slice(typ: Self) -> Self {
        Self::Container(Container::Slice(Box::new(typ)))
    }

    pub fn path(path: DeclarationRoot) -> Self {
        Self::Path(path)
    }

    pub fn reference(typ: Self) -> Self {
        Self::Reference(Box::new(typ))
    }

    pub fn into_reference(self) -> Self {
        Self::reference(self)
    }

    /// Extract the component path from `{}Id` or `Option<{}Id>`
    pub fn as_id_or_opt_id_path(&self) -> Option<ComponentId> {
        match self {
            Self::Path(DeclarationRoot::ObjectId(component)) => Some(*component),
            Self::Reference(inner) => inner.as_id_or_opt_id_path(),
            Self::Container(Container::Option(typ)) => match &**typ {
                Self::Path(DeclarationRoot::ObjectId(component)) => Some(*component),
                Self::Reference(inner) => inner.as_id_or_opt_id_path(),
                _ => None,
            },
            _ => None,
        }
    }

    /// Is this type `Option<>`?
    pub const fn is_option(&self) -> bool {
        matches!(self, Self::Container(Container::Option(_)))
    }

    pub fn with_option_stripped(&self) -> &Self {
        match self {
            RustType::Container(Container::Option(inner)) => inner,
            _ => self,
        }
    }

    pub fn component(component: ComponentId) -> Self {
        Self::path(DeclarationRoot::Component(component))
    }

    pub fn json_value() -> Self {
        Self::External(ExternalType::Value)
    }

    pub fn object_id(component: ComponentId) -> Self {
        Self::path(DeclarationRoot::ObjectId(component))
    }

    pub fn into_nullable(self) -> Self {
        match self {
            Self::Container(Container::Option(_)) => self,
            _ => Self::option(self),
        }
    }

    /// Find any `RustObject` that may be contained in this type
    pub fn extract_object(&self) -> Option<(&RustObject, &ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            Self::Primitive(_) | Self::External(_) | Self::Item(_) | Self::Path(_) => None,
            Self::Container(typ) => typ.value_typ().extract_object(),
            Self::Reference(inner) => inner.extract_object(),
        }
    }

    pub fn as_object(&self) -> Option<(&RustObject, &ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<(&mut RustObject, &mut ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            _ => None,
        }
    }

    pub fn into_object(self) -> Option<(RustObject, ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            _ => None,
        }
    }

    pub fn as_rust_object(&self) -> Option<&RustObject> {
        self.as_object().map(|r| r.0)
    }

    pub fn visit<'a, T: Visit<'a>>(&'a self, visitor: &mut T, usage: ObjectUsage) {
        use RustType::*;
        match self {
            Object(obj, meta) => {
                visitor.visit_obj(obj, Some(meta), usage);
            }
            Primitive(_) | External(_) | Item(_) | Path(_) => {}
            Container(inner) => visitor.visit_typ(inner.value_typ(), usage),
            Reference(inner) => visitor.visit_typ(inner, usage),
        }
    }

    pub fn visit_mut<T: VisitMut>(&mut self, visitor: &mut T, usage: ObjectUsage) {
        use RustType::*;
        match self {
            Object(obj, meta) => {
                visitor.visit_obj_mut(obj, Some(meta), usage);
            }
            Primitive(_) | External(_) | Item(_) | Path(_) => {}
            Container(inner) => visitor.visit_typ_mut(inner.value_typ_mut(), usage),
            Reference(inner) => visitor.visit_typ_mut(inner, usage),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MapKey {
    String,
    Currency,
}

/// Representation of a type containing an inner type.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Container {
    /// List<{typ}>
    List(Box<RustType>),
    /// SearchList<{typ}>
    SearchList(Box<RustType>),
    /// Vec<{typ}>
    Vec(Box<RustType>),
    /// &[{typ}]
    Slice(Box<RustType>),
    /// Expandable<{typ}>
    Expandable(Box<RustType>),
    /// Option<{typ}>
    Option(Box<RustType>),
    /// Box<{typ}>
    Box(Box<RustType>),
    /// HashMap<{key}, {typ}>
    Map {
        /// HashMap key type.
        key: MapKey,
        /// HashMap value type.
        value: Box<RustType>,
    },
}

impl Container {
    /// Get a reference to the inner contained type.
    pub fn value_typ(&self) -> &RustType {
        use Container::*;
        match self {
            List(typ) => typ,
            SearchList(typ) => typ,
            Vec(typ) => typ,
            Slice(typ) => typ,
            Expandable(typ) => typ,
            Option(typ) => typ,
            Box(typ) => typ,
            Map { value, .. } => value,
        }
    }

    /// Get a mutable reference to the inner contained type.
    pub fn value_typ_mut(&mut self) -> &mut RustType {
        use Container::*;
        match self {
            List(typ) => typ,
            SearchList(typ) => typ,
            Vec(typ) => typ,
            Slice(typ) => typ,
            Expandable(typ) => typ,
            Option(typ) => typ,
            Box(typ) => typ,
            Map { value, .. } => value,
        }
    }
}

/// A Rust primitive or standard-library scalar.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PrimitiveType {
    /// bool
    Bool,
    /// f64
    Float,
    /// `str`; borrowed strings are represented by `RustType::Reference`.
    Str,
    /// String
    String,
    /// One of primitive rust integer types.
    Int(IntType),
}

impl PrimitiveType {
    /// Does this type implement `Copy`?
    pub const fn is_copy(self) -> bool {
        !matches!(self, Self::String)
    }

    /// Does this type implement `Eq`?
    pub const fn is_eq(self) -> bool {
        // `f64` and `Value` (which contains f64) do not implement `Eq`
        !matches!(self, Self::Float)
    }

    pub const fn ident(self) -> &'static str {
        use PrimitiveType::*;
        match self {
            Bool => "bool",
            Float => "f64",
            String => "String",
            Str => "str",
            Int(typ) => typ.as_str(),
        }
    }

    /// Identifier used when this type is part of an enum.
    pub const fn display_name(self) -> &'static str {
        self.ident()
    }
}

impl Debug for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.ident())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum IntType {
    U8,
    U32,
    U64,
    I64,
}

impl IntType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::I64 => "i64",
        }
    }
}

impl Display for IntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A symbolic type defined outside of codegen.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ExternalType {
    Currency,
    RangeQueryTs,
    Timestamp,
    AlwaysTrue,
    /// Arbitrary JSON value
    Value,
}

impl ExternalType {
    /// Identifier used when this type is part of an enum. Its source path is
    /// deliberately owned by the rendering layer.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Currency => "Currency",
            Self::RangeQueryTs => "RangeQueryTs",
            Self::Timestamp => "Timestamp",
            Self::AlwaysTrue => "AlwaysTrue",
            Self::Value => "Value",
        }
    }

    pub const fn is_copy(self) -> bool {
        !matches!(self, Self::Value | Self::Currency)
    }

    pub const fn is_eq(self) -> bool {
        !matches!(self, Self::Value)
    }
}
