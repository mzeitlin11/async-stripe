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
                RustItem::Object(object) => object_properties(&object.definition, items, &|id| {
                    properties.get(id.index()).copied()
                }),
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

pub(crate) fn resolve(
    items: &[ItemRecord],
    properties: &[ItemProperties],
    typ: &RustType,
) -> anyhow::Result<ItemProperties> {
    type_properties(typ, items, &|id| properties.get(id.index()).copied())
}

fn object_properties<F>(
    object: &RustObject,
    items: &[ItemRecord],
    item_property: &F,
) -> anyhow::Result<ItemProperties>
where
    F: Fn(crate::item_catalog::ItemId) -> Option<ItemProperties>,
{
    let mut combined = ItemProperties { has_lifetime: false, is_copy: true, is_eq: true };
    match object {
        RustObject::Struct(struct_) => {
            for field in &struct_.fields {
                combine(&mut combined, type_properties(&field.rust_type, items, item_property)?);
            }
        }
        RustObject::Enum(variants) => {
            for variant in variants {
                if let Some(typ) = &variant.rust_type {
                    combine(&mut combined, type_properties(typ, items, item_property)?);
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

fn type_properties<F>(
    typ: &RustType,
    items: &[ItemRecord],
    item_property: &F,
) -> anyhow::Result<ItemProperties>
where
    F: Fn(crate::item_catalog::ItemId) -> Option<ItemProperties>,
{
    let scalar = |is_copy, is_eq| ItemProperties { has_lifetime: false, is_copy, is_eq };
    match typ {
        RustType::Object(object, _) => object_properties(object, items, item_property),
        RustType::Primitive(primitive) => Ok(scalar(primitive.is_copy(), primitive.is_eq())),
        RustType::External(external) => Ok(scalar(external.is_copy(), external.is_eq())),
        RustType::Item(item) => {
            let target = items.get(item.index()).ok_or_else(|| {
                anyhow::anyhow!("canonical type references missing item {item:?}")
            })?;
            let mut result = item_property(*item)
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
            let mut result = type_properties(inner, items, item_property)?;
            result.has_lifetime = true;
            result.is_copy = true;
            if matches!(&**inner, RustType::Container(Container::Map { .. })) {
                result.is_eq = true;
            }
            Ok(result)
        }
        RustType::Container(container) => {
            let mut result = type_properties(container.value_typ(), items, item_property)?;
            match container {
                Container::List(_) | Container::SearchList(_) => {
                    result.is_copy = false;
                    result.is_eq = false;
                }
                Container::Vec(_) => result.is_copy = false,
                Container::Slice(_) => result.is_copy = true,
                Container::Expandable(_) => {
                    result.is_copy = false;
                    result.is_eq = false;
                }
                Container::Option(_) | Container::Box(_) => {}
                Container::Map { .. } => {
                    result.is_copy = false;
                    result.is_eq = false;
                }
            }
            Ok(result)
        }
    }
}
