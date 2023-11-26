use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::spec_inference::infer_id_name;
use crate::types::ComponentPath;

#[derive(Deserialize)]
#[serde(untagged)]
enum IdPrefix {
    Single(String),
    Multi(Vec<String>),
}

fn load_id_prefixes() -> anyhow::Result<HashMap<String, IdPrefix>> {
    let prefixes = serde_json::from_reader(File::open("id_prefixes.json")?)?;
    Ok(prefixes)
}

lazy_static! {
    static ref ID_PREFIXES: HashMap<String, IdPrefix> =
        load_id_prefixes().expect("Invalid id prefix file");
}

pub fn write_object_id(out: &mut String, path: &ComponentPath) {
    let crate_name = "stripe_types";
    let ident = infer_id_name(path);
    match ID_PREFIXES.get(path.as_ref()) {
        Some(IdPrefix::Single(prefix)) => {
            let _ = writeln!(out, r#"{crate_name}::def_id!({ident}, "{prefix}_");"#);
        }
        Some(IdPrefix::Multi(prefixes)) => {
            let prefix_arg =
                prefixes.iter().map(|p| format!(r#""{p}_""#)).collect::<Vec<_>>().join("|");
            let _ = writeln!(out, "{crate_name}::def_id!({ident}, {prefix_arg});");
        }
        None => {
            let _ = writeln!(out, "{crate_name}::def_id!({ident});");
        }
    }
}
