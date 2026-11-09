use miniserde::json::Value;
use serde::Serialize;

use super::{LazyExpandable, LazyExpandableError};
use crate::miniserde_helpers::FromValueOpt;
use crate::{AsCursor, FromCursor, Object};

// Deliberately not Clone: borrowing a decoded view only clones the JSON.
#[derive(Debug, Eq, PartialEq, Serialize)]
struct Id(String);

impl FromCursor for Id {
    fn from_cursor(value: &str) -> Option<Self> {
        (!value.is_empty()).then(|| Self(value.to_owned()))
    }
}

impl AsCursor for Id {
    fn as_cursor(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
struct Resource {
    id: Id,
    amount: i64,
}

impl Object for Resource {
    type Id = Id;

    fn id(&self) -> &Id {
        &self.id
    }

    fn into_id(self) -> Id {
        self.id
    }
}

impl FromValueOpt for Resource {
    fn from_value(value: Value) -> Option<Self> {
        let Value::Object(mut object) = value else { return None };
        let Value::String(id) = object.remove("id")? else { return None };
        Some(Self { id: Id::from_cursor(&id)?, amount: i64::from_value(object.remove("amount")?)? })
    }
}

fn parse(json: &str) -> LazyExpandable<Id> {
    miniserde::json::from_str(json).unwrap()
}

#[test]
fn ids_are_available_without_constructing_a_model() {
    for json in [r#""obj_123""#, r#"{"id":"obj_123","future":{"value":true}}"#] {
        let reference = parse(json);
        assert_eq!(reference.id().0, "obj_123");
        assert_eq!(reference.is_object(), json.starts_with('{'));
        assert_eq!(reference.into_id().0, "obj_123");
    }
}

#[test]
fn id_only_is_distinct_from_an_invalid_expanded_model() {
    assert!(parse(r#""obj_123""#).into_expanded_as::<Resource>().unwrap().is_none());
    let reference = parse(r#"{"id":"obj_123","amount":true}"#);
    assert_eq!(reference.id().0, "obj_123");
    assert!(matches!(
        reference.to_expanded_as::<Resource>(),
        Err(LazyExpandableError::InvalidObject)
    ));
    assert!(matches!(
        reference.into_expanded_as::<Resource>(),
        Err(LazyExpandableError::InvalidObject)
    ));
}

#[test]
fn typed_decoding_uses_the_existing_model_rules_including_tagless_resources() {
    for json in
        [r#"{"id":"obj_123","amount":100}"#, r#"{"id":"obj_123","object":"resource","amount":100}"#]
    {
        let reference = parse(json);
        let first = reference.to_expanded_as::<Resource>().unwrap().unwrap();
        let second = reference.to_expanded_as::<Resource>().unwrap().unwrap();
        assert_eq!(first.id.0, "obj_123");
        assert_eq!(first.amount, second.amount);
        assert_eq!(reference.into_expanded_as::<Resource>().unwrap().unwrap().amount, 100);
    }
}

#[test]
fn json_shape_and_required_identity_validate_immediately() {
    for json in [
        "null",
        "true",
        "17",
        "[]",
        "{}",
        r#"{"id":null}"#,
        r#"{"id":17}"#,
        r#"{"id":""}"#,
        r#""""#,
    ] {
        assert!(miniserde::json::from_str::<LazyExpandable<Id>>(json).is_err(), "{json}");
        assert!(
            LazyExpandable::<Id>::from_value(miniserde::json::from_str(json).unwrap()).is_none(),
            "{json}"
        );
    }
}

#[test]
fn optional_identity_accepts_missing_and_null_but_rejects_invalid_strings() {
    for json in ["{}", r#"{"id":null}"#] {
        let reference: LazyExpandable<Option<Id>> = miniserde::json::from_str(json).unwrap();
        assert!(reference.id().is_none());
        assert!(reference.is_object());
    }
    for json in [r#""obj_123""#, r#"{"id":"obj_123"}"#] {
        let reference: LazyExpandable<Option<Id>> = miniserde::json::from_str(json).unwrap();
        assert_eq!(reference.id().as_ref().unwrap().0, "obj_123");
    }
    for json in [r#""""#, r#"{"id":""}"#, r#"{"id":17}"#] {
        assert!(miniserde::json::from_str::<LazyExpandable<Option<Id>>>(json).is_err());
    }
}

#[test]
fn serialization_preserves_future_objects_and_large_integer_values() {
    let json = r#"{"id":"obj_123","object":"future","new":[18446744073709551615,-9223372036854775808,null,true]}"#;
    let reference = parse(json);
    let expected: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(serde_json::to_value(&reference).unwrap(), expected);
    let value = reference.into_object().unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&miniserde::json::to_string(&value)).unwrap(),
        expected
    );
    assert_eq!(
        serde_json::to_string(&LazyExpandable::from_id(Id("obj_123".into()))).unwrap(),
        r#""obj_123""#
    );
}

#[test]
fn debug_respects_generated_redaction() {
    let reference = parse(r#"{"id":"obj_private","secret":"retained secret"}"#);
    let output = format!("{reference:?}");
    assert_eq!(output.contains("obj_private"), !cfg!(feature = "redact-generated-debug"));
    assert_eq!(output.contains("retained secret"), !cfg!(feature = "redact-generated-debug"));
}

#[cfg(feature = "deserialize")]
#[test]
fn serde_and_miniserde_agree_on_lazy_payloads_and_identity_validation() {
    for json in [
        r#""obj_123""#,
        r#"{"id":"obj_123","amount":false}"#,
        r#"{"id":"obj_123","object":"future","value":18446744073709551615}"#,
    ] {
        let mini = parse(json);
        let serde: LazyExpandable<Id> = serde_json::from_str(json).unwrap();
        assert_eq!(mini.id(), serde.id());
        assert_eq!(serde_json::to_value(mini).unwrap(), serde_json::to_value(serde).unwrap());
    }
    for json in ["null", "[]", "{}", r#"{"id":null}"#, r#"{"id":17}"#, r#"{"id":""}"#] {
        assert!(serde_json::from_str::<LazyExpandable<Id>>(json).is_err(), "{json}");
    }
    for json in ["{}", r#"{"id":null}"#] {
        let reference: LazyExpandable<Option<Id>> = serde_json::from_str(json).unwrap();
        assert!(reference.id().is_none());
    }
    assert!(serde_json::from_str::<LazyExpandable<Option<Id>>>(r#"{"id":""}"#).is_err());
}
