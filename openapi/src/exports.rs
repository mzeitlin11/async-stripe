use crate::component_catalog::ComponentId;
use crate::crates::Crate;
use crate::item_catalog::{ItemCatalog, ItemId};
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::layout::ComponentLayout;
use crate::output_locations::OutputLocations;

/// A public consumer of a generated declaration or its component module.
/// Rendering and Cargo dependency calculation use these same decisions.
pub(crate) enum PublicExport {
    Item { item: ItemId, facade: Crate, target: Crate },
    ComponentModule { component: ComponentId, facade: Crate, target: Crate },
}

impl PublicExport {
    pub(crate) fn item(item: ItemId, facade: Crate, layout: &ItemLayout) -> anyhow::Result<Self> {
        let ItemAddress::CrateRoot(target) = layout.placement(item).address() else {
            anyhow::bail!("public declaration is not exported: {item:?}");
        };
        Ok(Self::Item { item, facade, target })
    }

    pub(crate) fn dependency(&self) -> Option<(Crate, Crate)> {
        let (facade, target) = match *self {
            Self::Item { facade, target, .. } | Self::ComponentModule { facade, target, .. } => {
                (facade, target)
            }
        };
        (facade != target).then_some((facade, target))
    }

    pub(crate) fn render(
        &self,
        items: &ItemCatalog,
        locations: OutputLocations<'_>,
    ) -> Option<(Crate, String)> {
        let (facade, target) = self.dependency()?;
        let text = match *self {
            Self::Item { item, .. } => {
                format!(
                    "pub use {}::{};",
                    target.lib_name(locations.config),
                    items.get(item).metadata.ident
                )
            }
            Self::ComponentModule { component, .. } => {
                let module = locations.components.get_by_id(component).mod_path();
                format!("pub use {}::{module}::*;", target.lib_name(locations.config))
            }
        };
        Some((facade, text))
    }
}

pub(crate) fn component_exports(
    component: ComponentId,
    items: &ItemCatalog,
    layout: &ItemLayout,
    component_layout: &ComponentLayout,
) -> anyhow::Result<Vec<PublicExport>> {
    let facade = component_layout.placement(component).request_crate();
    let model = items.component_root(component);
    let placement = layout.placement(model);
    let mut exports = Vec::new();
    let mut wildcard_site = None;
    // Preserve existing wildcard exports, including public helper declarations
    // in the model's module. Independently placed IDs need their own export.
    if let ItemAddress::CrateRoot(target) = placement.address()
        && target != facade
        && placement.emission_site() == EmissionSite::ComponentTypes(component)
    {
        exports.push(PublicExport::ComponentModule { component, facade, target });
        wildcard_site = Some(placement.emission_site());
    }
    for item in std::iter::once(model).chain(items.component_id_item(component)) {
        let export = PublicExport::item(item, facade, layout)?;
        if Some(layout.placement(item).emission_site()) != wildcard_site {
            exports.push(export);
        }
    }
    Ok(exports)
}
