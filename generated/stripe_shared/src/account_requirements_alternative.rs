#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountRequirementsAlternative {
    /// Fields that can be provided to satisfy all fields in `original_fields_due`.
    pub alternative_fields_due: Vec<String>,
    /// Fields that are due and can be satisfied by providing all fields in `alternative_fields_due`.
    pub original_fields_due: Vec<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountRequirementsAlternativeBuilder {
    alternative_fields_due: Option<Vec<String>>,
    original_fields_due: Option<Vec<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountRequirementsAlternative {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountRequirementsAlternative>,
        builder: AccountRequirementsAlternativeBuilder,
    }

    impl Visitor for Place<AccountRequirementsAlternative> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountRequirementsAlternativeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountRequirementsAlternativeBuilder {
        type Out = AccountRequirementsAlternative;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "alternative_fields_due" => Ok(Deserialize::begin(&mut self.alternative_fields_due)),
                "original_fields_due" => Ok(Deserialize::begin(&mut self.original_fields_due)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { alternative_fields_due: Deserialize::default(), original_fields_due: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let alternative_fields_due = self.alternative_fields_due.take()?;
            let original_fields_due = self.original_fields_due.take()?;

            Some(Self::Out { alternative_fields_due, original_fields_due })
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

    impl ObjectDeser for AccountRequirementsAlternative {
        type Builder = AccountRequirementsAlternativeBuilder;
    }
};
