#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    pub local_amusement_tax: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    pub local_lease_tax: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,
    /// Type of registration in the US.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsUnitedStatesType,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder {
    local_amusement_tax: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>>,
    local_lease_tax: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>>,
    state: Option<String>,
    type_: Option<TaxProductRegistrationsResourceCountryOptionsUnitedStatesType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsUnitedStates>,
        builder: TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsUnitedStates> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsUnitedStates;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "local_amusement_tax" => Ok(Deserialize::begin(&mut self.local_amusement_tax)),
                "local_lease_tax" => Ok(Deserialize::begin(&mut self.local_lease_tax)),
                "state" => Ok(Deserialize::begin(&mut self.state)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { local_amusement_tax: Deserialize::default(), local_lease_tax: Deserialize::default(), state: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let local_amusement_tax = self.local_amusement_tax.take()?;
            let local_lease_tax = self.local_lease_tax.take()?;
            let state = self.state.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { local_amusement_tax, local_lease_tax, state, type_ })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
        type Builder = TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder;
    }
};
/// Type of registration in the US.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    LocalAmusementTax,
    LocalLeaseTax,
    StateCommunicationsTax,
    StateSalesTax,
}
impl TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match self {
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateSalesTax => "state_sales_tax",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match s {
            "local_amusement_tax" => Ok(LocalAmusementTax),
            "local_lease_tax" => Ok(LocalLeaseTax),
            "state_communications_tax" => Ok(StateCommunicationsTax),
            "state_sales_tax" => Ok(StateSalesTax),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxProductRegistrationsResourceCountryOptionsUnitedStatesType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
