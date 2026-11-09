use crate::item_catalog::{ItemProperties, ItemRecord, RustItem};
use crate::item_origin::TypeRoot;
use crate::rust_object::RustObject;
use crate::rust_type::{Container, DeclarationRoot, RustType};

/// Compute graph-derived declaration properties to a fixed point.
///
/// Lifetime propagation is monotone from `false` to `true`; `Copy` and `Eq`
/// propagation are monotone from `true` to `false`. Starting at those bounds
/// gives cycles deterministic semantics without recursive model queries.
pub(crate) fn infer(items: &[ItemRecord]) -> anyhow::Result<(Vec<ItemProperties>, usize)> {
    let mut properties =
        vec![ItemProperties { has_lifetime: false, is_copy: true, is_eq: true }; items.len()];

    for round in 1..=items.len() + 1 {
        let next = items
            .iter()
            .map(|item| match &item.item {
                RustItem::Object(object) => {
                    object_properties(&object.definition, items, &properties)
                }
                RustItem::IdNewtype => {
                    Ok(ItemProperties { has_lifetime: false, is_copy: false, is_eq: true })
                }
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        if next == properties {
            return Ok((next, round));
        }
        properties = next;
    }

    anyhow::bail!("canonical item property analysis did not converge")
}

fn object_properties(
    object: &RustObject,
    items: &[ItemRecord],
    properties: &[ItemProperties],
) -> anyhow::Result<ItemProperties> {
    let mut combined = ItemProperties { has_lifetime: false, is_copy: true, is_eq: true };
    match object {
        RustObject::Struct(struct_) => {
            for field in &struct_.fields {
                combine(&mut combined, resolve(items, properties, &field.rust_type)?);
            }
        }
        RustObject::Enum(variants) => {
            for variant in variants {
                if let Some(typ) = &variant.rust_type {
                    combine(&mut combined, resolve(items, properties, typ)?);
                }
            }
        }
        RustObject::FieldlessEnum(_) => combined.is_copy = false,
    }
    Ok(combined)
}

fn combine(combined: &mut ItemProperties, next: ItemProperties) {
    combined.has_lifetime |= next.has_lifetime;
    combined.is_copy &= next.is_copy;
    combined.is_eq &= next.is_eq;
}

pub(crate) fn resolve(
    items: &[ItemRecord],
    properties: &[ItemProperties],
    typ: &RustType,
) -> anyhow::Result<ItemProperties> {
    let scalar = |is_copy, is_eq| ItemProperties { has_lifetime: false, is_copy, is_eq };
    match typ {
        RustType::Object(object, _) => object_properties(object, items, properties),
        RustType::Primitive(primitive) => Ok(scalar(primitive.is_copy(), primitive.is_eq())),
        RustType::External(external) => Ok(scalar(external.is_copy(), external.is_eq())),
        RustType::Item(item) => {
            let target = items.get(item.index()).ok_or_else(|| {
                anyhow::anyhow!("canonical type references missing item {item:?}")
            })?;
            let mut result = properties
                .get(item.index())
                .copied()
                .ok_or_else(|| anyhow::anyhow!("canonical properties missing for item {item:?}"))?;
            // Preserve the existing API rule that top-level components do not
            // carry lifetimes through references to other components.
            if target.origin.path.is_empty()
                && matches!(
                    target.origin.type_root,
                    TypeRoot::Declaration(DeclarationRoot::Component(_))
                )
            {
                result.has_lifetime = false;
            }
            Ok(result)
        }
        RustType::Path(path) => {
            anyhow::bail!("named path remained in canonical item model: {path:?}")
        }
        RustType::Reference(inner) => {
            let mut result = resolve(items, properties, inner)?;
            result.has_lifetime = true;
            result.is_copy = true;
            if let RustType::Container(Container::Map { value, .. }) = &**inner {
                // Both supported map key types implement Eq. Borrowing the
                // map does not remove the value type's equality requirements.
                result.is_eq = resolve(items, properties, value)?.is_eq;
            }
            Ok(result)
        }
        RustType::Container(container) => {
            let mut result = resolve(items, properties, container.value_typ())?;
            match container {
                Container::List(_) | Container::SearchList(_) => {
                    result.is_copy = false;
                    result.is_eq = false;
                }
                Container::Vec(_) | Container::Box(_) => result.is_copy = false,
                Container::Slice(_) => result.is_copy = true,
                Container::Expandable(_) | Container::LazyExpandable(_) => {
                    result.is_copy = false;
                    result.is_eq = false;
                }
                Container::Option(_) => {}
                Container::Map { .. } => {
                    result.is_copy = false;
                    result.is_eq = false;
                }
            }
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::resolve;
    use crate::rust_type::RustType;

    #[test]
    fn boxes_do_not_inherit_copy_from_their_payloads() {
        for typ in
            [RustType::boxed(RustType::bool()), RustType::option(RustType::boxed(RustType::bool()))]
        {
            let properties = resolve(&[], &[], &typ).unwrap();
            assert!(!properties.is_copy);
            assert!(properties.is_eq);
        }
    }

    #[test]
    fn borrowed_maps_preserve_their_values_equality_requirements() {
        for (value, expected_eq) in [(RustType::bool(), true), (RustType::float(), false)] {
            let typ = RustType::reference(RustType::str_map(value));
            let properties = resolve(&[], &[], &typ).unwrap();
            assert_eq!(properties.is_eq, expected_eq);
            assert!(properties.is_copy);
            assert!(properties.has_lifetime);
        }
        // Keep the existing conservative policy for owned maps.
        let owned = RustType::str_map(RustType::bool());
        assert!(!resolve(&[], &[], &owned).unwrap().is_eq);
    }
}
