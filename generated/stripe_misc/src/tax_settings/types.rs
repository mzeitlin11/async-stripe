/// You can use Tax `Settings` to manage configurations used by Stripe Tax calculations.
///
/// Related guide: [Using the Settings API](https://stripe.com/docs/tax/settings-api)
///
/// For more details see <<https://stripe.com/docs/api/tax/settings/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxSettings {
    pub defaults: stripe_misc::TaxProductResourceTaxSettingsDefaults,
    /// The place where your business is located.
    pub head_office: Option<stripe_misc::TaxProductResourceTaxSettingsHeadOffice>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The `active` status indicates you have all required settings to calculate tax.
    /// A status can transition out of `active` when new required settings are introduced.
    pub status: TaxSettingsStatus,
    pub status_details: stripe_misc::TaxProductResourceTaxSettingsStatusDetails,
}
#[cfg(feature = "min-ser")]
pub struct TaxSettingsBuilder {
    defaults: Option<stripe_misc::TaxProductResourceTaxSettingsDefaults>,
    head_office: Option<Option<stripe_misc::TaxProductResourceTaxSettingsHeadOffice>>,
    livemode: Option<bool>,
    status: Option<TaxSettingsStatus>,
    status_details: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetails>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxSettings>,
        builder: TaxSettingsBuilder,
    }

    impl Visitor for Place<TaxSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxSettingsBuilder {
        type Out = TaxSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "defaults" => Ok(Deserialize::begin(&mut self.defaults)),
                "head_office" => Ok(Deserialize::begin(&mut self.head_office)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "status_details" => Ok(Deserialize::begin(&mut self.status_details)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { defaults: Deserialize::default(), head_office: Deserialize::default(), livemode: Deserialize::default(), status: Deserialize::default(), status_details: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let defaults = self.defaults.take()?;
            let head_office = self.head_office.take()?;
            let livemode = self.livemode.take()?;
            let status = self.status.take()?;
            let status_details = self.status_details.take()?;

            Some(Self::Out { defaults, head_office, livemode, status, status_details })
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

    impl ObjectDeser for TaxSettings {
        type Builder = TaxSettingsBuilder;
    }
};
/// The `active` status indicates you have all required settings to calculate tax.
/// A status can transition out of `active` when new required settings are introduced.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxSettingsStatus {
    Active,
    Pending,
}
impl TaxSettingsStatus {
    pub fn as_str(self) -> &'static str {
        use TaxSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for TaxSettingsStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TaxSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxSettingsStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxSettingsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxSettingsStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxSettingsStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
