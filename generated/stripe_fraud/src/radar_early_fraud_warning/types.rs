/// An early fraud warning indicates that the card issuer has notified us that a
/// charge may be fraudulent.
///
/// Related guide: [Early fraud warnings](https://stripe.com/docs/disputes/measuring#early-fraud-warnings).
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RadarEarlyFraudWarning {
    /// An EFW is actionable if it has not received a dispute and has not been fully refunded.
    /// You may wish to proactively refund a charge that receives an EFW, in order to avoid receiving a dispute later.
    pub actionable: bool,
    /// ID of the charge this early fraud warning is for, optionally expanded.
    pub charge: stripe_types::Expandable<stripe_shared::Charge>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The type of fraud labelled by the issuer.
    /// One of `card_never_received`, `fraudulent_card_application`, `made_with_counterfeit_card`, `made_with_lost_card`, `made_with_stolen_card`, `misc`, `unauthorized_use_of_card`.
    pub fraud_type: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarEarlyFraudWarningId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the Payment Intent this early fraud warning is for, optionally expanded.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
}
#[cfg(feature = "min-ser")]
pub struct RadarEarlyFraudWarningBuilder {
    actionable: Option<bool>,
    charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    created: Option<stripe_types::Timestamp>,
    fraud_type: Option<String>,
    id: Option<stripe_fraud::RadarEarlyFraudWarningId>,
    livemode: Option<bool>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for RadarEarlyFraudWarning {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarEarlyFraudWarning>,
        builder: RadarEarlyFraudWarningBuilder,
    }

    impl Visitor for Place<RadarEarlyFraudWarning> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RadarEarlyFraudWarningBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RadarEarlyFraudWarningBuilder {
        type Out = RadarEarlyFraudWarning;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "actionable" => Ok(Deserialize::begin(&mut self.actionable)),
                "charge" => Ok(Deserialize::begin(&mut self.charge)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "fraud_type" => Ok(Deserialize::begin(&mut self.fraud_type)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "payment_intent" => Ok(Deserialize::begin(&mut self.payment_intent)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                actionable: Deserialize::default(),
                charge: Deserialize::default(),
                created: Deserialize::default(),
                fraud_type: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                payment_intent: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let actionable = self.actionable.take()?;
            let charge = self.charge.take()?;
            let created = self.created.take()?;
            let fraud_type = self.fraud_type.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let payment_intent = self.payment_intent.take()?;

            Some(Self::Out { actionable, charge, created, fraud_type, id, livemode, payment_intent })
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

    impl ObjectDeser for RadarEarlyFraudWarning {
        type Builder = RadarEarlyFraudWarningBuilder;
    }
};
impl stripe_types::Object for RadarEarlyFraudWarning {
    type Id = stripe_fraud::RadarEarlyFraudWarningId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(RadarEarlyFraudWarningId);
