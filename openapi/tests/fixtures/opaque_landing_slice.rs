#[cfg(test)]
mod tests {
    use stripe_shared::{Charge, Customer, CustomerId};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{ExpandableValue, ObjectDecodeError};

    fn fixture(name: &str) -> serde_json::Value {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../openapi/fixtures.json");
        let fixtures: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        fixtures["resources"][name].clone()
    }

    fn charge_with(customer: serde_json::Value) -> String {
        let mut charge = fixture("charge");
        charge["customer"] = customer;
        serde_json::to_string(&charge).unwrap()
    }

    // This optional adapter owns its Customer DTO dependency. Opaque storage
    // itself is parameterized by CustomerId and retains JSON independently.
    trait DecodeCustomer {
        fn into_customer(self) -> Result<Option<Customer>, ObjectDecodeError>;
    }

    impl DecodeCustomer for ExpandableValue<CustomerId> {
        fn into_customer(self) -> Result<Option<Customer>, ObjectDecodeError> {
            let Some(object) = self.into_object() else { return Ok(None) };
            if object.object_type() != "customer" {
                return Err(ObjectDecodeError::UnexpectedObjectType {
                    expected: "customer",
                    actual: object.object_type().into(),
                });
            }
            Customer::from_value(object.into_value())
                .map(Some)
                .ok_or(ObjectDecodeError::InvalidObject { object_type: "customer" })
        }
    }

    #[test]
    fn string_ids_and_null_work_in_both_decoders() {
        let json = charge_with(serde_json::json!("cus_probe"));
        let mini: Charge = miniserde::json::from_str(&json).unwrap();
        let serde: Charge = serde_json::from_str(&json).unwrap();
        for charge in [mini, serde] {
            let value = charge.customer.unwrap();
            assert_eq!(value.id().to_string(), "cus_probe");
            assert!(!value.is_object());
            assert_eq!(serde_json::to_value(&value).unwrap(), "cus_probe");
            assert!(value.into_customer().unwrap().is_none());
        }
        let json = charge_with(serde_json::Value::Null);
        assert!(miniserde::json::from_str::<Charge>(&json).unwrap().customer.is_none());
        assert!(serde_json::from_str::<Charge>(&json).unwrap().customer.is_none());
    }

    #[test]
    fn expanded_json_and_unknown_fields_survive_until_optional_decoding() {
        let mut payload = fixture("customer");
        payload["future_field"] = serde_json::json!({"kept": [1, true, "yes"]});
        let json = charge_with(payload.clone());
        let mini: Charge = miniserde::json::from_str(&json).unwrap();
        let serde: Charge = serde_json::from_str(&json).unwrap();
        for charge in [mini, serde] {
            let value = charge.customer.unwrap();
            assert!(value.is_object());
            assert_eq!(serde_json::to_value(&value).unwrap(), payload);
            let decoded = value.into_customer().unwrap().unwrap();
            assert_eq!(decoded.id.to_string(), payload["id"].as_str().unwrap());
        }
    }

    #[test]
    fn unknown_tags_are_retained_and_typed_adapter_checks_the_tag() {
        let payload =
            serde_json::json!({"id":"cus_future", "object":"future_customer", "extra":42});
        let charge: Charge = miniserde::json::from_str(&charge_with(payload.clone())).unwrap();
        let value = charge.customer.unwrap();
        assert_eq!(serde_json::to_value(&value).unwrap(), payload);
        assert!(matches!(
            value.into_customer(),
            Err(ObjectDecodeError::UnexpectedObjectType { .. })
        ));
    }

    #[test]
    fn malformed_expanded_values_are_rejected() {
        for payload in [
            serde_json::json!({"id": 42, "object": "customer"}),
            serde_json::json!({"object": "customer"}),
            serde_json::json!({"id": "cus_bad"}),
        ] {
            let json = charge_with(payload);
            assert!(miniserde::json::from_str::<Charge>(&json).is_err());
            assert!(serde_json::from_str::<Charge>(&json).is_err());
        }
    }
}
