#[cfg(test)]
mod mixed_records;

#[cfg(all(test, feature = "balance_transaction"))]
mod tests {
    use bytes::Bytes;
    use stripe::{BalanceTransaction, BalanceTransactionId, RetrieveBalanceTransaction};
    use stripe_client_core::{CustomizedStripeRequest, StripeBlockingClient, StripeMethod};

    struct FixtureClient(String);

    impl StripeBlockingClient for FixtureClient {
        type Err = stripe::StripeError;

        fn execute(&self, request: CustomizedStripeRequest) -> Result<Bytes, Self::Err> {
            let (request, _) = request.into_pieces();
            assert!(matches!(request.method, StripeMethod::Get));
            assert_eq!(request.path, "/balance_transactions/txn_example");
            assert!(request.body.is_none());
            Ok(Bytes::copy_from_slice(self.0.as_bytes()))
        }
    }

    fn retrieve(source: &str) -> BalanceTransaction {
        let response = include_str!("../balance_transaction.json").replace("\"SOURCE\"", source);
        let request = RetrieveBalanceTransaction::new(BalanceTransactionId::from("txn_example"));
        request.send_blocking(&FixtureClient(response)).unwrap()
    }

    #[test]
    fn retrieve_keeps_an_unexpanded_id_without_a_source_model() {
        let transaction = retrieve(r#""ch_example""#);
        let source = transaction.source.unwrap();
        assert_eq!(source.id().as_str(), "ch_example");
        assert!(!source.is_object());
    }

    #[test]
    fn retrieve_preserves_an_unknown_expanded_source() {
        let transaction =
            retrieve(r#"{"id":"future_example","object":"future_source","new_field":{"count":3}}"#);
        let source = transaction.source.unwrap();
        assert_eq!(source.id().as_str(), "future_example");
        let object = source.as_object().unwrap();
        assert_eq!(object.object_type(), "future_source");
        assert!(miniserde::json::to_string(object.as_value()).contains("new_field"));
    }

    #[test]
    fn a_named_polymorphic_source_does_not_require_its_target_models() {
        let source: stripe::BalanceTransactionSource = miniserde::json::from_str(
            r#"{"id":"future_example","object":"future_source","new_field":true}"#,
        )
        .unwrap();
        assert_eq!(source.id().as_str(), "future_example");
        assert_eq!(source.object_type(), "future_source");
        assert!(miniserde::json::to_string(source.as_value()).contains("new_field"));
    }

    #[test]
    fn error_details_preserve_resource_diagnostics_without_target_models() {
        let error: stripe::ApiErrors = miniserde::json::from_str(
            r#"{"type":"api_error","message":"example failure","payment_intent":{"id":"pi_example","object":"payment_intent","new_field":true}}"#,
        )
        .unwrap();
        assert_eq!(error.message.as_deref(), Some("example failure"));
        let diagnostic = error.payment_intent.unwrap();
        assert_eq!(diagnostic.id().as_str(), "pi_example");
        assert_eq!(diagnostic.object_type(), "payment_intent");
        assert!(miniserde::json::to_string(diagnostic.as_value()).contains("new_field"));
    }

    #[cfg(feature = "charge")]
    #[test]
    fn charge_feature_adds_typed_decoding_to_the_same_retrieval() {
        let transaction = retrieve(include_str!("../charge.json"));
        let charge =
            transaction.source.unwrap().into_expanded_as::<stripe::Charge>().unwrap().unwrap();
        let id: stripe::ChargeId = charge.id;
        assert_eq!(id.as_str(), "ch_example");
        assert_eq!(charge.amount, 1000);
    }

    #[cfg(feature = "charge")]
    #[test]
    fn charge_feature_adds_a_typed_view_of_the_named_polymorphic_source() {
        let source: stripe::BalanceTransactionSource =
            miniserde::json::from_str(include_str!("../charge.json")).unwrap();
        let charge = source.into_typed::<stripe::Charge>().unwrap();
        assert_eq!(charge.id.as_str(), "ch_example");
        assert_eq!(charge.amount, 1000);
    }

    #[cfg(feature = "charge")]
    #[test]
    fn id_only_is_distinct_from_a_malformed_expanded_charge() {
        assert!(
            retrieve(r#""ch_example""#)
                .source
                .unwrap()
                .into_expanded_as::<stripe::Charge>()
                .unwrap()
                .is_none()
        );
        assert!(
            retrieve(r#"{"id":"ch_example","object":"charge"}"#)
                .source
                .unwrap()
                .into_expanded_as::<stripe::Charge>()
                .is_err()
        );
    }

    #[cfg(feature = "charge")]
    #[test]
    fn enabling_charge_preserves_unknown_sources_and_rejects_the_wrong_decoder() {
        let transaction = retrieve(r#"{"id":"future_example","object":"future_source"}"#);
        let source = transaction.source.unwrap();
        assert_eq!(source.as_object().unwrap().object_type(), "future_source");
        assert!(source.into_expanded_as::<stripe::Charge>().is_err());
    }
}

#[cfg(all(test, feature = "charge"))]
mod charge_tests {
    #[test]
    fn an_untagged_expanded_rule_preserves_raw_data_without_its_model() {
        use serde_json::json;
        for value in [
            json!("rule_example"),
            json!({"id":"rule_example","action":"allow","predicate":":is_test:"}),
        ] {
            let expanded = value.is_object();
            let mut payload: serde_json::Value =
                serde_json::from_str(include_str!("../charge.json")).unwrap();
            payload["outcome"] = json!({"type":"authorized","advice_code":null,"network_advice_code":null,"network_decline_code":null,"network_status":"approved_by_network","reason":null,"risk_level":"normal","seller_message":"Payment complete.","rule":value});
            let charge: stripe::Charge = miniserde::json::from_str(&payload.to_string()).unwrap();
            let rule = charge.outcome.unwrap().rule.unwrap();
            assert_eq!(rule.id().as_str(), "rule_example");
            assert_eq!(rule.as_object().is_some(), expanded);
            if let Some(miniserde::json::Value::Object(object)) = rule.as_object() {
                assert!(
                    matches!(object.get("action"), Some(miniserde::json::Value::String(action)) if action == "allow")
                );
            }
            #[cfg(feature = "rule")]
            {
                let decoded = rule.into_expanded_as::<stripe::Rule>().unwrap();
                assert_eq!(decoded.is_some(), expanded);
                if let Some(rule) = decoded {
                    assert_eq!(rule.predicate, ":is_test:");
                }
            }
            #[cfg(feature = "deserialize")]
            {
                let charge: stripe::Charge = serde_json::from_value(payload).unwrap();
                let rule = charge.outcome.unwrap().rule.unwrap();
                assert_eq!(rule.id().as_str(), "rule_example");
                assert_eq!(rule.as_object().is_some(), expanded);
            }
        }
    }

    #[test]
    fn the_facade_supports_request_construction_without_another_model_group() {
        let _request = stripe::CreateCharge::new().amount(1000).currency(stripe::Currency::USD);
        let _address = stripe::charge::OptionalFieldsAddress::new();
        let source: stripe::ObjectValue =
            miniserde::json::from_str(include_str!("../charge.json")).unwrap();
        assert_eq!(source.into_typed::<stripe::Charge>().unwrap().amount, 1000);
    }

    #[test]
    fn deferred_generated_model_decoding_agrees_with_eager_validation() {
        use miniserde::json::Value;
        for (field, json) in [
            ("amount", "18446744073709551615"),
            ("application_fee_amount", "\"invalid\""),
            ("application_fee_amount", "null"),
            ("application_fee_amount", "100"),
            ("billing_details", r#"{"address":{"city":false}}"#),
        ] {
            let mut value: Value =
                miniserde::json::from_str(include_str!("../charge.json")).unwrap();
            let Value::Object(object) = &mut value else { panic!() };
            object.insert(field.into(), miniserde::json::from_str(json).unwrap());
            let eager =
                miniserde::json::from_str::<stripe::Charge>(&miniserde::json::to_string(&value));
            let deferred =
                stripe::ObjectValue::try_from_value(value).unwrap().into_typed::<stripe::Charge>();
            assert_eq!(eager.is_ok(), deferred.is_ok(), "{field}: {json}");
        }
    }
}

#[cfg(all(test, feature = "charge", feature = "invoice"))]
mod combined_group_tests {
    #[test]
    fn helpers_with_the_same_name_remain_addressable_by_resource() {
        let _charge = stripe::charge::OptionalFieldsAddress::new();
        let _invoice = stripe::invoice::OptionalFieldsAddress::new();
        let _request = stripe::CreateInvoice::new().currency(stripe::Currency::USD);
    }
}

#[cfg(test)]
mod reference_foundation_tests {
    #[test]
    fn a_lightweight_family_is_available_without_a_feature() {
        let source: stripe::BalanceTransactionSource =
            miniserde::json::from_str(r#"{"id":"future_123","object":"future_source"}"#).unwrap();
        assert_eq!(source.object_type(), "future_source");
    }
}

#[cfg(all(test, feature = "application"))]
mod application_tests {
    #[test]
    fn an_ordinary_model_only_group_provides_typed_decoding_and_ids() {
        let object: stripe::ObjectValue = miniserde::json::from_str(
            r#"{"id":"ca_example","object":"application","name":"Example"}"#,
        )
        .unwrap();
        let application = object.into_typed::<stripe::Application>().unwrap();
        let id: stripe::ApplicationId = application.id;
        assert_eq!(id.as_str(), "ca_example");
        assert_eq!(application.name.as_deref(), Some("Example"));
    }
}

#[cfg(all(test, feature = "customer", feature = "deleted_customer"))]
mod shared_discriminator_tests {
    #[test]
    fn a_shared_object_tag_preserves_existing_structural_decoding() {
        assert_eq!(
            <stripe::Customer as stripe::ObjectType>::OBJECT_TYPE,
            <stripe::DeletedCustomer as stripe::ObjectType>::OBJECT_TYPE,
        );
        let json = r#"{"id":"cus_example","object":"customer","created":1,"livemode":false,"deleted":true}"#;
        let object: stripe::ObjectValue = miniserde::json::from_str(json).unwrap();

        // The object tag identifies a wire family. It does not make its model
        // schemas disjoint: the ordinary model ignores the extra deleted field.
        let eager: stripe::Customer = miniserde::json::from_str(json).unwrap();
        let customer = object.clone().into_typed::<stripe::Customer>().unwrap();
        assert_eq!(customer.id, eager.id);
        assert_eq!(customer.created, eager.created);
        let deleted = object.into_typed::<stripe::DeletedCustomer>().unwrap();
        assert_eq!(deleted.id.as_str(), "cus_example");
    }

    #[test]
    fn the_deleted_model_still_requires_its_deleted_discriminator() {
        for json in [
            r#"{"id":"cus_example","object":"customer","created":1,"livemode":false}"#,
            r#"{"id":"cus_example","object":"customer","created":1,"livemode":false,"deleted":false}"#,
        ] {
            assert!(miniserde::json::from_str::<stripe::DeletedCustomer>(json).is_err());
            let object: stripe::ObjectValue = miniserde::json::from_str(json).unwrap();
            assert!(matches!(
                object.into_typed::<stripe::DeletedCustomer>(),
                Err(stripe::ObjectDecodeError::InvalidObject { object_type: "customer" })
            ));
        }
    }
}

#[cfg(all(test, feature = "customer"))]
mod maybe_deleted_tests {
    use stripe::customer::RetrieveCustomerReturned;

    fn assert_variant(family: &RetrieveCustomerReturned, deleted: bool) {
        if deleted {
            let RetrieveCustomerReturned::DeletedCustomer(customer) = family else {
                panic!("expected the deleted Customer variant: {family:?}");
            };
            // Enabling the response enables its deleted model transitively.
            let customer: &stripe::DeletedCustomer = customer;
            assert_eq!(customer.id.as_str(), "cus_example");
        } else {
            let RetrieveCustomerReturned::Customer(customer) = family else {
                panic!("expected the live Customer variant: {family:?}");
            };
            assert_eq!(customer.id.as_str(), "cus_example");
            assert_eq!(customer.created, 1);
        }
    }

    #[test]
    fn live_deleted_responses_decode_into_matchable_variants() {
        for (json, deleted) in [
            (r#"{"id":"cus_example","object":"customer","created":1,"livemode":false}"#, false),
            (r#"{"id":"cus_example","object":"customer","deleted":true}"#, true),
        ] {
            let family: RetrieveCustomerReturned = miniserde::json::from_str(json).unwrap();
            assert_variant(&family, deleted);
            #[cfg(feature = "deserialize")]
            {
                let family: RetrieveCustomerReturned = serde_json::from_str(json).unwrap();
                assert_variant(&family, deleted);
            }
            #[cfg(feature = "serialize")]
            {
                let wire = serde_json::to_value(&family).unwrap();
                assert_eq!(wire["id"], "cus_example");
                assert_eq!(wire["object"], "customer");
                assert_eq!(wire["deleted"].as_bool().unwrap_or(false), deleted);
                let reparsed = miniserde::json::from_str(&wire.to_string()).unwrap();
                assert_variant(&reparsed, deleted);
            }
        }
    }

    #[test]
    fn overlapping_shapes_keep_the_existing_enum_decoder_behavior() {
        // Restore the existing decoders as well as the enum's public shape.
        // Miniserde selects the deleted flag; serde's untagged enum tries
        // the live variant first when both schemas accept the payload.
        let json = r#"{"id":"cus_example","object":"customer","created":1,"livemode":false,"deleted":true}"#;
        let family: RetrieveCustomerReturned = miniserde::json::from_str(json).unwrap();
        assert_variant(&family, true);
        #[cfg(feature = "deserialize")]
        {
            let family: RetrieveCustomerReturned = serde_json::from_str(json).unwrap();
            assert_variant(&family, false);
        }
    }

    #[test]
    fn expanded_customers_decode_into_the_same_live_deleted_enum() {
        for (json, deleted) in [
            (r#"{"id":"cus_example","object":"customer","created":1,"livemode":false}"#, false),
            (r#"{"id":"cus_example","object":"customer","deleted":true}"#, true),
        ] {
            let object: stripe::ExpandableValue<stripe::CustomerId> =
                miniserde::json::from_str(json).unwrap();
            let id: &stripe::CustomerId = object.id();
            assert_eq!(id.as_str(), "cus_example");
            let family = object.into_expanded_as::<RetrieveCustomerReturned>().unwrap().unwrap();
            assert_variant(&family, deleted);
            #[cfg(feature = "deserialize")]
            {
                let object: stripe::ExpandableValue<stripe::CustomerId> =
                    serde_json::from_str(json).unwrap();
                let family =
                    object.into_expanded_as::<RetrieveCustomerReturned>().unwrap().unwrap();
                assert_variant(&family, deleted);
            }
        }
    }

    #[test]
    fn live_deleted_responses_validate_concrete_fields_during_initial_parsing() {
        for json in [
            r#"{"id":"cus_example","object":"customer"}"#,
            r#"{"object":"customer","deleted":true}"#,
            r#"{"id":17,"object":"customer","deleted":true}"#,
        ] {
            assert!(miniserde::json::from_str::<RetrieveCustomerReturned>(json).is_err());
            #[cfg(feature = "deserialize")]
            assert!(serde_json::from_str::<RetrieveCustomerReturned>(json).is_err());
        }
    }

    #[test]
    fn the_union_wire_tag_is_checked_before_its_structural_decoder() {
        let object: stripe::ObjectValue = miniserde::json::from_str(
            r#"{"id":"cus_example","object":"charge","created":1,"livemode":false}"#,
        )
        .unwrap();
        assert!(matches!(
            object.into_typed::<RetrieveCustomerReturned>(),
            Err(stripe::ObjectDecodeError::UnexpectedObjectType { expected: "customer", .. })
        ));
    }
}

#[cfg(all(test, feature = "charge"))]
mod record_tests {
    use serde_json::{Value, json};
    #[cfg(any(
        feature = "payment_method_details_card",
        feature = "payment_method_details_ideal"
    ))]
    use stripe::PaymentMethodDetailsExt;

    fn charge_json(details: Value) -> String {
        let mut charge: Value = serde_json::from_str(include_str!("../charge.json")).unwrap();
        charge["payment_method_details"] = details;
        charge.to_string()
    }

    fn record(details: Value) -> stripe::PaymentMethodDetails {
        let charge: stripe::Charge = miniserde::json::from_str(&charge_json(details)).unwrap();
        charge.payment_method_details.unwrap()
    }

    #[test]
    fn charge_retains_unknown_method_details_without_a_decoder() {
        let value =
            json!({"type":"future_method", "future_method":{"nested":[true,null,u64::MAX]}});
        let details = record(value.clone());
        assert_eq!(details.type_(), "future_method");
        assert_eq!(serde_json::to_value(&details).unwrap(), value);
        #[cfg(feature = "deserialize")]
        {
            let charge: stripe::Charge = serde_json::from_str(&charge_json(value.clone())).unwrap();
            assert_eq!(
                serde_json::to_value(charge.payment_method_details.unwrap()).unwrap(),
                value
            );
        }
    }

    #[test]
    fn record_parsing_checks_its_envelope_and_defers_field_validation() {
        for value in [Value::Null, json!([]), json!({}), json!({"type":123})] {
            assert!(
                miniserde::json::from_str::<stripe::PaymentMethodDetails>(&value.to_string())
                    .is_err()
            );
            if !value.is_null() {
                assert!(miniserde::json::from_str::<stripe::Charge>(&charge_json(value)).is_err());
            }
        }
        assert!(
            miniserde::json::from_str::<stripe::Charge>(&charge_json(Value::Null))
                .unwrap()
                .payment_method_details
                .is_none()
        );
        let value = json!({"type":"card", "card":{"exp_month":"invalid"}});
        let details = record(value.clone());
        assert_eq!(serde_json::to_value(details).unwrap(), value);
    }

    #[cfg(feature = "payment_method_details_card")]
    #[test]
    fn card_decoder_adds_repeatable_borrowed_access_without_changing_storage() {
        let value = json!({"type":"card", "card":{"exp_month":12,"exp_year":2030,"last4":"4242","brand":"visa"}, "extra":true});
        let details = record(value.clone());
        let first: stripe::PaymentMethodDetailsCard = details.card().unwrap().unwrap();
        let second = details.card().unwrap().unwrap();
        assert_eq!(first.last4.as_deref(), Some("4242"));
        assert_eq!(second.exp_year, 2030);
        assert_eq!(serde_json::to_value(&details).unwrap(), value);
    }

    #[cfg(feature = "payment_method_details_card")]
    #[test]
    fn card_access_preserves_optional_field_semantics() {
        for value in [json!({"type":"card"}), json!({"type":"card", "card":null})] {
            assert!(record(value).card().unwrap().is_none());
        }
        let value = json!({"type":"future_method", "card":{"exp_month":12,"exp_year":2030}});
        assert!(record(value).card().unwrap().is_some());
        let details = record(json!({"type":"card", "card":{"exp_month":"invalid"}}));
        assert_eq!(
            details.card().unwrap_err(),
            stripe::RecordDecodeError::InvalidField { field: "card".into() }
        );
        assert_eq!(details.type_(), "card");
    }

    #[cfg(feature = "payment_method_details_ideal")]
    #[test]
    fn another_decoder_can_be_enabled_independently() {
        let details = record(
            json!({"type":"ideal", "ideal":{"bank":"ing","bic":"INGBNL2A","iban_last4":"1234"}}),
        );
        let ideal: stripe::PaymentMethodDetailsIdeal = details.ideal().unwrap().unwrap();
        assert_eq!(ideal.iban_last4.as_deref(), Some("1234"));
    }
}

#[cfg(all(test, feature = "payment_method_details_card"))]
mod standalone_card_decoder_tests {
    use stripe::prelude::*;

    #[test]
    fn the_decoder_can_be_used_without_charge_or_its_request_package() {
        let details: stripe::PaymentMethodDetails = miniserde::json::from_str(
            r#"{"type":"card","card":{"exp_month":12,"exp_year":2030,"last4":"4242"}}"#,
        )
        .unwrap();
        let card: stripe::PaymentMethodDetailsCard = details.card().unwrap().unwrap();
        assert_eq!(card.last4.as_deref(), Some("4242"));
    }
}
#[cfg(all(test, feature = "invoice"))]
mod optional_invoice_id_tests {
    use serde_json::{Value, json};

    type InvoiceReference = stripe::ExpandableValue<Option<stripe::InvoiceId>>;

    fn invoice() -> Value {
        serde_json::from_str(include_str!("../invoice.json")).unwrap()
    }

    #[test]
    fn optional_ids_match_the_eager_invoice_decoder_and_preserve_the_payload() {
        for id in [None, Some(Value::Null), Some(json!("in_example"))] {
            let mut value = invoice();
            match id {
                Some(id) => {
                    value["id"] = id;
                }
                None => {
                    value.as_object_mut().unwrap().remove("id");
                }
            }
            value["future"] = json!({"nested":[true,null,u64::MAX]});
            let json = value.to_string();
            let eager: stripe::Invoice = miniserde::json::from_str(&json).unwrap();
            let deferred: InvoiceReference = miniserde::json::from_str(&json).unwrap();
            assert!(deferred.is_object());
            assert_eq!(deferred.id(), &eager.id);
            assert_eq!(deferred.as_object().unwrap().id(), &eager.id);
            assert_eq!(serde_json::to_value(&deferred).unwrap(), value);
            let decoded = deferred.into_expanded_as::<stripe::Invoice>().unwrap().unwrap();
            assert_eq!(decoded.id, eager.id);
            assert_eq!(decoded.amount_due, eager.amount_due);
            #[cfg(feature = "serialize")]
            assert_eq!(
                serde_json::to_value(&decoded).unwrap(),
                serde_json::to_value(&eager).unwrap()
            );

            #[cfg(feature = "deserialize")]
            {
                let eager: stripe::Invoice = serde_json::from_str(&json).unwrap();
                let deferred: InvoiceReference = serde_json::from_str(&json).unwrap();
                assert_eq!(deferred.id(), &eager.id);
                assert_eq!(serde_json::to_value(&deferred).unwrap(), value);
                let decoded = deferred.into_expanded_as::<stripe::Invoice>().unwrap().unwrap();
                assert_eq!(decoded.id, eager.id);
                assert_eq!(decoded.amount_due, eager.amount_due);
            }
        }
    }

    #[test]
    fn optional_invoice_ids_reject_invalid_nonstring_values() {
        for id in [json!(123), json!(false), json!([]), json!({})] {
            let mut value = invoice();
            value["id"] = id;
            let json = value.to_string();
            assert!(miniserde::json::from_str::<stripe::Invoice>(&json).is_err());
            assert!(miniserde::json::from_str::<InvoiceReference>(&json).is_err());
            #[cfg(feature = "deserialize")]
            {
                assert!(serde_json::from_str::<stripe::Invoice>(&json).is_err());
                assert!(serde_json::from_str::<InvoiceReference>(&json).is_err());
            }
        }
    }

    #[test]
    fn unexpanded_invoice_ids_remain_typed() {
        let reference: InvoiceReference = miniserde::json::from_str(r#""in_example""#).unwrap();
        assert_eq!(reference.id().as_ref().unwrap().as_str(), "in_example");
        assert!(!reference.is_object());
        assert!(reference.into_expanded_as::<stripe::Invoice>().unwrap().is_none());
    }
}

#[cfg(all(test, feature = "payment_method_details_card_present"))]
mod standalone_card_present_decoder_tests {
    use stripe::prelude::*;

    #[test]
    fn shared_card_present_payload_has_one_independent_decoder_for_both_records() {
        let payload = r#"{
            "type":"card_present",
            "card_present":{
                "amount_authorized":null,"brand":"visa","brand_product":null,
                "cardholder_name":null,"country":null,"emv_auth_data":null,
                "exp_month":12,"exp_year":2030,"fingerprint":null,"funding":null,
                "generated_card":null,"incremental_authorization_supported":false,
                "last4":"4242","network":null,"network_transaction_id":null,
                "offline":null,"overcapture_supported":false,"preferred_locales":null,
                "read_method":null,"receipt":null
            }
        }"#;
        let details: stripe::PaymentMethodDetails = miniserde::json::from_str(payload).unwrap();
        let generated: stripe::CardGeneratedFromPaymentMethodDetails =
            miniserde::json::from_str(payload).unwrap();
        let first: stripe::PaymentMethodDetailsCardPresent =
            details.card_present().unwrap().unwrap();
        let second: stripe::PaymentMethodDetailsCardPresent =
            generated.card_present().unwrap().unwrap();
        assert_eq!(first.last4.as_deref(), Some("4242"));
        assert_eq!(second.exp_year, 2030);
    }
}
