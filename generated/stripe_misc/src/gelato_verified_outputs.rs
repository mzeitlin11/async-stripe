#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct GelatoVerifiedOutputs {
    /// The user's verified address.
    pub address: Option<stripe_shared::Address>,
    /// The user’s verified date of birth.
    pub dob: Option<stripe_misc::GelatoDataVerifiedOutputsDate>,
    /// The user's verified first name.
    pub first_name: Option<String>,
    /// The user's verified id number.
    pub id_number: Option<String>,
    /// The user's verified id number type.
    pub id_number_type: Option<GelatoVerifiedOutputsIdNumberType>,
    /// The user's verified last name.
    pub last_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct GelatoVerifiedOutputsBuilder {
    address: Option<Option<stripe_shared::Address>>,
    dob: Option<Option<stripe_misc::GelatoDataVerifiedOutputsDate>>,
    first_name: Option<Option<String>>,
    id_number: Option<Option<String>>,
    id_number_type: Option<Option<GelatoVerifiedOutputsIdNumberType>>,
    last_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoVerifiedOutputs {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoVerifiedOutputs>,
        builder: GelatoVerifiedOutputsBuilder,
    }

    impl Visitor for Place<GelatoVerifiedOutputs> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: GelatoVerifiedOutputsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for GelatoVerifiedOutputsBuilder {
        type Out = GelatoVerifiedOutputs;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),
                "dob" => Ok(Deserialize::begin(&mut self.dob)),
                "first_name" => Ok(Deserialize::begin(&mut self.first_name)),
                "id_number" => Ok(Deserialize::begin(&mut self.id_number)),
                "id_number_type" => Ok(Deserialize::begin(&mut self.id_number_type)),
                "last_name" => Ok(Deserialize::begin(&mut self.last_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                dob: Deserialize::default(),
                first_name: Deserialize::default(),
                id_number: Deserialize::default(),
                id_number_type: Deserialize::default(),
                last_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;
            let dob = self.dob.take()?;
            let first_name = self.first_name.take()?;
            let id_number = self.id_number.take()?;
            let id_number_type = self.id_number_type.take()?;
            let last_name = self.last_name.take()?;

            Some(Self::Out { address, dob, first_name, id_number, id_number_type, last_name })
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

    impl ObjectDeser for GelatoVerifiedOutputs {
        type Builder = GelatoVerifiedOutputsBuilder;
    }
};
/// The user's verified id number type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoVerifiedOutputsIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}
impl GelatoVerifiedOutputsIdNumberType {
    pub fn as_str(self) -> &'static str {
        use GelatoVerifiedOutputsIdNumberType::*;
        match self {
            BrCpf => "br_cpf",
            SgNric => "sg_nric",
            UsSsn => "us_ssn",
        }
    }
}

impl std::str::FromStr for GelatoVerifiedOutputsIdNumberType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoVerifiedOutputsIdNumberType::*;
        match s {
            "br_cpf" => Ok(BrCpf),
            "sg_nric" => Ok(SgNric),
            "us_ssn" => Ok(UsSsn),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for GelatoVerifiedOutputsIdNumberType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoVerifiedOutputsIdNumberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoVerifiedOutputsIdNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for GelatoVerifiedOutputsIdNumberType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for GelatoVerifiedOutputsIdNumberType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<GelatoVerifiedOutputsIdNumberType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoVerifiedOutputsIdNumberType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
