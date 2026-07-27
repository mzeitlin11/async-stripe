use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, bail, ensure};
use indoc::writedoc;
use openapiv3::Schema;

use crate::component_catalog::ComponentCatalog;
use crate::crates::Crate;
use crate::item_catalog::TypeRoot;
use crate::output::GeneratedWorkspace;
use crate::render::RenderContext;
use crate::rust_object::{ObjectUsage, RustObject};
use crate::rust_type::RustType;
use crate::spec_inference::Inference;
use crate::templates::ObjectWriter;
use crate::templates::object_writer::{write_derives_line, write_redacted_debug_impl};
use crate::templates::utils::write_doc_comment;
use crate::types::RustIdent;

pub fn write_generated_for_webhooks(
    context: RenderContext<'_>,
    webhooks: &[WebhookObject],
    output: &mut GeneratedWorkspace,
) -> anyhow::Result<()> {
    let base = Path::new("async-stripe-webhook");
    write_event_object(context, webhooks, base, output)?;
    Ok(())
}

fn write_event_object(
    context: RenderContext<'_>,
    webhooks: &[WebhookObject],
    out_path: &Path,
    output: &mut GeneratedWorkspace,
) -> anyhow::Result<()> {
    let mut out = String::new();
    let mut enum_body = String::new();
    let mut match_inner = String::new();
    let mut match_inner_serde = String::new();
    for webhook_obj in webhooks {
        let root = TypeRoot::Webhook(webhook_obj.event_type.clone());
        let payload_root = context
            .root(&root)
            .with_context(|| format!("webhook payload root was not lowered: {root:?}"))?;
        let payload = &payload_root.typ;
        let item = payload_root.declaration;
        let ident = item
            .map(|item| context.item(item).metadata.ident.clone())
            .unwrap_or_else(|| RustIdent::create(&webhook_obj.event_type));

        let (printable, feature_gate) = if let Some(enum_objs) = item
            .and_then(|item| context.item(item).object())
            .and_then(|object| object.enum_of_objects.as_ref())
        {
            ObjectWriter::new(context, &ident, ObjectUsage::type_def())
                .write_enum_of_objects(&mut out, enum_objs);
            (ident.to_string(), None)
        } else {
            let component = context
                .component_for_type(payload)
                .context("expected webhook data type to be enum of objects or component path")?;
            let belonging_crate = context.types_crate(component);
            (
                context.display_type(payload).to_string(),
                if belonging_crate != Crate::SHARED {
                    Some(belonging_crate.crate_name())
                } else {
                    None
                },
            )
        };

        let comment = write_doc_comment(&webhook_obj.doc, 1);
        let _ = write!(enum_body, "{comment}");
        if let Some(gate) = &feature_gate {
            let _ = writeln!(enum_body, r#"#[cfg(feature = "{gate}")]"#);
        }
        let _ = writeln!(enum_body, "{ident}(Box<{printable}>),");

        if let Some(gate) = &feature_gate {
            let _ = writeln!(match_inner, r#"#[cfg(feature = "{gate}")]"#);
            let _ = writeln!(match_inner_serde, r#"#[cfg(feature = "{gate}")]"#);
        }
        let evt_type = &webhook_obj.event_type;
        let _ = writeln!(
            match_inner,
            r#"if typ == "{evt_type}" {{ return parse_and_box(data).map(Self::{ident}); }}"#
        );
        let _ = writeln!(
            match_inner_serde,
            r#"if typ == "{evt_type}" {{ return parse_and_box(data).map(Self::{ident}); }}"#
        );
    }
    let _ = writedoc! {enum_body, r#"
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json")
    )]
    Unknown(miniserde::json::Value),
    "#};

    write_derives_line(&mut out, Default::default());
    let _ = writedoc! {out, r#"
    #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
    #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    #[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(untagged))]
    #[non_exhaustive]
    /// The event data for a webhook event.
    pub enum EventObject {{
        {enum_body}
    }}
    "#};
    write_redacted_debug_impl(&mut out, "EventObject", "");

    let _ = writedoc! {out, r#"
    impl EventObject {{
        #[inline(never)]
        pub(crate) fn from_raw_data(typ: &str, data: miniserde::json::Value) -> Option<Self> {{
            use stripe_types::miniserde_helpers::FromValueOpt;

            // Helper to avoid stack allocation for each branch
            #[inline(always)]
            fn parse_and_box<T: FromValueOpt>(data: miniserde::json::Value) -> Option<Box<T>> {{
                FromValueOpt::from_value(data).map(Box::new)
            }}

            {match_inner}

            Some(Self::Unknown(data))
        }}

        #[cfg(feature = "deserialize")]
        #[inline(never)]
        pub(crate) fn from_json_value(typ: &str, data: serde_json::Value) -> Result<Self, String> {{
            // Helper to avoid stack allocation for each branch
            #[inline(always)]
            fn parse_and_box<T: serde::de::DeserializeOwned>(data: serde_json::Value) -> Result<Box<T>, String> {{
                use serde::de::IntoDeserializer;
                let deserializer = data.into_deserializer();
                #[cfg(feature = "detailed-errors")]
                {{
                serde_path_to_error::deserialize(deserializer).map(Box::new).map_err(|e| e.to_string())
                }}
                #[cfg(not(feature = "detailed-errors"))]
                {{
                T::deserialize(deserializer).map(Box::new).map_err(|e| e.to_string())
                }}
            }}

            {match_inner_serde}

            // Unknown event type - error instead of silently accepting
            Err(format!("unknown event type '{{typ}}'"))
        }}
    }}
    "#};

    output.append(out_path.join("mod.rs"), out)?;
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WebhookObject {
    pub event_type: String,
    pub doc: String,
    pub typ: RustType,
}

impl WebhookObject {
    pub fn is_webhook_schema(schema: &Schema) -> bool {
        schema.schema_data.extensions.contains_key("x-stripeEvent")
    }

    pub fn from_schema(
        schema: &Schema,
        catalog: &ComponentCatalog,
    ) -> anyhow::Result<Option<Self>> {
        let Some(event_info) = schema.schema_data.extensions.get("x-stripeEvent") else {
            return Ok(None);
        };
        let event_type =
            event_info.get("type").context("no type")?.as_str().context("expected string")?;
        let desc = schema.schema_data.description.as_ref().context("expected description")?;

        let ident = RustIdent::create(event_type);
        let infer_ctx = Inference::new(&ident, catalog);
        let typ = infer_ctx.required(true).infer_schema_type(schema);

        Ok(Some(Self {
            event_type: event_type.to_string(),
            doc: desc.to_string(),
            typ: extract_object_type(typ)?,
        }))
    }
}

/// Extract and validate the type of the object field
fn extract_object_type(typ: RustType) -> anyhow::Result<RustType> {
    let (obj, _) = typ.into_object().context("expected object")?;

    let RustObject::Struct(struct_) = obj else {
        bail!("expected struct");
    };
    ensure!(struct_.fields.len() == 1);

    let object_field =
        struct_.fields.into_iter().find(|f| f.field_name == "object").context("no object field")?;
    Ok(object_field.rust_type)
}
