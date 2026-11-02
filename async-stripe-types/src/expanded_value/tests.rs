use miniserde::json::Value;

use super::ExpandedValue;
use crate::miniserde_helpers::FromValueOpt;
use crate::{ExpandableValue, FromCursor, ObjectDecodeError, ObjectValue};

crate::def_id!(RuleId);

#[derive(Debug, Eq, PartialEq, miniserde::Deserialize)]
struct Rule {
    id: RuleId,
    action: String,
    predicate: String,
}

// A tagless model intentionally has no ObjectType implementation.
impl FromValueOpt for Rule {
    fn from_value(value: Value) -> Option<Self> {
        let Value::Object(mut object) = value else { return None };
        Some(Self {
            id: RuleId::from_value(object.remove("id")?)?,
            action: String::from_value(object.remove("action")?)?,
            predicate: String::from_value(object.remove("predicate")?)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct StrictRuleId(String);

impl FromCursor for StrictRuleId {
    fn from_cursor(value: &str) -> Option<Self> {
        value.starts_with("r_").then(|| Self(value.to_string()))
    }
}

fn value(json: &str) -> Value {
    miniserde::json::from_str(json).unwrap()
}

#[test]
fn id_only_references_keep_their_type_and_do_not_require_a_model() {
    let reference: ExpandedValue<RuleId> = miniserde::json::from_str(r#""r_123""#).unwrap();
    let id: &RuleId = reference.id();
    assert_eq!(id.as_str(), "r_123");
    assert!(!reference.is_object());
    assert!(reference.as_object().is_none());
    assert_eq!(serde_json::to_string(&reference).unwrap(), r#""r_123""#);
    assert_eq!(reference.into_expanded_as::<Rule>().unwrap(), None);
}

#[test]
fn expanded_tagless_resources_decode_without_an_object_discriminator() {
    let json = r#"{"id":"r_123","action":"block","predicate":"true","future":{"counter":18446744073709551615}}"#;
    let reference: ExpandedValue<RuleId> = miniserde::json::from_str(json).unwrap();
    assert!(reference.is_object());
    assert_eq!(reference.id().as_str(), "r_123");
    assert_eq!(
        serde_json::to_value(&reference).unwrap(),
        serde_json::from_str::<serde_json::Value>(json).unwrap()
    );
    assert_eq!(
        miniserde::json::to_string(reference.as_object().unwrap()),
        miniserde::json::to_string(&value(json))
    );
    let eager: Rule = miniserde::json::from_str(json).unwrap();
    assert_eq!(reference.into_expanded_as::<Rule>().unwrap().unwrap(), eager);
}

#[test]
fn tagless_payload_validation_is_deferred_and_matches_the_existing_decoder() {
    for json in [
        r#"{"id":"r_123"}"#,
        r#"{"id":"r_123","action":false,"predicate":"true"}"#,
        r#"{"id":"r_123","action":"block","predicate":1}"#,
    ] {
        assert!(miniserde::json::from_str::<Rule>(json).is_err());
        let reference: ExpandedValue<RuleId> = miniserde::json::from_str(json).unwrap();
        assert_eq!(reference.id().as_str(), "r_123");
        assert!(matches!(
            reference.into_expanded_as::<Rule>(),
            Err(ObjectDecodeError::InvalidUntaggedObject)
        ));
    }
}

#[test]
fn tagged_object_envelopes_still_require_their_discriminator() {
    let json = r#"{"id":"r_123","action":"block","predicate":"true"}"#;
    assert!(ExpandedValue::<RuleId>::try_from_value(value(json)).is_ok());
    assert!(matches!(
        ObjectValue::try_from_value(value(json)),
        Err(ObjectDecodeError::MissingObjectType)
    ));
    assert!(matches!(
        ExpandableValue::<RuleId>::try_from_value(value(json)),
        Err(ObjectDecodeError::MissingObjectType)
    ));
    assert!(miniserde::json::from_str::<ObjectValue>(json).is_err());
    assert!(miniserde::json::from_str::<ExpandableValue<RuleId>>(json).is_err());
}

#[test]
fn untagged_objects_preserve_unknown_fields_without_interpreting_them() {
    let json =
        r#"{"id":"r_123","action":"block","predicate":"true","object":false,"unknown":[1,null]}"#;
    let reference: ExpandedValue<RuleId> = miniserde::json::from_str(json).unwrap();
    let raw = reference.clone().into_object().unwrap();
    assert_eq!(miniserde::json::to_string(&raw), miniserde::json::to_string(&value(json)));
    assert_eq!(reference.clone().into_id().as_str(), "r_123");
    assert_eq!(reference.into_expanded_as::<Rule>().unwrap().unwrap().action, "block");
}

#[test]
fn invalid_envelopes_and_typed_ids_fail_before_model_decoding() {
    for json in ["null", "[]", "true", "1", "{}", r#"{"id":false}"#, r#"{"id":null}"#] {
        assert!(ExpandedValue::<RuleId>::try_from_value(value(json)).is_err(), "{json}");
        assert!(miniserde::json::from_str::<ExpandedValue<RuleId>>(json).is_err(), "{json}");
    }
    for json in [r#""not_a_rule""#, r#"{"id":"not_a_rule"}"#] {
        assert!(matches!(
            ExpandedValue::<StrictRuleId>::try_from_value(value(json)),
            Err(ObjectDecodeError::InvalidId)
        ));
        assert!(miniserde::json::from_str::<ExpandedValue<StrictRuleId>>(json).is_err());
    }
}

#[test]
fn optional_ids_distinguish_missing_values_from_rejected_strings() {
    for json in ["{}", r#"{"id":null}"#] {
        let reference: ExpandedValue<Option<StrictRuleId>> =
            miniserde::json::from_str(json).unwrap();
        assert_eq!(reference.id(), &None);
        assert!(reference.is_object());
        assert_eq!(
            miniserde::json::to_string(&reference.into_object().unwrap()),
            miniserde::json::to_string(&value(json))
        );
    }
    for json in [r#""not_a_rule""#, r#"{"id":"not_a_rule"}"#] {
        assert!(matches!(
            ExpandedValue::<Option<StrictRuleId>>::try_from_value(value(json)),
            Err(ObjectDecodeError::InvalidId)
        ));
        assert!(miniserde::json::from_str::<ExpandedValue<Option<StrictRuleId>>>(json).is_err());
    }
    assert_eq!(Option::<StrictRuleId>::from_cursor("not_a_rule"), None);
    assert_eq!(Option::<StrictRuleId>::from_missing_id(), Some(None));
    assert!(
        ObjectValue::<Option<StrictRuleId>>::from_value(value(
            r#"{"id":"not_a_rule","object":"rule"}"#
        ))
        .is_none()
    );
    assert!(
        miniserde::json::from_str::<ObjectValue<Option<StrictRuleId>>>(
            r#"{"id":"not_a_rule","object":"rule"}"#
        )
        .is_err()
    );
    assert!(
        miniserde::json::from_str::<ExpandableValue<Option<StrictRuleId>>>(r#""not_a_rule""#)
            .is_err()
    );
}

#[cfg(feature = "deserialize")]
#[test]
fn serde_and_miniserde_preserve_the_same_tagless_references() {
    for json in [
        r#""r_123""#,
        r#"{"id":"r_123","action":"block","predicate":"true","unknown":18446744073709551615}"#,
        r#"{"id":"r_123","action":false}"#,
    ] {
        let serde: ExpandedValue<RuleId> = serde_json::from_str(json).unwrap();
        let mini: ExpandedValue<RuleId> = miniserde::json::from_str(json).unwrap();
        assert_eq!(serde.id(), mini.id());
        assert_eq!(serde_json::to_value(&serde).unwrap(), serde_json::to_value(&mini).unwrap());
        assert_eq!(serde.into_expanded_as::<Rule>(), mini.into_expanded_as::<Rule>());
    }
    for json in ["null", "[]", "{}", r#"{"id":false}"#] {
        assert!(serde_json::from_str::<ExpandedValue<RuleId>>(json).is_err());
    }
    assert!(
        serde_json::from_str::<ExpandedValue<Option<StrictRuleId>>>(r#""not_a_rule""#).is_err()
    );
}

#[cfg(feature = "redact-generated-debug")]
#[test]
fn id_and_expanded_debug_output_respect_redaction() {
    for json in [r#""r_sensitive""#, r#"{"id":"r_sensitive","action":"sensitive"}"#] {
        let reference: ExpandedValue<RuleId> = miniserde::json::from_str(json).unwrap();
        assert!(!format!("{reference:?}").contains("sensitive"));
    }
}
