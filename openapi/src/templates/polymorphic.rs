use std::fmt::Write;

use crate::rust_object::Struct;
use crate::rust_type::{ExternalType, RustType};
use crate::templates::ObjectWriter;
use crate::templates::object_writer::{write_derives_line, write_redacted_debug_impl};
use crate::templates::utils::write_doc_comment;

impl ObjectWriter<'_> {
    /// Keep common fields typed and preserve every other field for lazy decoding.
    pub(crate) fn write_polymorphic_struct(&self, out: &mut String, record: &Struct) {
        let name = self.ident;
        let visibility = record.vis;
        write_derives_line(out, self.derives.copy(false).eq(false));
        let _ = writeln!(out, "{} struct {name} {{", record.vis);
        for field in &record.fields {
            if let Some(doc) = &field.doc_comment {
                out.push_str(&write_doc_comment(doc, 4));
            }
            let _ = writeln!(
                out,
                "{} {}: {},",
                field.vis,
                field.variable_name(),
                self.display_type(&field.rust_type)
            );
        }
        let _ = writeln!(out, "variant_fields: stripe_types::DeferredFields,\n}}");
        write_redacted_debug_impl(out, name.as_ref(), "");
        let _ = writeln!(out, "impl {name} {{");
        out.push_str(&write_doc_comment("Borrow the retained variant and unknown fields.", 4));
        out.push_str("pub fn variant_fields(&self) -> &stripe_types::DeferredFields { &self.variant_fields }\n");
        out.push_str(&write_doc_comment("Decode a retained field without consuming this record.\n\n# Errors\nReturns an error when a present non-null field does not satisfy T.", 4));
        out.push_str("pub fn field_as<T: stripe_types::miniserde_helpers::FromValueOpt>(&self, field: &str) -> Result<Option<T>, stripe_types::RecordDecodeError> { self.variant_fields.field_as(field) }\n}\n");

        // Parse one JSON object, moving common fields into their typed slots.
        // The remainder is the authoritative raw bag; no duplicate root cache
        // can become stale when callers update a public common field.
        let mutable =
            if record.fields.is_empty() && record.object_field.is_none() { "" } else { "mut " };
        let _ = writeln!(
            out,
            "impl stripe_types::miniserde_helpers::FromValueOpt for {name} {{\nfn from_value(value: miniserde::json::Value) -> Option<Self> {{\nlet miniserde::json::Value::Object({mutable}fields) = value else {{ return None; }};"
        );
        if !record.fields.is_empty() {
            out.push_str("use stripe_types::miniserde_helpers::FromValueOpt;\n");
        }
        for field in &record.fields {
            let field_name = field.variable_name();
            let wire = field.wire_name();
            let value = if field.rust_type.is_option() {
                format!("fields.remove({wire:?}).unwrap_or(miniserde::json::Value::Null)")
            } else {
                format!("fields.remove({wire:?})?")
            };
            let _ = writeln!(
                out,
                "let {field_name}: {} = FromValueOpt::from_value({value})?;",
                self.display_type(&field.rust_type)
            );
        }
        if record.object_field.is_some() {
            out.push_str("fields.remove(\"object\");\n");
        }
        out.push_str("Some(Self {\n");
        for field in &record.fields {
            let _ = writeln!(out, "{},", field.variable_name());
        }
        out.push_str(
            "variant_fields: stripe_types::DeferredFields::from_object(fields),\n})\n}\n}\n",
        );
        let _ = writeln!(
            out,
            r#"
impl miniserde::Deserialize for {name} {{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {{ crate::Place::new(out) }}
}}
impl miniserde::de::Visitor for crate::Place<{name}> {{
    fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {{
        Ok(stripe_types::miniserde_helpers::object_value_map(&mut self.out))
    }}
}}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for {name} {{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
        let value = stripe_types::with_serde_json::deserialize(deserializer)?;
        <Self as stripe_types::miniserde_helpers::FromValueOpt>::from_value(value)
            .ok_or_else(|| serde::de::Error::custom("invalid {name}"))
    }}
}}
#[doc(hidden)]
#[derive(Default)]
{visibility} struct {name}Builder {{ fields: stripe_types::miniserde_helpers::ObjectBuilderInner }}
impl stripe_types::MapBuilder for {name}Builder {{
    type Out = {name};
    fn deser_default() -> Self {{ Self::default() }}
    fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {{ self.fields.key_inner(key) }}
    fn take_out(&mut self) -> Option<Self::Out> {{
        <{name} as stripe_types::miniserde_helpers::FromValueOpt>::from_value(miniserde::json::Value::Object(self.fields.take_object()))
    }}
}}
impl stripe_types::ObjectDeser for {name} {{ type Builder = {name}Builder; }}
#[cfg(feature = "serialize")]
impl serde::Serialize for {name} {{
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {{
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
"#
        );
        for field in &record.fields {
            let field_name = field.variable_name();
            let value = match &field.rust_type {
                RustType::External(ExternalType::Value) => {
                    format!("stripe_types::with_serde_json::as_serialize(&self.{field_name})")
                }
                typ if typ.is_option()
                    && matches!(
                        typ.with_option_stripped(),
                        RustType::External(ExternalType::Value)
                    ) =>
                {
                    format!("stripe_types::with_serde_json_opt::as_serialize(&self.{field_name})")
                }
                _ => format!("self.{field_name}"),
            };
            let _ = writeln!(out, "map.serialize_entry({:?}, &{value})?;", field.wire_name());
        }
        out.push_str("self.variant_fields.serialize_entries(&mut map)?;\n");
        if let Some(tag) = &record.object_field {
            let _ = writeln!(out, "map.serialize_entry(\"object\", {tag:?})?;");
        }
        out.push_str("map.end()\n}\n}\n");
    }
}
