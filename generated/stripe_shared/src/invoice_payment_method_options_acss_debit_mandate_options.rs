#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    pub transaction_type: Option<InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
#[cfg(feature = "min-ser")]
pub struct InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder {
    transaction_type: Option<Option<InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicePaymentMethodOptionsAcssDebitMandateOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsAcssDebitMandateOptions>,
        builder: InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsAcssDebitMandateOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder {
        type Out = InvoicePaymentMethodOptionsAcssDebitMandateOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "transaction_type" => Ok(Deserialize::begin(&mut self.transaction_type)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { transaction_type: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let transaction_type = self.transaction_type.take()?;

            Some(Self::Out { transaction_type })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsAcssDebitMandateOptions {
        type Builder = InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder;
    }
};
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}
impl InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
