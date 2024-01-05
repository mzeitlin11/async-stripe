#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountUnificationAccountController {
    /// `true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts).
    /// Otherwise, this field is null.
    pub is_controller: Option<bool>,
    /// The controller type.
    /// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: AccountUnificationAccountControllerType,
}
#[cfg(feature = "min-ser")]
pub struct AccountUnificationAccountControllerBuilder {
    is_controller: Option<Option<bool>>,
    type_: Option<AccountUnificationAccountControllerType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountUnificationAccountController {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountUnificationAccountController>,
        builder: AccountUnificationAccountControllerBuilder,
    }

    impl Visitor for Place<AccountUnificationAccountController> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountUnificationAccountControllerBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountUnificationAccountControllerBuilder {
        type Out = AccountUnificationAccountController;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "is_controller" => Ok(Deserialize::begin(&mut self.is_controller)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { is_controller: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let is_controller = self.is_controller.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { is_controller, type_ })
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

    impl ObjectDeser for AccountUnificationAccountController {
        type Builder = AccountUnificationAccountControllerBuilder;
    }
};
/// The controller type.
/// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountUnificationAccountControllerType {
    Account,
    Application,
}
impl AccountUnificationAccountControllerType {
    pub fn as_str(self) -> &'static str {
        use AccountUnificationAccountControllerType::*;
        match self {
            Account => "account",
            Application => "application",
        }
    }
}

impl std::str::FromStr for AccountUnificationAccountControllerType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountUnificationAccountControllerType::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for AccountUnificationAccountControllerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for AccountUnificationAccountControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountUnificationAccountControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountUnificationAccountControllerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountUnificationAccountControllerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for AccountUnificationAccountControllerType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountUnificationAccountControllerType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<AccountUnificationAccountControllerType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountUnificationAccountControllerType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
