use std::fmt::{Display, Formatter};

use openapiv3::Parameter;
use {derive_more::AsRef, derive_more::Display};

#[derive(Clone, Copy, Debug)]
pub enum CopyOrClone {
    Copy,
    Clone,
}

#[derive(Clone, Debug, Copy)]
pub enum TypeError {
    IsObject,
    NoType,
    Unhandled,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredEnum {
    pub parent: RustObjectTypeName,
    pub field: FieldName,
    pub options: Vec<EnumVariantName>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredUnion {
    pub field: FieldName,
    pub schema_variants: Vec<SchemaName>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredStruct {
    pub field: FieldName,
    pub schema: openapiv3::Schema,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredParams {
    pub method: MethodTypes,
    pub rust_type: RustObjectTypeName,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredObject {
    pub rust_type: String,
    pub schema: openapiv3::Schema,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
pub enum MethodTypes {
    List,
    Create,
    Retrieve,
    Update,
    Delete,
}

impl MethodTypes {
    pub fn as_method_name(&self) -> &'static str {
        match self {
            MethodTypes::List => "list",
            MethodTypes::Create => "create",
            MethodTypes::Retrieve => "retrieve",
            MethodTypes::Update => "update",
            MethodTypes::Delete => "delete",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, AsRef, Display, Ord, PartialOrd)]
pub struct SchemaName(String);

impl SchemaName {
    // TODO: assert snakecase
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, AsRef, Display, Ord, PartialOrd)]
pub struct RustObjectTypeName(String);

impl RustObjectTypeName {
    pub fn new<T: ToString>(name: T) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, AsRef, Display)]
pub struct FieldName(String);

impl FieldName {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

/// Items that can be imported from crate::params
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum UseParams {
    IdOrCreate,
    Metadata,
    Expand,
    Timestamp,
    Object,
    Deleted,
    List,
    RangeQuery,
    Expandable,
    Paginable,
}

impl UseParams {
    pub fn as_str(&self) -> &'static str {
        match self {
            UseParams::IdOrCreate => "IdOrCreate",
            UseParams::Metadata => "Metadata",
            UseParams::Expand => "Expand",
            UseParams::Timestamp => "Timestamp",
            UseParams::Object => "Object",
            UseParams::Deleted => "Deleted",
            UseParams::List => "List",
            UseParams::RangeQuery => "RangeQuery",
            UseParams::Expandable => "Expandable",
            UseParams::Paginable => "Paginable",
        }
    }
}

/// Items that can be imported from crate::client
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum UseConfig {
    Client,
    Response,
}

impl UseConfig {
    pub fn as_str(&self) -> &'static str {
        match self {
            UseConfig::Client => "Client",
            UseConfig::Response => "Response",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct IdType(String);

impl IdType {
    pub fn new(id_typ: String) -> Self {
        // Sanity check to ensure we've not mistakenly using a non-id value here
        debug_assert!(id_typ.ends_with("Id"), "Expected string with Id, found {}", id_typ);
        Self(id_typ)
    }
}

impl AsRef<str> for IdType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Copy, Clone)]
pub enum FeatureGroups {
    Checkout,
    Billing,
    Connect,
    Fraud,
    Issuing,
    Orders,
    Sigma,
    WebhookEndpoints,
}

impl FeatureGroups {
    pub fn as_str(&self) -> &'static str {
        match self {
            FeatureGroups::Checkout => "checkout",
            FeatureGroups::Billing => "billing",
            FeatureGroups::Connect => "connect",
            FeatureGroups::Fraud => "fraud",
            FeatureGroups::Issuing => "issuing",
            FeatureGroups::Orders => "orders",
            FeatureGroups::Sigma => "sigma",
            FeatureGroups::WebhookEndpoints => "webhook-endpoints",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EnumVariantName {
    All,
    ObjectName(RustObjectTypeName),
}

impl Display for EnumVariantName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnumVariantName::All => write!(f, "All"),
            EnumVariantName::ObjectName(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UseResources {
    Object(RustObjectTypeName),
    Currency,
    Timestamp,
    CreateProduct,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntTypes {
    U8,
    U32,
    U64,
    I32,
    I64,
}

impl Display for IntTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let as_static = match self {
            IntTypes::U8 => "u8",
            IntTypes::U32 => "u32",
            IntTypes::U64 => "u64",
            IntTypes::I32 => "i32",
            IntTypes::I64 => "i64",
        };
        write!(f, "{as_static}")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RustType {
    Object(RustObjectTypeName),
    Bool,
    Float,
    Int(IntTypes),
    String,
    Str,
    Metadata,
    Currency,
    Timestamp,
    JSONValue,
    Scheduled,
    APIVersion,
    RangeQueryTs,
    IdOrCreate,
    Address,
    UpTo,
    Boxed(Box<RustType>),
    Expandable(Box<RustType>),
    Option(Box<RustType>),
    List(Box<RustType>),
    Vec(Box<RustType>),
}

impl RustType {
    pub fn option(typ: RustType) -> Self {
        Self::Option(Box::new(typ))
    }

    pub fn list(typ: RustType) -> Self {
        Self::List(Box::new(typ))
    }

    pub fn vec(typ: RustType) -> Self {
        Self::Vec(Box::new(typ))
    }

    pub fn expandable(typ: RustType) -> Self {
        Self::Expandable(Box::new(typ))
    }

    pub fn boxed(typ: RustType) -> Self {
        Self::Boxed(Box::new(typ))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }
}

impl Display for RustType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustType::Object(name) => write!(f, "{name}"),
            RustType::Bool => write!(f, "bool"),
            RustType::Float => write!(f, "f64"),
            RustType::Int(int_typ) => write!(f, "{int_typ}"),
            RustType::String => write!(f, "String"),
            RustType::Str => write!(f, "&'a str"),
            RustType::Metadata => write!(f, "Metadata"),
            RustType::Currency => write!(f, "Currency"),
            RustType::Timestamp => write!(f, "Timestamp"),
            RustType::JSONValue => write!(f, "serde_json::Value"),
            RustType::Scheduled => write!(f, "Scheduled"),
            RustType::APIVersion => write!(f, "ApiVersion"),
            RustType::RangeQueryTs => write!(f, "RangeQuery<Timestamp>"),
            RustType::IdOrCreate => write!(f, "IdOrCreate<'a, CreateProduct<'a>>"),
            RustType::Address => write!(f, "Address"),
            RustType::UpTo => write!(f, "UpTo"),
            RustType::Boxed(typ) => write!(f, "Box<{typ}>"),
            RustType::Expandable(typ) => write!(f, "Expandable<{typ}>"),
            RustType::Option(typ) => write!(f, "Option<{typ}>"),
            RustType::List(typ) => write!(f, "List<{typ}>"),
            RustType::Vec(typ) => write!(f, "Vec<{typ}>"),
        }
    }
}
