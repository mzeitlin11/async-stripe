use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::components::Components;
use crate::types::RustIdent;
use crate::utils::write_to_file;

/// Write a table describing where all generated requests live
pub fn write_crate_table(components: &Components) -> anyhow::Result<()> {
    #[derive(Tabled)]
    struct CrateDisplay {
        #[tabled(rename = "Struct Name")]
        name: String,
        #[tabled(rename = "Path Implied Name")]
        path_implied_name: String,
        #[tabled(rename = "Crate")]
        krate: String,
        #[tabled(rename = "Feature Gate")]
        feature_gate: String,
    }

    let mut comps = vec![];
    for obj in components.components.values() {
        if obj.requests.is_empty() {
            continue;
        }
        comps.push(CrateDisplay {
            name: obj.resource.ident().to_string(),
            krate: obj.krate_unwrapped().base().name(),
            feature_gate: obj.mod_path(),
            path_implied_name: RustIdent::create(obj.path()).to_string(),
        })
    }
    comps.sort_by_key(|c| c.krate.clone());
    let mut table = Table::new(comps);
    table.with(Style::markdown());
    let display = table.to_string();
    write_to_file(display, "crate_info.md")?;
    Ok(())
}
