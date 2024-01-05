#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EphemeralKey {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which the key will expire. Measured in seconds since the Unix epoch.
    pub expires: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::EphemeralKeyId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The key's secret. You can use this value to make authorized requests to the Stripe API.
    pub secret: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct EphemeralKeyBuilder {
    created: Option<stripe_types::Timestamp>,
    expires: Option<stripe_types::Timestamp>,
    id: Option<stripe_misc::EphemeralKeyId>,
    livemode: Option<bool>,
    secret: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for EphemeralKey {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<EphemeralKey>,
        builder: EphemeralKeyBuilder,
    }

    impl Visitor for Place<EphemeralKey> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: EphemeralKeyBuilder::deser_default() }))
        }
    }

    impl MapBuilder for EphemeralKeyBuilder {
        type Out = EphemeralKey;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "expires" => Ok(Deserialize::begin(&mut self.expires)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "secret" => Ok(Deserialize::begin(&mut self.secret)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { created: Deserialize::default(), expires: Deserialize::default(), id: Deserialize::default(), livemode: Deserialize::default(), secret: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let expires = self.expires.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let secret = self.secret.take()?;

            Some(Self::Out { created, expires, id, livemode, secret })
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

    impl ObjectDeser for EphemeralKey {
        type Builder = EphemeralKeyBuilder;
    }
};
impl stripe_types::Object for EphemeralKey {
    type Id = stripe_misc::EphemeralKeyId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(EphemeralKeyId, "ephkey_");
