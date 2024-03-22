#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceSettingSubscriptionSchedulePhaseSetting {
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    pub days_until_due: Option<u32>,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceSettingSubscriptionSchedulePhaseSettingBuilder {
    days_until_due: Option<Option<u32>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceSettingSubscriptionSchedulePhaseSetting {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceSettingSubscriptionSchedulePhaseSetting>,
        builder: InvoiceSettingSubscriptionSchedulePhaseSettingBuilder,
    }

    impl Visitor for Place<InvoiceSettingSubscriptionSchedulePhaseSetting> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceSettingSubscriptionSchedulePhaseSettingBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceSettingSubscriptionSchedulePhaseSettingBuilder {
        type Out = InvoiceSettingSubscriptionSchedulePhaseSetting;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "days_until_due" => Ok(Deserialize::begin(&mut self.days_until_due)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { days_until_due: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let days_until_due = self.days_until_due.take()?;

            Some(Self::Out { days_until_due })
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

    impl ObjectDeser for InvoiceSettingSubscriptionSchedulePhaseSetting {
        type Builder = InvoiceSettingSubscriptionSchedulePhaseSettingBuilder;
    }
};
