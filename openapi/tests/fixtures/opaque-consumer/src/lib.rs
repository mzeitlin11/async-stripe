#[cfg(test)]
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
