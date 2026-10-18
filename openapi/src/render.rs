use std::fmt::{Display, Formatter, Write as _};

use crate::STRIPE_TYPES;
use crate::component_catalog::ComponentId;
use crate::crates::Crate;
use crate::item_catalog::{CanonicalTypeRoot, ItemCatalog, ItemId, ItemProperties, ItemRecord};
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::item_origin::TypeRoot;
use crate::render_plan::{ParameterConversion, constructor_parameter_conversion};
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
    conversion: ParameterConversion,
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
            conversion: constructor_parameter_conversion(self.typ, lifetime.is_some()),
        };
        self
    }
}

impl Display for DisplayType<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_type(f, self.context, self.typ, self.use_site)
    }
}

/// A display adapter for a direct reference to a top-level component.
#[derive(Copy, Clone)]
pub struct DisplayComponent<'model> {
    context: RenderContext<'model>,
    component: ComponentId,
}

impl Display for DisplayComponent<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let item = self.context.resolve_component(self.component);
        fmt_named_type(f, self.context.emission_site, &item, TypeUse::default())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ItemLocation {
    Local,
    CrateRoot(Crate),
    ExternalModule(&'static str),
}

struct ResolvedTypeItem<'a> {
    location: ItemLocation,
    ident: &'a str,
    has_lifetime: bool,
}

impl<'a> ResolvedTypeItem<'a> {
    fn local(ident: &'a RustIdent, has_lifetime: bool) -> Self {
        Self { location: ItemLocation::Local, ident: ident.as_ref(), has_lifetime }
    }

    fn crate_root(krate: Crate, ident: &'a RustIdent, has_lifetime: bool) -> Self {
        Self { location: ItemLocation::CrateRoot(krate), ident: ident.as_ref(), has_lifetime }
    }

    fn external(module: &'static str, ident: &'static str) -> ResolvedTypeItem<'static> {
        ResolvedTypeItem {
            location: ItemLocation::ExternalModule(module),
            ident,
            has_lifetime: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RenderContext<'a> {
    item_layout: &'a ItemLayout,
    items: &'a ItemCatalog,
    emission_site: EmissionSite,
}

impl<'model> RenderContext<'model> {
    pub const fn new(
        item_layout: &'model ItemLayout,
        items: &'model ItemCatalog,
        emission_site: EmissionSite,
    ) -> Self {
        Self { item_layout, items, emission_site }
    }

    pub(crate) fn item_address(self, item: ItemId) -> ItemAddress {
        self.item_layout.placement(item).address()
    }

    pub fn display_type<'type_>(self, typ: &'type_ RustType) -> DisplayType<'model, 'type_> {
        DisplayType { context: self, typ, use_site: TypeUse::default() }
    }

    pub fn display_component(self, component: ComponentId) -> DisplayComponent<'model> {
        DisplayComponent { context: self, component }
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

    fn resolve_component(self, component: ComponentId) -> ResolvedTypeItem<'model> {
        self.resolve_item(self.items.component_root(component))
    }

    fn resolve_item(self, item: ItemId) -> ResolvedTypeItem<'model> {
        let record = self.items.get(item);
        let properties = self.items.properties(item);
        match self.item_layout.placement(item).address() {
            ItemAddress::Local => {
                self.assert_emits(item);
                ResolvedTypeItem::local(&record.metadata.ident, properties.has_lifetime)
            }
            ItemAddress::CrateRoot(krate) => {
                ResolvedTypeItem::crate_root(krate, &record.metadata.ident, properties.has_lifetime)
            }
        }
    }

    fn resolve_external(self, typ: ExternalType) -> ResolvedTypeItem<'static> {
        match typ {
            ExternalType::Currency => ResolvedTypeItem::external(STRIPE_TYPES, "Currency"),
            ExternalType::RangeQueryTs => ResolvedTypeItem::external(STRIPE_TYPES, "RangeQueryTs"),
            ExternalType::Timestamp => ResolvedTypeItem::external(STRIPE_TYPES, "Timestamp"),
            ExternalType::AlwaysTrue => ResolvedTypeItem::external(STRIPE_TYPES, "AlwaysTrue"),
            ExternalType::Value => ResolvedTypeItem::external("miniserde::json", "Value"),
        }
    }
}

fn fmt_type(
    f: &mut Formatter<'_>,
    context: RenderContext<'_>,
    typ: &RustType,
    use_site: TypeUse,
) -> std::fmt::Result {
    if use_site.conversion == ParameterConversion::Into {
        f.write_str("impl Into<")?;
        fmt_type(
            f,
            context,
            typ,
            TypeUse { lifetime: use_site.lifetime, conversion: ParameterConversion::Direct },
        )?;
        return f.write_char('>');
    }

    match typ {
        RustType::Object(_, _) => panic!("inline object reached canonical type rendering"),
        RustType::Primitive(primitive) => fmt_primitive_type(f, *primitive),
        RustType::External(external) => {
            let item = context.resolve_external(*external);
            fmt_named_type(f, context.emission_site, &item, use_site)
        }
        RustType::Item(item) => {
            let item = context.resolve_item(*item);
            fmt_named_type(f, context.emission_site, &item, use_site)
        }
        RustType::Path(path) => panic!("unlowered named path reached type rendering: {path:?}"),
        RustType::Container(container) => fmt_container(f, context, container, use_site),
        RustType::Reference(inner) => fmt_reference(f, context, inner, use_site),
    }
}

fn fmt_named_type(
    f: &mut Formatter<'_>,
    emission_site: EmissionSite,
    item: &ResolvedTypeItem<'_>,
    use_site: TypeUse,
) -> std::fmt::Result {
    if let Some(lifetime) = use_site.lifetime {
        fmt_type_name(f, emission_site, item)?;
        if item.has_lifetime {
            write!(f, "<{lifetime}>")?;
        }
        return Ok(());
    }

    fmt_type_name(f, emission_site, item)
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
    fmt_type(
        f,
        context,
        inner,
        TypeUse { lifetime: use_site.lifetime, conversion: ParameterConversion::Direct },
    )
}

fn fmt_type_name(
    f: &mut Formatter<'_>,
    emission_site: EmissionSite,
    item: &ResolvedTypeItem<'_>,
) -> std::fmt::Result {
    match item.location {
        ItemLocation::Local => {}
        ItemLocation::CrateRoot(krate) => {
            write_crate_qualifier(f, emission_site, krate)?;
        }
        ItemLocation::ExternalModule(module) => write!(f, "{module}::")?,
    }
    write!(f, "{}", item.ident)
}

fn write_crate_qualifier(
    f: &mut Formatter<'_>,
    _emission_site: EmissionSite,
    target: Crate,
) -> std::fmt::Result {
    // The current output contract always uses the generated crate's absolute
    // alias, including for same-crate references. The emission site is
    // intentionally available here so a later, separate change can choose
    // `crate::` without changing semantic item resolution.
    write!(f, "{}::", target.lib_name())
}

fn fmt_primitive_type(f: &mut Formatter<'_>, typ: PrimitiveType) -> std::fmt::Result {
    f.write_str(typ.ident())
}

fn fmt_container(
    f: &mut Formatter<'_>,
    context: RenderContext<'_>,
    container: &Container,
    use_site: TypeUse,
) -> std::fmt::Result {
    let nested_use =
        TypeUse { lifetime: use_site.lifetime, conversion: ParameterConversion::Direct };
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
            fmt_map_key(f, context, *key)?;
            f.write_str(", ")?;
            fmt_type(f, context, value, nested_use)?;
            f.write_char('>')?;
        }
    }

    Ok(())
}

fn fmt_map_key(f: &mut Formatter<'_>, context: RenderContext<'_>, key: MapKey) -> std::fmt::Result {
    match key {
        MapKey::String => f.write_str(PrimitiveType::String.ident()),
        MapKey::Currency => {
            let item = context.resolve_external(ExternalType::Currency);
            fmt_named_type(f, context.emission_site, &item, TypeUse::default())
        }
    }
}
