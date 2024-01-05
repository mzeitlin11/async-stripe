#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryReceivedCreditsResourceLinkedFlows {
    /// The CreditReversal created as a result of this ReceivedCredit being reversed.
    pub credit_reversal: Option<String>,
    /// Set if the ReceivedCredit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
    pub issuing_authorization: Option<String>,
    /// Set if the ReceivedCredit is also viewable as an [Issuing transaction](https://stripe.com/docs/api#issuing_transactions) object.
    pub issuing_transaction: Option<String>,
    /// ID of the source flow.
    /// Set if `network` is `stripe` and the source flow is visible to the user.
    /// Examples of source flows include OutboundPayments, payouts, or CreditReversals.
    pub source_flow: Option<String>,
    /// The expandable object of the source flow.
    pub source_flow_details: Option<stripe_treasury::TreasuryReceivedCreditsResourceSourceFlowsDetails>,
    /// The type of flow that originated the ReceivedCredit (for example, `outbound_payment`).
    pub source_flow_type: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryReceivedCreditsResourceLinkedFlowsBuilder {
    credit_reversal: Option<Option<String>>,
    issuing_authorization: Option<Option<String>>,
    issuing_transaction: Option<Option<String>>,
    source_flow: Option<Option<String>>,
    source_flow_details: Option<Option<stripe_treasury::TreasuryReceivedCreditsResourceSourceFlowsDetails>>,
    source_flow_type: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryReceivedCreditsResourceLinkedFlows {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCreditsResourceLinkedFlows>,
        builder: TreasuryReceivedCreditsResourceLinkedFlowsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCreditsResourceLinkedFlows> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryReceivedCreditsResourceLinkedFlowsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryReceivedCreditsResourceLinkedFlowsBuilder {
        type Out = TreasuryReceivedCreditsResourceLinkedFlows;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "credit_reversal" => Ok(Deserialize::begin(&mut self.credit_reversal)),
                "issuing_authorization" => Ok(Deserialize::begin(&mut self.issuing_authorization)),
                "issuing_transaction" => Ok(Deserialize::begin(&mut self.issuing_transaction)),
                "source_flow" => Ok(Deserialize::begin(&mut self.source_flow)),
                "source_flow_details" => Ok(Deserialize::begin(&mut self.source_flow_details)),
                "source_flow_type" => Ok(Deserialize::begin(&mut self.source_flow_type)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                credit_reversal: Deserialize::default(),
                issuing_authorization: Deserialize::default(),
                issuing_transaction: Deserialize::default(),
                source_flow: Deserialize::default(),
                source_flow_details: Deserialize::default(),
                source_flow_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let credit_reversal = self.credit_reversal.take()?;
            let issuing_authorization = self.issuing_authorization.take()?;
            let issuing_transaction = self.issuing_transaction.take()?;
            let source_flow = self.source_flow.take()?;
            let source_flow_details = self.source_flow_details.take()?;
            let source_flow_type = self.source_flow_type.take()?;

            Some(Self::Out { credit_reversal, issuing_authorization, issuing_transaction, source_flow, source_flow_details, source_flow_type })
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

    impl ObjectDeser for TreasuryReceivedCreditsResourceLinkedFlows {
        type Builder = TreasuryReceivedCreditsResourceLinkedFlowsBuilder;
    }
};
