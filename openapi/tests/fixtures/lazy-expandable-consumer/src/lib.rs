#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use serde_json::{Value, json};
    use stripe_client_core::{CustomizedStripeRequest, StripeBlockingClient, StripeMethod};
    use stripe_core::BalanceTransactionId;
    use stripe_core::balance_transaction::RetrieveBalanceTransaction;
    use stripe_shared::{
        BalanceTransaction, BalanceTransactionSource, Charge, Customer, CustomerId, PaymentMethod,
        PaymentMethodCard,
    };
    use stripe_types::LazyExpandable;

    fn fixture(name: &str) -> Value {
        let fixtures: Value =
            serde_json::from_str(include_str!("../../../../fixtures.json")).unwrap();
        fixtures["resources"][name].clone()
    }

    struct FixtureClient(String);

    impl StripeBlockingClient for FixtureClient {
        type Err = stripe::StripeError;

        fn execute(&self, request: CustomizedStripeRequest) -> Result<Bytes, Self::Err> {
            let (request, _) = request.into_pieces();
            assert!(matches!(request.method, StripeMethod::Get));
            assert_eq!(request.path, "/balance_transactions/txn_example");
            Ok(Bytes::copy_from_slice(self.0.as_bytes()))
        }
    }

    fn retrieve(source: Value) -> BalanceTransaction {
        let mut payload = fixture("balance_transaction");
        payload["source"] = source;
        RetrieveBalanceTransaction::new(BalanceTransactionId::from("txn_example"))
            .send_blocking(&FixtureClient(payload.to_string()))
            .unwrap()
    }

    #[test]
    fn retrieval_retains_ids_and_unknown_expansions_without_decoding() {
        let source = retrieve(json!("ch_example")).source.unwrap();
        assert_eq!(source.id().as_str(), "ch_example");
        assert!(source.into_expanded_as::<BalanceTransactionSource>().unwrap().is_none());
        let payload = json!({"id":"future_example","object":"future","count":u64::MAX});
        let source = retrieve(payload.clone()).source.unwrap();
        assert_eq!(serde_json::to_value(&source).unwrap(), payload);
        assert!(source.into_expanded_as::<BalanceTransactionSource>().is_err());
    }

    #[test]
    fn expanded_union_decodes_into_the_existing_matchable_enum() {
        let source = retrieve(fixture("charge")).source.unwrap();
        let BalanceTransactionSource::Charge(charge) =
            source.into_expanded_as::<BalanceTransactionSource>().unwrap().unwrap()
        else {
            panic!("expected a concrete Charge variant");
        };
        assert_eq!(charge.amount, fixture("charge")["amount"].as_i64().unwrap());
    }

    #[test]
    fn customer_relationship_keeps_its_typed_id_and_defers_required_fields() {
        let mut charge = fixture("charge");
        charge["customer"] = json!({"id":"cus_example","object":"customer"});
        let charge: Charge = miniserde::json::from_str(&charge.to_string()).unwrap();
        let customer: LazyExpandable<CustomerId> = charge.customer.unwrap();
        assert_eq!(customer.id().as_str(), "cus_example");
        assert!(customer.into_expanded_as::<Customer>().is_err());
        let mut charge = fixture("charge");
        charge["customer"] = fixture("customer");
        let charge: Charge = serde_json::from_value(charge).unwrap();
        let customer = charge.customer.unwrap();
        let first = customer.to_expanded_as::<Customer>().unwrap().unwrap();
        let second = customer.into_expanded_as::<Customer>().unwrap().unwrap();
        assert_eq!(first.id, second.id);
    }

    #[test]
    fn payment_method_details_and_api_error_context_stay_eager_and_typed() {
        let payload = fixture("payment_method");
        let method: PaymentMethod = miniserde::json::from_str(&payload.to_string()).unwrap();
        let _: &PaymentMethodCard = method.card.as_ref().unwrap();
        let mut malformed = payload;
        malformed["card"] = json!(true);
        assert!(miniserde::json::from_str::<PaymentMethod>(&malformed.to_string()).is_err());
        assert!(serde_json::from_value::<PaymentMethod>(malformed).is_err());
        let malformed =
            r#"{"type":"api_error","payment_intent":{"id":"pi_bad","object":"payment_intent"}}"#;
        assert!(miniserde::json::from_str::<stripe_shared::ApiErrors>(malformed).is_err());
    }

    #[test]
    fn tagless_expandable_rules_use_the_same_lazy_wrapper() {
        let mut charge = fixture("charge");
        charge["outcome"]["rule"] =
            json!({"id":"rule_example","action":"allow","predicate":":is_test:"});
        let charge: Charge = miniserde::json::from_str(&charge.to_string()).unwrap();
        let rule = charge.outcome.unwrap().rule.unwrap();
        assert_eq!(rule.id().as_str(), "rule_example");
        let rule = rule.into_expanded_as::<stripe_shared::Rule>().unwrap().unwrap();
        assert_eq!(rule.action, "allow");
        assert_eq!(rule.predicate, ":is_test:");
    }

    #[test]
    fn expanded_invoices_preserve_permitted_missing_and_null_ids() {
        for id in [None, Some(Value::Null)] {
            let mut inner = fixture("invoice");
            inner.as_object_mut().unwrap().remove("id");
            if let Some(id) = id {
                inner["id"] = id;
            }
            let mut outer = fixture("invoice");
            outer["latest_revision"] = inner;
            let invoice: stripe_shared::Invoice =
                miniserde::json::from_str(&outer.to_string()).unwrap();
            let revision: LazyExpandable<Option<stripe_shared::InvoiceId>> =
                invoice.latest_revision.unwrap();
            assert!(revision.id().is_none());
            assert!(
                revision
                    .into_expanded_as::<stripe_shared::Invoice>()
                    .unwrap()
                    .unwrap()
                    .id
                    .is_none()
            );
        }
    }
}
