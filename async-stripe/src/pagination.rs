// Necessary under tokio-blocking since `Response` is a type alias to a `Result`
#![allow(clippy::missing_errors_doc)]
use serde::de::DeserializeOwned;
use serde::Serialize;
use stripe_types::{AsCursorOpt, List, Object};

use crate::{Client, Response};

/// Should only be implemented by `List*` parameter types. Kept public so that
/// the generated code crates can access this trait.
#[doc(hidden)]
pub trait PaginationParams: Serialize {}

#[derive(Debug)]
pub struct ListPaginator<T> {
    data: Vec<T>,
    url: String,
    has_more: bool,
    total_count: Option<u64>,
    params: serde_json::Value,
}

pub trait PaginationExt<T> {
    fn into_paginator(self) -> ListPaginator<T>;
}

impl<T> PaginationExt<T> for List<T>
where
    T: Object + DeserializeOwned + Send + Sync + 'static,
    T::Id: ToString,
{
    fn into_paginator(self) -> ListPaginator<T> {
        let mut paginator = ListPaginator {
            data: self.data,
            // the url we get back is prefixed
            url: self.url.trim_start_matches("/v1/").to_string(),
            has_more: self.has_more,
            total_count: self.total_count,
            params: Default::default(),
        };
        if let Some(curr_cursor) = paginator.data.last().and_then(|t| t.id().as_cursor_opt()) {
            paginator.update_cursor(curr_cursor.to_string());
        }
        paginator
    }
}

impl<T> ListPaginator<T> {
    /// Kept public so that the generated code crates can access this trait. Used by `List*` params
    /// to implement `PaginationExt`.
    #[doc(hidden)]
    pub fn from_params(url: &str, params: impl PaginationParams) -> Self {
        ListPaginator {
            data: vec![],
            url: url.to_string(),
            has_more: true,
            total_count: None,
            params: serde_json::to_value(params).expect("Invalid pagination params"),
        }
    }
}

impl<T> ListPaginator<T>
where
    T: Object + DeserializeOwned + Send + Sync + 'static,
{
    /// Repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// Stripe's default page size.
    ///
    /// Requires `feature = "blocking"`.
    #[cfg(feature = "blocking")]
    pub fn get_all(self, client: &Client) -> Response<Vec<T>> {
        let mut data = Vec::with_capacity(self.total_count.unwrap_or(0) as usize);
        let mut paginator = self;
        loop {
            if !paginator.has_more {
                data.extend(paginator.data);
                break;
            }
            let next_page = paginator.fetch_page_with_curr_params(client)?;
            paginator.update_with_new_data(next_page);
        }
        Ok(data)
    }

    /// Get all values in this List, consuming self and lazily paginating until all values are fetched.
    ///
    /// This function repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// the page size specified in params, or Stripe's default page size if none is specified.
    ///
    ///
    /// Requires `feature = ["async", "stream"]`.
    #[cfg(all(feature = "async", feature = "stream"))]
    pub fn stream(
        mut self,
        client: &Client,
    ) -> impl futures_util::Stream<Item = Result<T, crate::StripeError>> + Unpin {
        // We are going to be popping items off the end of the list, so we need to reverse it.
        self.data.reverse();

        Box::pin(futures_util::stream::unfold(Some((self, client.clone())), Self::unfold_stream))
    }

    /// unfold a single item from the stream
    #[cfg(all(feature = "async", feature = "stream"))]
    async fn unfold_stream(
        state: Option<(Self, Client)>,
    ) -> Option<(Result<T, crate::StripeError>, Option<(Self, Client)>)> {
        let (mut paginator, client) = state?; // If none, we sent the last item in the last iteration

        if let Some(next_val) = paginator.data.pop() {
            // We have more data on this page
            return Some((Ok(next_val), Some((paginator, client))));
        }

        // Final value of the stream, no errors
        if !paginator.has_more {
            return None;
        }

        match paginator.fetch_page_with_curr_params(&client).await {
            Ok(next_page) => {
                debug_assert!(paginator.data.is_empty());
                paginator.update_with_new_data(next_page);

                // We are going to be popping items off the end of the list, so we need to reverse it.
                // The assert above ensures we are only reversing this specific list we've
                // just received
                paginator.data.reverse();

                let next_val = paginator.data.pop()?;

                // Yield last value of this page, the next page (and client) becomes the state
                Some((Ok(next_val), Some((paginator, client))))
            }
            Err(e) => Some((Err(e), None)), // We ran into an error. The last value of the stream will be the error.
        }
    }

    fn fetch_page_with_curr_params(&self, client: &Client) -> Response<List<T>> {
        client.get_query(&self.url, &self.params)
    }

    fn update_cursor(&mut self, id: String) {
        self.params["starting_after"] = serde_json::Value::String(id);
    }

    fn update_with_new_data(&mut self, list: List<T>) {
        self.has_more = list.has_more;
        self.total_count = list.total_count;
        if let Some(new_cursor) = list.data.last().and_then(|l| l.id().as_cursor_opt()) {
            self.update_cursor(new_cursor.to_string());
        } else {
            self.has_more = false;
        }
        self.data.extend(list.data);
    }
}
