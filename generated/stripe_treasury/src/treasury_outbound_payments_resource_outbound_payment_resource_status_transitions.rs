#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
    /// Timestamp describing when an OutboundPayment changed status to `canceled`.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `failed`.
    pub failed_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `posted`.
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `returned`.
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder {
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    failed_at: Option<Option<stripe_types::Timestamp>>,
    posted_at: Option<Option<stripe_types::Timestamp>>,
    returned_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions>,
        builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder {
        type Out = TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "canceled_at" => Ok(Deserialize::begin(&mut self.canceled_at)),
                "failed_at" => Ok(Deserialize::begin(&mut self.failed_at)),
                "posted_at" => Ok(Deserialize::begin(&mut self.posted_at)),
                "returned_at" => Ok(Deserialize::begin(&mut self.returned_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { canceled_at: Deserialize::default(), failed_at: Deserialize::default(), posted_at: Deserialize::default(), returned_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let canceled_at = self.canceled_at.take()?;
            let failed_at = self.failed_at.take()?;
            let posted_at = self.posted_at.take()?;
            let returned_at = self.returned_at.take()?;

            Some(Self::Out { canceled_at, failed_at, posted_at, returned_at })
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

    impl ObjectDeser for TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
        type Builder = TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder;
    }
};
