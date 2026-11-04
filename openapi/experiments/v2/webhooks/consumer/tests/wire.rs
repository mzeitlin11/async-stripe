use std::time::{SystemTime, UNIX_EPOCH};

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use stripe_client_core::{EventNotification, NotificationError};

fn sign(payload: &str, timestamp: i64) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(b"whsec_test").unwrap();
    mac.update(format!("{timestamp}.{payload}").as_bytes());
    format!("t={timestamp},v1={}", hex::encode(mac.finalize().into_bytes()))
}
fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}
#[cfg(feature = "typed-events")]
fn notification(name: &str) -> EventNotification {
    let body = match name {
        "account" => include_str!("../fixtures/account-notification.json"),
        "billing" => include_str!("../fixtures/billing-notification.json"),
        _ => unreachable!(),
    };
    EventNotification::parse_verified(body, &sign(body, now()), "whsec_test").unwrap()
}

#[test]
fn verifies_before_parsing_and_retains_future_notifications() {
    let body = include_str!("../fixtures/account-notification.json");
    let signed = sign(body, now());
    assert!(matches!(
        EventNotification::parse_verified(&format!("{body} "), &signed, "whsec_test"),
        Err(NotificationError::Signature(_))
    ));
    assert!(matches!(
        EventNotification::parse_verified("invalid-json", &signed, "whsec_test"),
        Err(NotificationError::Signature(_))
    ));
    assert!(EventNotification::parse_verified(body, &sign(body, 1), "whsec_test").is_err());
    assert!(EventNotification::parse_verified(body, &signed, "wrong-secret").is_err());
    let body = body
        .replace("v2.core.account.updated", "v9.future.event")
        .replace("\"reason\":", "\"future_field\":42,\"reason\":");
    let event =
        EventNotification::parse_verified(&body, &sign(&body, now()), "whsec_test").unwrap();
    assert_eq!(event.additional_fields["future_field"], 42);
    assert_eq!(event.reason.as_ref().unwrap().request.as_ref().unwrap().id, "req_123");
    #[cfg(feature = "typed-events")]
    assert!(matches!(
        stripe_events::Notification::from_notification(event).unwrap(),
        stripe_events::Notification::Unknown(_)
    ));
    let snapshot =
        include_str!("../fixtures/account-notification.json").replace("v2.core.event", "event");
    assert!(matches!(
        EventNotification::parse_verified(&snapshot, &sign(&snapshot, now()), "whsec_test"),
        Err(NotificationError::UnexpectedObject(_))
    ));
}

#[cfg(feature = "typed-events")]
mod typed {
    use httpmock::{Method::GET, MockServer};
    use stripe::{ClientBuilder, StripeError};
    use stripe_client_core::StripeClient;
    use stripe_events::{Notification, V1BillingMeterErrorReportTriggered, V2CoreAccountUpdated};

    use super::*;
    const REVISION: &str = "2026-08-26.dahlia";

    #[test]
    fn rejects_incompatible_typed_selection() {
        assert!(notification("account").typed::<V1BillingMeterErrorReportTriggered>().is_err());
        let mut event = notification("account");
        event.related_object.as_mut().unwrap().object_type = "billing.meter".into();
        assert!(event.typed::<V2CoreAccountUpdated>().is_err());
    }

    async fn wire<C: StripeClient<Err = StripeError>>(server: &MockServer, client: &C) {
        let Notification::V2CoreAccountUpdated(event) =
            Notification::from_notification(notification("account")).unwrap()
        else {
            panic!("wrong dispatch")
        };
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/core/events/evt_account%2Fencoded")
                .header("stripe-context", "org/account")
                .header("stripe-version", REVISION)
                .header("stripe-request-trigger", "event=evt_account/encoded");
            then.status(200).body(include_str!("../fixtures/account-event.json"));
        });
        let fetched: V2CoreAccountUpdated = event.event_request().send(client).await.unwrap();
        assert_eq!(fetched.id, event.notification().id);
        assert_eq!(fetched.related_object.id, "acct_test");
        let _: V2CoreAccountUpdated =
            serde_json::from_str(include_str!("../fixtures/account-event.json")).unwrap();
        serde_json::to_value(&fetched).unwrap();
        mock.assert();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/core/accounts/acct_test")
                .header("stripe-context", "org/account")
                .header("stripe-version", "custom.revision")
                .header("stripe-request-trigger", "event=evt_account/encoded");
            then.status(200).body(include_str!("../fixtures/account.json"));
        });
        #[cfg(feature = "accounts")]
        type Account = stripe_v2::core::Account;
        #[cfg(not(feature = "accounts"))]
        #[derive(miniserde::Deserialize)]
        struct Account {
            id: String,
        }
        let resource = event
            .related_request::<Account>()
            .unwrap()
            .api_version("custom.revision")
            .send(client)
            .await
            .unwrap();
        assert_eq!(resource.id.as_str(), "acct_test");
        mock.assert();

        let Notification::V1BillingMeterErrorReportTriggered(event) =
            Notification::from_notification(notification("billing")).unwrap()
        else {
            panic!("wrong dispatch")
        };
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/core/events/evt_billing")
                .header("stripe-context", "org/meter")
                .header("stripe-version", REVISION)
                .header("stripe-request-trigger", "event=evt_billing");
            then.status(200).body(include_str!("../fixtures/billing-event.json"));
        });
        let fetched: V1BillingMeterErrorReportTriggered =
            event.event_request().send(client).await.unwrap();
        assert_eq!(fetched.data.reason.error_count, 2);
        assert_eq!(
            fetched.data.reason.error_types[0].sample_errors[0].request.identifier,
            "usage-123"
        );
        let _: V1BillingMeterErrorReportTriggered =
            serde_json::from_str(include_str!("../fixtures/billing-event.json")).unwrap();
        serde_json::to_value(&fetched).unwrap();
        mock.assert();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v1/billing/meters/mtr_test")
                .header("stripe-context", "org/meter")
                .header("stripe-version", REVISION)
                .header("stripe-request-trigger", "event=evt_billing");
            then.status(200).json_body(serde_json::json!({"id":"mtr_test"}));
        });
        #[derive(miniserde::Deserialize)]
        struct Meter {
            id: String,
        }
        assert_eq!(
            event.related_request::<Meter>().unwrap().send(client).await.unwrap().id,
            "mtr_test"
        );
        mock.assert();

        let mut future = notification("account");
        future.event_type = "future.event".into();
        future.related_object.as_mut().unwrap().url = "https://elsewhere.invalid/object".into();
        let Notification::Unknown(future) = Notification::from_notification(future).unwrap() else {
            panic!("wrong fallback")
        };
        assert!(matches!(
            future.related_request::<Meter>(REVISION).unwrap().send(client).await,
            Err(StripeError::ConfigError(_))
        ));
        let mut no_resource = future;
        no_resource.related_object = None;
        assert!(no_resource.related_request::<Meter>(REVISION).is_none());
    }

    #[tokio::test]
    async fn hyper_flow() {
        let server = MockServer::start_async().await;
        let client = ClientBuilder::new("sk_test").url(server.base_url()).build().unwrap();
        wire(&server, &client).await;
    }
    #[tokio::test]
    async fn surf_flow() {
        let server = MockServer::start_async().await;
        let client = stripe::async_std::ClientBuilder::new("sk_test")
            .url(server.base_url())
            .build()
            .unwrap();
        wire(&server, &client).await;
    }
    #[test]
    fn blocking_fetch() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/core/events/evt_account%2Fencoded")
                .header("stripe-context", "org/account")
                .header("stripe-request-trigger", "event=evt_account/encoded");
            then.status(200).body(include_str!("../fixtures/account-event.json"));
        });
        let client = ClientBuilder::new("sk_test").url(server.base_url()).build_sync().unwrap();
        let event = notification("account").typed::<V2CoreAccountUpdated>().unwrap();
        event.event_request().send_blocking(&client).unwrap();
        mock.assert();
    }
}
