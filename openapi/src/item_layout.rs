use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::crates::Crate;
use crate::item_catalog::{ItemCatalog, ItemId};
use crate::item_origin::{ItemOrigin, TypeRoot};
use crate::layout::ComponentLayout;
use crate::rust_object::{ObjectKind, RustObject};
use crate::rust_type::DeclarationRoot;

/// The source file or module group in which an item is emitted.
///
/// This is deliberately distinct from a crate. Component models and requests
/// can share a crate while still requiring separate local namespaces.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum EmissionSite {
    ComponentTypes(ComponentId),
    ComponentRequests(ComponentId),
    /// A public owner and any local children in their own module.
    Standalone {
        item: ItemId,
        krate: Crate,
    },
    Webhooks,
}

impl EmissionSite {
    pub(crate) fn krate(self, component_layout: &ComponentLayout) -> Option<Crate> {
        match self {
            Self::ComponentTypes(component) => {
                Some(component_layout.placement(component).types_crate())
            }
            Self::ComponentRequests(component) => {
                Some(component_layout.placement(component).request_crate())
            }
            Self::Standalone { krate, .. } => Some(krate),
            Self::Webhooks => None,
        }
    }
}

/// How generated code names an item from another type.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ItemAddress {
    /// The item is only valid within its emission site and is written without a
    /// module or crate qualifier.
    Local,
    /// The item is publicly reachable from this generated crate's root.
    CrateRoot(Crate),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct ItemPlacement {
    emission_site: EmissionSite,
    address: ItemAddress,
}

impl ItemPlacement {
    pub(crate) const fn emission_site(self) -> EmissionSite {
        self.emission_site
    }

    pub(crate) const fn address(self) -> ItemAddress {
        self.address
    }
}

/// Explicit emission placement for every canonical Rust item.
///
/// The current constructor records the existing component-based behavior. A
/// future item-level layout policy can produce the same table without changing
/// rendering.
#[derive(Debug, Clone)]
pub(crate) struct ItemLayout {
    placements: Vec<ItemPlacement>,
}

impl ItemLayout {
    #[cfg(test)]
    pub(crate) fn for_test(placements: Vec<(EmissionSite, ItemAddress)>) -> Self {
        Self {
            placements: placements
                .into_iter()
                .map(|(emission_site, address)| ItemPlacement { emission_site, address })
                .collect(),
        }
    }

    pub(crate) fn from_component_layout(
        components: &Components,
        items: &ItemCatalog,
        component_layout: &ComponentLayout,
    ) -> anyhow::Result<Self> {
        let mut placements = Vec::with_capacity(items.len());
        for (item, record) in items.iter() {
            let emission_site = emission_site(components, items, &record.origin)?;
            let address =
                item_address(record.origin.path.is_empty(), emission_site, component_layout);
            debug_assert_eq!(item.index(), placements.len());
            placements.push(ItemPlacement { emission_site, address });
        }

        let item_layout = Self { placements };
        item_layout.validate(items, component_layout)?;
        Ok(item_layout)
    }

    #[track_caller]
    pub(crate) fn placement(&self, item: ItemId) -> ItemPlacement {
        self.placements[item.index()]
    }

    pub(crate) fn validate(
        &self,
        items: &ItemCatalog,
        component_layout: &ComponentLayout,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.placements.len() == items.len(),
            "item layout and canonical item counts differed"
        );
        let mut local_names = std::collections::HashSet::new();
        let mut public_names = std::collections::HashSet::new();
        for (item, record) in items.iter() {
            let placement = self.placement(item);
            anyhow::ensure!(
                local_names.insert((placement.emission_site, &record.metadata.ident)),
                "duplicate item name in emission site: {}",
                record.metadata.ident
            );
            if let ItemAddress::CrateRoot(krate) = placement.address {
                anyhow::ensure!(
                    public_names.insert((krate, &record.metadata.ident)),
                    "duplicate public item name in {}: {}",
                    krate.crate_name(),
                    record.metadata.ident
                );
                if let Some(object) = record.object()
                    && let RustObject::Struct(strukt) = &object.definition
                {
                    anyhow::ensure!(
                        strukt.vis.is_public(),
                        "private declaration cannot have crate-root addressability: {}",
                        record.metadata.ident
                    );
                }
                anyhow::ensure!(
                    !matches!(
                        placement.emission_site,
                        EmissionSite::ComponentRequests(_) | EmissionSite::Webhooks
                    ),
                    "item has crate-root addressability in a local-only module: {item:?}"
                );
                anyhow::ensure!(
                    placement.emission_site.krate(component_layout) == Some(krate),
                    "item crate differed from its emission site: {item:?}"
                );
            }
            if let EmissionSite::Standalone { item: owner, .. } = placement.emission_site {
                anyhow::ensure!(
                    owner.index() < items.len()
                        && self.placement(owner).emission_site == placement.emission_site,
                    "standalone module owner had a different emission site: {item:?}"
                );
                anyhow::ensure!(
                    matches!(self.placement(owner).address, ItemAddress::CrateRoot(_)),
                    "standalone module owner must be exported: {owner:?}"
                );
                if matches!(placement.address, ItemAddress::CrateRoot(_)) {
                    anyhow::ensure!(
                        item == owner,
                        "standalone module only exports its owner: {owner:?}, not {item:?}"
                    );
                }
            }
        }
        Ok(())
    }
}

fn emission_site(
    components: &Components,
    items: &ItemCatalog,
    origin: &ItemOrigin,
) -> anyhow::Result<EmissionSite> {
    let site = match &origin.type_root {
        TypeRoot::Declaration(
            DeclarationRoot::Component(component) | DeclarationRoot::ObjectId(component),
        ) => EmissionSite::ComponentTypes(*component),
        TypeRoot::OperationPathParam { operation, .. }
        | TypeRoot::OperationParam(operation)
        | TypeRoot::OperationReturn(operation) => {
            EmissionSite::ComponentRequests(components.operation_record(*operation).component)
        }
        root @ TypeRoot::Declaration(DeclarationRoot::Deduplicated { component, .. }) => {
            let declaration = items
                .root(root)
                .and_then(|root| root.declaration)
                .ok_or_else(|| anyhow::anyhow!("deduplicated root had no declaration: {root:?}"))?;
            let object = items
                .get(declaration)
                .object()
                .ok_or_else(|| anyhow::anyhow!("deduplicated root was not an object: {root:?}"))?;
            match object.usage.kind {
                ObjectKind::Type => EmissionSite::ComponentTypes(*component),
                ObjectKind::RequestParam | ObjectKind::RequestReturned => {
                    EmissionSite::ComponentRequests(*component)
                }
            }
        }
        root @ TypeRoot::Declaration(DeclarationRoot::Shared(_)) => {
            let declaration = items
                .root(root)
                .and_then(|root| root.declaration)
                .ok_or_else(|| anyhow::anyhow!("shared root had no declaration: {root:?}"))?;
            EmissionSite::Standalone { item: declaration, krate: Crate::SHARED }
        }
        TypeRoot::Webhook(_) => EmissionSite::Webhooks,
    };
    Ok(site)
}

fn item_address(
    is_root: bool,
    emission_site: EmissionSite,
    component_layout: &ComponentLayout,
) -> ItemAddress {
    if !is_root {
        return ItemAddress::Local;
    }
    match emission_site {
        EmissionSite::ComponentTypes(component) => {
            ItemAddress::CrateRoot(component_layout.placement(component).types_crate())
        }
        EmissionSite::Standalone { krate, .. } => ItemAddress::CrateRoot(krate),
        EmissionSite::ComponentRequests(_) | EmissionSite::Webhooks => ItemAddress::Local,
    }
}
