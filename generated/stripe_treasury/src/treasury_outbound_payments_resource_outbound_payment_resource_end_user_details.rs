#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
    /// IP address of the user initiating the OutboundPayment.
    /// Set if `present` is set to `true`.
    /// IP address collection is required for risk and compliance reasons.
    /// This will be used to help determine if the OutboundPayment is authorized or should be blocked.
    pub ip_address: Option<String>,
    /// `true` if the OutboundPayment creation request is being made on behalf of an end user by a platform.
    /// Otherwise, `false`.
    pub present: bool,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder {
    ip_address: Option<Option<String>>,
    present: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails>,
        builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder {
        type Out = TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "ip_address" => Ok(Deserialize::begin(&mut self.ip_address)),
                "present" => Ok(Deserialize::begin(&mut self.present)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { ip_address: Deserialize::default(), present: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let ip_address = self.ip_address.take()?;
            let present = self.present.take()?;

            Some(Self::Out { ip_address, present })
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

    impl ObjectDeser for TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
        type Builder = TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder;
    }
};
