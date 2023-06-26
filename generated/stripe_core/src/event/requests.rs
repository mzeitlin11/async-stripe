impl stripe_core::event::Event {
    /// List events, going back up to 30 days.
    ///
    /// Each event data is rendered according to Stripe API version at its creation time, specified in [event object](https://stripe.com/docs/api/events/object) `api_version` attribute (not according to your current Stripe API version or `Stripe-Version` header).
    pub fn list(
        client: &stripe::Client,
        params: ListEvent,
    ) -> stripe::Response<stripe_types::List<stripe_core::event::Event>> {
        client.get_query("/events", params)
    }
    /// Retrieves the details of an event.
    ///
    /// Supply the unique identifier of the event, which you might have received in a webhook.
    pub fn retrieve(
        client: &stripe::Client,
        id: &str,
        params: RetrieveEvent,
    ) -> stripe::Response<stripe_core::event::Event> {
        client.get_query(&format!("/events/{id}", id = id), params)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListEvent<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Filter events by whether all webhooks were successfully delivered.
    ///
    /// If false, events which are still pending or have failed all delivery attempts to a webhook endpoint will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_success: Option<bool>,
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
    /// A string containing a specific event name, or group of events using * as a wildcard.
    ///
    /// The list will be filtered to include only events with a matching event property.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
    /// An array of up to 20 strings containing specific event names.
    ///
    /// The list will be filtered to include only events with a matching event property.
    /// You may pass either `type` or `types`, but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<&'a [&'a str]>,
}
impl<'a> ListEvent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveEvent<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveEvent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
