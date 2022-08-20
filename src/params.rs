use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::StripeError;
use crate::resources::ApiVersion;
use crate::{
    client::{
        config::{err, ok},
        Client, Response,
    },
    AccountId, ApplicationId,
};

#[derive(Clone, Default)]
pub struct AppInfo {
    pub name: String,
    pub url: Option<String>,
    pub version: Option<String>,
}

impl ToString for AppInfo {
    /// Formats a plugin's 'App Info' into a string that can be added to the end of an User-Agent string.
    ///
    /// This formatting matches that of other libraries, and if changed then it should be changed everywhere.
    fn to_string(&self) -> String {
        match (&self.version, &self.url) {
            (Some(a), Some(b)) => format!("{}/{} ({})", &self.name, a, b),
            (Some(a), None) => format!("{}/{}", &self.name, a),
            (None, Some(b)) => format!("{} ({})", &self.name, b),
            _ => self.name.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Headers {
    pub stripe_version: ApiVersion,
    pub user_agent: String,

    pub client_id: Option<ApplicationId>,
    pub stripe_account: Option<AccountId>,
}

impl Headers {
    pub fn to_array(&self) -> [(&str, Option<&str>); 4] {
        [
            ("Client-Id", self.client_id.as_deref()),
            ("Stripe-Account", self.stripe_account.as_deref()),
            ("Stripe-Version", Some(self.stripe_version.as_str())),
            ("User-Agent", Some(&self.user_agent)),
        ]
    }
}

/// Implemented by types which represent stripe objects.
pub trait Object {
    /// The canonical id type for this object.
    type Id;
    /// The id of the object.
    fn id(&self) -> Self::Id;
    /// The object's type, typically represented in wire format as the `object` property.
    fn object(&self) -> &'static str;
}

/// A deleted object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deleted<T> {
    /// Unique identifier for the object.
    pub id: T,
    /// Always true for a deleted object.
    pub deleted: bool,
}

/// The `Expand` struct is used to serialize `expand` arguments in retrieve and list apis.
#[doc(hidden)]
#[derive(Serialize)]
pub struct Expand<'a> {
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl Expand<'_> {
    pub(crate) fn is_empty(expand: &[&str]) -> bool {
        expand.is_empty()
    }
}

/// An id or object.
///
/// By default stripe will return an id for most fields, but if more detail is
/// necessary the `expand` parameter can be provided to ask for the id to be
/// loaded as an object instead:
///
/// ```rust,ignore
/// Charge::retrieve(&client, &charge_id, &["invoice.customer"])
/// ```
///
/// For more details see <https://stripe.com/docs/api/expanding_objects>.
#[derive(Clone, Debug, Serialize, Deserialize)] // TODO: Implement deserialize by hand for better error messages
#[serde(untagged)]
pub enum Expandable<T: Object> {
    Id(T::Id),
    Object(Box<T>),
}

impl<T> Expandable<T>
where
    T: Object,
    T::Id: Clone + Default,
{
    pub fn id(&self) -> T::Id {
        match self {
            Expandable::Id(id) => id.clone(),
            Expandable::Object(obj) => obj.id(),
        }
    }
}

impl<T: Object> Default for Expandable<T>
where
    T::Id: Default,
{
    fn default() -> Self {
        Expandable::Id(Default::default())
    }
}

impl<T: Object> Expandable<T> {
    pub fn is_object(&self) -> bool {
        match self {
            Expandable::Id(_) => false,
            Expandable::Object(_) => true,
        }
    }

    pub fn as_object(&self) -> Option<&T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(obj),
        }
    }

    pub fn into_object(self) -> Option<T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(*obj),
        }
    }
}

/// Implemented by types which support cursor-based pagination,
/// typically with an id, allowing them to be fetched using a `List`
/// returned by the corresponding "list" api request.
pub trait Paginate {
    type Cursor: AsCursor;
    fn cursor(&self) -> Self::Cursor;
}

pub trait AsCursor: AsRef<str> {}

impl<'a> AsCursor for &'a str {}
impl AsCursor for String {}

impl<T> Paginate for T
where
    T: Object,
    T::Id: AsCursor,
{
    type Cursor = T::Id;
    fn cursor(&self) -> Self::Cursor {
        self.id()
    }
}

pub trait Paginable {
    type O: Object + Send;
    fn set_last(&mut self, item: Self::O);
}

#[derive(Debug)]
pub struct ListPaginator<T, P> {
    pub page: List<T>,
    pub params: P,
}

/// A single page of a cursor-paginated list of an object.
///
/// For more details, see <https://stripe.com/docs/api/pagination>
#[derive(Debug, Deserialize, Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: Option<u64>,
    pub url: String,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List { data: Vec::new(), has_more: false, total_count: None, url: String::new() }
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        List {
            data: self.data.clone(),
            has_more: self.has_more,
            total_count: self.total_count,
            url: self.url.clone(),
        }
    }
}

impl<T> List<T> {
    pub fn paginate<P>(self, params: P) -> ListPaginator<T, P> {
        ListPaginator { page: self, params }
    }
}

impl<
        T: Paginate + DeserializeOwned + Send + Sync + 'static + Clone + std::fmt::Debug,
        P: Clone + Serialize + Send + 'static + std::fmt::Debug,
    > ListPaginator<T, P>
where
    P: Paginable<O = T>,
{
    /// Repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// Stripe's default page size.
    ///
    /// Requires `feature = "blocking"`.
    #[cfg(feature = "blocking")]
    pub fn get_all(self, client: &Client) -> Response<Vec<T>> {
        let mut data = Vec::with_capacity(self.page.total_count.unwrap_or(0) as usize);
        let mut paginator = self;
        loop {
            if !paginator.page.has_more {
                data.extend(paginator.page.data.into_iter());
                break;
            }
            let next_paginator = paginator.next(client)?;
            data.extend(paginator.page.data.into_iter());
            paginator = next_paginator
        }
        Ok(data)
    }

    /// Get all values in this List, consuming self and lazily paginating until all values are fetched.
    ///
    /// This function repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// the page size specified in params, or Stripe's default page size if none is specified.
    ///
    /// ```no_run
    /// # use stripe::{Customer, ListCustomers, StripeError, Client};
    /// # use futures_util::TryStreamExt;
    /// # fn main() {
    /// # tokio_test::block_on(run());
    /// # }
    /// # async fn run() -> Result<(), StripeError> {
    /// # let client = Client::new("sk_test_123");
    /// # let params = ListCustomers { ..Default::default() };
    ///
    /// let list = Customer::list(&client, &params).await.unwrap().paginate(params);
    /// let mut stream = list.stream(&client);
    ///
    /// // take a value out from the stream
    /// if let Some(val) = stream.try_next().await? {
    ///     println!("GOT = {:?}", val);
    /// }
    ///
    /// // alternatively, you can use stream combinators
    /// let all_values = stream.try_collect::<Vec<_>>().await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Requires `feature = ["async", "stream"]`.
    #[cfg(all(feature = "async", feature = "stream"))]
    pub fn stream(
        mut self,
        client: &Client,
    ) -> impl futures_util::Stream<Item = Result<T, StripeError>> + Unpin {
        // We are going to be popping items off the end of the list, so we need to reverse it.
        self.page.data.reverse();

        Box::pin(futures_util::stream::unfold(Some((self, client.clone())), Self::unfold_stream))
    }

    /// unfold a single item from the stream
    #[cfg(all(feature = "async", feature = "stream"))]
    async fn unfold_stream(
        state: Option<(Self, Client)>,
    ) -> Option<(Result<T, StripeError>, Option<(Self, Client)>)> {
        let (mut paginator, client) = state?; // If none, we sent the last item in the last iteration

        if paginator.page.data.len() > 1 {
            return Some((Ok(paginator.page.data.pop()?), Some((paginator, client))));
            // We have more data on this page
        }

        if !paginator.page.has_more {
            return Some((Ok(paginator.page.data.pop()?), None)); // Final value of the stream, no errors
        }

        match paginator.next(&client).await {
            Ok(mut next_paginator) => {
                let data = paginator.page.data.pop()?;
                next_paginator.page.data.reverse();

                // Yield last value of thimuts page, the next page (and client) becomes the state
                Some((Ok(data), Some((next_paginator, client))))
            }
            Err(e) => Some((Err(e), None)), // We ran into an error. The last value of the stream will be the error.
        }
    }

    /// Fetch an additional page of data from stripe.
    pub fn next(&self, client: &Client) -> Response<Self> {
        if let Some(last) = self.page.data.last() {
            if self.page.url.starts_with("/v1/") {
                let path = self.page.url.trim_start_matches("/v1/").to_string(); // the url we get back is prefixed

                // clone the params and set the cursor
                let params_next = {
                    let mut p = self.params.clone();
                    p.set_last(last.clone());
                    p
                };

                println!("next");
                let page = client.get_query(&path, &params_next);

                ListPaginator::create_paginator(page, params_next)
            } else {
                err(StripeError::UnsupportedVersion)
            }
        } else {
            ok(ListPaginator {
                page: List {
                    data: Vec::new(),
                    has_more: false,
                    total_count: self.page.total_count,
                    url: self.page.url.clone(),
                },
                params: self.params.clone(),
            })
        }
    }

    /// Pin a new future which maps the result inside the page future into
    /// a ListPaginator
    #[cfg(feature = "async")]
    fn create_paginator(page: Response<List<T>>, params: P) -> Response<Self> {
        use futures_util::FutureExt;
        Box::pin(page.map(|page| page.map(|page| ListPaginator { page, params })))
    }

    #[cfg(feature = "blocking")]
    fn create_paginator(page: Response<List<T>>, params: P) -> Response<Self> {
        page.map(|page| ListPaginator { page, params })
    }
}

pub type Metadata = HashMap<String, String>;
pub type Timestamp = i64;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RangeBounds<T> {
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
}

impl<T> Default for RangeBounds<T> {
    fn default() -> Self {
        RangeBounds { gt: None, gte: None, lt: None, lte: None }
    }
}

/// A set of generic request parameters that can be used on
/// list endpoints to filter their results by some timestamp.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RangeQuery<T> {
    Exact(T),
    Bounds(RangeBounds<T>),
}

impl<T> RangeQuery<T> {
    /// Filter results to exactly match a given value
    pub fn eq(value: T) -> RangeQuery<T> {
        RangeQuery::Exact(value)
    }

    /// Filter results to be after a given value
    pub fn gt(value: T) -> RangeQuery<T> {
        RangeQuery::Bounds(RangeBounds { gt: Some(value), ..Default::default() })
    }

    /// Filter results to be after or equal to a given value
    pub fn gte(value: T) -> RangeQuery<T> {
        RangeQuery::Bounds(RangeBounds { gte: Some(value), ..Default::default() })
    }

    /// Filter results to be before to a given value
    pub fn lt(value: T) -> RangeQuery<T> {
        RangeQuery::Bounds(RangeBounds { lt: Some(value), ..Default::default() })
    }

    /// Filter results to be before or equal to a given value
    pub fn lte(value: T) -> RangeQuery<T> {
        RangeQuery::Bounds(RangeBounds { lte: Some(value), ..Default::default() })
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum IdOrCreate<'a, T> {
    Id(&'a str),
    Create(&'a T),
}

// NOTE: Only intended to handle conversion from ASCII CamelCase to SnakeCase
//   This function is used to convert static Rust identifiers to snakecase
// TODO: pub(crate) fn
pub fn to_snakecase(camel: &str) -> String {
    let mut i = 0;
    let mut snake = String::new();
    let mut chars = camel.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if i > 0 && !chars.peek().unwrap_or(&'A').is_uppercase() {
                snake.push('_');
            }
            snake.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            snake.push(ch);
        }
        i += 1;
    }

    snake
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_snakecase() {
        use super::to_snakecase;

        assert_eq!(to_snakecase("snake_case").as_str(), "snake_case");
        assert_eq!(to_snakecase("CamelCase").as_str(), "camel_case");
        assert_eq!(to_snakecase("XMLHttpRequest").as_str(), "xml_http_request");
        assert_eq!(to_snakecase("UPPER").as_str(), "upper");
        assert_eq!(to_snakecase("lower").as_str(), "lower");
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn list() {
        use httpmock::Method::GET;
        use httpmock::MockServer;

        use crate::Client;
        use crate::{Customer, ListCustomers};

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        let client = Client::from_url(&*server.url("/"), "fake_key");

        let next_item = server.mock(|when, then| {
            when.method(GET).path("/v1/customers").query_param("starting_after", "cus_1");
            then.status(200).body(
                r#"{"object": "list", "data": [{
                "id": "cus_2",
                "object": "customer",
                "balance": 0,
                "created": 1649316731,
                "currency": "gbp",
                "delinquent": false,
                "email": null,
                "invoice_prefix": "4AF7482",
                "invoice_settings": {},
                "livemode": false,
                "metadata": {},
                "preferred_locales": [],
                "tax_exempt": "none"
              }], "has_more": false, "url": "/v1/customers"}"#,
            );
        });

        let first_item = server.mock(|when, then| {
            when.method(GET).path("/v1/customers");
            then.status(200).body(
                r#"{"object": "list", "data": [{
                "id": "cus_1",
                "object": "customer",
                "balance": 0,
                "created": 1649316731,
                "currency": "gbp",
                "delinquent": false,
                "invoice_prefix": "4AF7482",
                "invoice_settings": {},
                "livemode": false,
                "metadata": {},
                "preferred_locales": [],
                "tax_exempt": "none"
              }], "has_more": true, "url": "/v1/customers"}"#,
            );
        });

        let params = ListCustomers::new();
        let res = Customer::list(&client, &params).await.unwrap().paginate(params);

        println!("{:?}", res);

        let res2 = res.next(&client).await.unwrap();

        println!("{:?}", res2);

        first_item.assert_hits_async(1).await;
        next_item.assert_hits_async(1).await;
    }

    #[cfg(all(feature = "async", feature = "stream"))]
    #[tokio::test]
    async fn stream() {
        use futures_util::StreamExt;
        use httpmock::Method::GET;
        use httpmock::MockServer;

        use crate::Client;
        use crate::{Customer, ListCustomers};

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        let client = Client::from_url(&*server.url("/"), "fake_key");

        let next_item = server.mock(|when, then| {
            when.method(GET).path("/v1/customers").query_param("starting_after", "cus_1");
            then.status(200).body(
                r#"{"object": "list", "data": [{
                "id": "cus_2",
                "object": "customer",
                "balance": 0,
                "created": 1649316731,
                "currency": "gbp",
                "delinquent": false,
                "email": null,
                "invoice_prefix": "4AF7482",
                "invoice_settings": {},
                "livemode": false,
                "metadata": {},
                "preferred_locales": [],
                "tax_exempt": "none"
              }], "has_more": false, "url": "/v1/customers"}"#,
            );
        });

        let first_item = server.mock(|when, then| {
            when.method(GET).path("/v1/customers");
            then.status(200).body(
                r#"{"object": "list", "data": [{
                "id": "cus_1",
                "object": "customer",
                "balance": 0,
                "created": 1649316731,
                "currency": "gbp",
                "delinquent": false,
                "invoice_prefix": "4AF7482",
                "invoice_settings": {},
                "livemode": false,
                "metadata": {},
                "preferred_locales": [],
                "tax_exempt": "none"
              }], "has_more": true, "url": "/v1/customers"}"#,
            );
        });

        let params = ListCustomers::new();
        let res = Customer::list(&client, &params).await.unwrap().paginate(params);

        let stream = res.stream(&client).collect::<Vec<_>>().await;

        println!("{:#?}", stream);
        assert_eq!(stream.len(), 2);

        first_item.assert_hits_async(1).await;
        next_item.assert_hits_async(1).await;
    }
}
