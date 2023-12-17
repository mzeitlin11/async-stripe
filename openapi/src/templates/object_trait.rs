use std::fmt::Write;

use indexmap::IndexMap;
use indoc::writedoc;

use crate::components::Components;
use crate::printable::PrintableType;
use crate::rust_object::ObjectRef;
use crate::types::RustIdent;
use crate::STRIPE_TYPES;

pub fn write_object_trait(out: &mut String, ident: &RustIdent, id_type: &PrintableType) {
    let _ = writedoc!(
        out,
        r#"
            impl {STRIPE_TYPES}::Object for {ident} {{
                type Id = {id_type};
                fn id(&self) -> &Self::Id {{
                    &self.id
                }}
            }}
            "#
    );
}

pub fn write_object_trait_for_enum(
    components: &Components,
    out: &mut String,
    ident: &RustIdent,
    variants: &IndexMap<&str, ObjectRef>,
) {
    let mut match_inner = String::with_capacity(32);
    for obj in variants.values() {
        let comp = components.get(&obj.path);
        let ident = comp.ident();
        let _ = writeln!(match_inner, "Self::{ident}(v) => v.id.inner(),");
    }
    let _ = writedoc!(
        out,
        r#"
            impl {STRIPE_TYPES}::Object for {ident} {{
                type Id = smol_str::SmolStr;
                fn id(&self) -> &Self::Id {{
                    match self {{
                    {match_inner}
                    }}
                }}
            }}
            "#
    );
}
