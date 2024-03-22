/// You can configure [webhook endpoints](https://stripe.com/docs/webhooks/) via the API to be
/// notified about events that happen in your Stripe account or connected
/// accounts.
///
/// Most users configure webhooks from [the dashboard](https://dashboard.stripe.com/webhooks), which provides a user interface for registering and testing your webhook endpoints.
///
/// Related guide: [Setting up webhooks](https://stripe.com/docs/webhooks/configure)
///
/// For more details see <<https://stripe.com/docs/api/webhook_endpoints/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct WebhookEndpoint {
    /// The API version events are rendered as for this webhook endpoint.
    pub api_version: Option<String>,
    /// The ID of the associated Connect application.
    pub application: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// An optional description of what the webhook is used for.
    pub description: Option<String>,
    /// The list of events to enable for this endpoint.
    /// `['*']` indicates that all events are enabled, except those that require explicit selection.
    pub enabled_events: Vec<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::WebhookEndpointId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures).
    /// Only returned at creation.
    pub secret: Option<String>,
    /// The status of the webhook. It can be `enabled` or `disabled`.
    pub status: String,
    /// The URL of the webhook endpoint.
    pub url: String,
}
#[cfg(feature = "min-ser")]
pub struct WebhookEndpointBuilder {
    api_version: Option<Option<String>>,
    application: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    description: Option<Option<String>>,
    enabled_events: Option<Vec<String>>,
    id: Option<stripe_misc::WebhookEndpointId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    secret: Option<Option<String>>,
    status: Option<String>,
    url: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for WebhookEndpoint {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<WebhookEndpoint>,
        builder: WebhookEndpointBuilder,
    }

    impl Visitor for Place<WebhookEndpoint> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: WebhookEndpointBuilder::deser_default() }))
        }
    }

    impl MapBuilder for WebhookEndpointBuilder {
        type Out = WebhookEndpoint;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "api_version" => Ok(Deserialize::begin(&mut self.api_version)),
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "enabled_events" => Ok(Deserialize::begin(&mut self.enabled_events)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "secret" => Ok(Deserialize::begin(&mut self.secret)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                api_version: Deserialize::default(),
                application: Deserialize::default(),
                created: Deserialize::default(),
                description: Deserialize::default(),
                enabled_events: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                secret: Deserialize::default(),
                status: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let api_version = self.api_version.take()?;
            let application = self.application.take()?;
            let created = self.created.take()?;
            let description = self.description.take()?;
            let enabled_events = self.enabled_events.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let secret = self.secret.take()?;
            let status = self.status.take()?;
            let url = self.url.take()?;

            Some(Self::Out { api_version, application, created, description, enabled_events, id, livemode, metadata, secret, status, url })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for WebhookEndpoint {
        type Builder = WebhookEndpointBuilder;
    }
};
impl stripe_types::Object for WebhookEndpoint {
    type Id = stripe_misc::WebhookEndpointId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(WebhookEndpointId, "we_");
