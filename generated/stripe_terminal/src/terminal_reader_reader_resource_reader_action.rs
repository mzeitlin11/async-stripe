/// Represents an action performed by the reader
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceReaderAction {
    /// Failure code, only set if status is `failed`.
    pub failure_code: Option<String>,
    /// Detailed failure message, only set if status is `failed`.
    pub failure_message: Option<String>,
    pub process_payment_intent: Option<stripe_terminal::TerminalReaderReaderResourceProcessPaymentIntentAction>,
    pub process_setup_intent: Option<stripe_terminal::TerminalReaderReaderResourceProcessSetupIntentAction>,
    pub refund_payment: Option<stripe_terminal::TerminalReaderReaderResourceRefundPaymentAction>,
    pub set_reader_display: Option<stripe_terminal::TerminalReaderReaderResourceSetReaderDisplayAction>,
    /// Status of the action performed by the reader.
    pub status: TerminalReaderReaderResourceReaderActionStatus,
    /// Type of action performed by the reader.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: TerminalReaderReaderResourceReaderActionType,
}
#[cfg(feature = "min-ser")]
pub struct TerminalReaderReaderResourceReaderActionBuilder {
    failure_code: Option<Option<String>>,
    failure_message: Option<Option<String>>,
    process_payment_intent: Option<Option<stripe_terminal::TerminalReaderReaderResourceProcessPaymentIntentAction>>,
    process_setup_intent: Option<Option<stripe_terminal::TerminalReaderReaderResourceProcessSetupIntentAction>>,
    refund_payment: Option<Option<stripe_terminal::TerminalReaderReaderResourceRefundPaymentAction>>,
    set_reader_display: Option<Option<stripe_terminal::TerminalReaderReaderResourceSetReaderDisplayAction>>,
    status: Option<TerminalReaderReaderResourceReaderActionStatus>,
    type_: Option<TerminalReaderReaderResourceReaderActionType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalReaderReaderResourceReaderAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceReaderAction>,
        builder: TerminalReaderReaderResourceReaderActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceReaderAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TerminalReaderReaderResourceReaderActionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceReaderActionBuilder {
        type Out = TerminalReaderReaderResourceReaderAction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "failure_code" => Ok(Deserialize::begin(&mut self.failure_code)),
                "failure_message" => Ok(Deserialize::begin(&mut self.failure_message)),
                "process_payment_intent" => Ok(Deserialize::begin(&mut self.process_payment_intent)),
                "process_setup_intent" => Ok(Deserialize::begin(&mut self.process_setup_intent)),
                "refund_payment" => Ok(Deserialize::begin(&mut self.refund_payment)),
                "set_reader_display" => Ok(Deserialize::begin(&mut self.set_reader_display)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                failure_code: Deserialize::default(),
                failure_message: Deserialize::default(),
                process_payment_intent: Deserialize::default(),
                process_setup_intent: Deserialize::default(),
                refund_payment: Deserialize::default(),
                set_reader_display: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let failure_code = self.failure_code.take()?;
            let failure_message = self.failure_message.take()?;
            let process_payment_intent = self.process_payment_intent.take()?;
            let process_setup_intent = self.process_setup_intent.take()?;
            let refund_payment = self.refund_payment.take()?;
            let set_reader_display = self.set_reader_display.take()?;
            let status = self.status.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { failure_code, failure_message, process_payment_intent, process_setup_intent, refund_payment, set_reader_display, status, type_ })
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

    impl ObjectDeser for TerminalReaderReaderResourceReaderAction {
        type Builder = TerminalReaderReaderResourceReaderActionBuilder;
    }
};
/// Status of the action performed by the reader.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
}
impl TerminalReaderReaderResourceReaderActionStatus {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceReaderActionStatus::*;
        match self {
            Failed => "failed",
            InProgress => "in_progress",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceReaderActionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceReaderActionStatus::*;
        match s {
            "failed" => Ok(Failed),
            "in_progress" => Ok(InProgress),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TerminalReaderReaderResourceReaderActionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalReaderReaderResourceReaderActionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceReaderActionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TerminalReaderReaderResourceReaderActionStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TerminalReaderReaderResourceReaderActionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceReaderActionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalReaderReaderResourceReaderActionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Type of action performed by the reader.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceReaderActionType {
    ProcessPaymentIntent,
    ProcessSetupIntent,
    RefundPayment,
    SetReaderDisplay,
}
impl TerminalReaderReaderResourceReaderActionType {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceReaderActionType::*;
        match self {
            ProcessPaymentIntent => "process_payment_intent",
            ProcessSetupIntent => "process_setup_intent",
            RefundPayment => "refund_payment",
            SetReaderDisplay => "set_reader_display",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceReaderActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceReaderActionType::*;
        match s {
            "process_payment_intent" => Ok(ProcessPaymentIntent),
            "process_setup_intent" => Ok(ProcessSetupIntent),
            "refund_payment" => Ok(RefundPayment),
            "set_reader_display" => Ok(SetReaderDisplay),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for TerminalReaderReaderResourceReaderActionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalReaderReaderResourceReaderActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceReaderActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TerminalReaderReaderResourceReaderActionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TerminalReaderReaderResourceReaderActionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceReaderActionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalReaderReaderResourceReaderActionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
