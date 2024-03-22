/// A Mandate is a record of the permission that your customer gives you to debit their payment method.
///
/// For more details see <<https://stripe.com/docs/api/mandates/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Mandate {
    pub customer_acceptance: stripe_shared::CustomerAcceptance,
    /// Unique identifier for the object.
    pub id: stripe_shared::MandateId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub multi_use: Option<stripe_shared::MandateMultiUse>,
    /// The account (if any) that the mandate is intended for.
    pub on_behalf_of: Option<String>,
    /// ID of the payment method associated with this mandate.
    pub payment_method: stripe_types::Expandable<stripe_shared::PaymentMethod>,
    pub payment_method_details: stripe_shared::MandatePaymentMethodDetails,
    pub single_use: Option<stripe_shared::MandateSingleUse>,
    /// The mandate status indicates whether or not you can use it to initiate a payment.
    pub status: MandateStatus,
    /// The type of the mandate.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: MandateType,
}
#[cfg(feature = "min-ser")]
pub struct MandateBuilder {
    customer_acceptance: Option<stripe_shared::CustomerAcceptance>,
    id: Option<stripe_shared::MandateId>,
    livemode: Option<bool>,
    multi_use: Option<Option<stripe_shared::MandateMultiUse>>,
    on_behalf_of: Option<Option<String>>,
    payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    payment_method_details: Option<stripe_shared::MandatePaymentMethodDetails>,
    single_use: Option<Option<stripe_shared::MandateSingleUse>>,
    status: Option<MandateStatus>,
    type_: Option<MandateType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Mandate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Mandate>,
        builder: MandateBuilder,
    }

    impl Visitor for Place<Mandate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: MandateBuilder::deser_default() }))
        }
    }

    impl MapBuilder for MandateBuilder {
        type Out = Mandate;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "customer_acceptance" => Ok(Deserialize::begin(&mut self.customer_acceptance)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "multi_use" => Ok(Deserialize::begin(&mut self.multi_use)),
                "on_behalf_of" => Ok(Deserialize::begin(&mut self.on_behalf_of)),
                "payment_method" => Ok(Deserialize::begin(&mut self.payment_method)),
                "payment_method_details" => Ok(Deserialize::begin(&mut self.payment_method_details)),
                "single_use" => Ok(Deserialize::begin(&mut self.single_use)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                customer_acceptance: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                multi_use: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                payment_method: Deserialize::default(),
                payment_method_details: Deserialize::default(),
                single_use: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let customer_acceptance = self.customer_acceptance.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let multi_use = self.multi_use.take()?;
            let on_behalf_of = self.on_behalf_of.take()?;
            let payment_method = self.payment_method.take()?;
            let payment_method_details = self.payment_method_details.take()?;
            let single_use = self.single_use.take()?;
            let status = self.status.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { customer_acceptance, id, livemode, multi_use, on_behalf_of, payment_method, payment_method_details, single_use, status, type_ })
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

    impl ObjectDeser for Mandate {
        type Builder = MandateBuilder;
    }
};
/// The mandate status indicates whether or not you can use it to initiate a payment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
}
impl MandateStatus {
    pub fn as_str(self) -> &'static str {
        use MandateStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for MandateStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for MandateStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateType {
    MultiUse,
    SingleUse,
}
impl MandateType {
    pub fn as_str(self) -> &'static str {
        use MandateType::*;
        match self {
            MultiUse => "multi_use",
            SingleUse => "single_use",
        }
    }
}

impl std::str::FromStr for MandateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateType::*;
        match s {
            "multi_use" => Ok(MultiUse),
            "single_use" => Ok(SingleUse),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for MandateType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Mandate {
    type Id = stripe_shared::MandateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(MandateId, "mandate_");
