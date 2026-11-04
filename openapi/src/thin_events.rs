//! Thin event delivery metadata is separate from resource namespaces and layout.
use std::collections::BTreeMap;
use std::fmt::Write as _;

use anyhow::{Context, ensure};
use openapiv3::Schema;

use crate::codegen::CodeGen;
use crate::component_catalog::ComponentId;
use crate::crates::Crate;
use crate::output::GeneratedWorkspace;
use crate::spec::{as_enum_strings, as_object_enum_name, as_object_properties};

pub(crate) fn is_thin_event_schema(schema: &Schema) -> bool {
    schema
        .schema_data
        .extensions
        .get("x-stripeEvent")
        .is_some_and(|event| event.get("kind").and_then(serde_json::Value::as_str) == Some("thin"))
}

#[derive(Debug, Clone)]
pub(crate) struct ThinEvent {
    pub component: ComponentId,
    event_type: String,
    related_object_type: Option<String>,
}

impl ThinEvent {
    pub(crate) fn from_schema(
        component: ComponentId,
        schema: &Schema,
    ) -> anyhow::Result<Option<Self>> {
        if !is_thin_event_schema(schema) {
            return Ok(None);
        }
        let event_type = schema.schema_data.extensions["x-stripeEvent"]["type"]
            .as_str()
            .context("thin event has no type")?
            .to_owned();
        ensure!(
            as_object_enum_name(schema).as_deref() == Some("v2.core.event"),
            "thin event must have the v2.core.event object tag: {event_type}"
        );
        let fields = as_object_properties(schema).context("thin event must be an object")?;
        let type_values =
            fields.get("type").and_then(|typ| typ.as_item()).and_then(|typ| as_enum_strings(typ));
        ensure!(
            type_values.as_deref() == Some(std::slice::from_ref(&event_type)),
            "thin event type field disagrees with x-stripeEvent: {event_type}"
        );
        let related_object_type = fields
            .get("related_object")
            .and_then(|related| related.as_item())
            .and_then(|related| as_object_properties(related))
            .and_then(|fields| fields.get("type"))
            .and_then(|tag| tag.as_item())
            .and_then(|tag| as_enum_strings(tag))
            .and_then(|tags| if tags.len() == 1 { tags.into_iter().next() } else { None });
        Ok(Some(Self { component, event_type, related_object_type }))
    }
}

pub(crate) fn write(codegen: &CodeGen, output: &mut GeneratedWorkspace) -> anyhow::Result<()> {
    if !codegen.spec.is_v2_experiment() {
        return Ok(());
    }
    let mut groups = BTreeMap::<Crate, Vec<&ThinEvent>>::new();
    for event in &codegen.components.thin_events {
        groups
            .entry(codegen.output_locations().placement(event.component).types_crate())
            .or_default()
            .push(event);
    }
    for (krate, events) in groups {
        ensure!(krate != Crate::SHARED, "assign thin events to an explicit experimental crate");
        let mut out = String::new();
        for event in &events {
            let ident = codegen.components.get_by_id(event.component).ident();
            let related = match &event.related_object_type {
                Some(tag) => format!("Some({tag:?})"),
                None => "None".into(),
            };
            writeln!(
                out,
                "impl stripe_client_core::ThinEvent for {ident} {{\nconst EVENT_TYPE: &'static str = {:?};\nconst API_VERSION: &'static str = {:?};\nconst RELATED_OBJECT_TYPE: Option<&'static str> = {related};\n}}",
                event.event_type,
                codegen.spec.version()
            )?;
            writeln!(
                out,
                "pub type {ident}Notification = stripe_client_core::TypedEventNotification<{ident}>;"
            )?;
        }
        out.push_str("#[derive(Clone, Debug)]\n#[non_exhaustive]\npub enum Notification {\n");
        for event in &events {
            let ident = codegen.components.get_by_id(event.component).ident();
            writeln!(out, "{ident}({ident}Notification),")?;
        }
        out.push_str("Unknown(stripe_client_core::EventNotification),\n}\nimpl Notification {\npub fn from_notification(notification: stripe_client_core::EventNotification) -> Result<Self, stripe_client_core::NotificationError> {\nmatch notification.event_type.as_str() {\n");
        for event in &events {
            let ident = codegen.components.get_by_id(event.component).ident();
            writeln!(
                out,
                "{:?} => notification.typed::<{ident}>().map(Self::{ident}),",
                event.event_type
            )?;
        }
        out.push_str("_ => Ok(Self::Unknown(notification)),\n}\n}\n}\n");
        output.append(krate.get_path_to_mod(), out)?;
    }
    Ok(())
}
