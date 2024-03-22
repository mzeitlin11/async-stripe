#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PersonRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Fields that need to be collected to keep the person's account enabled.
    /// If not collected by the account's `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Vec<String>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<stripe_shared::AccountRequirementsError>,
    /// Fields that need to be collected assuming all volume thresholds are reached.
    /// As they become required, they appear in `currently_due` as well, and the account's `current_deadline` becomes set.
    pub eventually_due: Vec<String>,
    /// Fields that weren't collected by the account's `current_deadline`.
    /// These fields need to be collected to enable the person's account.
    pub past_due: Vec<String>,
    /// Fields that may become required depending on the results of verification or review.
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Vec<String>,
}
#[cfg(feature = "min-ser")]
pub struct PersonRequirementsBuilder {
    alternatives: Option<Option<Vec<stripe_shared::AccountRequirementsAlternative>>>,
    currently_due: Option<Vec<String>>,
    errors: Option<Vec<stripe_shared::AccountRequirementsError>>,
    eventually_due: Option<Vec<String>>,
    past_due: Option<Vec<String>>,
    pending_verification: Option<Vec<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PersonRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonRequirements>,
        builder: PersonRequirementsBuilder,
    }

    impl Visitor for Place<PersonRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PersonRequirementsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PersonRequirementsBuilder {
        type Out = PersonRequirements;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "alternatives" => Ok(Deserialize::begin(&mut self.alternatives)),
                "currently_due" => Ok(Deserialize::begin(&mut self.currently_due)),
                "errors" => Ok(Deserialize::begin(&mut self.errors)),
                "eventually_due" => Ok(Deserialize::begin(&mut self.eventually_due)),
                "past_due" => Ok(Deserialize::begin(&mut self.past_due)),
                "pending_verification" => Ok(Deserialize::begin(&mut self.pending_verification)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                alternatives: Deserialize::default(),
                currently_due: Deserialize::default(),
                errors: Deserialize::default(),
                eventually_due: Deserialize::default(),
                past_due: Deserialize::default(),
                pending_verification: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let alternatives = self.alternatives.take()?;
            let currently_due = self.currently_due.take()?;
            let errors = self.errors.take()?;
            let eventually_due = self.eventually_due.take()?;
            let past_due = self.past_due.take()?;
            let pending_verification = self.pending_verification.take()?;

            Some(Self::Out { alternatives, currently_due, errors, eventually_due, past_due, pending_verification })
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

    impl ObjectDeser for PersonRequirements {
        type Builder = PersonRequirementsBuilder;
    }
};
