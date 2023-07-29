#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedWebhookEndpoint {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_misc::webhook_endpoint::WebhookEndpointId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedWebhookEndpointObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedWebhookEndpoint {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedWebhookEndpointObject {
    WebhookEndpoint,
}

impl DeletedWebhookEndpointObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WebhookEndpoint => "webhook_endpoint",
        }
    }
}

impl std::str::FromStr for DeletedWebhookEndpointObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "webhook_endpoint" => Ok(Self::WebhookEndpoint),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedWebhookEndpointObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedWebhookEndpointObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedWebhookEndpointObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedWebhookEndpointObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedWebhookEndpointObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedWebhookEndpointObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<DeletedWebhookEndpointObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedWebhookEndpointObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedWebhookEndpoint {
    type Id = stripe_misc::webhook_endpoint::WebhookEndpointId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
