use crate::operations::OperationId;
use crate::rust_object::ObjectUsage;
use crate::rust_type::DeclarationRoot;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypeRoot {
    Declaration(DeclarationRoot),
    OperationPathParam { operation: OperationId, index: usize },
    OperationParam(OperationId),
    OperationReturn(OperationId),
    Webhook(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ItemOrigin {
    pub type_root: TypeRoot,
    pub path: Vec<String>,
}

/// An inferred object occurrence contributing to a generated declaration.
/// Several occurrences can share one declaration while retaining their own
/// locations and usage requirements.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ItemOccurrence {
    pub origin: ItemOrigin,
    pub usage: ObjectUsage,
}
