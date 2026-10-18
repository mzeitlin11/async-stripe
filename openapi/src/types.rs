use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

use heck::ToUpperCamelCase;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RustIdent(String);

impl RustIdent {
    /// Only to be used safely when `val` is known to be a valid
    /// and we don't want to convert to camelcase since it strips
    /// underscores (e.g. `ApiVersion`).
    pub fn unchanged<T: ToString>(val: T) -> Self {
        Self(val.to_string())
    }

    pub fn create<T: AsRef<str>>(val: T) -> Self {
        let val = val.as_ref();
        if val.contains('.') {
            Self(val.replace('.', "_").to_upper_camel_case())
        } else {
            Self(val.to_upper_camel_case())
        }
    }

    pub fn joined<T: Display, U: Display>(piece1: T, piece2: U) -> Self {
        Self::create(format!("{piece1}_{piece2}"))
    }
}

impl Deref for RustIdent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for RustIdent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for RustIdent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ComponentName(String);

impl ComponentName {
    pub const fn new(path: String) -> Self {
        Self(path)
    }

    pub fn as_not_deleted(&self) -> Self {
        Self::new(self.0.trim_start_matches("deleted_").to_string())
    }

    pub fn is_deleted_obj(&self) -> bool {
        self.0.starts_with("deleted_")
    }
}

/// A validated JSON Reference to a top-level OpenAPI component schema.
#[derive(Debug, Copy, Clone)]
pub struct ComponentSchemaRef<'a>(&'a str);

impl<'a> ComponentSchemaRef<'a> {
    const PREFIX: &'static str = "#/components/schemas/";

    pub fn parse(reference: &'a str) -> anyhow::Result<Self> {
        let name = reference.strip_prefix(Self::PREFIX).ok_or_else(|| {
            anyhow::anyhow!("expected component schema reference, got {reference}")
        })?;
        anyhow::ensure!(!name.is_empty(), "component schema reference had an empty name");
        anyhow::ensure!(
            !name.contains('/'),
            "component schema reference had nested path: {reference}"
        );
        Ok(Self(name))
    }

    pub const fn name(self) -> &'a str {
        self.0
    }
}

// Petgraph uses `Debug` for DOT node labels. Print only the OpenAPI component name so generated
// graphs contain `charge` rather than the newtype's structural debug representation.
impl Debug for ComponentName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for ComponentName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ComponentName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ComponentName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<str> for ComponentName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentSchemaRef;

    #[test]
    fn parses_component_schema_reference() {
        let parsed = ComponentSchemaRef::parse("#/components/schemas/charge").unwrap();
        assert_eq!(parsed.name(), "charge");
    }

    #[test]
    fn rejects_non_component_reference() {
        assert!(ComponentSchemaRef::parse("#/paths/charges").is_err());
        assert!(ComponentSchemaRef::parse("#/components/schemas/").is_err());
        assert!(ComponentSchemaRef::parse("charge").is_err());
    }
}
