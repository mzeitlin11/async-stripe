#![cfg(any(
    feature = "payment_method",
    feature = "payment_intent",
    feature = "source",
    feature = "treasury_transaction"
))]

#[cfg(any(feature = "payment_method", feature = "source"))]
use serde_json::Value;
use serde_json::json;

#[cfg(feature = "payment_method")]
mod payment_methods {
    use super::*;

    fn resource() -> Value {
        serde_json::from_str(include_str!("../payment_method.json")).unwrap()
    }

    #[test]
    fn common_payment_method_fields_stay_typed_without_decoding_card_data() {
        let mut value = resource();
        value["card"] = json!(true);
        let method: stripe::PaymentMethod = miniserde::json::from_str(&value.to_string()).unwrap();
        let id: &stripe::PaymentMethodId = &method.id;
        assert_eq!(id.as_str(), value["id"].as_str().unwrap());
        assert_eq!(method.type_.as_str(), "card");
        assert_eq!(method.created, 1234567890);
        assert!(!method.livemode);
        assert_eq!(method.billing_details.email.as_deref(), Some("jenny@example.com"));
        assert_eq!(serde_json::to_value(method.variant_fields()).unwrap()["card"], true);

        #[cfg(feature = "payment_decoders")]
        assert!(matches!(
            method.card(),
            Err(stripe::RecordDecodeError::InvalidField { field: "card" })
        ));
        #[cfg(feature = "deserialize")]
        {
            let method: stripe::PaymentMethod = serde_json::from_value(value).unwrap();
            assert_eq!(method.created, 1234567890);
            assert_eq!(method.type_.as_str(), "card");
            assert_eq!(method.billing_details.email.as_deref(), Some("jenny@example.com"));
            #[cfg(feature = "payment_decoders")]
            assert!(matches!(
                method.card(),
                Err(stripe::RecordDecodeError::InvalidField { field: "card" })
            ));
        }
    }

    #[test]
    fn malformed_or_missing_common_fields_fail_during_payment_method_parsing() {
        for (field, replacement) in [
            ("id", Some(json!(17))),
            ("type", Some(json!(false))),
            ("created", Some(json!("invalid"))),
            ("billing_details", Some(json!(true))),
            ("created", None),
            ("billing_details", None),
        ] {
            let mut value = resource();
            if let Some(replacement) = replacement {
                value[field] = replacement;
            } else {
                value.as_object_mut().unwrap().remove(field);
            }
            let payload = value.to_string();
            assert!(
                miniserde::json::from_str::<stripe::PaymentMethod>(&payload).is_err(),
                "{field}"
            );
            #[cfg(feature = "deserialize")]
            assert!(serde_json::from_str::<stripe::PaymentMethod>(&payload).is_err(), "{field}");
        }
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn payment_method_serialization_keeps_unknown_data_and_uses_updated_common_fields() {
        let mut value = resource();
        value["type"] = json!("future_method");
        value["card"] = json!(true);
        value["future_method"] = json!({"nested":[true,null,u64::MAX]});
        value["future_common"] = json!({"counter":u64::MAX});
        let mut method: stripe::PaymentMethod =
            miniserde::json::from_str(&value.to_string()).unwrap();
        assert_eq!(method.type_.as_str(), "future_method");
        method.id = stripe::PaymentMethodId::from("pm_updated");
        method.livemode = true;
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized.matches(r#""object":"payment_method""#).count(), 1);
        let serialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(serialized["id"], "pm_updated");
        assert_eq!(serialized["livemode"], true);
        assert_eq!(serialized["object"], "payment_method");
        assert_eq!(serialized["type"], "future_method");
        for field in ["future_method", "future_common", "card"] {
            assert_eq!(serialized[field], value[field]);
        }
        #[cfg(feature = "deserialize")]
        {
            let reparsed: stripe::PaymentMethod = serde_json::from_value(serialized).unwrap();
            assert_eq!(reparsed.id.as_str(), "pm_updated");
            assert_eq!(reparsed.type_.as_str(), "future_method");
            assert_eq!(
                serde_json::to_value(reparsed.variant_fields()).unwrap()["future_method"],
                value["future_method"]
            );
        }
    }
}

#[cfg(feature = "source")]
mod sources {
    use super::*;

    fn resource() -> Value {
        serde_json::from_str(include_str!("../source.json")).unwrap()
    }

    #[test]
    fn source_defers_instrument_and_authentication_flow_payloads() {
        let mut value = resource();
        value["type"] = json!("card");
        value["flow"] = json!("redirect");
        value["card"] = json!(true);
        value["redirect"] = json!("malformed");
        let source: stripe::Source = miniserde::json::from_str(&value.to_string()).unwrap();
        let id: &stripe::SourceId = &source.id;
        assert_eq!(id.as_str(), value["id"].as_str().unwrap());
        assert_eq!(source.type_.as_str(), "card");
        assert_eq!(source.flow, "redirect");
        assert_eq!(source.created, 1234567890);
        assert_eq!(
            source.owner.as_ref().unwrap().email.as_deref(),
            Some("jenny.rosen@example.com")
        );
        let retained = serde_json::to_value(source.variant_fields()).unwrap();
        assert_eq!(retained["card"], true);
        assert_eq!(retained["redirect"], "malformed");
        #[cfg(feature = "core_decoders")]
        {
            assert!(matches!(
                source.card(),
                Err(stripe::RecordDecodeError::InvalidField { field: "card" })
            ));
            assert!(matches!(
                source.redirect(),
                Err(stripe::RecordDecodeError::InvalidField { field: "redirect" })
            ));
        }
        #[cfg(feature = "deserialize")]
        {
            let source: stripe::Source = serde_json::from_value(value).unwrap();
            assert_eq!(source.type_.as_str(), "card");
            assert_eq!(source.flow, "redirect");
            #[cfg(feature = "core_decoders")]
            {
                assert!(matches!(
                    source.card(),
                    Err(stripe::RecordDecodeError::InvalidField { field: "card" })
                ));
                assert!(matches!(
                    source.redirect(),
                    Err(stripe::RecordDecodeError::InvalidField { field: "redirect" })
                ));
            }
        }
    }

    #[test]
    fn malformed_or_missing_common_fields_fail_during_source_parsing() {
        for (field, replacement) in [
            ("id", Some(json!(17))),
            ("type", Some(json!(false))),
            ("flow", Some(json!({}))),
            ("owner", Some(json!(false))),
            ("created", None),
            ("flow", None),
        ] {
            let mut value = resource();
            if let Some(replacement) = replacement {
                value[field] = replacement;
            } else {
                value.as_object_mut().unwrap().remove(field);
            }
            let payload = value.to_string();
            assert!(miniserde::json::from_str::<stripe::Source>(&payload).is_err(), "{field}");
            #[cfg(feature = "deserialize")]
            assert!(serde_json::from_str::<stripe::Source>(&payload).is_err(), "{field}");
        }
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn source_serialization_keeps_unknown_data_and_uses_updated_common_fields() {
        let mut value = resource();
        value["type"] = json!("future_source");
        value["flow"] = json!("future_flow");
        value["future_source"] = json!({"nested":[null,u64::MAX]});
        value["future_common"] = json!([true,{"key":"preserved"}]);
        let mut source: stripe::Source = miniserde::json::from_str(&value.to_string()).unwrap();
        assert_eq!(source.type_.as_str(), "future_source");
        source.id = stripe::SourceId::from("src_updated");
        source.livemode = true;
        let serialized = serde_json::to_string(&source).unwrap();
        assert_eq!(serialized.matches(r#""object":"source""#).count(), 1);
        let serialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(serialized["id"], "src_updated");
        assert_eq!(serialized["livemode"], true);
        assert_eq!(serialized["object"], "source");
        assert_eq!(serialized["type"], "future_source");
        assert_eq!(serialized["flow"], "future_flow");
        for field in ["future_source", "future_common"] {
            assert_eq!(serialized[field], value[field]);
        }
        #[cfg(feature = "deserialize")]
        {
            let reparsed: stripe::Source = serde_json::from_value(serialized).unwrap();
            assert_eq!(reparsed.id.as_str(), "src_updated");
            assert_eq!(reparsed.flow, "future_flow");
            assert_eq!(
                serde_json::to_value(reparsed.variant_fields()).unwrap()["future_source"],
                value["future_source"]
            );
        }
    }
}

#[cfg(feature = "treasury_transaction")]
mod treasury_flows {
    use super::*;

    #[test]
    fn treasury_flows_keep_malformed_issuing_data_until_accessed() {
        let value = json!({
            "type":"issuing_authorization", "issuing_authorization":true,
            "future_flow":{"counter":u64::MAX}
        });
        let flow: stripe::TreasuryTransactionsResourceFlowDetails =
            miniserde::json::from_str(&value.to_string()).unwrap();
        assert_eq!(flow.type_.as_str(), "issuing_authorization");
        assert_eq!(
            serde_json::to_value(flow.variant_fields()).unwrap()["issuing_authorization"],
            true
        );
        #[cfg(feature = "issuing_decoders")]
        assert!(matches!(
            flow.issuing_authorization(),
            Err(stripe::RecordDecodeError::InvalidField { field: "issuing_authorization" })
        ));
        #[cfg(feature = "serialize")]
        assert_eq!(serde_json::to_value(&flow).unwrap(), value);
        #[cfg(feature = "deserialize")]
        {
            let flow: stripe::TreasuryTransactionsResourceFlowDetails =
                serde_json::from_value(value).unwrap();
            assert_eq!(flow.type_.as_str(), "issuing_authorization");
            #[cfg(feature = "issuing_decoders")]
            assert!(matches!(
                flow.issuing_authorization(),
                Err(stripe::RecordDecodeError::InvalidField { field: "issuing_authorization" })
            ));
        }
    }
}

#[cfg(feature = "payment_intent")]
mod option_bags {
    use super::json;

    #[test]
    fn response_options_defer_independent_methods_and_preserve_unknown_settings() {
        let value =
            json!({"card":{"request_three_d_secure":true},"future_method":{"setting":"kept"}});
        let options: stripe::PaymentIntentPaymentMethodOptions =
            miniserde::json::from_str(&value.to_string()).unwrap();
        assert_eq!(serde_json::to_value(options.variant_fields()).unwrap(), value);
        #[cfg(feature = "core_decoders")]
        assert!(matches!(
            options.card(),
            Err(stripe::RecordDecodeError::InvalidField { field: "card" })
        ));
        #[cfg(feature = "deserialize")]
        {
            let options: stripe::PaymentIntentPaymentMethodOptions =
                serde_json::from_value(value.clone()).unwrap();
            assert_eq!(serde_json::to_value(options.variant_fields()).unwrap(), value);
        }
        #[cfg(feature = "serialize")]
        assert_eq!(serde_json::to_value(&options).unwrap(), value);
    }
}
