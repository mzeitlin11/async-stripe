use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::output_locations::OutputLocations;
use crate::stripe_object::StripeObject;
/// Generate a table describing where all generated requests live.
pub fn generate_crate_table(locations: OutputLocations<'_>) -> String {
    #[derive(Tabled)]
    struct CrateDisplay {
        #[tabled(rename = "Name")]
        name_cell: String,
        #[tabled(rename = "Crate")]
        krate: String,
        #[tabled(rename = "Feature Gate")]
        feature_gate: String,
    }

    let mut comps = vec![];
    for (&component, obj) in &locations.components.components {
        if !locations.components.has_requests(component) {
            continue;
        }
        comps.push(CrateDisplay {
            krate: locations.request_crate(component).crate_name(),
            feature_gate: obj.mod_path(),
            name_cell: name_cell(obj),
        })
    }
    // Cloning when sorting definitely inefficient, but not a hot path at all
    comps.sort_unstable_by_key(|c| (c.krate.clone(), c.name_cell.clone()));
    let mut table = Table::new(comps);
    table.with(Style::markdown());
    table.to_string()
}

fn name_cell(obj: &StripeObject) -> String {
    let ident = obj.ident();
    if let Some(doc_url) = &obj.stripe_doc_url {
        format!("[{ident}]({doc_url})")
    } else {
        ident.to_string()
    }
}
