#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardShipping {
    pub address: stripe_shared::Address,
    /// The delivery company that shipped a card.
    pub carrier: Option<IssuingCardShippingCarrier>,
    /// Additional information that may be required for clearing customs.
    pub customs: Option<stripe_shared::IssuingCardShippingCustoms>,
    /// A unix timestamp representing a best estimate of when the card will be delivered.
    pub eta: Option<stripe_types::Timestamp>,
    /// Recipient name.
    pub name: String,
    /// The phone number of the receiver of the shipment.
    /// Our courier partners will use this number to contact you in the event of card delivery issues.
    /// For individual shipments to the EU/UK, if this field is empty, we will provide them with the phone number provided when the cardholder was initially created.
    pub phone_number: Option<String>,
    /// Whether a signature is required for card delivery.
    /// This feature is only supported for US users.
    /// Standard shipping service does not support signature on delivery.
    /// The default value for standard shipping service is false and for express and priority services is true.
    pub require_signature: Option<bool>,
    /// Shipment service, such as `standard` or `express`.
    pub service: IssuingCardShippingService,
    /// The delivery status of the card.
    pub status: Option<IssuingCardShippingStatus>,
    /// A tracking number for a card shipment.
    pub tracking_number: Option<String>,
    /// A link to the shipping carrier's site where you can view detailed information about a card shipment.
    pub tracking_url: Option<String>,
    /// Packaging options.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: IssuingCardShippingType,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardShippingBuilder {
    address: Option<stripe_shared::Address>,
    carrier: Option<Option<IssuingCardShippingCarrier>>,
    customs: Option<Option<stripe_shared::IssuingCardShippingCustoms>>,
    eta: Option<Option<stripe_types::Timestamp>>,
    name: Option<String>,
    phone_number: Option<Option<String>>,
    require_signature: Option<Option<bool>>,
    service: Option<IssuingCardShippingService>,
    status: Option<Option<IssuingCardShippingStatus>>,
    tracking_number: Option<Option<String>>,
    tracking_url: Option<Option<String>>,
    type_: Option<IssuingCardShippingType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardShipping {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardShipping>,
        builder: IssuingCardShippingBuilder,
    }

    impl Visitor for Place<IssuingCardShipping> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardShippingBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardShippingBuilder {
        type Out = IssuingCardShipping;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),
                "carrier" => Ok(Deserialize::begin(&mut self.carrier)),
                "customs" => Ok(Deserialize::begin(&mut self.customs)),
                "eta" => Ok(Deserialize::begin(&mut self.eta)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "phone_number" => Ok(Deserialize::begin(&mut self.phone_number)),
                "require_signature" => Ok(Deserialize::begin(&mut self.require_signature)),
                "service" => Ok(Deserialize::begin(&mut self.service)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "tracking_number" => Ok(Deserialize::begin(&mut self.tracking_number)),
                "tracking_url" => Ok(Deserialize::begin(&mut self.tracking_url)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                carrier: Deserialize::default(),
                customs: Deserialize::default(),
                eta: Deserialize::default(),
                name: Deserialize::default(),
                phone_number: Deserialize::default(),
                require_signature: Deserialize::default(),
                service: Deserialize::default(),
                status: Deserialize::default(),
                tracking_number: Deserialize::default(),
                tracking_url: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;
            let carrier = self.carrier.take()?;
            let customs = self.customs.take()?;
            let eta = self.eta.take()?;
            let name = self.name.take()?;
            let phone_number = self.phone_number.take()?;
            let require_signature = self.require_signature.take()?;
            let service = self.service.take()?;
            let status = self.status.take()?;
            let tracking_number = self.tracking_number.take()?;
            let tracking_url = self.tracking_url.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { address, carrier, customs, eta, name, phone_number, require_signature, service, status, tracking_number, tracking_url, type_ })
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

    impl ObjectDeser for IssuingCardShipping {
        type Builder = IssuingCardShippingBuilder;
    }
};
/// The delivery company that shipped a card.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingCarrier {
    Dhl,
    Fedex,
    RoyalMail,
    Usps,
}
impl IssuingCardShippingCarrier {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingCarrier::*;
        match self {
            Dhl => "dhl",
            Fedex => "fedex",
            RoyalMail => "royal_mail",
            Usps => "usps",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingCarrier {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingCarrier::*;
        match s {
            "dhl" => Ok(Dhl),
            "fedex" => Ok(Fedex),
            "royal_mail" => Ok(RoyalMail),
            "usps" => Ok(Usps),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingCardShippingCarrier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingCardShippingCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardShippingCarrier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardShippingCarrier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardShippingCarrier"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingCardShippingCarrier {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingCardShippingCarrier> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingCarrier::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Shipment service, such as `standard` or `express`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingService {
    Express,
    Priority,
    Standard,
}
impl IssuingCardShippingService {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingService::*;
        match self {
            Express => "express",
            Priority => "priority",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingService::*;
        match s {
            "express" => Ok(Express),
            "priority" => Ok(Priority),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingCardShippingService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingCardShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardShippingService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardShippingService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardShippingService"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingCardShippingService {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingCardShippingService> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingService::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The delivery status of the card.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingStatus {
    Canceled,
    Delivered,
    Failure,
    Pending,
    Returned,
    Shipped,
}
impl IssuingCardShippingStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingStatus::*;
        match self {
            Canceled => "canceled",
            Delivered => "delivered",
            Failure => "failure",
            Pending => "pending",
            Returned => "returned",
            Shipped => "shipped",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "delivered" => Ok(Delivered),
            "failure" => Ok(Failure),
            "pending" => Ok(Pending),
            "returned" => Ok(Returned),
            "shipped" => Ok(Shipped),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingCardShippingStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingCardShippingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardShippingStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardShippingStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardShippingStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingCardShippingStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingCardShippingStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Packaging options.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingType {
    Bulk,
    Individual,
}
impl IssuingCardShippingType {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingType::*;
        match self {
            Bulk => "bulk",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingType::*;
        match s {
            "bulk" => Ok(Bulk),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for IssuingCardShippingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for IssuingCardShippingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardShippingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardShippingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardShippingType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IssuingCardShippingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IssuingCardShippingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
