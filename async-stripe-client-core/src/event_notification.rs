use crate::{CustomizableStripeRequest, RequestBuilder, StripeMethod};

/// A thin notification carrying identifiers rather than a resource snapshot.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct EventNotification {
    /// ID of the complete event, retrievable through the Events v2 API.
    pub id: String,
    /// Event type; unknown future types remain available to the caller.
    #[serde(rename = "type")]
    pub event_type: String,
    /// RFC 3339 creation time.
    pub created: String,
    /// Whether the notification belongs to live mode.
    pub livemode: bool,
    /// Authentication context to preserve when fetching related objects.
    pub context: Option<String>,
    /// Reference to the associated resource, when present.
    pub related_object: Option<RelatedObject>,
}

/// An API resource identified by a thin event notification.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct RelatedObject {
    /// Resource ID without a dependency on its model crate.
    pub id: String,
    /// Qualified resource type.
    #[serde(rename = "type")]
    pub object_type: String,
    /// Relative API target, including its namespace.
    pub url: String,
}

impl EventNotification {
    /// Parse the notification after the webhook signature has been verified.
    ///
    /// # Errors
    /// Returns the JSON decoding error for an invalid notification.
    pub fn parse(payload: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(payload)
    }

    /// Build a typed related-resource request, preserving authentication context.
    /// The caller selects the model and dated API revision explicitly.
    pub fn related_request<T>(
        &self,
        api_version: impl Into<String>,
    ) -> Option<CustomizableStripeRequest<T>> {
        let related = self.related_object.as_ref()?;
        let mut request = RequestBuilder::new(StripeMethod::Get, &related.url)
            .api_version(api_version)
            .customize();
        if let Some(context) = &self.context {
            request = request.context(context);
        }
        Some(request)
    }
}
