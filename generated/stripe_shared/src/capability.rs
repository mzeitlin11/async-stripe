/// This is an object representing a capability for a Stripe account.
///
/// Related guide: [Account capabilities](https://stripe.com/docs/connect/account-capabilities)
///
/// For more details see <<https://stripe.com/docs/api/capabilities/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Capability {
    /// The account for which the capability enables functionality.
    pub account: stripe_types::Expandable<stripe_shared::Account>,
    pub future_requirements: Option<stripe_shared::AccountCapabilityFutureRequirements>,
    /// The identifier for the capability.
    pub id: stripe_shared::CapabilityId,
    /// Whether the capability has been requested.
    pub requested: bool,
    /// Time at which the capability was requested. Measured in seconds since the Unix epoch.
    pub requested_at: Option<stripe_types::Timestamp>,
    pub requirements: Option<stripe_shared::AccountCapabilityRequirements>,
    /// The status of the capability. Can be `active`, `inactive`, `pending`, or `unrequested`.
    pub status: CapabilityStatus,
}
#[cfg(feature = "min-ser")]
pub struct CapabilityBuilder {
    account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    future_requirements: Option<Option<stripe_shared::AccountCapabilityFutureRequirements>>,
    id: Option<stripe_shared::CapabilityId>,
    requested: Option<bool>,
    requested_at: Option<Option<stripe_types::Timestamp>>,
    requirements: Option<Option<stripe_shared::AccountCapabilityRequirements>>,
    status: Option<CapabilityStatus>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Capability {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Capability>,
        builder: CapabilityBuilder,
    }

    impl Visitor for Place<Capability> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CapabilityBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CapabilityBuilder {
        type Out = Capability;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account" => Ok(Deserialize::begin(&mut self.account)),
                "future_requirements" => Ok(Deserialize::begin(&mut self.future_requirements)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "requested" => Ok(Deserialize::begin(&mut self.requested)),
                "requested_at" => Ok(Deserialize::begin(&mut self.requested_at)),
                "requirements" => Ok(Deserialize::begin(&mut self.requirements)),
                "status" => Ok(Deserialize::begin(&mut self.status)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                future_requirements: Deserialize::default(),
                id: Deserialize::default(),
                requested: Deserialize::default(),
                requested_at: Deserialize::default(),
                requirements: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account = self.account.take()?;
            let future_requirements = self.future_requirements.take()?;
            let id = self.id.take()?;
            let requested = self.requested.take()?;
            let requested_at = self.requested_at.take()?;
            let requirements = self.requirements.take()?;
            let status = self.status.take()?;

            Some(Self::Out { account, future_requirements, id, requested, requested_at, requirements, status })
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

    impl ObjectDeser for Capability {
        type Builder = CapabilityBuilder;
    }
};
/// The status of the capability. Can be `active`, `inactive`, `pending`, or `unrequested`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CapabilityStatus {
    Active,
    Disabled,
    Inactive,
    Pending,
    Unrequested,
}
impl CapabilityStatus {
    pub fn as_str(self) -> &'static str {
        use CapabilityStatus::*;
        match self {
            Active => "active",
            Disabled => "disabled",
            Inactive => "inactive",
            Pending => "pending",
            Unrequested => "unrequested",
        }
    }
}

impl std::str::FromStr for CapabilityStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CapabilityStatus::*;
        match s {
            "active" => Ok(Active),
            "disabled" => Ok(Disabled),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            "unrequested" => Ok(Unrequested),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CapabilityStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CapabilityStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilityStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CapabilityStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilityStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CapabilityStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilityStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Capability {
    type Id = stripe_shared::CapabilityId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CapabilityId);
