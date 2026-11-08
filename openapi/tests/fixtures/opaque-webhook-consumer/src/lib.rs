#[cfg(test)]
mod tests {
    use serde_json::{Value, json};
    use stripe_webhook::{Event, EventObject, Webhook};

    fn payload(event_type: &str, object: Value) -> Value {
        json!({
            "id": "evt_example",
            "object": "event",
            "api_version": "2017-05-25",
            "created": 1700000000,
            "livemode": false,
            "pending_webhooks": 1,
            "type": event_type,
            "data": {"object": object}
        })
    }

    fn charge() -> Value {
        serde_json::from_str(include_str!("../../opaque-consumer/charge.json")).unwrap()
    }

    fn assert_raw(event: &Event, expected: &Value) {
        let EventObject::Unknown(raw) = &event.data.object else {
            panic!("expected an opaque event payload");
        };
        let preserved: Value = serde_json::from_str(&miniserde::json::to_string(raw)).unwrap();
        assert_eq!(&preserved, expected);
    }

    #[cfg(not(feature = "charge"))]
    #[test]
    fn disabled_charge_model_preserves_its_payload() {
        let object = charge();
        let payload = payload("charge.succeeded", object.clone());
        let event = Webhook::insecure(&payload.to_string()).unwrap();
        assert_eq!(event.type_.as_str(), "charge.succeeded");
        assert_raw(&event, &object);

        #[cfg(feature = "deserialize")]
        {
            let event: Event = serde_json::from_value(payload).unwrap();
            assert_eq!(event.type_.as_str(), "charge.succeeded");
            assert_raw(&event, &object);
        }
    }

    #[cfg(not(feature = "charge"))]
    #[test]
    fn disabled_charge_model_does_not_validate_the_opaque_payload() {
        let mut object = charge();
        object.as_object_mut().unwrap().remove("amount");
        let payload = payload("charge.succeeded", object.clone());
        let event = Webhook::insecure(&payload.to_string()).unwrap();
        assert_raw(&event, &object);

        #[cfg(feature = "deserialize")]
        {
            let event: Event = serde_json::from_value(payload).unwrap();
            assert_raw(&event, &object);
        }
    }

    #[cfg(feature = "customer")]
    #[test]
    fn customer_model_decodes_without_its_endpoints_return_models() {
        let object = json!({
            "object": "customer", "id": "cus_model_only", "created": 1700000000,
            "default_source": null, "description": null, "email": null,
            "livemode": false, "shipping": null
        });
        let payload = payload("customer.created", object);
        let event = Webhook::insecure(&payload.to_string()).unwrap();
        let EventObject::CustomerCreated(customer) = event.data.object else {
            panic!("expected a typed Customer payload");
        };
        assert_eq!(customer.id.as_str(), "cus_model_only");

        #[cfg(feature = "deserialize")]
        {
            let event: Event = serde_json::from_value(payload).unwrap();
            let EventObject::CustomerCreated(customer) = event.data.object else {
                panic!("expected a typed Customer payload");
            };
            assert_eq!(customer.created, 1700000000);
        }
    }

    #[cfg(feature = "charge")]
    fn assert_charge(event: &Event) {
        let EventObject::ChargeSucceeded(charge) = &event.data.object else {
            panic!("expected a typed charge payload");
        };
        assert_eq!(charge.id.as_str(), "ch_example");
        assert_eq!(charge.amount, 1000);
    }

    #[cfg(feature = "charge")]
    #[test]
    fn enabled_charge_model_decodes_the_payload() {
        let payload = payload("charge.succeeded", charge());
        let event = Webhook::insecure(&payload.to_string()).unwrap();
        assert_charge(&event);

        #[cfg(feature = "deserialize")]
        {
            let event: Event = serde_json::from_value(payload).unwrap();
            assert_charge(&event);
        }
    }

    #[cfg(feature = "charge")]
    #[test]
    fn enabled_charge_model_rejects_malformed_payloads() {
        let mut object = charge();
        object.as_object_mut().unwrap().remove("amount");
        let payload = payload("charge.succeeded", object);
        assert!(Webhook::insecure(&payload.to_string()).is_err());

        #[cfg(feature = "deserialize")]
        assert!(serde_json::from_value::<Event>(payload).is_err());
    }

    #[test]
    fn unknown_events_preserve_nested_data_and_integer_extremes() {
        let object = json!({
            "nested": [null, false, {"unsigned": u64::MAX, "signed": i64::MIN}],
            "future": {"field": "preserved"}
        });
        let payload = payload("future_resource.created", object.clone());
        let event = Webhook::insecure(&payload.to_string()).unwrap();
        assert_eq!(event.type_.as_str(), "future_resource.created");
        assert_raw(&event, &object);

        #[cfg(feature = "deserialize")]
        {
            let event: Event = serde_json::from_value(payload).unwrap();
            assert_eq!(event.type_.as_str(), "future_resource.created");
            assert_raw(&event, &object);
        }
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn opaque_events_roundtrip_without_losing_wire_data() {
        let cases = [
            ("future_resource.created", json!({"unsigned": u64::MAX, "nested": [true, null]})),
            #[cfg(not(feature = "charge"))]
            ("charge.succeeded", charge()),
        ];
        for (event_type, object) in cases {
            let payload = payload(event_type, object.clone());
            let event = Webhook::insecure(&payload.to_string()).unwrap();
            let serialized = serde_json::to_value(&event).unwrap();
            assert_eq!(serialized, payload);

            #[cfg(feature = "deserialize")]
            {
                let reparsed: Event = serde_json::from_value(serialized).unwrap();
                assert_eq!(reparsed.type_.as_str(), event_type);
                assert_raw(&reparsed, &object);
            }
        }
    }
}
