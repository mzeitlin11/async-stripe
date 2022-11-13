use std::collections::BTreeMap;
use std::fmt::Write as _;

use indoc::writedoc;

use crate::metadata::gen_variant_name;
use crate::types::{InferredEnum, RustObjectTypeName, SchemaName};
use crate::util::write_serde_rename;

#[tracing::instrument(skip_all)]
pub fn gen_enums(out: &mut String, enums: &BTreeMap<RustObjectTypeName, InferredEnum>) {
    for (enum_name, enum_) in enums {
        log::trace!("enum {} {{ ... }}", enum_name);

        out.push('\n');
        out.push_str(&format!(
            "/// An enum representing the possible values of an `{}`'s `{}` field.\n",
            enum_.parent, enum_.field
        ));
        out.push_str("#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]\n");
        out.push_str("#[serde(rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(enum_name);
        out.push_str(" {\n");
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = gen_variant_name(wire_name.as_str());
            if variant_name.trim().is_empty() {
                panic!("unhandled enum variant: {:?}", wire_name)
            }
            if &variant_name.to_snake_case() != wire_name {
                write_serde_rename(out, wire_name);
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push_str(",\n");
        }
        out.push_str("}\n");
        out.push('\n');
        out.push_str("impl ");
        out.push_str(enum_name);
        out.push_str(" {\n");
        out.push_str("    pub fn as_str(self) -> &'static str {\n");
        out.push_str("        match self {\n");
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = gen_variant_name(wire_name.as_str());
            out.push_str("            ");
            out.push_str(enum_name);
            out.push_str("::");
            out.push_str(&variant_name);
            out.push_str(" => ");
            out.push_str(&format!("{:?}", wire_name));
            out.push_str(",\n");
        }
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n");
        out.push('\n');
        writedoc!(
            out,
            r"
        impl AsRef<str> for {enum_name} {{
            fn as_ref(&self) -> &str {{
                self.as_str()
            }}
        }}
        
        impl std::fmt::Display for {enum_name} {{
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                self.as_str().fmt(f)
            }}
        }}
        "
        )
        .unwrap();
        if let Some(first) = enum_
            .options
            .iter()
            .filter_map(|var| match var.trim() {
                "" => None,
                n => Some(n),
            })
            .map(gen_variant_name)
            .next()
        {
            out.push_str("impl std::default::Default for ");
            out.push_str(enum_name);
            out.push_str(" {\n");
            out.push_str("    fn default() -> Self {\n");
            out.push_str(&format!("        Self::{}\n", first));
            out.push_str("    }\n");
            out.push_str("}\n");
        }
    }
}
