//! Count represented fields for generated deserializers, once per canonical item.
use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::codegen::CodeGen;
use crate::crates::ALL_CRATES;
use crate::item_catalog::ObjectItem;
use crate::rust_object::RustObject;

fn fields(object: &ObjectItem) -> Option<usize> {
    if !object.usage.should_impl_deserialize() {
        return None;
    }
    match &object.definition {
        RustObject::Struct(strukt) => Some(strukt.fields.len()),
        _ => None,
    }
}

pub(super) fn reports(codegen: &CodeGen) -> (String, String) {
    let mut rows = Vec::new();
    let mut totals: BTreeMap<String, (usize, usize)> =
        ALL_CRATES.iter().map(|krate| (krate.crate_name(), (0, 0))).collect();
    let items = codegen.model.items();
    let locations = codegen.output_locations();
    for (id, record) in items.iter() {
        let Some(count) = record.object().and_then(fields) else {
            continue;
        };
        let site = codegen.item_layout.placement(id).emission_site();
        let krate = locations
            .emission_crate(site)
            .map_or_else(|| "async-stripe-webhook".into(), |krate| krate.crate_name());
        let path = locations.emission_path(site, items).display().to_string();
        let total = totals.entry(krate.clone()).or_default();
        total.0 += 1;
        total.1 += count;
        rows.push((krate, path, record.metadata.ident.to_string(), id.index(), count));
    }
    rows.sort();
    let mut detailed = "crate,emission_path,ident,item_id,fields\n".to_owned();
    for (krate, path, ident, id, count) in rows {
        writeln!(
            detailed,
            "{},{},{},{id},{count}",
            super::csv(&krate),
            super::csv(&path),
            super::csv(&ident)
        )
        .unwrap();
    }
    let mut summary = "crate,structs,fields\n".to_owned();
    for (krate, (structs, fields)) in totals {
        writeln!(summary, "{},{structs},{fields}", super::csv(&krate)).unwrap();
    }
    (detailed, summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust_object::{ObjectUsage, Struct, StructField};
    use crate::rust_type::RustType;

    #[test]
    fn counts_only_deserializable_struct_fields() {
        let mut object = ObjectItem {
            definition: RustObject::Struct(Struct::new(vec![StructField::new(
                "id",
                RustType::string(),
            )])),
            usage: ObjectUsage::request_param(),
            enum_of_objects: None,
            identity: None,
        };
        assert_eq!(fields(&object), None);
        object.usage = ObjectUsage::return_type();
        assert_eq!(fields(&object), Some(1));
        object.usage = ObjectUsage::type_def();
        assert_eq!(fields(&object), Some(1));
        object.definition = RustObject::FieldlessEnum(vec![]);
        assert_eq!(fields(&object), None);
    }
}
