use std::collections::HashMap;

use super::{ItemCatalogDraft, ItemId, RustItem};
use crate::rust_object::{ObjectIdentity, RustObject};
use crate::rust_type::{Container, ExternalType, RustType};

impl ItemCatalogDraft {
    /// Erase only expandable storage edges, before properties and requests are
    /// finalized. Ordinary fields, unions, identities, and provenance stay intact.
    pub(crate) fn use_lazy_expandables(&mut self) -> anyhow::Result<()> {
        let ids = self
            .catalog
            .iter()
            .filter_map(|(item, record)| {
                let object = record.object()?;
                // The same metadata drives the generated Object impl. Enum
                // classification alone does not give a return union an identity.
                let id = match object.identity.as_ref()? {
                    ObjectIdentity::IdType(id) => id.clone(),
                    ObjectIdentity::ObjectUnion => RustType::External(ExternalType::SmolStr),
                };
                Some((item, id))
            })
            .collect::<HashMap<_, _>>();

        for record in &mut self.catalog.items {
            if let RustItem::Object(object) = &mut record.item {
                match &mut object.definition {
                    RustObject::Struct(record) => {
                        for field in &mut record.fields {
                            lower(&mut field.rust_type, &ids)?;
                        }
                    }
                    RustObject::Enum(variants) => {
                        for typ in
                            variants.iter_mut().filter_map(|variant| variant.rust_type.as_mut())
                        {
                            lower(typ, &ids)?;
                        }
                    }
                    RustObject::FieldlessEnum(_) => {}
                }
            }
        }
        for root in self.catalog.roots.values_mut() {
            lower(&mut root.typ, &ids)?;
        }
        Ok(())
    }
}

fn lower(typ: &mut RustType, ids: &HashMap<ItemId, RustType>) -> anyhow::Result<()> {
    match typ {
        RustType::Container(Container::Expandable(target)) => {
            let item = target.item_id().ok_or_else(|| {
                anyhow::anyhow!("expandable target has no canonical declaration: {target:?}")
            })?;
            let id = ids.get(&item).ok_or_else(|| {
                anyhow::anyhow!("expandable target has no canonical identity: {item:?}")
            })?;
            *typ = RustType::Container(Container::LazyExpandable(Box::new(id.clone())));
        }
        RustType::Container(container) => lower(container.value_typ_mut(), ids)?,
        RustType::Reference(inner) => lower(inner, ids)?,
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests;
