#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetails {
    pub credit_reversal: Option<stripe_treasury::TreasuryCreditReversal>,
    pub outbound_payment: Option<stripe_treasury::TreasuryOutboundPayment>,
    pub payout: Option<stripe_shared::Payout>,
    /// The type of the source flow that originated the ReceivedCredit.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: TreasuryReceivedCreditsResourceSourceFlowsDetailsType,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder {
    credit_reversal: Option<Option<stripe_treasury::TreasuryCreditReversal>>,
    outbound_payment: Option<Option<stripe_treasury::TreasuryOutboundPayment>>,
    payout: Option<Option<stripe_shared::Payout>>,
    type_: Option<TreasuryReceivedCreditsResourceSourceFlowsDetailsType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryReceivedCreditsResourceSourceFlowsDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCreditsResourceSourceFlowsDetails>,
        builder: TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCreditsResourceSourceFlowsDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder {
        type Out = TreasuryReceivedCreditsResourceSourceFlowsDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "credit_reversal" => Ok(Deserialize::begin(&mut self.credit_reversal)),
                "outbound_payment" => Ok(Deserialize::begin(&mut self.outbound_payment)),
                "payout" => Ok(Deserialize::begin(&mut self.payout)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { credit_reversal: Deserialize::default(), outbound_payment: Deserialize::default(), payout: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let credit_reversal = self.credit_reversal.take()?;
            let outbound_payment = self.outbound_payment.take()?;
            let payout = self.payout.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { credit_reversal, outbound_payment, payout, type_ })
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

    impl ObjectDeser for TreasuryReceivedCreditsResourceSourceFlowsDetails {
        type Builder = TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder;
    }
};
/// The type of the source flow that originated the ReceivedCredit.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}
impl TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditsResourceSourceFlowsDetailsType::*;
        match self {
            CreditReversal => "credit_reversal",
            Other => "other",
            OutboundPayment => "outbound_payment",
            Payout => "payout",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceSourceFlowsDetailsType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "payout" => Ok(Payout),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditsResourceSourceFlowsDetailsType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryReceivedCreditsResourceSourceFlowsDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedCreditsResourceSourceFlowsDetailsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
