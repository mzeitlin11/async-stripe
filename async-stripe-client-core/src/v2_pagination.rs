use std::collections::VecDeque;
use std::time::Duration;

use crate::{
    CustomizableStripeRequest, RequestBuilder, RequestStrategy, StripeBlockingClient, StripeClient,
    StripeClientErr,
};

/// A concrete v2 list response with server-provided pagination links.
pub trait V2Page: miniserde::Deserialize {
    /// The resource on each page.
    type Item;
    /// Consume the page, retaining its next-page target.
    fn into_page(self) -> (Vec<Self::Item>, Option<String>);
}

/// A paginator that follows v2 links and preserves per-request configuration.
#[derive(Debug)]
pub struct V2Paginator<P: V2Page> {
    request: CustomizableStripeRequest<P>,
}

impl<P: V2Page> V2Paginator<P> {
    /// Start from a generated v2 list request.
    pub fn new(request: RequestBuilder) -> Self {
        Self { request: request.customize() }
    }

    /// Set the authentication context for every page.
    pub fn context(mut self, context: impl Into<String>) -> Self {
        self.request = self.request.context(context);
        self
    }

    /// Set the connected account for every page.
    pub fn account_id(mut self, account: impl AsRef<str>) -> Self {
        self.request = self.request.account_id(account);
        self
    }

    /// Override the dated API revision for every page.
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.request = self.request.api_version(version);
        self
    }

    /// Set the request strategy for every page.
    pub fn request_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.request = self.request.request_strategy(strategy);
        self
    }

    /// Set the per-attempt timeout for every page.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.request = self.request.timeout(timeout);
        self
    }

    fn advance<E: StripeClientErr>(&mut self, next: String) -> Result<(), E> {
        if !next.starts_with("/v2/") {
            return Err(E::deserialize_err("v2 pagination requires a relative /v2/ target"));
        }
        let request = &mut self.request.inner.request;
        request.path = next;
        // The server's link already contains the filters and page token.
        request.query = None;
        request.url("https://api.stripe.com").map_err(E::deserialize_err)?;
        Ok(())
    }

    /// Fetch all pages using a blocking client.
    ///
    /// # Errors
    /// Propagates HTTP/decoding errors and invalid next-page targets.
    pub fn get_all<C: StripeBlockingClient>(mut self, client: &C) -> Result<Vec<P::Item>, C::Err> {
        let mut all = Vec::new();
        loop {
            let page = CustomizableStripeRequest::<P>::new(self.request.inner.request.clone());
            let mut page = page;
            page.inner.config_override = self.request.inner.config_override.clone();
            let (mut data, next) = page.send_blocking(client)?.into_page();
            all.append(&mut data);
            let Some(next) = next else { return Ok(all) };
            self.advance::<C::Err>(next)?;
        }
    }

    /// Stream resources, including across empty intermediate pages.
    pub fn stream<C: StripeClient + Clone>(
        self,
        client: &C,
    ) -> impl futures_util::Stream<Item = Result<P::Item, C::Err>> + Unpin {
        Box::pin(futures_util::stream::unfold(
            Some((self, client.clone(), VecDeque::new(), false)),
            |state| async move {
                let (mut paginator, client, mut data, mut finished) = state?;
                loop {
                    if let Some(item) = data.pop_front() {
                        return Some((Ok(item), Some((paginator, client, data, finished))));
                    }
                    if finished {
                        return None;
                    }
                    let request = paginator.request.inner.clone();
                    let mut page = CustomizableStripeRequest::<P>::new(request.request);
                    page.inner.config_override = request.config_override;
                    let page = match page.send(&client).await {
                        Ok(page) => page,
                        Err(error) => return Some((Err(error), None)),
                    };
                    let (items, next) = page.into_page();
                    data = items.into();
                    if let Some(next) = next {
                        if let Err(error) = paginator.advance::<C::Err>(next) {
                            return Some((Err(error), None));
                        }
                    } else {
                        finished = true;
                    }
                }
            },
        ))
    }
}
