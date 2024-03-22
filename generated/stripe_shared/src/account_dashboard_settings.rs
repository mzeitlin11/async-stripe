#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountDashboardSettings {
    /// The display name for this account.
    /// This is used on the Stripe Dashboard to differentiate between accounts.
    pub display_name: Option<String>,
    /// The timezone used in the Stripe Dashboard for this account.
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    pub timezone: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountDashboardSettingsBuilder {
    display_name: Option<Option<String>>,
    timezone: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountDashboardSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountDashboardSettings>,
        builder: AccountDashboardSettingsBuilder,
    }

    impl Visitor for Place<AccountDashboardSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountDashboardSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountDashboardSettingsBuilder {
        type Out = AccountDashboardSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "display_name" => Ok(Deserialize::begin(&mut self.display_name)),
                "timezone" => Ok(Deserialize::begin(&mut self.timezone)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { display_name: Deserialize::default(), timezone: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let display_name = self.display_name.take()?;
            let timezone = self.timezone.take()?;

            Some(Self::Out { display_name, timezone })
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

    impl ObjectDeser for AccountDashboardSettings {
        type Builder = AccountDashboardSettingsBuilder;
    }
};
