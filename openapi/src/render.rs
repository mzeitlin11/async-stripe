use std::fmt::{Display, Formatter, Write as _};

use crate::STRIPE_TYPES;
use crate::component_catalog::ComponentId;
use crate::crates::CrateConfig;
use crate::item_catalog::{CanonicalTypeRoot, ItemCatalog, ItemId, ItemProperties, ItemRecord};
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::rust_object::RustObject;
use crate::rust_type::{Container, ExternalType, MapKey, PrimitiveType, RustType};
use crate::types::RustIdent;

/// For now, generated types use at most one lifetime, always named `'a`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Lifetime;

impl Lifetime {
    pub fn as_str(self) -> &'static str {
        "'a"
    }

    pub fn as_param(self) -> &'static str {
        "<'a>"
    }
}

impl Display for Lifetime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct TypeUse {
    lifetime: Option<Lifetime>,
    impl_into: bool,
}

/// A context-bound display adapter for rendering one semantic `RustType`.
///
/// This borrows the original type instead of lowering it into a duplicate
/// printable type tree. Crate paths and referenced item properties are resolved
/// through `RenderContext` only when the type is formatted.
#[derive(Copy, Clone)]
pub struct DisplayType<'model, 'type_> {
    context: RenderContext<'model>,
    typ: &'type_ RustType,
    use_site: TypeUse,
}

impl DisplayType<'_, '_> {
    pub fn with_lifetime(mut self, lifetime: Option<Lifetime>) -> Self {
        self.use_site.lifetime = lifetime;
        self
    }

    pub fn constructor_parameter(mut self, lifetime: Option<Lifetime>) -> Self {
        self.use_site = TypeUse {
            lifetime,
            impl_into: lifetime.is_none()
                || !matches!(
                    self.typ,
                    RustType::Reference(_)
                        | RustType::Container(Container::Slice(_) | Container::Expandable(_))
                ),
        };
        self
    }
}

impl Display for DisplayType<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_type(f, self.context, self.typ, self.use_site)
    }
}

#[derive(Copy, Clone)]
pub struct RenderContext<'a> {
    item_layout: &'a ItemLayout,
    items: &'a ItemCatalog,
    emission_site: EmissionSite,
    pub(crate) config: &'a CrateConfig,
}

impl<'model> RenderContext<'model> {
    pub const fn new(
        item_layout: &'model ItemLayout,
        items: &'model ItemCatalog,
        emission_site: EmissionSite,
        config: &'model CrateConfig,
    ) -> Self {
        Self { item_layout, items, emission_site, config }
    }

    pub(crate) fn item_address(self, item: ItemId) -> ItemAddress {
        self.item_layout.placement(item).address()
    }

    pub fn display_type<'type_>(self, typ: &'type_ RustType) -> DisplayType<'model, 'type_> {
        DisplayType { context: self, typ, use_site: TypeUse::default() }
    }

    pub(crate) fn component_item(self, component: ComponentId) -> ItemId {
        self.items.component_root(component)
    }

    pub(crate) fn component_ident(self, component: ComponentId) -> &'model RustIdent {
        &self.items.get(self.items.component_root(component)).metadata.ident
    }

    pub(crate) fn item(self, item: ItemId) -> &'model ItemRecord {
        self.items.get(item)
    }

    pub(crate) fn item_properties(self, item: ItemId) -> ItemProperties {
        self.items.properties(item)
    }

    pub(crate) fn assert_emits(self, item: ItemId) {
        let actual = self.item_layout.placement(item).emission_site();
        assert_eq!(
            actual, self.emission_site,
            "attempted to emit {item:?} in the wrong source location"
        );
    }

    pub(crate) fn root(self, root: &TypeRoot) -> Option<&'model CanonicalTypeRoot> {
        self.items.root(root)
    }

    pub(crate) fn item_object_for_type(self, typ: &RustType) -> Option<&'model RustObject> {
        self.items.object_for_type(typ)
    }

    pub(crate) fn type_has_lifetime(self, typ: &RustType) -> bool {
        self.items
            .properties_for_type(typ)
            .unwrap_or_else(|error| panic!("could not resolve canonical type properties: {error}"))
            .has_lifetime
    }

    pub(crate) fn type_is_copy(self, typ: &RustType) -> bool {
        self.items
            .properties_for_type(typ)
            .unwrap_or_else(|error| panic!("could not resolve canonical type properties: {error}"))
            .is_copy
    }
}

fn fmt_type(
    f: &mut Formatter<'_>,
    context: RenderContext<'_>,
    typ: &RustType,
    use_site: TypeUse,
) -> std::fmt::Result {
    if use_site.impl_into {
        f.write_str("impl Into<")?;
        fmt_type(f, context, typ, TypeUse { lifetime: use_site.lifetime, impl_into: false })?;
        return f.write_char('>');
    }

    match typ {
        RustType::Object(_, _) => panic!("inline object reached canonical type rendering"),
        RustType::Primitive(primitive) => f.write_str(primitive.ident()),
        RustType::External(external) => fmt_external(f, *external),
        RustType::Item(item) => fmt_item(f, context, *item, use_site.lifetime),
        RustType::Path(path) => panic!("unlowered named path reached type rendering: {path:?}"),
        RustType::Container(container) => fmt_container(f, context, container, use_site),
        RustType::Reference(inner) => fmt_reference(f, context, inner, use_site),
    }
}

fn fmt_item(
    f: &mut Formatter<'_>,
    context: RenderContext<'_>,
    item: ItemId,
    lifetime: Option<Lifetime>,
) -> std::fmt::Result {
    match context.item_address(item) {
        ItemAddress::Local => context.assert_emits(item),
        ItemAddress::CrateRoot(krate) => {
            write!(f, "{}::", krate.lib_name(context.config))?;
        }
    }
    write!(f, "{}", context.item(item).metadata.ident)?;
    if let Some(lifetime) = lifetime
        && context.item_properties(item).has_lifetime
    {
        write!(f, "<{lifetime}>")?;
    }
    Ok(())
}

fn fmt_external(f: &mut Formatter<'_>, typ: ExternalType) -> std::fmt::Result {
    let (module, ident) = match typ {
        ExternalType::Currency => (STRIPE_TYPES, "Currency"),
        ExternalType::RangeQueryTs => (STRIPE_TYPES, "RangeQueryTs"),
        ExternalType::Timestamp => (STRIPE_TYPES, "Timestamp"),
        ExternalType::AlwaysTrue => (STRIPE_TYPES, "AlwaysTrue"),
        ExternalType::Value => ("miniserde::json", "Value"),
    };
    write!(f, "{module}::{ident}")
}

fn fmt_reference(
    f: &mut Formatter<'_>,
    context: RenderContext<'_>,
    inner: &RustType,
    use_site: TypeUse,
) -> std::fmt::Result {
    if let Some(lifetime) = use_site.lifetime {
        write!(f, "&{lifetime} ")?;
    } else {
        f.write_char('&')?;
    }
    fmt_type(f, context, inner, TypeUse { lifetime: use_site.lifetime, impl_into: false })
}

fn fmt_container(
    f: &mut Formatter<'_>,
    context: RenderContext<'_>,
    container: &Container,
    use_site: TypeUse,
) -> std::fmt::Result {
    let nested_use = TypeUse { lifetime: use_site.lifetime, impl_into: false };
    match container {
        Container::List(inner) => {
            write!(f, "{STRIPE_TYPES}::List<")?;
            fmt_type(f, context, inner, nested_use)?;
            f.write_char('>')?;
        }
        Container::SearchList(inner) => {
            write!(f, "{STRIPE_TYPES}::SearchList<")?;
            fmt_type(f, context, inner, nested_use)?;
            f.write_char('>')?;
        }
        Container::Vec(inner) => {
            f.write_str("Vec<")?;
            fmt_type(f, context, inner, nested_use)?;
            f.write_char('>')?;
        }
        Container::Slice(inner) => {
            f.write_char('[')?;
            fmt_type(f, context, inner, nested_use)?;
            f.write_char(']')?;
        }
        Container::Expandable(inner) => {
            write!(f, "{STRIPE_TYPES}::Expandable<")?;
            fmt_type(f, context, inner, nested_use)?;
            f.write_char('>')?;
        }
        Container::Option(inner) => {
            f.write_str("Option<")?;
            fmt_type(f, context, inner, nested_use)?;
            f.write_char('>')?;
        }
        Container::Box(inner) => {
            f.write_str("Box<")?;
            fmt_type(f, context, inner, nested_use)?;
            f.write_char('>')?;
        }
        Container::Map { key, value } => {
            f.write_str("std::collections::HashMap<")?;
            fmt_map_key(f, *key)?;
            f.write_str(", ")?;
            fmt_type(f, context, value, nested_use)?;
            f.write_char('>')?;
        }
    }

    Ok(())
}

fn fmt_map_key(f: &mut Formatter<'_>, key: MapKey) -> std::fmt::Result {
    match key {
        MapKey::String => f.write_str(PrimitiveType::String.ident()),
        MapKey::Currency => fmt_external(f, ExternalType::Currency),
    }
}

#[cfg(test)]
mod tests {
    use super::{Lifetime, RenderContext};
    use crate::item_catalog::ItemCatalog;
    use crate::item_layout::{EmissionSite, ItemLayout};
    use crate::rust_type::{PrimitiveType, RustType};

    #[test]
    fn constructor_signatures_preserve_borrowing() {
        let items = ItemCatalog::default();
        let layout = ItemLayout::for_test(vec![]);
        let config = crate::crates::test_config();
        let context = RenderContext::new(&layout, &items, EmissionSite::Webhooks, &config);

        for (typ, lifetime, expected) in [
            (RustType::string(), Some(Lifetime), "impl Into<String>"),
            (RustType::Primitive(PrimitiveType::Str).into_reference(), Some(Lifetime), "&'a str"),
            (RustType::Primitive(PrimitiveType::Str).into_reference(), None, "impl Into<&str>"),
            (RustType::slice(RustType::string()), Some(Lifetime), "[String]"),
            (
                RustType::expandable(RustType::json_value()),
                Some(Lifetime),
                "stripe_types::Expandable<miniserde::json::Value>",
            ),
            (
                RustType::expandable(RustType::json_value()),
                None,
                "impl Into<stripe_types::Expandable<miniserde::json::Value>>",
            ),
        ] {
            assert_eq!(
                context.display_type(&typ).constructor_parameter(lifetime).to_string(),
                expected
            );
        }
    }
}
