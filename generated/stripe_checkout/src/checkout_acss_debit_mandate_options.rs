#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CheckoutAcssDebitMandateOptions {
    /// A URL for custom mandate text
    pub custom_mandate_url: Option<String>,
    /// List of Stripe products where this mandate can be selected automatically.
    /// Returned when the Session is in `setup` mode.
    pub default_for: Option<Vec<CheckoutAcssDebitMandateOptionsDefaultFor>>,
    /// Description of the interval.
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule: Option<CheckoutAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    pub transaction_type: Option<CheckoutAcssDebitMandateOptionsTransactionType>,
}
#[cfg(feature = "min-ser")]
pub struct CheckoutAcssDebitMandateOptionsBuilder {
    custom_mandate_url: Option<Option<String>>,
    default_for: Option<Option<Vec<CheckoutAcssDebitMandateOptionsDefaultFor>>>,
    interval_description: Option<Option<String>>,
    payment_schedule: Option<Option<CheckoutAcssDebitMandateOptionsPaymentSchedule>>,
    transaction_type: Option<Option<CheckoutAcssDebitMandateOptionsTransactionType>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutAcssDebitMandateOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutAcssDebitMandateOptions>,
        builder: CheckoutAcssDebitMandateOptionsBuilder,
    }

    impl Visitor for Place<CheckoutAcssDebitMandateOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CheckoutAcssDebitMandateOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CheckoutAcssDebitMandateOptionsBuilder {
        type Out = CheckoutAcssDebitMandateOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "custom_mandate_url" => Ok(Deserialize::begin(&mut self.custom_mandate_url)),
                "default_for" => Ok(Deserialize::begin(&mut self.default_for)),
                "interval_description" => Ok(Deserialize::begin(&mut self.interval_description)),
                "payment_schedule" => Ok(Deserialize::begin(&mut self.payment_schedule)),
                "transaction_type" => Ok(Deserialize::begin(&mut self.transaction_type)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                custom_mandate_url: Deserialize::default(),
                default_for: Deserialize::default(),
                interval_description: Deserialize::default(),
                payment_schedule: Deserialize::default(),
                transaction_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let custom_mandate_url = self.custom_mandate_url.take()?;
            let default_for = self.default_for.take()?;
            let interval_description = self.interval_description.take()?;
            let payment_schedule = self.payment_schedule.take()?;
            let transaction_type = self.transaction_type.take()?;

            Some(Self::Out { custom_mandate_url, default_for, interval_description, payment_schedule, transaction_type })
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

    impl ObjectDeser for CheckoutAcssDebitMandateOptions {
        type Builder = CheckoutAcssDebitMandateOptionsBuilder;
    }
};
/// List of Stripe products where this mandate can be selected automatically.
/// Returned when the Session is in `setup` mode.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}
impl CheckoutAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        use CheckoutAcssDebitMandateOptionsDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitMandateOptionsDefaultFor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitMandateOptionsDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutAcssDebitMandateOptionsDefaultFor"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutAcssDebitMandateOptionsDefaultFor> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutAcssDebitMandateOptionsDefaultFor::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}
impl CheckoutAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use CheckoutAcssDebitMandateOptionsPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitMandateOptionsPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutAcssDebitMandateOptionsPaymentSchedule"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutAcssDebitMandateOptionsPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutAcssDebitMandateOptionsPaymentSchedule::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}
impl CheckoutAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use CheckoutAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitMandateOptionsTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CheckoutAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CheckoutAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutAcssDebitMandateOptionsTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitMandateOptionsTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutAcssDebitMandateOptionsTransactionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CheckoutAcssDebitMandateOptionsTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CheckoutAcssDebitMandateOptionsTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutAcssDebitMandateOptionsTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
