use std::collections::HashMap;

use anyhow::{Context, bail, ensure};

use crate::types::{ComponentName, ComponentSchemaRef};

/// Stable identity for a top-level component schema within one codegen run.
///
/// IDs are allocated in the OpenAPI document's component order.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ComponentId(u32);

impl ComponentId {
    fn from_index(index: usize) -> anyhow::Result<Self> {
        Ok(Self(index.try_into().context("impossible number of component schemas")?))
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Bidirectional mapping between component identities and their spec names.
///
/// The catalog describes all non-webhook component schemas inferred from the
/// spec. Later pipeline stages may decide not to retain or emit a component,
/// but that does not change the component's identity.
#[derive(Debug, Clone)]
pub struct ComponentCatalog {
    names: Vec<ComponentName>,
    by_name: HashMap<ComponentName, ComponentId>,
}

impl ComponentCatalog {
    pub fn from_names(names: impl IntoIterator<Item = ComponentName>) -> anyhow::Result<Self> {
        let mut catalog = Self { names: Vec::new(), by_name: HashMap::new() };
        for name in names {
            let id = ComponentId::from_index(catalog.names.len())?;
            if catalog.by_name.insert(name.clone(), id).is_some() {
                bail!("duplicate component schema name: {name}");
            }
            catalog.names.push(name);
        }
        Ok(catalog)
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn id(&self, name: &ComponentName) -> Option<ComponentId> {
        self.by_name.get(name).copied()
    }

    pub fn id_str(&self, name: &str) -> Option<ComponentId> {
        self.by_name.get(name).copied()
    }

    pub fn resolve_reference(&self, reference: &str) -> anyhow::Result<ComponentId> {
        let name = ComponentSchemaRef::parse(reference)?.name();
        self.id_str(name).ok_or_else(|| {
            anyhow::anyhow!("component schema reference was not cataloged: {reference}")
        })
    }

    #[track_caller]
    pub fn name(&self, id: ComponentId) -> &ComponentName {
        &self.names[id.index()]
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = (ComponentId, &ComponentName)> {
        self.names.iter().enumerate().map(|(index, name)| {
            let id = ComponentId::from_index(index).expect("catalog already validated its length");
            (id, name)
        })
    }

    // TODO: inline at usage?
    pub fn validate_retained(
        &self,
        retained: impl IntoIterator<Item = ComponentId>,
    ) -> anyhow::Result<()> {
        for id in retained {
            let name = self.names.get(id.index()).ok_or_else(|| {
                anyhow::anyhow!("retained component ID was outside the catalog: {id:?}")
            })?;
            ensure!(self.id(name) == Some(id), "component catalog did not round trip {name}");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentCatalog;
    use crate::types::ComponentName;

    fn name(value: &str) -> ComponentName {
        ComponentName::new(value.to_string())
    }

    #[test]
    fn preserves_order_and_round_trips_ids() {
        let catalog = ComponentCatalog::from_names([name("charge"), name("invoice")]).unwrap();
        let entries = catalog.iter().collect::<Vec<_>>();

        assert_eq!(catalog.len(), 2);
        assert_eq!(entries[0].1.as_ref(), "charge");
        assert_eq!(entries[1].1.as_ref(), "invoice");
        assert_eq!(catalog.id(&name("invoice")), Some(entries[1].0));
        assert_eq!(catalog.name(entries[0].0).as_ref(), "charge");
    }

    #[test]
    fn rejects_duplicate_names() {
        let result = ComponentCatalog::from_names([name("charge"), name("charge")]);
        assert!(result.is_err());
    }

    #[test]
    fn resolves_component_schema_references() {
        let catalog = ComponentCatalog::from_names([name("charge")]).unwrap();
        let id = catalog.resolve_reference("#/components/schemas/charge").unwrap();

        assert_eq!(catalog.name(id).as_ref(), "charge");
        assert!(catalog.resolve_reference("#/components/schemas/invoice").is_err());
    }
}
