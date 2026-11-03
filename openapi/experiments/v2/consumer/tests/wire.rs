use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use futures_util::TryStreamExt;
use httpmock::{
    Method::{DELETE, GET, POST},
    MockServer,
};
use serde_json::{Value, json};
use stripe::{ClientBuilder, StripeError};
use stripe_client_core::{EventNotification, RequestStrategy, StripeClient, StripeRequest};
use stripe_v2::core::event_destination::*;

const REVISION: &str = "2026-08-26.dahlia";
const ID: &str = "ed_test_61RM8ltWcTW4mbsxf16RJyfa2xSQLHJJh1sxm7H0KVT6";

fn destination() -> Value {
    serde_json::from_str(include_str!("../fixtures/event_destination.json")).unwrap()
}
fn event() -> Value {
    serde_json::from_str(include_str!("../fixtures/event.json")).unwrap()
}

async fn wire<C: StripeClient<Err = StripeError> + Clone>(server: &MockServer, client: C) {
    let response = destination();
    let create = Create::new(
        vec!["v2.core.event_destination.ping".to_owned()],
        stripe_v2::V2CoreEventDestinationEventPayload::Thin,
        "example",
        stripe_v2::V2CoreEventDestinationType::WebhookEndpoint,
    )
    .webhook_endpoint(CreateV2CoreEventDestinationWebhookEndpoint::new("https://example.com/hook"));
    let post = server.mock(|when, then| {
        when.method(POST)
            .path("/v2/core/event_destinations")
            .header("authorization", "Bearer sk_test_v2")
            .header("stripe-version", REVISION)
            .header("content-type", "application/json")
            .header_exists("idempotency-key")
            .json_body(json!({"enabled_events":["v2.core.event_destination.ping"],
                "event_payload":"thin","name":"example","type":"webhook_endpoint",
                "webhook_endpoint":{"url":"https://example.com/hook"}}));
        then.status(200).json_body(response.clone());
    });
    let created = create.send(&client).await.unwrap();
    assert_eq!(created.id.as_str(), ID);
    assert!(created.created.contains('T'));
    post.assert();

    let mut retrieve = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v2/core/event_destinations/{ID}"))
            .header("stripe-version", REVISION)
            .header("stripe-account", "acct_context")
            .header("stripe-context", "org/context")
            .query_param("include[0]", "webhook_endpoint.url");
        then.status(200).json_body(response.clone());
    });
    let retrieved = Retrieve::new(ID)
        .include(vec![RetrieveV2CoreEventDestinationInclude::WebhookEndpointUrl])
        .customize()
        .account_id("acct_context")
        .context("org/context")
        .send(&client)
        .await
        .unwrap();
    assert_eq!(retrieved.id, created.id);
    retrieve.assert();
    retrieve.delete();

    let mut update = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v2/core/event_destinations/{ID}"))
            .header("content-type", "application/json")
            .json_body(json!({"metadata":{"remove":null,"keep":""}}));
        then.status(200).json_body(response.clone());
    });
    Update::new(ID)
        .metadata(HashMap::from([
            ("remove".to_owned(), None),
            ("keep".to_owned(), Some(String::new())),
        ]))
        .send(&client)
        .await
        .unwrap();
    update.assert();
    update.delete();
    for (verb, target) in
        [("disable", Disable::new(ID).build()), ("enable", Enable::new(ID).build())]
    {
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path(format!("/v2/core/event_destinations/{ID}/{verb}"))
                .header("content-type", "application/json")
                .body("{}");
            then.status(200).json_body(response.clone());
        });
        target.customize::<stripe_v2::core::EventDestination>().send(&client).await.unwrap();
        mock.assert();
    }
    let ping = server.mock(|when, then| {
        when.method(POST).path(format!("/v2/core/event_destinations/{ID}/ping")).body("{}");
        then.status(200).json_body(event());
    });
    let _: stripe_v2::core::Event = Ping::new(ID).send(&client).await.unwrap();
    ping.assert();

    let mut delete = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v2/core/event_destinations/{ID}"))
            .header_exists("idempotency-key")
            .body("");
        then.status(200).json_body(json!({"id": ID,"object":"v2.core.event_destination"}));
    });
    let deleted: stripe_v2::V2DeletedObject = Delete::new(ID).send(&client).await.unwrap();
    assert_eq!(deleted.id.as_str(), ID);
    delete.assert();
    delete.delete();

    let page = |data: Value, next: Value| json!({"data":data,"next_page_url":next,"previous_page_url":null});
    let first = server.mock(|when, then| {
        when.method(GET)
            .path("/v2/core/event_destinations")
            .query_param("limit", "1")
            .query_param("include[0]", "webhook_endpoint.url")
            .query_param_missing("page")
            .header("stripe-context", "page-context")
            .header("stripe-version", REVISION);
        then.status(200).json_body(page(
            json!([response.clone()]),
            json!("/v2/core/event_destinations?page=second&limit=1"),
        ));
    });
    let second = server.mock(|when, then| {
        when.method(GET)
            .path("/v2/core/event_destinations")
            .query_param("page", "second")
            .query_param("limit", "1")
            .query_param_missing("include[0]")
            .header("stripe-context", "page-context")
            .header("stripe-version", REVISION);
        then.status(200)
            .json_body(page(json!([]), json!("/v2/core/event_destinations?page=third&limit=1")));
    });
    let third = server.mock(|when, then| {
        when.method(GET)
            .path("/v2/core/event_destinations")
            .query_param("page", "third")
            .header("stripe-context", "page-context");
        then.status(200).json_body(page(json!([response]), Value::Null));
    });
    let all: Vec<_> = List::new()
        .limit(1)
        .include(vec![ListV2CoreEventDestinationInclude::WebhookEndpointUrl])
        .paginate()
        .context("page-context")
        .stream(&client)
        .try_collect()
        .await
        .unwrap();
    assert_eq!(all.len(), 2);
    first.assert();
    second.assert();
    third.assert();

    let keys = Arc::new(Mutex::new(HashSet::<String>::new()));
    let received = keys.clone();
    let failure = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v2/core/event_destinations/{ID}"))
            .header_exists("idempotency-key")
            .is_true(move |request| {
                received
                    .lock()
                    .unwrap()
                    .insert(request.headers()["idempotency-key"].to_str().unwrap().to_owned());
                true
            });
        then.status(503)
            .header("Stripe-Should-Retry", "true")
            .json_body(json!({"error":{"code":"temporarily_unavailable","message":"retry"}}));
    });
    let error = Delete::new(ID)
        .customize()
        .request_strategy(RequestStrategy::Retry(2))
        .send(&client)
        .await
        .unwrap_err();
    assert!(
        matches!(error, StripeError::Api(ref error,503) if error.code.as_deref()==Some("temporarily_unavailable"))
    );
    failure.assert_calls(2);
    assert_eq!(keys.lock().unwrap().len(), 1);

    #[cfg(feature = "accounts")]
    {
        use stripe_v2::core::account;
        let raw: Value = serde_json::from_str(include_str!("../fixtures/account.json")).unwrap();
        let id = raw["id"].as_str().unwrap();
        let create = server.mock(|when, then| {
            when.method(POST)
                .path("/v2/core/accounts")
                .header("content-type", "application/json")
                .json_body(json!({"display_name":"example"}));
            then.status(200).json_body(raw.clone());
        });
        let _: stripe_v2::core::Account =
            account::Create::new().display_name("example").send(&client).await.unwrap();
        create.assert();
        let update = server.mock(|when, then| {
            when.method(POST)
                .path(format!("/v2/core/accounts/{id}"))
                .json_body(json!({"metadata":{"remove":null}}));
            then.status(200).json_body(raw.clone());
        });
        account::Update::new(id)
            .metadata(HashMap::from([("remove".into(), None)]))
            .send(&client)
            .await
            .unwrap();
        update.assert();
        let list = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/core/accounts")
                .query_param("applied_configurations[0]", "merchant");
            then.status(200)
                .json_body(json!({"data":[raw],"next_page_url":null,"previous_page_url":null}));
        });
        let page = account::List::new()
            .applied_configurations(vec![account::ListV2CoreAccountAppliedConfigurations::Merchant])
            .send(&client)
            .await
            .unwrap();
        assert_eq!(page.data.len(), 1);
        list.assert();
    }
}

#[tokio::test]
async fn hyper_wire() {
    let server = MockServer::start_async().await;
    wire(&server, ClientBuilder::new("sk_test_v2").url(server.base_url()).build().unwrap()).await;
}

#[tokio::test]
async fn surf_wire() {
    let server = MockServer::start_async().await;
    wire(
        &server,
        stripe::async_std::ClientBuilder::new("sk_test_v2").url(server.base_url()).build().unwrap(),
    )
    .await;
}

#[tokio::test]
async fn prefixed_api_base_keeps_namespace_and_revision_semantics() {
    let server = MockServer::start_async().await;
    let failure = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/proxy/v2/core/event_destinations/{ID}"))
            .header_exists("idempotency-key")
            .header("stripe-version", "request-override");
        then.status(400)
            .json_body(json!({"error":{"code":"invalid_destination","message":"example"}}));
    });
    let base = format!("{}/proxy/", server.base_url());
    let hyper =
        ClientBuilder::new("sk_test_v2").url(&base).api_version("client-override").build().unwrap();
    let surf = stripe::async_std::ClientBuilder::new("sk_test_v2")
        .url(&base)
        .api_version("client-override")
        .build()
        .unwrap();
    let error =
        Delete::new(ID).customize().api_version("request-override").send(&hyper).await.unwrap_err();
    assert!(matches!(error, StripeError::Api(_, 400)));
    let error =
        Delete::new(ID).customize().api_version("request-override").send(&surf).await.unwrap_err();
    assert!(matches!(error, StripeError::Api(_, 400)));
    failure.assert_calls(2);
}

#[test]
fn blocking_wire() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v2/core/event_destinations/{ID}"))
            .header("stripe-version", REVISION);
        then.status(200).json_body(destination());
    });
    let client = ClientBuilder::new("sk_test_v2").url(server.base_url()).build_sync().unwrap();
    assert_eq!(Retrieve::new(ID).send_blocking(&client).unwrap().id.as_str(), ID);
    mock.assert();
    let list = server.mock(|when, then| {
        when.method(GET).path("/v2/core/event_destinations").query_param("limit", "1");
        then.status(200).json_body(
            json!({"data":[destination()],"next_page_url":null,"previous_page_url":null}),
        );
    });
    assert_eq!(List::new().limit(1).paginate().get_all(&client).unwrap().len(), 1);
    list.assert();
}

#[tokio::test]
async fn pagination_rejects_external_targets() {
    let server = MockServer::start_async().await;
    let first = server.mock(|when, then| {
        when.method(GET).path("/v2/core/event_destinations");
        then.status(200).json_body(json!({"data":[],"next_page_url":"https://other.invalid/v2/core/event_destinations?page=next","previous_page_url":null}));
    });
    let client = ClientBuilder::new("sk_test_v2").url(server.base_url()).build().unwrap();
    let error = List::new().paginate().stream(&client).try_collect::<Vec<_>>().await.unwrap_err();
    assert!(matches!(error, StripeError::JSONDeserialize(_)));
    first.assert_calls(1);
}

#[cfg(not(feature = "v1"))]
#[tokio::test]
async fn manual_v2_request_requires_revision() {
    let server = MockServer::start_async().await;
    let client = ClientBuilder::new("sk_test_v2").url(server.base_url()).build().unwrap();
    let error = stripe_client_core::RequestBuilder::new(
        stripe_client_core::StripeMethod::Get,
        "/v2/core/events",
    )
    .customize::<miniserde::json::Value>()
    .send(&client)
    .await
    .unwrap_err();
    assert!(matches!(error, StripeError::ConfigError(_)));
}

#[test]
fn fixtures_and_open_event_changes() {
    assert_eq!(
        Retrieve::new("ed_part/with?query#fragment").build().path,
        "/v2/core/event_destinations/ed_part%2Fwith%3Fquery%23fragment"
    );
    let mut raw = event();
    raw["changes"] = json!({"before":{"name":"old"},"after":{"name":"new"}});
    let event: stripe_v2::core::Event = miniserde::json::from_str(&raw.to_string()).unwrap();
    assert_eq!(serde_json::to_value(&event).unwrap()["changes"], raw["changes"]);
    let _: stripe_v2::core::Event = serde_json::from_value(raw).unwrap();
    #[cfg(feature = "accounts")]
    {
        let mut raw: Value =
            serde_json::from_str(include_str!("../fixtures/account.json")).unwrap();
        raw["configuration"] =
            json!({"customer":{"applied":true,"billing":{"default_payment_method":"pm_example"}}});
        let account: stripe_v2::core::Account =
            miniserde::json::from_str(&raw.to_string()).unwrap();
        assert_eq!(
            account
                .configuration
                .unwrap()
                .customer
                .unwrap()
                .billing
                .unwrap()
                .default_payment_method
                .as_deref(),
            Some("pm_example")
        );
        let _: stripe_v2::core::Account =
            serde_json::from_str(include_str!("../fixtures/account.json")).unwrap();
    }
}

#[tokio::test]
async fn thin_notification_can_reference_v1_without_v1_models() {
    let server = MockServer::start_async().await;
    let notification = EventNotification::parse(r#"{"id":"evt_thin","type":"v1.billing.meter.error_report_triggered","created":"2026-08-26T00:00:00.000Z","livemode":false,"context":"event-context","related_object":{"id":"mtr_123","type":"billing.meter","url":"/v1/billing/meters/mtr_123"}}"#).unwrap();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/v1/billing/meters/mtr_123")
            .header("stripe-context", "event-context")
            .header("stripe-version", REVISION);
        then.status(200).json_body(json!({"id":"mtr_123"}));
    });
    #[derive(miniserde::Deserialize)]
    struct MeterId {
        id: String,
    }
    let client = ClientBuilder::new("sk_test_v2").url(server.base_url()).build().unwrap();
    let meter =
        notification.related_request::<MeterId>(REVISION).unwrap().send(&client).await.unwrap();
    assert_eq!(meter.id, "mtr_123");
    mock.assert();
}

#[cfg(feature = "v1")]
#[tokio::test]
async fn v1_and_v2_accounts_coexist() {
    use stripe_v2::core::account;
    let server = MockServer::start_async().await;
    let raw: Value = serde_json::from_str(include_str!("../fixtures/account.json")).unwrap();
    let id = raw["id"].as_str().unwrap();
    let v2 = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v2/core/accounts/{id}"))
            .query_param("include[0]", "identity");
        then.status(200).json_body(raw.clone());
    });
    let v1 = server.mock(|when, then| {
        when.method(GET).path(format!("/v1/accounts/{id}"));
        then.status(200).body(include_str!("../fixtures/v1-account.json"));
    });
    let client = ClientBuilder::new("sk_test_v2").url(server.base_url()).build().unwrap();
    let current: stripe_v2::core::Account = account::Retrieve::new(id)
        .include(vec![account::RetrieveV2CoreAccountInclude::Identity])
        .send(&client)
        .await
        .unwrap();
    let legacy: stripe_shared::Account =
        stripe_connect::account::RetrieveAccount::new(current.id.as_str())
            .send(&client)
            .await
            .unwrap();
    assert_eq!(legacy.id.as_str(), id);
    assert_eq!(current.id.as_str(), id);
    v1.assert();
    v2.assert();
}
