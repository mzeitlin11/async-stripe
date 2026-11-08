use anyhow::Context as _;

use super::ItemCatalogDraft;
use crate::components::Components;
use crate::dependencies::EdgePolicy;
use crate::rust_object::RustObject;
use crate::rust_type::{Container, RustType};

impl ItemCatalogDraft {
    pub(crate) fn opaque_charge_customer(&mut self, components: &Components) -> anyhow::Result<()> {
        let component = |name: &str| {
            components
                .components
                .keys()
                .copied()
                .find(|id| components.component_name(*id).as_ref() == name)
                .with_context(|| format!("probe component missing: {name}"))
        };
        let charge = component("charge")?;
        let customer = component("customer")?;
        anyhow::ensure!(
            components
                .deps_for_component_type(charge, EdgePolicy::current_layout())
                .contains(&customer),
            "parsed customer relationship must remain available"
        );
        let customer_item = self.catalog.component_root(customer);
        let customer_id =
            self.catalog.component_id_item(customer).context("customer ID missing")?;
        let charge_item = self.catalog.component_root(charge);
        let record = &mut self.catalog.items[charge_item.index()];
        let object = record.object().context("charge is not an object")?;
        anyhow::ensure!(
            matches!(object.definition, RustObject::Struct(_)),
            "charge is not a struct"
        );
        let super::RustItem::Object(object) = &mut record.item else { unreachable!() };
        let RustObject::Struct(strukt) = &mut object.definition else { unreachable!() };
        let field = strukt
            .fields
            .iter_mut()
            .find(|field| field.field_name == "customer")
            .context("charge.customer missing")?;
        anyhow::ensure!(
            field.rust_type
                == RustType::option(RustType::expandable(RustType::Item(customer_item))),
            "unexpected charge.customer representation: {:?}",
            field.rust_type
        );
        field.rust_type = RustType::option(RustType::Container(Container::OpaqueExpandable(
            Box::new(RustType::Item(customer_id)),
        )));
        Ok(())
    }
}
