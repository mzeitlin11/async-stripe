#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceMandateOptionsCard {
    /// Amount to be charged for future payments.
    pub amount: Option<i64>,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: Option<InvoiceMandateOptionsCardAmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct InvoiceMandateOptionsCardBuilder {
    amount: Option<Option<i64>>,
    amount_type: Option<Option<InvoiceMandateOptionsCardAmountType>>,
    description: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceMandateOptionsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceMandateOptionsCard>,
        builder: InvoiceMandateOptionsCardBuilder,
    }

    impl Visitor for Place<InvoiceMandateOptionsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceMandateOptionsCardBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceMandateOptionsCardBuilder {
        type Out = InvoiceMandateOptionsCard;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_type" => Ok(Deserialize::begin(&mut self.amount_type)),
                "description" => Ok(Deserialize::begin(&mut self.description)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), amount_type: Deserialize::default(), description: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_type = self.amount_type.take()?;
            let description = self.description.take()?;

            Some(Self::Out { amount, amount_type, description })
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

    impl ObjectDeser for InvoiceMandateOptionsCard {
        type Builder = InvoiceMandateOptionsCardBuilder;
    }
};
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceMandateOptionsCardAmountType {
    Fixed,
    Maximum,
}
impl InvoiceMandateOptionsCardAmountType {
    pub fn as_str(self) -> &'static str {
        use InvoiceMandateOptionsCardAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr for InvoiceMandateOptionsCardAmountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceMandateOptionsCardAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for InvoiceMandateOptionsCardAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for InvoiceMandateOptionsCardAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceMandateOptionsCardAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceMandateOptionsCardAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceMandateOptionsCardAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoiceMandateOptionsCardAmountType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceMandateOptionsCardAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InvoiceMandateOptionsCardAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceMandateOptionsCardAmountType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
