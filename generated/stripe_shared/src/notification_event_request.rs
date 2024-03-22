#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NotificationEventRequest {
    /// ID of the API request that caused the event.
    /// If null, the event was automatic (e.g., Stripe's automatic subscription handling).
    /// Request logs are available in the [dashboard](https://dashboard.stripe.com/logs), but currently not in the API.
    pub id: Option<String>,
    /// The idempotency key transmitted during the request, if any.
    /// *Note: This property is populated only for events on or after May 23, 2017*.
    pub idempotency_key: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct NotificationEventRequestBuilder {
    id: Option<Option<String>>,
    idempotency_key: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for NotificationEventRequest {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<NotificationEventRequest>,
        builder: NotificationEventRequestBuilder,
    }

    impl Visitor for Place<NotificationEventRequest> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: NotificationEventRequestBuilder::deser_default() }))
        }
    }

    impl MapBuilder for NotificationEventRequestBuilder {
        type Out = NotificationEventRequest;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "idempotency_key" => Ok(Deserialize::begin(&mut self.idempotency_key)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), idempotency_key: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let id = self.id.take()?;
            let idempotency_key = self.idempotency_key.take()?;

            Some(Self::Out { id, idempotency_key })
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

    impl ObjectDeser for NotificationEventRequest {
        type Builder = NotificationEventRequestBuilder;
    }
};
