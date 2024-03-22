/// An Issuing `Cardholder` object represents an individual or business entity who is [issued](https://stripe.com/docs/issuing) cards.
///
/// Related guide: [How to create a cardholder](https://stripe.com/docs/issuing/cards#create-cardholder).
///
/// For more details see <<https://stripe.com/docs/api/issuing/cardholders/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardholder {
    pub billing: stripe_shared::IssuingCardholderAddress,
    /// Additional information about a `company` cardholder.
    pub company: Option<stripe_shared::IssuingCardholderCompany>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The cardholder's email address.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingCardholderId,
    /// Additional information about an `individual` cardholder.
    pub individual: Option<stripe_shared::IssuingCardholderIndividual>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The cardholder's name. This will be printed on cards issued to them.
    pub name: String,
    /// The cardholder's phone number.
    /// This is required for all cardholders who will be creating EU cards.
    /// See the [3D Secure documentation](https://stripe.com/docs/issuing/3d-secure#when-is-3d-secure-applied) for more details.
    pub phone_number: Option<String>,
    /// The cardholder’s preferred locales (languages), ordered by preference.
    /// Locales can be `de`, `en`, `es`, `fr`, or `it`.
    /// This changes the language of the [3D Secure flow](https://stripe.com/docs/issuing/3d-secure) and one-time password messages sent to the cardholder.
    pub preferred_locales: Option<Vec<stripe_shared::IssuingCardholderPreferredLocales>>,
    pub requirements: stripe_shared::IssuingCardholderRequirements,
    /// Rules that control spending across this cardholder's cards.
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    pub spending_controls: Option<stripe_shared::IssuingCardholderAuthorizationControls>,
    /// Specifies whether to permit authorizations on this cardholder's cards.
    pub status: stripe_shared::IssuingCardholderStatus,
    /// One of `individual` or `company`.
    /// See [Choose a cardholder type](https://stripe.com/docs/issuing/other/choose-cardholder) for more details.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: stripe_shared::IssuingCardholderType,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardholderBuilder {
    billing: Option<stripe_shared::IssuingCardholderAddress>,
    company: Option<Option<stripe_shared::IssuingCardholderCompany>>,
    created: Option<stripe_types::Timestamp>,
    email: Option<Option<String>>,
    id: Option<stripe_shared::IssuingCardholderId>,
    individual: Option<Option<stripe_shared::IssuingCardholderIndividual>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    name: Option<String>,
    phone_number: Option<Option<String>>,
    preferred_locales: Option<Option<Vec<stripe_shared::IssuingCardholderPreferredLocales>>>,
    requirements: Option<stripe_shared::IssuingCardholderRequirements>,
    spending_controls: Option<Option<stripe_shared::IssuingCardholderAuthorizationControls>>,
    status: Option<stripe_shared::IssuingCardholderStatus>,
    type_: Option<stripe_shared::IssuingCardholderType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardholder {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholder>,
        builder: IssuingCardholderBuilder,
    }

    impl Visitor for Place<IssuingCardholder> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardholderBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardholderBuilder {
        type Out = IssuingCardholder;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "billing" => Ok(Deserialize::begin(&mut self.billing)),
                "company" => Ok(Deserialize::begin(&mut self.company)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "email" => Ok(Deserialize::begin(&mut self.email)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "individual" => Ok(Deserialize::begin(&mut self.individual)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "phone_number" => Ok(Deserialize::begin(&mut self.phone_number)),
                "preferred_locales" => Ok(Deserialize::begin(&mut self.preferred_locales)),
                "requirements" => Ok(Deserialize::begin(&mut self.requirements)),
                "spending_controls" => Ok(Deserialize::begin(&mut self.spending_controls)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                billing: Deserialize::default(),
                company: Deserialize::default(),
                created: Deserialize::default(),
                email: Deserialize::default(),
                id: Deserialize::default(),
                individual: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
                phone_number: Deserialize::default(),
                preferred_locales: Deserialize::default(),
                requirements: Deserialize::default(),
                spending_controls: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let billing = self.billing.take()?;
            let company = self.company.take()?;
            let created = self.created.take()?;
            let email = self.email.take()?;
            let id = self.id.take()?;
            let individual = self.individual.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let name = self.name.take()?;
            let phone_number = self.phone_number.take()?;
            let preferred_locales = self.preferred_locales.take()?;
            let requirements = self.requirements.take()?;
            let spending_controls = self.spending_controls.take()?;
            let status = self.status.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { billing, company, created, email, id, individual, livemode, metadata, name, phone_number, preferred_locales, requirements, spending_controls, status, type_ })
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

    impl ObjectDeser for IssuingCardholder {
        type Builder = IssuingCardholderBuilder;
    }
};
impl stripe_types::Object for IssuingCardholder {
    type Id = stripe_shared::IssuingCardholderId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IssuingCardholderId, "ich_");
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardholderPreferredLocales {
    De,
    En,
    Es,
    Fr,
    It,
}
impl IssuingCardholderPreferredLocales {
    pub fn as_str(self) -> &'static str {
        use IssuingCardholderPreferredLocales::*;
        match self {
            De => "de",
            En => "en",
            Es => "es",
            Fr => "fr",
            It => "it",
        }
    }
}

impl std::str::FromStr for IssuingCardholderPreferredLocales {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderPreferredLocales::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "es" => Ok(Es),
            "fr" => Ok(Fr),
            "it" => Ok(It),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingCardholderPreferredLocales {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingCardholderPreferredLocales {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderPreferredLocales {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardholderPreferredLocales {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardholderPreferredLocales {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardholderPreferredLocales"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingCardholderPreferredLocales {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingCardholderPreferredLocales> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardholderPreferredLocales::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardholderStatus {
    Active,
    Blocked,
    Inactive,
}
impl IssuingCardholderStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingCardholderStatus::*;
        match self {
            Active => "active",
            Blocked => "blocked",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for IssuingCardholderStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderStatus::*;
        match s {
            "active" => Ok(Active),
            "blocked" => Ok(Blocked),
            "inactive" => Ok(Inactive),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingCardholderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingCardholderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardholderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardholderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardholderStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingCardholderStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingCardholderStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardholderStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardholderType {
    Company,
    Individual,
}
impl IssuingCardholderType {
    pub fn as_str(self) -> &'static str {
        use IssuingCardholderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for IssuingCardholderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingCardholderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingCardholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardholderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardholderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardholderType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingCardholderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingCardholderType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardholderType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
