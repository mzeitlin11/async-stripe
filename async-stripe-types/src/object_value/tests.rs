use miniserde::json::Value;
use smol_str::SmolStr;

use super::{ExpandableValue, ObjectDecodeError, ObjectType, ObjectValue};
use crate::FromCursor;
use crate::miniserde_helpers::FromValueOpt;

crate::def_id!(TestId);
crate::def_object_value!(PaymentSource);

#[derive(Debug, miniserde::Deserialize)]
struct Charge {
    id: TestId,
    amount: i64,
}

impl ObjectType for Charge {
    const OBJECT_TYPE: &'static str = "charge";
}

impl FromValueOpt for Charge {
    fn from_value(value: Value) -> Option<Self> {
        let Value::Object(mut object) = value else { return None };
        Some(Self {
            id: TestId::from_value(object.remove("id")?)?,
            amount: i64::from_value(object.remove("amount")?)?,
        })
    }
}

#[derive(Debug)]
struct Customer;

impl ObjectType for Customer {
    const OBJECT_TYPE: &'static str = "customer";
}

impl FromValueOpt for Customer {
    fn from_value(_: Value) -> Option<Self> {
        panic!("a wrong discriminator must fail before invoking the model decoder")
    }
}

fn value(json: &str) -> Value {
    miniserde::json::from_str(json).unwrap()
}

#[test]
fn id_only_values_do_not_need_a_model() {
    let reference: ExpandableValue<TestId> = miniserde::json::from_str(r#""ch_123""#).unwrap();
    assert_eq!(reference.id().as_str(), "ch_123");
    assert!(!reference.is_object());
    assert!(reference.as_object().is_none());
    assert_eq!(serde_json::to_string(&reference).unwrap(), r#""ch_123""#);
    assert!(reference.into_expanded_as::<Charge>().unwrap().is_none());
}

#[test]
fn expanded_objects_keep_typed_ids_and_decode_on_demand() {
    let reference: ExpandableValue<TestId> =
        miniserde::json::from_str(r#"{"id":"ch_123","object":"charge","amount":120}"#).unwrap();
    assert_eq!(reference.id().as_str(), "ch_123");
    assert!(reference.is_object());
    assert_eq!(reference.as_object().unwrap().object_type(), "charge");
    let charge = reference.into_expanded_as::<Charge>().unwrap().unwrap();
    assert_eq!(charge.id.as_str(), "ch_123");
    assert_eq!(charge.amount, 120);
}

#[test]
fn polymorphic_ids_do_not_require_the_models_id_newtype() {
    let reference: ExpandableValue<SmolStr> =
        miniserde::json::from_str(r#"{"id":"ch_123","object":"charge","amount":120}"#).unwrap();
    assert_eq!(reference.id(), "ch_123");
    assert_eq!(reference.into_expanded_as::<Charge>().unwrap().unwrap().amount, 120);
}

#[test]
fn named_opaque_objects_keep_unknown_variants_and_support_typed_views() {
    let known: PaymentSource =
        miniserde::json::from_str(r#"{"id":"ch_123","object":"charge","amount":120}"#).unwrap();
    assert_eq!(known.id(), "ch_123");
    assert_eq!(known.object_type(), "charge");
    assert_eq!(known.into_typed::<Charge>().unwrap().amount, 120);
    let future: PaymentSource = miniserde::json::from_str(
        r#"{"id":"future_123","object":"future_source","additional":true}"#,
    )
    .unwrap();
    assert_eq!(future.object_type(), "future_source");
    assert!(matches!(future.as_value(), Value::Object(_)));
    assert_eq!(crate::Object::into_id(future.clone()), "future_123");
    assert!(matches!(future.clone().into_value(), Value::Object(_)));
    assert_eq!(serde_json::to_value(&future).unwrap()["additional"], true);
    assert!(miniserde::json::from_str::<PaymentSource>(r#"{"object":"future"}"#).is_err());
}

#[cfg(feature = "deserialize")]
#[test]
fn named_opaque_objects_support_serde_decoding() {
    let source: PaymentSource =
        serde_json::from_str(r#"{"id":"ch_123","object":"charge","amount":120}"#).unwrap();
    assert_eq!(source.into_typed::<Charge>().unwrap().amount, 120);
}

#[test]
fn unknown_types_and_fields_round_trip_without_a_decoder() {
    let json = r#"{"id":"future_123","object":"future_source","extra":{"nested":[true,null,18446744073709551615]}}"#;
    let reference: ExpandableValue<SmolStr> = miniserde::json::from_str(json).unwrap();
    let object = reference.as_object().unwrap();
    assert_eq!(object.object_type(), "future_source");
    assert_eq!(object.id(), "future_123");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&serde_json::to_string(&reference).unwrap())
            .unwrap(),
        serde_json::from_str::<serde_json::Value>(json).unwrap()
    );
    assert!(matches!(
        reference.into_expanded_as::<Charge>(),
        Err(ObjectDecodeError::UnexpectedObjectType { expected: "charge", actual })
            if actual == "future_source"
    ));
}

#[test]
fn decoding_checks_the_discriminator_before_the_model() {
    let object =
        ObjectValue::try_from_value(value(r#"{"id":"ch_123","object":"charge","amount":120}"#))
            .unwrap();
    assert!(matches!(
        object.into_typed::<Customer>(),
        Err(ObjectDecodeError::UnexpectedObjectType { expected: "customer", actual })
            if actual == "charge"
    ));
}

#[test]
fn malformed_known_models_fail_only_when_typed_decoding_is_requested() {
    let reference: ExpandableValue<TestId> =
        miniserde::json::from_str(r#"{"id":"ch_123","object":"charge","amount":"invalid"}"#)
            .unwrap();
    assert_eq!(reference.id().as_str(), "ch_123");
    assert!(matches!(
        reference.into_expanded_as::<Charge>(),
        Err(ObjectDecodeError::InvalidObject { object_type: "charge" })
    ));
}

#[test]
fn deferred_decoding_rejects_integer_overflow_like_eager_decoding() {
    for amount in [
        "-9223372036854775808",
        "9223372036854775807",
        "9223372036854775808",
        "18446744073709551615",
    ] {
        let json = format!(r#"{{"id":"ch_123","object":"charge","amount":{amount}}}"#);
        let eager = miniserde::json::from_str::<Charge>(&json);
        let deferred = ObjectValue::try_from_value(value(&json)).unwrap().into_typed::<Charge>();
        assert_eq!(eager.is_ok(), deferred.is_ok(), "{amount}");
        if let (Ok(eager), Ok(deferred)) = (eager, deferred) {
            assert_eq!(eager.amount, deferred.amount);
        }
    }
}

#[test]
fn optional_value_decoding_distinguishes_null_from_invalid_values() {
    for json in ["null", "0", "9223372036854775807", "9223372036854775808", "\"invalid\"", "{}"] {
        let eager = miniserde::json::from_str::<Option<i64>>(json).ok();
        let deferred = Option::<i64>::from_value(value(json));
        assert_eq!(eager, deferred, "{json}");
    }
}

#[test]
fn malformed_opaque_objects_are_rejected_by_both_entry_points() {
    for json in [
        "null",
        "[]",
        "123",
        r#"{"object":"charge"}"#,
        r#"{"id":null,"object":"charge"}"#,
        r#"{"id":123,"object":"charge"}"#,
        r#"{"id":"ch_123"}"#,
        r#"{"id":"ch_123","object":123}"#,
    ] {
        assert!(ObjectValue::try_from_value(value(json)).is_err(), "{json}");
        assert!(miniserde::json::from_str::<ObjectValue>(json).is_err(), "{json}");
        assert!(miniserde::json::from_str::<ExpandableValue<TestId>>(json).is_err(), "{json}");
    }
}

#[test]
fn optional_ids_preserve_missing_null_and_present_wire_values() {
    for json in [
        r#"{"object":"invoice","future":{"number":18446744073709551615}}"#,
        r#"{"id":null,"object":"invoice","future":{"number":18446744073709551615}}"#,
        r#"{"id":"in_123","object":"invoice","future":{"number":18446744073709551615}}"#,
    ] {
        let expected = serde_json::from_str::<serde_json::Value>(json).unwrap();
        let expected_id = expected.get("id").and_then(|id| id.as_str());
        let reference: ExpandableValue<Option<TestId>> = miniserde::json::from_str(json).unwrap();
        assert_eq!(reference.id().as_ref().map(TestId::as_str), expected_id);
        assert!(reference.is_object());
        assert_eq!(reference.as_object().unwrap().id(), reference.id());
        assert_eq!(reference.clone().into_id(), *reference.id());
        assert_eq!(serde_json::to_value(&reference).unwrap(), expected);
        let object: ObjectValue<Option<TestId>> = reference.into_object().unwrap();
        assert_eq!(object.id().as_ref().map(TestId::as_str), expected_id);
        assert_eq!(serde_json::to_value(&object).unwrap(), expected);

        let direct: ObjectValue<Option<TestId>> = miniserde::json::from_str(json).unwrap();
        assert_eq!(direct.id(), object.id());
        if expected_id.is_none() {
            assert!(ObjectValue::try_from_value(value(json)).is_err());
            assert!(miniserde::json::from_str::<ExpandableValue<TestId>>(json).is_err());
        }

        #[cfg(feature = "deserialize")]
        {
            let reference: ExpandableValue<Option<TestId>> = serde_json::from_str(json).unwrap();
            let direct: ObjectValue<Option<TestId>> = serde_json::from_str(json).unwrap();
            assert_eq!(reference.id(), object.id());
            assert_eq!(direct.id(), object.id());
            assert_eq!(serde_json::to_value(&reference).unwrap(), expected);
            assert_eq!(serde_json::to_value(&direct).unwrap(), expected);
            if expected_id.is_none() {
                assert!(serde_json::from_str::<ObjectValue>(json).is_err());
                assert!(serde_json::from_str::<ExpandableValue<TestId>>(json).is_err());
            }
        }
    }
}

#[test]
fn optional_ids_reject_malformed_ids_and_still_require_object_tags() {
    for json in [
        r#"{"id":123,"object":"invoice"}"#,
        r#"{"id":false,"object":"invoice"}"#,
        r#"{"id":[],"object":"invoice"}"#,
        r#"{"id":{},"object":"invoice"}"#,
        r#"{}"#,
        r#"{"id":null}"#,
        r#"{"object":null}"#,
        r#"{"object":123}"#,
        r#"{"id":"in_123","object":false}"#,
        "null",
        "[]",
    ] {
        assert!(ObjectValue::<Option<TestId>>::from_value(value(json)).is_none(), "{json}");
        assert!(ExpandableValue::<Option<TestId>>::try_from_value(value(json)).is_err(), "{json}");
        assert!(miniserde::json::from_str::<ObjectValue<Option<TestId>>>(json).is_err(), "{json}");
        assert!(
            miniserde::json::from_str::<ExpandableValue<Option<TestId>>>(json).is_err(),
            "{json}"
        );
        #[cfg(feature = "deserialize")]
        {
            assert!(serde_json::from_str::<ObjectValue<Option<TestId>>>(json).is_err(), "{json}");
            assert!(
                serde_json::from_str::<ExpandableValue<Option<TestId>>>(json).is_err(),
                "{json}"
            );
        }
    }
}

#[test]
fn cached_ids_can_be_borrowed_and_moved_without_clone_or_cursor_bounds() {
    struct PlainId(u32);
    let reference = ExpandableValue::from_id(PlainId(17));
    assert_eq!(reference.id().0, 17);
    assert_eq!(reference.into_id().0, 17);

    struct NonCloneId(String);
    impl FromCursor for NonCloneId {
        fn from_cursor(value: &str) -> Option<Self> {
            Some(Self(value.to_string()))
        }
    }
    let reference = ExpandableValue::from_id(NonCloneId("in_123".to_string()));
    assert_eq!(reference.id().0, "in_123");
    assert_eq!(reference.into_id().0, "in_123");
    let reference = ExpandableValue::<NonCloneId>::try_from_value(value(
        r#"{"id":"in_123","object":"invoice"}"#,
    ))
    .unwrap();
    assert_eq!(reference.id().0, "in_123");
    assert_eq!(reference.as_object().unwrap().id().0, "in_123");
    assert_eq!(reference.into_id().0, "in_123");
}

#[test]
fn id_conversion_errors_are_not_ignored_for_expanded_payloads() {
    #[derive(Debug)]
    struct ChargeOnlyId;
    impl FromCursor for ChargeOnlyId {
        fn from_cursor(id: &str) -> Option<Self> {
            id.starts_with("ch_").then_some(Self)
        }
    }
    for json in [r#""cus_123""#, r#"{"id":"cus_123","object":"customer"}"#] {
        assert!(matches!(
            ExpandableValue::<ChargeOnlyId>::try_from_value(value(json)),
            Err(ObjectDecodeError::InvalidId)
        ));
        assert!(miniserde::json::from_str::<ExpandableValue<ChargeOnlyId>>(json).is_err());
    }
}

#[test]
fn opaque_values_expose_the_complete_payload_without_mutating_identity() {
    let object: ObjectValue =
        miniserde::json::from_str(r#"{"id":"ch_123","object":"charge","amount":120}"#).unwrap();
    let Value::Object(raw) = object.as_value() else { panic!() };
    assert!(raw.contains_key("amount"));
    assert_eq!(object.id(), "ch_123");
    assert!(matches!(object.into_value(), Value::Object(_)));
}

#[cfg(feature = "deserialize")]
#[test]
fn serde_and_miniserde_agree_and_preserve_large_integer_payloads() {
    for json in [
        r#""ch_123""#,
        r#"{"id":"ch_123","object":"charge","amount":120}"#,
        r#"{"id":"future_123","object":"future","value":18446744073709551615}"#,
    ] {
        let serde: ExpandableValue<SmolStr> = serde_json::from_str(json).unwrap();
        let mini: ExpandableValue<SmolStr> = miniserde::json::from_str(json).unwrap();
        assert_eq!(serde.id(), mini.id());
        assert_eq!(serde.is_object(), mini.is_object());
        assert_eq!(serde_json::to_value(&serde).unwrap(), serde_json::to_value(&mini).unwrap());
        assert_eq!(
            serde_json::to_value(&serde).unwrap(),
            serde_json::from_str::<serde_json::Value>(json).unwrap()
        );
    }
    for json in ["null", "123", r#"{"object":"charge"}"#, r#"{"id":"ch_123"}"#] {
        assert!(serde_json::from_str::<ExpandableValue<SmolStr>>(json).is_err(), "{json}");
        assert!(serde_json::from_str::<ObjectValue>(json).is_err(), "{json}");
    }
}

#[cfg(feature = "redact-generated-debug")]
#[test]
fn object_debug_respects_redaction() {
    let object = ObjectValue::try_from_value(value(
        r#"{"id":"ch_123","object":"charge","secret":"sensitive"}"#,
    ))
    .unwrap();
    let debug = format!("{object:?}");
    assert!(!debug.contains("sensitive"));
    assert!(!debug.contains("ch_123"));

    for json in [r#""ch_123""#, r#"{"id":"ch_123","object":"charge","secret":"sensitive"}"#] {
        let reference: ExpandableValue<SmolStr> = miniserde::json::from_str(json).unwrap();
        let debug = format!("{reference:?}");
        assert!(!debug.contains("ch_123"));
        assert!(!debug.contains("sensitive"));
    }
}
