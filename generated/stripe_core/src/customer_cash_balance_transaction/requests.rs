
/// Retrieves a specific cash balance transaction, which updated the customer’s [cash balance](https://stripe.com/docs/payments/customer-balance).
pub fn retrieve(
    client: &stripe::Client,
    customer: &stripe_types::customer::CustomerId,
    transaction: &str,
    params: RetrieveCustomerCashBalanceTransaction,
) -> stripe::Response<stripe_types::customer_cash_balance_transaction::CustomerCashBalanceTransaction>
{
    client.get_query(
        &format!(
            "/customers/{customer}/cash_balance_transactions/{transaction}",
            customer = customer,
            transaction = transaction
        ),
        params,
    )
}
/// Returns a list of transactions that modified the customer’s [cash balance](https://stripe.com/docs/payments/customer-balance).
pub fn list(
    client: &stripe::Client,
    customer: &stripe_types::customer::CustomerId,
    params: ListCustomerCashBalanceTransaction,
) -> stripe::Response<
    stripe_types::List<
        stripe_types::customer_cash_balance_transaction::CustomerCashBalanceTransaction,
    >,
> {
    client.get_query(
        &format!("/customers/{customer}/cash_balance_transactions", customer = customer),
        params,
    )
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCustomerCashBalanceTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCustomerCashBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListCustomerCashBalanceTransaction<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListCustomerCashBalanceTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
