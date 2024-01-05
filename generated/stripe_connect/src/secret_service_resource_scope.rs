#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SecretServiceResourceScope {
    /// The secret scope type.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: SecretServiceResourceScopeType,
    /// The user ID, if type is set to "user"
    pub user: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SecretServiceResourceScopeBuilder {
    type_: Option<SecretServiceResourceScopeType>,
    user: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SecretServiceResourceScope {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SecretServiceResourceScope>,
        builder: SecretServiceResourceScopeBuilder,
    }

    impl Visitor for Place<SecretServiceResourceScope> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SecretServiceResourceScopeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SecretServiceResourceScopeBuilder {
        type Out = SecretServiceResourceScope;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "user" => Ok(Deserialize::begin(&mut self.user)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { type_: Deserialize::default(), user: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let type_ = self.type_.take()?;
            let user = self.user.take()?;

            Some(Self::Out { type_, user })
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

    impl ObjectDeser for SecretServiceResourceScope {
        type Builder = SecretServiceResourceScopeBuilder;
    }
};
/// The secret scope type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SecretServiceResourceScopeType {
    Account,
    User,
}
impl SecretServiceResourceScopeType {
    pub fn as_str(self) -> &'static str {
        use SecretServiceResourceScopeType::*;
        match self {
            Account => "account",
            User => "user",
        }
    }
}

impl std::str::FromStr for SecretServiceResourceScopeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SecretServiceResourceScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SecretServiceResourceScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SecretServiceResourceScopeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecretServiceResourceScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SecretServiceResourceScopeType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SecretServiceResourceScopeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SecretServiceResourceScopeType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SecretServiceResourceScopeType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
