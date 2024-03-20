//! This example shows how to make a request with a specific `RequestStrategy`.
use stripe::{Client, RequestStrategy, StripeError};
use stripe_core::customer::ListCustomer;

pub async fn run_strategy_example() -> Result<(), StripeError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key).with_strategy(RequestStrategy::idempotent_with_uuid());
    let first_page = ListCustomer::new().send(&client).await?;

    println!(
        "first page of customers: {:#?}",
        first_page.data.iter().map(|c| c.name.as_ref().unwrap()).collect::<Vec<_>>()
    );
    Ok(())
}
