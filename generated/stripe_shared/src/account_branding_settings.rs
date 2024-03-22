#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountBrandingSettings {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    /// Must be square and at least 128px x 128px.
    pub icon: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    /// Must be at least 128px x 128px.
    pub logo: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// A CSS hex color value representing the primary branding color for this account
    pub primary_color: Option<String>,
    /// A CSS hex color value representing the secondary branding color for this account
    pub secondary_color: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountBrandingSettingsBuilder {
    icon: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    logo: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    primary_color: Option<Option<String>>,
    secondary_color: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountBrandingSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountBrandingSettings>,
        builder: AccountBrandingSettingsBuilder,
    }

    impl Visitor for Place<AccountBrandingSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountBrandingSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountBrandingSettingsBuilder {
        type Out = AccountBrandingSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "icon" => Ok(Deserialize::begin(&mut self.icon)),
                "logo" => Ok(Deserialize::begin(&mut self.logo)),
                "primary_color" => Ok(Deserialize::begin(&mut self.primary_color)),
                "secondary_color" => Ok(Deserialize::begin(&mut self.secondary_color)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { icon: Deserialize::default(), logo: Deserialize::default(), primary_color: Deserialize::default(), secondary_color: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let icon = self.icon.take()?;
            let logo = self.logo.take()?;
            let primary_color = self.primary_color.take()?;
            let secondary_color = self.secondary_color.take()?;

            Some(Self::Out { icon, logo, primary_color, secondary_color })
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

    impl ObjectDeser for AccountBrandingSettings {
        type Builder = AccountBrandingSettingsBuilder;
    }
};
