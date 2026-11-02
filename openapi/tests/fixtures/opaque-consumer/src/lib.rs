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
