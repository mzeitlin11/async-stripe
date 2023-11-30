use stripe_checkout::checkout_session::RetrieveCheckoutSession;

use crate::mock;

#[test]
fn is_checkout_session_retrievable() {
    mock::with_client(|client| {
        let id = "cs_test_123".parse().unwrap();
        let session = RetrieveCheckoutSession::new().send(client, &id).unwrap();
        assert_eq!(session.id, "cs_test_123");
    });
}
