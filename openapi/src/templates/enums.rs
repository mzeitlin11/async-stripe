use std::fmt::Write as _;

use indoc::writedoc;

use crate::components::Components;
use crate::printable::PrintableWithLifetime;
use crate::rust_object::{
    EnumOfObjects, EnumVariant, FieldlessVariant, ObjectRef, ShouldImplSerialize,
};
use crate::templates::miniserde::gen_enum_of_objects_miniserde;
use crate::templates::object_writer::{write_derives_line, ObjectWriter};
use crate::templates::utils::{write_gated_serde_rename, write_gated_serde_tag};

impl<'a> ObjectWriter<'a> {
    pub fn write_enum_variants(&self, out: &mut String, variants: &[EnumVariant]) {
        let enum_name = self.ident;

        // Build the body of the enum definition
        let mut enum_body = String::with_capacity(64);
        for EnumVariant { variant, rust_type } in variants {
            if let Some(typ) = rust_type {
                let printable = self.components.construct_printable_type(typ);
                let typ = PrintableWithLifetime::new(&printable, self.lifetime);
                let _ = writeln!(enum_body, "{variant}({typ}),");
            } else {
                let _ = writeln!(enum_body, "{variant},");
            }
        }
        self.write_automatic_derives(out);
        let lifetime_str = self.lifetime_param();
        let serde_tag = if self.obj_kind.is_request_param() {
            r#"#[serde(untagged)]"#
        } else {
            r#"#[cfg_attr(not(feature = "min-ser"), serde(untagged))]"#
        };
        let _ = writedoc!(
            out,
            r#"
            {serde_tag}
            pub enum {enum_name}{lifetime_str} {{
            {enum_body}
            }}
        "#
        );
        if self.obj_kind.should_impl_deserialize() {
            unimplemented!();
        }
    }

    pub fn write_enum_of_objects(
        &self,
        out: &mut String,
        components: &Components,
        objects: &EnumOfObjects,
    ) {
        if self.lifetime.is_some()
            || self.obj_kind.should_impl_serialize() != ShouldImplSerialize::SkipIfMinSer
        {
            unimplemented!();
        }
        let enum_name = self.ident;

        // Build the body of the enum definition
        let mut enum_body = String::with_capacity(64);

        match objects {
            EnumOfObjects::MaybeDeleted(maybe_deleted) => {
                for item in [&maybe_deleted.base, &maybe_deleted.deleted] {
                    let name = &item.ident;
                    let printable = components.construct_printable_type_from_path(&item.path);
                    let _ = writeln!(enum_body, "{name}({printable}),");
                }
            }
            EnumOfObjects::ObjectUnion(objects) => {
                for (obj_name, obj) in objects {
                    let name = &obj.ident;
                    let printable = components.construct_printable_type_from_path(&obj.path);
                    write_feature_gate(&mut enum_body, obj);
                    write_gated_serde_rename(&mut enum_body, &obj_name);
                    let _ = writeln!(enum_body, "{name}({printable}),");
                }
            }
        }
        if self.provide_unknown_variant {
            write_gated_serde_tag(&mut enum_body, "other");
            let _ = writeln!(enum_body, "Unknown");
        }

        self.write_automatic_derives(out);
        self.write_nonexhaustive_attr(out);

        match objects {
            EnumOfObjects::MaybeDeleted(_) => write_gated_serde_tag(out, "untagged"),
            EnumOfObjects::ObjectUnion(_) => write_gated_serde_tag(out, r#"tag = "object""#),
        }

        let miniserde_impl = gen_enum_of_objects_miniserde(enum_name, objects);
        let _ = writedoc!(
            out,
            r#"
            pub enum {enum_name} {{
            {enum_body}
            }}
            
            {miniserde_impl}
        "#
        );
    }

    /// Generate the enum definition, along with the methods `as_str`, `as_ref`, `impl Display`,
    /// and `impl Default`.
    pub fn write_fieldless_enum_variants(&self, out: &mut String, variants: &[FieldlessVariant]) {
        let enum_name = self.ident;
        // Build the body of the enum definition
        let mut enum_def_body = String::with_capacity(128);
        for FieldlessVariant { variant_name, .. } in variants {
            let _ = writeln!(enum_def_body, "{variant_name},");
        }
        if self.provide_unknown_variant {
            let _ = writedoc!(
                enum_def_body,
                r"
            /// An unrecognized value from Stripe. Should not be used as a request parameter.
            Unknown,
        "
            );
        }

        // Build the body of the `as_str` implementation
        let mut as_str_body = String::with_capacity(32);
        for FieldlessVariant { wire_name, variant_name } in variants {
            let _ = writeln!(as_str_body, r#"{variant_name} => "{wire_name}","#);
        }
        if self.provide_unknown_variant {
            let _ = writeln!(as_str_body, r#"Unknown => "unknown","#);
        }

        // Build the body of the `from_str` implementation
        let mut from_str_body = String::with_capacity(32);
        for FieldlessVariant { wire_name, variant_name } in variants {
            let _ = writeln!(from_str_body, r#""{wire_name}" => Ok({variant_name}),"#);
        }
        let _ = writeln!(from_str_body, "_ => Err(())");

        // NB: we manually implement Debug, Serialize, Deserialize to avoid duplicating
        // the (potentially many) strings in `as_str` and `from_str` used with the default derive.
        // These derived implementations often show up running `llvm-lines`, so easy
        // binary size + compile time win by doing this.

        write_derives_line(out, self.derives.eq(true).debug(false));
        self.write_nonexhaustive_attr(out);
        let _ = writedoc!(
            out,
            r#"
            pub enum {enum_name} {{
            {enum_def_body}
            }}
            impl {enum_name} {{
                pub fn as_str(self) -> &'static str {{
                    use {enum_name}::*;
                    match self {{
            {as_str_body}
                    }}
                }}
            }}
            
            impl std::str::FromStr for {enum_name} {{
                type Err = ();
                fn from_str(s: &str) -> Result<Self, Self::Err> {{
                    use {enum_name}::*;
                    match s {{
                {from_str_body}
                    }}
                }}
            }}
            impl AsRef<str> for {enum_name} {{
                fn as_ref(&self) -> &str {{
                    self.as_str()
                }}
            }}
            impl std::fmt::Display for {enum_name} {{
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                    f.write_str(self.as_str())
                }}
            }}
            
            impl std::fmt::Debug for {enum_name} {{
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                    f.write_str(self.as_str())
                }}
            }}
        "#
        );

        let _ = writedoc!(
            out,
            r#"
            impl serde::Serialize for {enum_name} {{
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{
                    serializer.serialize_str(self.as_str())
                }}
            }}
            "#
        );

        if self.obj_kind.should_impl_deserialize() {
            let serde_ret_line = if self.provide_unknown_variant {
                "Ok(Self::from_str(&s).unwrap_or(Self::Unknown))".into()
            } else {
                format!(
                    r#"Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for {enum_name}"))"#
                )
            };

            let miniserde_assign_line = if self.provide_unknown_variant {
                format!("Some({enum_name}::from_str(s).unwrap_or({enum_name}::Unknown))")
            } else {
                format!("Some({enum_name}::from_str(s).map_err(|_| miniserde::Error)?)")
            };
            let _ = writedoc!(
                out,
                r#"
            impl<'de> serde::Deserialize<'de> for {enum_name} {{
                fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
                    use std::str::FromStr;
                    let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
                    {serde_ret_line}
                }}
            }}
            #[cfg(feature = "min-ser")]
            impl miniserde::Deserialize for {enum_name} {{
                fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {{
                    crate::Place::new(out)
                }}
            }}
            
            #[cfg(feature = "min-ser")]
            impl miniserde::de::Visitor for crate::Place<{enum_name}> {{
                fn string(&mut self, s: &str) -> miniserde::Result<()> {{
                    use std::str::FromStr;
                    self.out = {miniserde_assign_line};
                    Ok(())
                }}
            }}
            "#
            );
        }
    }
}

pub fn write_feature_gate(out: &mut String, obj_ref: &ObjectRef) {
    if let Some(feature_gate) = &obj_ref.feature_gate {
        let _ = writeln!(out, r#"#[cfg(feature = "{feature_gate}")]"#);
    }
}
