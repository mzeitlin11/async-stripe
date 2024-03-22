#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountCardPaymentsSettings {
    pub decline_on: Option<stripe_shared::AccountDeclineChargeOn>,
    /// The default text that appears on credit card statements when a charge is made.
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix: Option<String>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kana: Option<String>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kanji: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountCardPaymentsSettingsBuilder {
    decline_on: Option<Option<stripe_shared::AccountDeclineChargeOn>>,
    statement_descriptor_prefix: Option<Option<String>>,
    statement_descriptor_prefix_kana: Option<Option<String>>,
    statement_descriptor_prefix_kanji: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountCardPaymentsSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCardPaymentsSettings>,
        builder: AccountCardPaymentsSettingsBuilder,
    }

    impl Visitor for Place<AccountCardPaymentsSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountCardPaymentsSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountCardPaymentsSettingsBuilder {
        type Out = AccountCardPaymentsSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "decline_on" => Ok(Deserialize::begin(&mut self.decline_on)),
                "statement_descriptor_prefix" => Ok(Deserialize::begin(&mut self.statement_descriptor_prefix)),
                "statement_descriptor_prefix_kana" => Ok(Deserialize::begin(&mut self.statement_descriptor_prefix_kana)),
                "statement_descriptor_prefix_kanji" => Ok(Deserialize::begin(&mut self.statement_descriptor_prefix_kanji)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                decline_on: Deserialize::default(),
                statement_descriptor_prefix: Deserialize::default(),
                statement_descriptor_prefix_kana: Deserialize::default(),
                statement_descriptor_prefix_kanji: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let decline_on = self.decline_on.take()?;
            let statement_descriptor_prefix = self.statement_descriptor_prefix.take()?;
            let statement_descriptor_prefix_kana = self.statement_descriptor_prefix_kana.take()?;
            let statement_descriptor_prefix_kanji = self.statement_descriptor_prefix_kanji.take()?;

            Some(Self::Out { decline_on, statement_descriptor_prefix, statement_descriptor_prefix_kana, statement_descriptor_prefix_kanji })
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

    impl ObjectDeser for AccountCardPaymentsSettings {
        type Builder = AccountCardPaymentsSettingsBuilder;
    }
};
