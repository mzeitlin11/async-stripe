#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: PaymentMethodOptionsCardMandateOptionsAmountType,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
    /// End date of the mandate or subscription.
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: PaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: String,
    /// Start date of the mandate or subscription. Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported. Possible values are `india`.
    pub supported_types: Option<Vec<PaymentMethodOptionsCardMandateOptionsSupportedTypes>>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodOptionsCardMandateOptionsBuilder {
    amount: Option<i64>,
    amount_type: Option<PaymentMethodOptionsCardMandateOptionsAmountType>,
    description: Option<Option<String>>,
    end_date: Option<Option<stripe_types::Timestamp>>,
    interval: Option<PaymentMethodOptionsCardMandateOptionsInterval>,
    interval_count: Option<Option<u64>>,
    reference: Option<String>,
    start_date: Option<stripe_types::Timestamp>,
    supported_types: Option<Option<Vec<PaymentMethodOptionsCardMandateOptionsSupportedTypes>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodOptionsCardMandateOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCardMandateOptions>,
        builder: PaymentMethodOptionsCardMandateOptionsBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCardMandateOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodOptionsCardMandateOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCardMandateOptionsBuilder {
        type Out = PaymentMethodOptionsCardMandateOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_type" => Ok(Deserialize::begin(&mut self.amount_type)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "end_date" => Ok(Deserialize::begin(&mut self.end_date)),
                "interval" => Ok(Deserialize::begin(&mut self.interval)),
                "interval_count" => Ok(Deserialize::begin(&mut self.interval_count)),
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "start_date" => Ok(Deserialize::begin(&mut self.start_date)),
                "supported_types" => Ok(Deserialize::begin(&mut self.supported_types)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_type: Deserialize::default(),
                description: Deserialize::default(),
                end_date: Deserialize::default(),
                interval: Deserialize::default(),
                interval_count: Deserialize::default(),
                reference: Deserialize::default(),
                start_date: Deserialize::default(),
                supported_types: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_type = self.amount_type.take()?;
            let description = self.description.take()?;
            let end_date = self.end_date.take()?;
            let interval = self.interval.take()?;
            let interval_count = self.interval_count.take()?;
            let reference = self.reference.take()?;
            let start_date = self.start_date.take()?;
            let supported_types = self.supported_types.take()?;

            Some(Self::Out { amount, amount_type, description, end_date, interval, interval_count, reference, start_date, supported_types })
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

    impl ObjectDeser for PaymentMethodOptionsCardMandateOptions {
        type Builder = PaymentMethodOptionsCardMandateOptionsBuilder;
    }
};
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}
impl PaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCardMandateOptionsAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCardMandateOptionsAmountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCardMandateOptionsAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodOptionsCardMandateOptionsAmountType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsCardMandateOptionsAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodOptionsCardMandateOptionsAmountType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}
impl PaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCardMandateOptionsInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Sporadic => "sporadic",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCardMandateOptionsInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCardMandateOptionsInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "sporadic" => Ok(Sporadic),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsCardMandateOptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCardMandateOptionsInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodOptionsCardMandateOptionsInterval"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptionsCardMandateOptionsInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsCardMandateOptionsInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodOptionsCardMandateOptionsInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Specifies the type of mandates supported. Possible values are `india`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}
impl PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match self {
            India => "india",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match s {
            "india" => Ok(India),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodOptionsCardMandateOptionsSupportedTypes"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsCardMandateOptionsSupportedTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodOptionsCardMandateOptionsSupportedTypes::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
