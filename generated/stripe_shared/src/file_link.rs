/// To share the contents of a `File` object with non-Stripe users, you can
/// create a `FileLink`. `FileLink`s contain a URL that you can use to
/// retrieve the contents of the file without authentication.
///
/// For more details see <<https://stripe.com/docs/api/file_links/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FileLink {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Returns if the link is already expired.
    pub expired: bool,
    /// Time that the link expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The file object this link points to.
    pub file: stripe_types::Expandable<stripe_shared::File>,
    /// Unique identifier for the object.
    pub id: stripe_shared::FileLinkId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The publicly accessible URL to download the file.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct FileLinkBuilder {
    created: Option<stripe_types::Timestamp>,
    expired: Option<bool>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    file: Option<stripe_types::Expandable<stripe_shared::File>>,
    id: Option<stripe_shared::FileLinkId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FileLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FileLink>,
        builder: FileLinkBuilder,
    }

    impl Visitor for Place<FileLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FileLinkBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FileLinkBuilder {
        type Out = FileLink;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "expired" => Ok(Deserialize::begin(&mut self.expired)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "file" => Ok(Deserialize::begin(&mut self.file)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                expired: Deserialize::default(),
                expires_at: Deserialize::default(),
                file: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let expired = self.expired.take()?;
            let expires_at = self.expires_at.take()?;
            let file = self.file.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let url = self.url.take()?;

            Some(Self::Out { created, expired, expires_at, file, id, livemode, metadata, url })
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

    impl ObjectDeser for FileLink {
        type Builder = FileLinkBuilder;
    }
};
impl stripe_types::Object for FileLink {
    type Id = stripe_shared::FileLinkId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(FileLinkId, "link_");
