#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountCapabilityRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Date by which the fields in `currently_due` must be collected to keep the capability enabled for the account.
    /// These fields may disable the capability sooner if the next threshold is reached before they are collected.
    pub current_deadline: Option<stripe_types::Timestamp>,
    /// Fields that need to be collected to keep the capability enabled.
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the capability is disabled.
    pub currently_due: Vec<String>,
    /// If the capability is disabled, this string describes why.
    /// Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`.
    ///
    /// `rejected.unsupported_business` means that the account's business is not supported by the capability.
    /// For example, payment methods may restrict the businesses they support in their terms of service:.
    ///
    /// - [Afterpay Clearpay's terms of service](/afterpay-clearpay/legal#restricted-businesses)
    ///
    /// If you believe that the rejection is in error, please contact support at <https://support.stripe.com/contact/> for assistance.
    pub disabled_reason: Option<String>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<stripe_shared::AccountRequirementsError>,
    /// Fields that need to be collected assuming all volume thresholds are reached.
    /// As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Vec<String>,
    /// Fields that weren't collected by `current_deadline`.
    /// These fields need to be collected to enable the capability on the account.
    pub past_due: Vec<String>,
    /// Fields that may become required depending on the results of verification or review.
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Vec<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountCapabilityRequirementsBuilder {
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

    impl Deserialize for AccountCapabilityRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCapabilityRequirements>,
        builder: AccountCapabilityRequirementsBuilder,
    }

    impl Visitor for Place<AccountCapabilityRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountCapabilityRequirementsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountCapabilityRequirementsBuilder {
        type Out = AccountCapabilityRequirements;
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

    impl ObjectDeser for AccountCapabilityRequirements {
        type Builder = AccountCapabilityRequirementsBuilder;
    }
};
