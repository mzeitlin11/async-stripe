#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountCapabilityFutureRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Date on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty.
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on the capability's enablement state prior to transitioning.
    pub current_deadline: Option<stripe_types::Timestamp>,
    /// Fields that need to be collected to keep the capability enabled.
    /// If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Vec<String>,
    /// This is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account.
    pub disabled_reason: Option<String>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<stripe_shared::AccountRequirementsError>,
    /// Fields that need to be collected assuming all volume thresholds are reached.
    /// As they become required, they appear in `currently_due` as well.
    pub eventually_due: Vec<String>,
    /// Fields that weren't collected by `requirements.current_deadline`.
    /// These fields need to be collected to enable the capability on the account.
    /// New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Vec<String>,
    /// Fields that may become required depending on the results of verification or review.
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Vec<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountCapabilityFutureRequirementsBuilder {
    alternatives: Option<Option<Vec<stripe_shared::AccountRequirementsAlternative>>>,
    current_deadline: Option<Option<stripe_types::Timestamp>>,
    currently_due: Option<Vec<String>>,
    disabled_reason: Option<Option<String>>,
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

    impl Deserialize for AccountCapabilityFutureRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCapabilityFutureRequirements>,
        builder: AccountCapabilityFutureRequirementsBuilder,
    }

    impl Visitor for Place<AccountCapabilityFutureRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountCapabilityFutureRequirementsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountCapabilityFutureRequirementsBuilder {
        type Out = AccountCapabilityFutureRequirements;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "alternatives" => Ok(Deserialize::begin(&mut self.alternatives)),
                "current_deadline" => Ok(Deserialize::begin(&mut self.current_deadline)),
                "currently_due" => Ok(Deserialize::begin(&mut self.currently_due)),
                "disabled_reason" => Ok(Deserialize::begin(&mut self.disabled_reason)),
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
                current_deadline: Deserialize::default(),
                currently_due: Deserialize::default(),
                disabled_reason: Deserialize::default(),
                errors: Deserialize::default(),
                eventually_due: Deserialize::default(),
                past_due: Deserialize::default(),
                pending_verification: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let alternatives = self.alternatives.take()?;
            let current_deadline = self.current_deadline.take()?;
            let currently_due = self.currently_due.take()?;
            let disabled_reason = self.disabled_reason.take()?;
            let errors = self.errors.take()?;
            let eventually_due = self.eventually_due.take()?;
            let past_due = self.past_due.take()?;
            let pending_verification = self.pending_verification.take()?;

            Some(Self::Out { alternatives, current_deadline, currently_due, disabled_reason, errors, eventually_due, past_due, pending_verification })
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

    impl ObjectDeser for AccountCapabilityFutureRequirements {
        type Builder = AccountCapabilityFutureRequirementsBuilder;
    }
};
