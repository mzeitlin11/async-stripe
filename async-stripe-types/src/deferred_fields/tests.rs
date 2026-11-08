use miniserde::json::Value;
use serde::Serialize;
use serde::ser::SerializeMap;

use super::DeferredFields;
use crate::RecordDecodeError;
use crate::miniserde_helpers::{FromValueOpt, ObjectBuilderInner};

#[derive(Debug, Eq, PartialEq, miniserde::Deserialize)]
struct Card {
    last4: String,
}

impl FromValueOpt for Card {
    fn from_value(value: Value) -> Option<Self> {
        let Value::Object(mut object) = value else { return None };
        Some(Self { last4: String::from_value(object.remove("last4")?)? })
    }
}

#[derive(Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
struct MixedRecord {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(flatten)]
    fields: DeferredFields,
}

impl FromValueOpt for MixedRecord {
    fn from_value(value: Value) -> Option<Self> {
        let Value::Object(mut object) = value else { return None };
        let id = String::from_value(object.remove("id")?)?;
        let type_ = String::from_value(object.remove("type")?)?;
        // Generated tagged serializers synthesize this marker separately.
        object.remove("object");
        Some(Self { id, type_, fields: DeferredFields::from_object(object) })
    }
}

impl miniserde::Deserialize for MixedRecord {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MixedRecord> {
    fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {
        Ok(crate::miniserde_helpers::object_value_map(&mut self.out))
    }
}

struct TaggedRecord<'a>(&'a MixedRecord);

impl Serialize for TaggedRecord<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.0.id)?;
        map.serialize_entry("type", &self.0.type_)?;
        self.0.fields.serialize_entries(&mut map)?;
        map.serialize_entry("object", "mixed")?;
        map.end()
    }
}

fn value(json: &str) -> Value {
    miniserde::json::from_str(json).unwrap()
}

#[test]
fn a_field_map_requires_no_discriminator_or_identity() {
    let fields: DeferredFields = miniserde::json::from_str(r#"{"card":{"last4":"4242"}}"#).unwrap();
    assert_eq!(fields.field_as::<Card>("card").unwrap().unwrap().last4, "4242");
    assert_eq!(serde_json::to_string(&DeferredFields::default()).unwrap(), "{}");
    assert_eq!(DeferredFields::default().field_as::<Card>("card").unwrap(), None);
    let raw = r#"{"id":false,"type":null,"object":123}"#;
    let fields: DeferredFields = miniserde::json::from_str(raw).unwrap();
    assert_eq!(
        serde_json::to_value(&fields).unwrap(),
        serde_json::from_str::<serde_json::Value>(raw).unwrap()
    );
}

#[test]
fn common_fields_are_typed_and_unknown_fields_are_retained() {
    let json = r#"{"id":"pm_example","type":"card","card":{"last4":"4242"},"future":{"counter":18446744073709551615}}"#;
    let record: MixedRecord = miniserde::json::from_str(json).unwrap();
    assert_eq!(record.id, "pm_example");
    assert_eq!(record.type_, "card");
    let Value::Object(fields) = record.fields.as_value() else { panic!() };
    assert!(!fields.contains_key("id"));
    assert!(!fields.contains_key("type"));
    assert!(fields.contains_key("future"));
    assert_eq!(
        serde_json::to_value(&record).unwrap(),
        serde_json::from_str::<serde_json::Value>(json).unwrap()
    );
}

#[test]
fn changing_a_common_field_serializes_its_current_value_once() {
    let mut record: MixedRecord = miniserde::json::from_str(
        r#"{"id":"pm_example","type":"card","card":{"last4":"4242"},"unknown":true}"#,
    )
    .unwrap();
    record.id = "pm_changed".into();
    let serialized = serde_json::to_string(&record).unwrap();
    assert_eq!(serialized.matches("\"id\"").count(), 1);
    let round_trip: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(round_trip["id"], "pm_changed");
    assert_eq!(round_trip["card"]["last4"], "4242");
    assert_eq!(round_trip["unknown"], true);
}

#[test]
fn custom_tagged_serialization_appends_entries_without_a_stale_marker() {
    let json = r#"{"id":"pm_example","type":"card","object":"mixed","card":{"last4":"4242"},"future":[null,18446744073709551615]}"#;
    let record: MixedRecord = miniserde::json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&TaggedRecord(&record)).unwrap();
    assert_eq!(serialized.matches("\"object\"").count(), 1);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
        serde_json::from_str::<serde_json::Value>(json).unwrap()
    );
}

#[test]
fn invalid_payloads_fail_only_on_access_and_remain_available_after_failure() {
    for json in ["false", "1", "{}", r#"{"last4":false}"#] {
        let record: MixedRecord = miniserde::json::from_str(&format!(
            r#"{{"id":"pm_example","type":"card","card":{json}}}"#
        ))
        .unwrap();
        assert_eq!(
            record.fields.field_as::<Card>("card"),
            Err(RecordDecodeError::InvalidField { field: "card".into() })
        );
        assert_eq!(record.id, "pm_example");
        assert!(matches!(record.fields.as_value(), Value::Object(_)));
    }
    for json in [r#"{"type":"card"}"#, r#"{"id":false,"type":"card"}"#] {
        assert!(miniserde::json::from_str::<MixedRecord>(json).is_err());
    }
}

#[test]
fn borrowed_access_is_repeatable_and_missing_or_null_fields_are_absent() {
    let fields: DeferredFields =
        miniserde::json::from_str(r#"{"card":{"last4":"4242"},"empty":null}"#).unwrap();
    let mut decoded = fields.field_as::<Card>("card").unwrap().unwrap();
    decoded.last4 = "9999".into();
    assert_eq!(decoded.last4, "9999");
    assert_eq!(fields.field_as::<Card>("card").unwrap().unwrap().last4, "4242");
    assert_eq!(fields.field_as::<Card>("empty").unwrap(), None);
    assert_eq!(fields.field_as::<Card>("missing").unwrap(), None);
}

#[test]
fn maps_reject_non_objects_at_all_entry_points() {
    for json in ["null", "[]", "false", "1", r#""string""#] {
        assert!(matches!(
            DeferredFields::try_from_value(value(json)),
            Err(RecordDecodeError::NotObject)
        ));
        assert!(miniserde::json::from_str::<DeferredFields>(json).is_err());
        assert!(DeferredFields::from_value(value(json)).is_none());
    }
}

#[test]
fn raw_object_take_finishes_the_pending_field_and_clears_the_builder() {
    let mut builder = ObjectBuilderInner::default();
    builder.key_inner("first").unwrap().string("retained").unwrap();
    builder.key_inner("last").unwrap().boolean(true).unwrap();
    let object = builder.take_object();
    assert!(matches!(object.get("first"), Some(Value::String(value)) if value == "retained"));
    assert!(matches!(object.get("last"), Some(Value::Bool(true))));
    assert!(builder.take_object().is_empty());
}

#[cfg(feature = "deserialize")]
#[test]
fn serde_flatten_and_miniserde_keep_the_same_unknown_fields() {
    for json in [
        r#"{"id":"pm_example","type":"card"}"#,
        r#"{"id":"pm_example","type":"card","card":{"last4":"4242"}}"#,
        r#"{"id":"pm_example","type":"future","card":false,"new_field":{"counter":18446744073709551615}}"#,
    ] {
        let serde: MixedRecord = serde_json::from_str(json).unwrap();
        let mini: MixedRecord = miniserde::json::from_str(json).unwrap();
        assert_eq!(serde.id, mini.id);
        assert_eq!(serde.type_, mini.type_);
        assert_eq!(serde.fields.field_as::<Card>("card"), mini.fields.field_as::<Card>("card"));
        assert_eq!(
            miniserde::json::to_string(serde.fields.as_value()),
            miniserde::json::to_string(mini.fields.as_value())
        );
        assert_eq!(serde_json::to_value(&serde).unwrap(), serde_json::to_value(&mini).unwrap());
    }
    for json in ["null", "[]", "1"] {
        assert!(serde_json::from_str::<DeferredFields>(json).is_err());
    }
    for json in ["null", "[]", r#"{"id":false,"type":"card"}"#] {
        assert!(serde_json::from_str::<MixedRecord>(json).is_err());
    }
}

#[cfg(feature = "redact-generated-debug")]
#[test]
fn debug_respects_redaction() {
    let fields: DeferredFields =
        miniserde::json::from_str(r#"{"secret":"sensitive","card":{"last4":"4242"}}"#).unwrap();
    let debug = format!("{fields:?}");
    assert!(!debug.contains("sensitive"));
    assert!(!debug.contains("4242"));
}

#[test]
fn dynamic_field_names_are_owned_only_by_decoding_errors() {
    let fields: DeferredFields =
        miniserde::json::from_str(r#"{"card":{"last4":"4242"},"invalid":false,"empty":null}"#)
            .unwrap();
    let field = ["ca", "rd"].concat();
    assert_eq!(fields.field_as::<Card>(&field).unwrap().unwrap().last4, "4242");
    let error = {
        let field = String::from("invalid");
        fields.field_as::<Card>(&field).unwrap_err()
    };
    assert_eq!(error, RecordDecodeError::InvalidField { field: "invalid".into() });
    for field in [String::from("missing"), String::from("empty")] {
        assert_eq!(fields.field_as::<Card>(&field).unwrap(), None);
    }
    assert_eq!(fields.field_as::<Card>("card").unwrap().unwrap().last4, "4242");
}
