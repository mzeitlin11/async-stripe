#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountPaymentsSettings {
    /// The default text that appears on credit card statements when a charge is made.
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    pub statement_descriptor: Option<String>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    pub statement_descriptor_kana: Option<String>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    pub statement_descriptor_kanji: Option<String>,
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
pub struct AccountPaymentsSettingsBuilder {
    statement_descriptor: Option<Option<String>>,
    statement_descriptor_kana: Option<Option<String>>,
    statement_descriptor_kanji: Option<Option<String>>,
    statement_descriptor_prefix_kana: Option<Option<String>>,
    statement_descriptor_prefix_kanji: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountPaymentsSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountPaymentsSettings>,
        builder: AccountPaymentsSettingsBuilder,
    }

    impl Visitor for Place<AccountPaymentsSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountPaymentsSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountPaymentsSettingsBuilder {
        type Out = AccountPaymentsSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),
                "statement_descriptor_kana" => Ok(Deserialize::begin(&mut self.statement_descriptor_kana)),
                "statement_descriptor_kanji" => Ok(Deserialize::begin(&mut self.statement_descriptor_kanji)),
                "statement_descriptor_prefix_kana" => Ok(Deserialize::begin(&mut self.statement_descriptor_prefix_kana)),
                "statement_descriptor_prefix_kanji" => Ok(Deserialize::begin(&mut self.statement_descriptor_prefix_kanji)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                statement_descriptor: Deserialize::default(),
                statement_descriptor_kana: Deserialize::default(),
                statement_descriptor_kanji: Deserialize::default(),
                statement_descriptor_prefix_kana: Deserialize::default(),
                statement_descriptor_prefix_kanji: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let statement_descriptor = self.statement_descriptor.take()?;
            let statement_descriptor_kana = self.statement_descriptor_kana.take()?;
            let statement_descriptor_kanji = self.statement_descriptor_kanji.take()?;
            let statement_descriptor_prefix_kana = self.statement_descriptor_prefix_kana.take()?;
            let statement_descriptor_prefix_kanji = self.statement_descriptor_prefix_kanji.take()?;

            Some(Self::Out { statement_descriptor, statement_descriptor_kana, statement_descriptor_kanji, statement_descriptor_prefix_kana, statement_descriptor_prefix_kanji })
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

    impl ObjectDeser for AccountPaymentsSettings {
        type Builder = AccountPaymentsSettingsBuilder;
    }
};
