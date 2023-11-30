use stripe_connect::account::{
    CreateAccount, CreateAccountCapabilities, CreateAccountCapabilitiesAcssDebitPayments,
    ListAccount,
};
use stripe_connect::account_capabilities::AccountCapabilitiesCardPayments;
use stripe_types::AccountId;

use crate::mock;

#[test]
fn is_account_listable() {
    mock::with_client(|client| {
        let expected_id: AccountId = "acct_1O8RSFF2YyVaukgl".parse().unwrap();
        let result = ListAccount::new().send(client).unwrap();

        // Check to ensure we are deserializing _something_ and this test
        // actually validates something worthwhile.
        assert_eq!(result.data.len(), 1);
        assert_eq!(result.data.first().unwrap().id, expected_id);

        let result = ListAccount::new().paginate().get_all(client).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap().id, expected_id);
    });
}

#[test]
fn create_account() {
    mock::with_client(|client| {
        let mut create = CreateAccount::new();
        let mut capabilities = CreateAccountCapabilities::new();
        capabilities.acss_debit_payments =
            Some(CreateAccountCapabilitiesAcssDebitPayments { requested: Some(true) });
        create.capabilities = Some(capabilities);
        let result = create.send(client).unwrap();
        assert_eq!(result.email, Some("site@stripe.com".to_string()));
        assert_eq!(
            result.capabilities.unwrap().card_payments,
            Some(AccountCapabilitiesCardPayments::Active)
        );
    });
}
