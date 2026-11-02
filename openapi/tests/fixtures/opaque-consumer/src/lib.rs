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

#[cfg(all(test, feature = "balance_transaction_source"))]
mod model_only_tests {
    #[test]
    fn selecting_a_lightweight_family_does_not_duplicate_its_export() {
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

    #[test]
    fn deferred_customer_union_decoding_uses_the_existing_deleted_flag() {
        for (json, expected_deleted) in [
            (r#"{"id":"cus_example","object":"customer","created":1,"livemode":false}"#, false),
            (r#"{"id":"cus_example","object":"customer","deleted":true}"#, true),
            (
                r#"{"id":"cus_example","object":"customer","created":1,"livemode":false,"deleted":true}"#,
                true,
            ),
            (
                r#"{"id":"cus_example","object":"customer","created":1,"livemode":false,"deleted":false}"#,
                false,
            ),
        ] {
            let eager: RetrieveCustomerReturned = miniserde::json::from_str(json).unwrap();
            let object: stripe::ObjectValue = miniserde::json::from_str(json).unwrap();
            let deferred = object.into_typed::<RetrieveCustomerReturned>().unwrap();
            for model in [eager, deferred] {
                match model {
                    RetrieveCustomerReturned::Customer(customer) => {
                        assert!(!expected_deleted, "{json}");
                        assert_eq!(customer.id.as_str(), "cus_example");
                    }
                    RetrieveCustomerReturned::DeletedCustomer(customer) => {
                        assert!(expected_deleted, "{json}");
                        assert_eq!(customer.id.as_str(), "cus_example");
                    }
                }
            }
        }
    }

    #[test]
    fn an_expanded_customer_keeps_its_typed_id_and_deleted_union_view() {
        let object: stripe::ExpandableValue<stripe::CustomerId> =
            miniserde::json::from_str(r#"{"id":"cus_example","object":"customer","deleted":true}"#)
                .unwrap();
        let id: &stripe::CustomerId = object.id();
        assert_eq!(id.as_str(), "cus_example");
        assert!(matches!(
            object.into_expanded_as::<RetrieveCustomerReturned>().unwrap(),
            Some(RetrieveCustomerReturned::DeletedCustomer(_))
        ));
    }

    #[test]
    fn a_matching_family_tag_still_requires_a_valid_union_branch() {
        for json in [
            r#"{"id":"cus_example","object":"customer"}"#,
            r#"{"id":"cus_example","object":"customer","deleted":false}"#,
            r#"{"id":"cus_example","object":"customer","created":"invalid","livemode":false}"#,
        ] {
            assert!(miniserde::json::from_str::<RetrieveCustomerReturned>(json).is_err());
            let object: stripe::ObjectValue = miniserde::json::from_str(json).unwrap();
            assert!(matches!(
                object.into_typed::<RetrieveCustomerReturned>(),
                Err(stripe::ObjectDecodeError::InvalidObject { object_type: "customer" })
            ));
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
            stripe::RecordDecodeError::InvalidField { field: "card" }
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
