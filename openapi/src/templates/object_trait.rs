use std::fmt::Write;

use indexmap::IndexMap;
use indoc::writedoc;

use crate::STRIPE_TYPES;
use crate::component_catalog::ComponentId;
use crate::render::RenderContext;
use crate::types::RustIdent;

pub fn write_object_trait(out: &mut String, ident: &RustIdent, id_type: impl std::fmt::Display) {
    let _ = writedoc!(
        out,
        r#"
            impl {STRIPE_TYPES}::Object for {ident} {{
                type Id = {id_type};
                fn id(&self) -> &Self::Id {{
                    &self.id
                }}

                fn into_id(self) -> Self::Id {{
                    self.id
                }}
            }}
            "#
    );
}

pub fn write_object_trait_for_enum(
    out: &mut String,
    ident: &RustIdent,
    variants: &IndexMap<String, ComponentId>,
    context: RenderContext<'_>,
) {
    let mut match_inner = String::with_capacity(32);
    let mut match_into_inner = String::with_capacity(32);
    for component in variants.values() {
        let ident = context.component_ident(*component);
        let _ = writeln!(match_inner, "Self::{ident}(v) => v.id.inner(),");
        let _ = writeln!(match_into_inner, "Self::{ident}(v) => v.id.into_inner(),");
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

                fn into_id(self) -> Self::Id {{
                    match self {{
                    {match_into_inner}
                    }}
                }}
            }}
            "#
    );
}
