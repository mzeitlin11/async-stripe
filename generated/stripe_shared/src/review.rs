/// Reviews can be used to supplement automated fraud detection with human expertise.
///
/// Learn more about [Radar](/radar) and reviewing payments
/// [here](https://stripe.com/docs/radar/reviews).
///
/// For more details see <<https://stripe.com/docs/api/radar/reviews/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Review {
    /// The ZIP or postal code of the card used, if applicable.
    pub billing_zip: Option<String>,
    /// The charge associated with this review.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// The reason the review was closed, or null if it has not yet been closed.
    /// One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub closed_reason: Option<ReviewClosedReason>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_shared::ReviewId,
    /// The IP address where the payment originated.
    pub ip_address: Option<String>,
    /// Information related to the location of the payment.
    /// Note that this information is an approximation and attempts to locate the nearest population center - it should not be used to determine a specific address.
    pub ip_address_location: Option<stripe_shared::RadarReviewResourceLocation>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// If `true`, the review needs action.
    pub open: bool,
    /// The reason the review was opened. One of `rule` or `manual`.
    pub opened_reason: ReviewOpenedReason,
    /// The PaymentIntent ID associated with this review, if one exists.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    /// The reason the review is currently open or closed.
    /// One of `rule`, `manual`, `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub reason: String,
    /// Information related to the browsing session of the user who initiated the payment.
    pub session: Option<stripe_shared::RadarReviewResourceSession>,
}
#[cfg(feature = "min-ser")]
pub struct ReviewBuilder {
    billing_zip: Option<Option<String>>,
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    closed_reason: Option<Option<ReviewClosedReason>>,
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_shared::ReviewId>,
    ip_address: Option<Option<String>>,
    ip_address_location: Option<Option<stripe_shared::RadarReviewResourceLocation>>,
    livemode: Option<bool>,
    open: Option<bool>,
    opened_reason: Option<ReviewOpenedReason>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    reason: Option<String>,
    session: Option<Option<stripe_shared::RadarReviewResourceSession>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Review {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Review>,
        builder: ReviewBuilder,
    }

    impl Visitor for Place<Review> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ReviewBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ReviewBuilder {
        type Out = Review;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "billing_zip" => Ok(Deserialize::begin(&mut self.billing_zip)),
                "charge" => Ok(Deserialize::begin(&mut self.charge)),
                "closed_reason" => Ok(Deserialize::begin(&mut self.closed_reason)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "ip_address" => Ok(Deserialize::begin(&mut self.ip_address)),
                "ip_address_location" => Ok(Deserialize::begin(&mut self.ip_address_location)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "open" => Ok(Deserialize::begin(&mut self.open)),
                "opened_reason" => Ok(Deserialize::begin(&mut self.opened_reason)),
                "payment_intent" => Ok(Deserialize::begin(&mut self.payment_intent)),
                "reason" => Ok(Deserialize::begin(&mut self.reason)),
                "session" => Ok(Deserialize::begin(&mut self.session)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                billing_zip: Deserialize::default(),
                charge: Deserialize::default(),
                closed_reason: Deserialize::default(),
                created: Deserialize::default(),
                id: Deserialize::default(),
                ip_address: Deserialize::default(),
                ip_address_location: Deserialize::default(),
                livemode: Deserialize::default(),
                open: Deserialize::default(),
                opened_reason: Deserialize::default(),
                payment_intent: Deserialize::default(),
                reason: Deserialize::default(),
                session: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let billing_zip = self.billing_zip.take()?;
            let charge = self.charge.take()?;
            let closed_reason = self.closed_reason.take()?;
            let created = self.created.take()?;
            let id = self.id.take()?;
            let ip_address = self.ip_address.take()?;
            let ip_address_location = self.ip_address_location.take()?;
            let livemode = self.livemode.take()?;
            let open = self.open.take()?;
            let opened_reason = self.opened_reason.take()?;
            let payment_intent = self.payment_intent.take()?;
            let reason = self.reason.take()?;
            let session = self.session.take()?;

            Some(Self::Out { billing_zip, charge, closed_reason, created, id, ip_address, ip_address_location, livemode, open, opened_reason, payment_intent, reason, session })
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

    impl ObjectDeser for Review {
        type Builder = ReviewBuilder;
    }
};
/// The reason the review was closed, or null if it has not yet been closed.
/// One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReviewClosedReason {
    Approved,
    Disputed,
    Redacted,
    Refunded,
    RefundedAsFraud,
}
impl ReviewClosedReason {
    pub fn as_str(self) -> &'static str {
        use ReviewClosedReason::*;
        match self {
            Approved => "approved",
            Disputed => "disputed",
            Redacted => "redacted",
            Refunded => "refunded",
            RefundedAsFraud => "refunded_as_fraud",
        }
    }
}

impl std::str::FromStr for ReviewClosedReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReviewClosedReason::*;
        match s {
            "approved" => Ok(Approved),
            "disputed" => Ok(Disputed),
            "redacted" => Ok(Redacted),
            "refunded" => Ok(Refunded),
            "refunded_as_fraud" => Ok(RefundedAsFraud),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ReviewClosedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ReviewClosedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReviewClosedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReviewClosedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReviewClosedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ReviewClosedReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReviewClosedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ReviewClosedReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReviewClosedReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The reason the review was opened. One of `rule` or `manual`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReviewOpenedReason {
    Manual,
    Rule,
}
impl ReviewOpenedReason {
    pub fn as_str(self) -> &'static str {
        use ReviewOpenedReason::*;
        match self {
            Manual => "manual",
            Rule => "rule",
        }
    }
}

impl std::str::FromStr for ReviewOpenedReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReviewOpenedReason::*;
        match s {
            "manual" => Ok(Manual),
            "rule" => Ok(Rule),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ReviewOpenedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ReviewOpenedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReviewOpenedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReviewOpenedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReviewOpenedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ReviewOpenedReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReviewOpenedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ReviewOpenedReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReviewOpenedReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Review {
    type Id = stripe_shared::ReviewId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ReviewId, "prv_");
