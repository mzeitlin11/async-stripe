use std::collections::BTreeMap;
#[cfg(feature = "v2")]
use std::marker::PhantomData;

use crate::{CustomizableStripeRequest, RequestBuilder, StripeMethod};

/// An invalid notification or an incompatible typed event selection.
#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    /// The notification was not valid JSON with the required envelope fields.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// A snapshot or another object was supplied instead of a thin notification.
    #[error("expected v2.core.event, got {0}")]
    UnexpectedObject(String),
    /// The notification does not describe the selected event type.
    #[error("expected event {expected}, got {actual}")]
    UnexpectedType {
        /// Selected event type.
        expected: &'static str,
        /// Delivered event type.
        actual: String,
    },
    /// The related resource tag does not match this event's schema.
    #[error("expected related object {expected}, got {actual}")]
    UnexpectedRelatedObject {
        /// Expected qualified resource tag.
        expected: &'static str,
        /// Delivered resource tag.
        actual: String,
    },
    /// The original delivery could not be authenticated.
    #[cfg(feature = "webhook-signatures")]
    #[error(transparent)]
    Signature(#[from] crate::SignatureError),
}

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
    /// Object tag, when supplied by the delivery.
    pub object: Option<String>,
    /// Reason the event was emitted.
    pub reason: Option<EventReason>,
    /// Additional notification properties retained for unknown future event types.
    #[serde(flatten)]
    pub additional_fields: BTreeMap<String, serde_json::Value>,
}

/// The request that caused an event.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct EventReasonRequest {
    /// Originating API request ID.
    pub id: String,
    /// Idempotency key used for the originating request.
    pub idempotency_key: Option<String>,
}

/// Why an event was emitted.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct EventReason {
    /// Reason tag, preserving unknown future values.
    #[serde(rename = "type")]
    pub reason_type: String,
    /// Originating request, when available.
    pub request: Option<EventReasonRequest>,
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

/// Generated metadata linking a notification type to its fetched event model.
/// This association creates no dependency on the related resource's DTO.
#[cfg(feature = "v2")]
pub trait ThinEvent: miniserde::Deserialize {
    /// Qualified event type from `x-stripeEvent`.
    const EVENT_TYPE: &'static str;
    /// Dated revision used to decode the fetched event.
    const API_VERSION: &'static str;
    /// Related resource tag, when the schema declares one.
    const RELATED_OBJECT_TYPE: Option<&'static str>;
}

/// A notification checked against the generated metadata for a fetched event.
#[cfg(feature = "v2")]
#[derive(Clone, Debug)]
pub struct TypedEventNotification<E> {
    notification: EventNotification,
    event: PhantomData<fn() -> E>,
}

impl EventNotification {
    /// Parse a notification after authenticating its delivery separately.
    ///
    /// # Errors
    /// Rejects invalid JSON and snapshot event envelopes.
    pub fn parse(payload: &str) -> Result<Self, NotificationError> {
        let notification: Self = serde_json::from_str(payload)?;
        if let Some(object) = &notification.object
            && object != "v2.core.event"
        {
            return Err(NotificationError::UnexpectedObject(object.clone()));
        }
        Ok(notification)
    }

    /// Verify the original body before parsing a thin notification.
    ///
    /// # Errors
    /// Returns signature verification or notification parsing errors.
    #[cfg(feature = "webhook-signatures")]
    pub fn parse_verified(
        payload: &str,
        header: &str,
        secret: &str,
    ) -> Result<Self, NotificationError> {
        crate::WebhookSignature::verify(payload.as_bytes(), header, secret)?;
        Self::parse(payload)
    }

    /// Check this envelope against a generated event's metadata.
    ///
    /// # Errors
    /// Rejects a different event type or related resource tag.
    #[cfg(feature = "v2")]
    pub fn typed<E: ThinEvent>(self) -> Result<TypedEventNotification<E>, NotificationError> {
        if self.event_type != E::EVENT_TYPE {
            return Err(NotificationError::UnexpectedType {
                expected: E::EVENT_TYPE,
                actual: self.event_type,
            });
        }
        if let (Some(expected), Some(related)) = (E::RELATED_OBJECT_TYPE, &self.related_object)
            && related.object_type != expected
        {
            return Err(NotificationError::UnexpectedRelatedObject {
                expected,
                actual: related.object_type.clone(),
            });
        }
        Ok(TypedEventNotification { notification: self, event: PhantomData })
    }

    /// Build a request for the complete versioned event, preserving context.
    #[cfg(feature = "v2")]
    pub fn event_request<T>(&self, api_version: impl Into<String>) -> CustomizableStripeRequest<T> {
        self.fetch_request(
            format!("/v2/core/events/{}", crate::encode_path_segment(&self.id)),
            api_version,
        )
    }

    /// Build a related-resource request, preserving context and namespace.
    /// The caller selects the model and dated API revision explicitly.
    pub fn related_request<T>(
        &self,
        api_version: impl Into<String>,
    ) -> Option<CustomizableStripeRequest<T>> {
        let related = self.related_object.as_ref()?;
        Some(self.fetch_request(&related.url, api_version))
    }

    fn fetch_request<T>(
        &self,
        url: impl Into<String>,
        api_version: impl Into<String>,
    ) -> CustomizableStripeRequest<T> {
        let mut request = RequestBuilder::new(StripeMethod::Get, url)
            .api_version(api_version)
            .event_trigger(&self.id)
            .customize();
        if let Some(context) = &self.context {
            request = request.context(context);
        }
        request
    }
}

#[cfg(feature = "v2")]
impl<E: ThinEvent> TypedEventNotification<E> {
    /// Access the authenticated or separately trusted notification metadata.
    pub fn notification(&self) -> &EventNotification {
        &self.notification
    }

    /// Fetch the full typed event using the generated dated API revision.
    pub fn event_request(&self) -> CustomizableStripeRequest<E> {
        self.notification.event_request(E::API_VERSION)
    }

    /// Fetch the current related resource using an explicitly selected output model.
    /// Resource-specific DTO dependencies are owned by the calling application.
    pub fn related_request<T>(&self) -> Option<CustomizableStripeRequest<T>> {
        self.notification.related_request(E::API_VERSION)
    }
}
