use miniserde::json::Value;

use super::{RecordDecodeError, RecordValue};
use crate::ObjectValue;
use crate::miniserde_helpers::FromValueOpt;

crate::def_record_value!(PaymentMethodDetails);

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

impl PaymentMethodDetails {
    fn card(&self) -> Result<Option<Card>, RecordDecodeError> {
        self.field_as("card")
    }
}

fn value(json: &str) -> Value {
    miniserde::json::from_str(json).unwrap()
}

#[test]
fn a_record_needs_a_type_without_needing_an_object_identity() {
    let json = r#"{"type":"card","card":{"last4":"4242"}}"#;
    let record: RecordValue = miniserde::json::from_str(json).unwrap();
    assert_eq!(record.type_(), "card");
    assert_eq!(record.field_as::<Card>("card").unwrap().unwrap().last4, "4242");
    // Generalizing the JSON collector must preserve ObjectValue's envelope.
    assert!(ObjectValue::try_from_value(value(json)).is_err());
    assert!(miniserde::json::from_str::<ObjectValue>(json).is_err());
}

#[test]
fn unknown_types_and_fields_round_trip_without_a_decoder() {
    let json = r#"{"type":"future_method","future_method":{"counter":18446744073709551615},"new_field":[true,null,"retained"]}"#;
    let record: PaymentMethodDetails = miniserde::json::from_str(json).unwrap();
    assert_eq!(record.type_(), "future_method");
    assert_eq!(record.card().unwrap(), None);
    let serialized = serde_json::to_string(&record).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
        serde_json::from_str::<serde_json::Value>(json).unwrap()
    );
    assert_eq!(
        miniserde::json::to_string(&record.into_value()),
        miniserde::json::to_string(&value(json))
    );
}

#[test]
fn named_accessors_borrow_the_record_and_leave_its_payload_unchanged() {
    let json =
        r#"{"type":"card","card":{"last4":"4242"},"ach_debit":"malformed","new_field":true}"#;
    let record: PaymentMethodDetails = miniserde::json::from_str(json).unwrap();
    let mut first = record.card().unwrap().unwrap();
    first.last4 = "9999".to_string();
    assert_eq!(first.last4, "9999");
    assert_eq!(record.card().unwrap().unwrap().last4, "4242");
    assert_eq!(record.type_(), "card");
    assert_eq!(
        miniserde::json::to_string(record.as_value()),
        miniserde::json::to_string(&value(json))
    );
    assert_eq!(
        record.field_as::<Card>("ach_debit"),
        Err(RecordDecodeError::InvalidField { field: "ach_debit".into() })
    );
}

#[test]
fn accessors_follow_field_presence_independently_of_the_type_tag() {
    for json in
        [r#"{"type":"card"}"#, r#"{"type":"card","card":null}"#, r#"{"type":"future_method"}"#]
    {
        let record: PaymentMethodDetails = miniserde::json::from_str(json).unwrap();
        assert_eq!(record.card().unwrap(), None);
    }
    let record: PaymentMethodDetails =
        miniserde::json::from_str(r#"{"type":"future_method","card":{"last4":"4242"}}"#).unwrap();
    assert_eq!(record.card().unwrap().unwrap().last4, "4242");
}

#[test]
fn malformed_fields_fail_when_accessed_without_consuming_the_record() {
    for field in ["false", "1", "[]", "{}", r#"{"last4":false}"#] {
        let json = format!(r#"{{"type":"card","card":{field}}}"#);
        let record: PaymentMethodDetails = miniserde::json::from_str(&json).unwrap();
        assert_eq!(record.card(), Err(RecordDecodeError::InvalidField { field: "card".into() }));
        assert_eq!(record.type_(), "card");
        assert_eq!(
            miniserde::json::to_string(record.as_value()),
            miniserde::json::to_string(&value(&json))
        );
    }
}

#[test]
fn record_envelopes_are_validated_at_both_entry_points() {
    for (json, expected) in [
        ("null", RecordDecodeError::NotObject),
        ("[]", RecordDecodeError::NotObject),
        ("1", RecordDecodeError::NotObject),
        ("true", RecordDecodeError::NotObject),
        (r#""card""#, RecordDecodeError::NotObject),
        ("{}", RecordDecodeError::MissingType),
        (r#"{"type":null}"#, RecordDecodeError::MissingType),
        (r#"{"type":false}"#, RecordDecodeError::MissingType),
        (r#"{"id":"pm_123","object":"payment_method"}"#, RecordDecodeError::MissingType),
    ] {
        assert_eq!(RecordValue::try_from_value(value(json)).unwrap_err(), expected, "{json}");
        assert!(miniserde::json::from_str::<RecordValue>(json).is_err(), "{json}");
        assert!(miniserde::json::from_str::<PaymentMethodDetails>(json).is_err(), "{json}");
        assert!(
            <PaymentMethodDetails as FromValueOpt>::from_value(value(json)).is_none(),
            "{json}"
        );
    }
}

#[test]
fn selected_field_validation_matches_the_existing_value_decoder() {
    for json in [r#"{"last4":"4242"}"#, r#"{"last4":false}"#, r#"{}"#, r#"[]"#] {
        let record =
            RecordValue::try_from_value(value(&format!(r#"{{"type":"card","card":{json}}}"#)))
                .unwrap();
        let eager = miniserde::json::from_str::<Card>(json);
        assert_eq!(record.field_as::<Card>("card").ok().flatten(), eager.ok(), "{json}");
    }
}

#[cfg(feature = "deserialize")]
#[test]
fn serde_and_miniserde_preserve_the_same_record_and_defer_the_same_field_errors() {
    for json in [
        r#"{"type":"card","card":{"last4":"4242"}}"#,
        r#"{"type":"card","card":false}"#,
        r#"{"type":"card","card":null}"#,
        r#"{"type":"future_method","future_method":{"counter":18446744073709551615}}"#,
    ] {
        let serde: PaymentMethodDetails = serde_json::from_str(json).unwrap();
        let mini: PaymentMethodDetails = miniserde::json::from_str(json).unwrap();
        assert_eq!(serde.type_(), mini.type_());
        assert_eq!(serde.card(), mini.card());
        assert_eq!(
            miniserde::json::to_string(serde.as_value()),
            miniserde::json::to_string(mini.as_value())
        );
    }
    for json in ["null", "[]", "{}", r#"{"type":1}"#] {
        assert!(serde_json::from_str::<RecordValue>(json).is_err());
        assert!(serde_json::from_str::<PaymentMethodDetails>(json).is_err());
    }
}

#[cfg(feature = "redact-generated-debug")]
#[test]
fn record_and_nominal_debug_output_respect_redaction() {
    let json = r#"{"type":"sensitive_type","card":{"last4":"4242"},"secret":"sensitive_payload"}"#;
    let record: RecordValue = miniserde::json::from_str(json).unwrap();
    let named: PaymentMethodDetails = miniserde::json::from_str(json).unwrap();
    for debug in [format!("{record:?}"), format!("{named:?}")] {
        assert!(!debug.contains("sensitive"));
        assert!(!debug.contains("4242"));
    }
}

#[test]
fn dynamically_selected_field_names_need_not_outlive_the_call() {
    let record: PaymentMethodDetails =
        miniserde::json::from_str(r#"{"type":"future","card":{"last4":"4242"},"invalid":false}"#)
            .unwrap();
    let field = ["ca", "rd"].concat();
    assert_eq!(record.field_as::<Card>(&field).unwrap().unwrap().last4, "4242");
    let error = {
        let field = String::from("invalid");
        record.field_as::<Card>(&field).unwrap_err()
    };
    assert_eq!(error, RecordDecodeError::InvalidField { field: "invalid".into() });
    assert_eq!(record.field_as::<Card>(&String::from("missing")).unwrap(), None);
    assert_eq!(record.card().unwrap().unwrap().last4, "4242");
}
