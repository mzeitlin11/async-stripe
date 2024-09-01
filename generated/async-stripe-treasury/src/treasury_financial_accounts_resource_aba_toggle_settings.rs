/// Toggle settings for enabling/disabling the ABA address feature
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceAbaToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceAbaToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details:
        Vec<stripe_treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceAbaToggleSettingsBuilder {
    requested: Option<bool>,
    status: Option<TreasuryFinancialAccountsResourceAbaToggleSettingsStatus>,
    status_details:
        Option<Vec<stripe_treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountsResourceAbaToggleSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceAbaToggleSettings>,
        builder: TreasuryFinancialAccountsResourceAbaToggleSettingsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceAbaToggleSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceAbaToggleSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceAbaToggleSettingsBuilder {
        type Out = TreasuryFinancialAccountsResourceAbaToggleSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "requested" => Deserialize::begin(&mut self.requested),
                "status" => Deserialize::begin(&mut self.status),
                "status_details" => Deserialize::begin(&mut self.status_details),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                requested: Deserialize::default(),
                status: Deserialize::default(),
                status_details: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(requested), Some(status), Some(status_details)) =
                (self.requested, self.status, self.status_details.take())
            else {
                return None;
            };
            Some(Self::Out { requested, status, status_details })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceAbaToggleSettings {
        type Builder = TreasuryFinancialAccountsResourceAbaToggleSettingsBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceAbaToggleSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryFinancialAccountsResourceAbaToggleSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "requested" => b.requested = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_details" => b.status_details = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether the Feature is operational.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}
impl TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
            Restricted => "restricted",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            "restricted" => Ok(Restricted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceAbaToggleSettingsStatus>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryFinancialAccountsResourceAbaToggleSettingsStatus
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus",
            )
        })
    }
}
