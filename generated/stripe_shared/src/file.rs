/// This object represents files hosted on Stripe's servers. You can upload
/// files with the [create file](https://stripe.com/docs/api#create_file) request
/// (for example, when uploading dispute evidence). Stripe also
/// creates files independetly (for example, the results of a [Sigma scheduled
/// query](#scheduled_queries)).
///
/// Related guide: [File upload guide](https://stripe.com/docs/file-upload)
///
/// For more details see <<https://stripe.com/docs/api/files/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct File {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The file expires and isn't available at this time in epoch seconds.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The suitable name for saving the file to a filesystem.
    pub filename: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::FileId,
    /// A list of [file links](https://stripe.com/docs/api#file_links) that point at this file.
    pub links: Option<stripe_types::List<stripe_shared::FileLink>>,
    /// The [purpose](https://stripe.com/docs/file-upload#uploading-a-file) of the uploaded file.
    pub purpose: stripe_shared::FilePurpose,
    /// The size of the file object in bytes.
    pub size: u64,
    /// A suitable title for the document.
    pub title: Option<String>,
    /// The returned file type (for example, `csv`, `pdf`, `jpg`, or `png`).
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: Option<String>,
    /// Use your live secret API key to download the file from this URL.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct FileBuilder {
    created: Option<stripe_types::Timestamp>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    filename: Option<Option<String>>,
    id: Option<stripe_shared::FileId>,
    links: Option<Option<stripe_types::List<stripe_shared::FileLink>>>,
    purpose: Option<stripe_shared::FilePurpose>,
    size: Option<u64>,
    title: Option<Option<String>>,
    type_: Option<Option<String>>,
    url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for File {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<File>,
        builder: FileBuilder,
    }

    impl Visitor for Place<File> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FileBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FileBuilder {
        type Out = File;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),
                "filename" => Ok(Deserialize::begin(&mut self.filename)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "links" => Ok(Deserialize::begin(&mut self.links)),
                "purpose" => Ok(Deserialize::begin(&mut self.purpose)),
                "size" => Ok(Deserialize::begin(&mut self.size)),
                "title" => Ok(Deserialize::begin(&mut self.title)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                expires_at: Deserialize::default(),
                filename: Deserialize::default(),
                id: Deserialize::default(),
                links: Deserialize::default(),
                purpose: Deserialize::default(),
                size: Deserialize::default(),
                title: Deserialize::default(),
                type_: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let expires_at = self.expires_at.take()?;
            let filename = self.filename.take()?;
            let id = self.id.take()?;
            let links = self.links.take()?;
            let purpose = self.purpose.take()?;
            let size = self.size.take()?;
            let title = self.title.take()?;
            let type_ = self.type_.take()?;
            let url = self.url.take()?;

            Some(Self::Out { created, expires_at, filename, id, links, purpose, size, title, type_, url })
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

    impl ObjectDeser for File {
        type Builder = FileBuilder;
    }
};
impl stripe_types::Object for File {
    type Id = stripe_shared::FileId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(FileId, "file_");
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FilePurpose {
    AccountRequirement,
    AdditionalVerification,
    BusinessIcon,
    BusinessLogo,
    CustomerSignature,
    DisputeEvidence,
    DocumentProviderIdentityDocument,
    FinanceReportRun,
    IdentityDocument,
    IdentityDocumentDownloadable,
    PciDocument,
    Selfie,
    SigmaScheduledQuery,
    TaxDocumentUserUpload,
    TerminalReaderSplashscreen,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl FilePurpose {
    pub fn as_str(self) -> &'static str {
        use FilePurpose::*;
        match self {
            AccountRequirement => "account_requirement",
            AdditionalVerification => "additional_verification",
            BusinessIcon => "business_icon",
            BusinessLogo => "business_logo",
            CustomerSignature => "customer_signature",
            DisputeEvidence => "dispute_evidence",
            DocumentProviderIdentityDocument => "document_provider_identity_document",
            FinanceReportRun => "finance_report_run",
            IdentityDocument => "identity_document",
            IdentityDocumentDownloadable => "identity_document_downloadable",
            PciDocument => "pci_document",
            Selfie => "selfie",
            SigmaScheduledQuery => "sigma_scheduled_query",
            TaxDocumentUserUpload => "tax_document_user_upload",
            TerminalReaderSplashscreen => "terminal_reader_splashscreen",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for FilePurpose {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FilePurpose::*;
        match s {
            "account_requirement" => Ok(AccountRequirement),
            "additional_verification" => Ok(AdditionalVerification),
            "business_icon" => Ok(BusinessIcon),
            "business_logo" => Ok(BusinessLogo),
            "customer_signature" => Ok(CustomerSignature),
            "dispute_evidence" => Ok(DisputeEvidence),
            "document_provider_identity_document" => Ok(DocumentProviderIdentityDocument),
            "finance_report_run" => Ok(FinanceReportRun),
            "identity_document" => Ok(IdentityDocument),
            "identity_document_downloadable" => Ok(IdentityDocumentDownloadable),
            "pci_document" => Ok(PciDocument),
            "selfie" => Ok(Selfie),
            "sigma_scheduled_query" => Ok(SigmaScheduledQuery),
            "tax_document_user_upload" => Ok(TaxDocumentUserUpload),
            "terminal_reader_splashscreen" => Ok(TerminalReaderSplashscreen),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for FilePurpose {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FilePurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FilePurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FilePurpose {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<FilePurpose> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FilePurpose::from_str(s).unwrap_or(FilePurpose::Unknown));
        Ok(())
    }
}
