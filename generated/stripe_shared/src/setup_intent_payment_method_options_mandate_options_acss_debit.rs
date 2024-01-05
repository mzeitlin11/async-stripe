#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    /// A URL for custom mandate text
    pub custom_mandate_url: Option<String>,
    /// List of Stripe products where this mandate can be selected automatically.
    pub default_for: Option<Vec<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor>>,
    /// Description of the interval.
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule: Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>,
    /// Transaction type of the mandate.
    pub transaction_type: Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder {
    custom_mandate_url: Option<Option<String>>,
    default_for: Option<Option<Vec<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor>>>,
    interval_description: Option<Option<String>>,
    payment_schedule: Option<Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>>,
    transaction_type: Option<Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>,
        builder: SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder {
        type Out = SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit;
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
        type Builder = SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder;
    }
};
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    Invoice,
    Subscription,
}
impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}
impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
}
impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
