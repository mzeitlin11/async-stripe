use openapiv3::Parameter;

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
    pub parent: String,
    pub field: String,
    pub options: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredUnion {
    pub field: String,
    pub schema_variants: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredStruct {
    pub field: String,
    pub schema: openapiv3::Schema,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredParams {
    pub method: String,
    pub rust_type: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredObject {
    pub rust_type: String,
    pub schema: openapiv3::Schema,
}

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
pub enum MethodTypes {
    List,
    Create,
    Retrieve,
    Update,
    Delete,
}

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
        Self(id_typ)
    }
}

impl AsRef<str> for IdType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
