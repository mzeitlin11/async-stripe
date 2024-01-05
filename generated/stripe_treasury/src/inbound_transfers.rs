/// For more details see <<https://stripe.com/docs/api/treasury/inbound_transfers>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InboundTransfers {
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    /// The type of the payment method used in the InboundTransfer.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: InboundTransfersType,
    pub us_bank_account: Option<stripe_treasury::InboundTransfersPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "min-ser")]
pub struct InboundTransfersBuilder {
    billing_details: Option<stripe_treasury::TreasurySharedResourceBillingDetails>,
    type_: Option<InboundTransfersType>,
    us_bank_account: Option<Option<stripe_treasury::InboundTransfersPaymentMethodDetailsUsBankAccount>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InboundTransfers {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InboundTransfers>,
        builder: InboundTransfersBuilder,
    }

    impl Visitor for Place<InboundTransfers> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InboundTransfersBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InboundTransfersBuilder {
        type Out = InboundTransfers;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "billing_details" => Ok(Deserialize::begin(&mut self.billing_details)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { billing_details: Deserialize::default(), type_: Deserialize::default(), us_bank_account: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let billing_details = self.billing_details.take()?;
            let type_ = self.type_.take()?;
            let us_bank_account = self.us_bank_account.take()?;

            Some(Self::Out { billing_details, type_, us_bank_account })
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

    impl ObjectDeser for InboundTransfers {
        type Builder = InboundTransfersBuilder;
    }
};
/// The type of the payment method used in the InboundTransfer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InboundTransfersType {
    UsBankAccount,
}
impl InboundTransfersType {
    pub fn as_str(self) -> &'static str {
        use InboundTransfersType::*;
        match self {
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for InboundTransfersType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InboundTransfersType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InboundTransfersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InboundTransfersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InboundTransfersType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InboundTransfersType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InboundTransfersType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InboundTransfersType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InboundTransfersType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InboundTransfersType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
