use stripe_core::customer::{
    CreateCustomer, DeleteCustomer, RetrieveCustomer, RetrieveCustomerReturned,
};

use crate::mock;

fn customer_create_and_delete(client: &stripe::Client) {
    // NB: the create step is not required for deletion to work since the stripe mock is stateless
    let customer = CreateCustomer::new().send(client).unwrap();
    let result = DeleteCustomer::new().send(client, &customer.id).unwrap();
    assert_eq!(result.id, customer.id);
}

#[test]
fn customer_create_and_delete_without_account() {
    mock::with_client(|client| {
        customer_create_and_delete(client);
    });
}

#[test]
fn customer_create_and_delete_with_account() {
    mock::with_client(|client| {
        let client = client
            .to_owned()
            .with_client_id("ca_123".parse().unwrap())
            .with_stripe_account("acct_123".parse().unwrap());
        customer_create_and_delete(&client);
    });
}

#[test]
fn retrieve_customer() {
    mock::with_client(|client| {
        let id = "cus_123".parse().unwrap();
        let ret = RetrieveCustomer::new().send(client, &id).unwrap();
        match ret {
            RetrieveCustomerReturned::Customer(cust) => {
                assert_eq!(cust.id, id);
                assert_eq!(cust.invoice_prefix, Some("D331735".into()));
                assert_eq!(cust.created, 1234567890);
            }
            RetrieveCustomerReturned::DeletedCustomer(_) => panic!("expected non-deleted response"),
        }
    })
}
