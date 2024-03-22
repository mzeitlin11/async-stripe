#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedWebhookEndpoint {
    /// Always true for a deleted object
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_misc::WebhookEndpointId,
}
#[cfg(feature = "min-ser")]
pub struct DeletedWebhookEndpointBuilder {
    deleted: Option<stripe_types::AlwaysTrue>,
    id: Option<stripe_misc::WebhookEndpointId>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DeletedWebhookEndpoint {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DeletedWebhookEndpoint>,
        builder: DeletedWebhookEndpointBuilder,
    }

    impl Visitor for Place<DeletedWebhookEndpoint> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DeletedWebhookEndpointBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DeletedWebhookEndpointBuilder {
        type Out = DeletedWebhookEndpoint;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "deleted" => Ok(Deserialize::begin(&mut self.deleted)),
                "id" => Ok(Deserialize::begin(&mut self.id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { deleted: Deserialize::default(), id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let deleted = self.deleted.take()?;
            let id = self.id.take()?;

            Some(Self::Out { deleted, id })
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

    impl ObjectDeser for DeletedWebhookEndpoint {
        type Builder = DeletedWebhookEndpointBuilder;
    }
};
impl stripe_types::Object for DeletedWebhookEndpoint {
    type Id = stripe_misc::WebhookEndpointId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
