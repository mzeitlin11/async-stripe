#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct MandateAcssDebit {
    /// List of Stripe products where this mandate can be selected automatically.
    pub default_for: Option<Vec<MandateAcssDebitDefaultFor>>,
    /// Description of the interval.
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule: MandateAcssDebitPaymentSchedule,
    /// Transaction type of the mandate.
    pub transaction_type: MandateAcssDebitTransactionType,
}
#[cfg(feature = "min-ser")]
pub struct MandateAcssDebitBuilder {
    default_for: Option<Option<Vec<MandateAcssDebitDefaultFor>>>,
    interval_description: Option<Option<String>>,
    payment_schedule: Option<MandateAcssDebitPaymentSchedule>,
    transaction_type: Option<MandateAcssDebitTransactionType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for MandateAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandateAcssDebit>,
        builder: MandateAcssDebitBuilder,
    }

    impl Visitor for Place<MandateAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: MandateAcssDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for MandateAcssDebitBuilder {
        type Out = MandateAcssDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "default_for" => Ok(Deserialize::begin(&mut self.default_for)),
                "interval_description" => Ok(Deserialize::begin(&mut self.interval_description)),
                "payment_schedule" => Ok(Deserialize::begin(&mut self.payment_schedule)),
                "transaction_type" => Ok(Deserialize::begin(&mut self.transaction_type)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { default_for: Deserialize::default(), interval_description: Deserialize::default(), payment_schedule: Deserialize::default(), transaction_type: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let default_for = self.default_for.take()?;
            let interval_description = self.interval_description.take()?;
            let payment_schedule = self.payment_schedule.take()?;
            let transaction_type = self.transaction_type.take()?;

            Some(Self::Out { default_for, interval_description, payment_schedule, transaction_type })
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

    impl ObjectDeser for MandateAcssDebit {
        type Builder = MandateAcssDebitBuilder;
    }
};
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateAcssDebitDefaultFor {
    Invoice,
    Subscription,
}
impl MandateAcssDebitDefaultFor {
    pub fn as_str(self) -> &'static str {
        use MandateAcssDebitDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for MandateAcssDebitDefaultFor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateAcssDebitDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for MandateAcssDebitDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for MandateAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateAcssDebitDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateAcssDebitDefaultFor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateAcssDebitDefaultFor"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateAcssDebitDefaultFor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateAcssDebitDefaultFor> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateAcssDebitDefaultFor::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}
impl MandateAcssDebitPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use MandateAcssDebitPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr for MandateAcssDebitPaymentSchedule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateAcssDebitPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for MandateAcssDebitPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for MandateAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateAcssDebitPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateAcssDebitPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateAcssDebitPaymentSchedule"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateAcssDebitPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateAcssDebitPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateAcssDebitPaymentSchedule::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateAcssDebitTransactionType {
    Business,
    Personal,
}
impl MandateAcssDebitTransactionType {
    pub fn as_str(self) -> &'static str {
        use MandateAcssDebitTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr for MandateAcssDebitTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateAcssDebitTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for MandateAcssDebitTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for MandateAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateAcssDebitTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateAcssDebitTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateAcssDebitTransactionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateAcssDebitTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateAcssDebitTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateAcssDebitTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
