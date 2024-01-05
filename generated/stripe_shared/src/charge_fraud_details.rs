#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ChargeFraudDetails {
    /// Assessments from Stripe. If set, the value is `fraudulent`.
    pub stripe_report: Option<String>,
    /// Assessments reported by you. If set, possible values of are `safe` and `fraudulent`.
    pub user_report: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct ChargeFraudDetailsBuilder {
    stripe_report: Option<Option<String>>,
    user_report: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ChargeFraudDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ChargeFraudDetails>,
        builder: ChargeFraudDetailsBuilder,
    }

    impl Visitor for Place<ChargeFraudDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ChargeFraudDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ChargeFraudDetailsBuilder {
        type Out = ChargeFraudDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "stripe_report" => Ok(Deserialize::begin(&mut self.stripe_report)),
                "user_report" => Ok(Deserialize::begin(&mut self.user_report)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { stripe_report: Deserialize::default(), user_report: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let stripe_report = self.stripe_report.take()?;
            let user_report = self.user_report.take()?;

            Some(Self::Out { stripe_report, user_report })
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

    impl ObjectDeser for ChargeFraudDetails {
        type Builder = ChargeFraudDetailsBuilder;
    }
};
