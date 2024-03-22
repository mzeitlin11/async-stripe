#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFields {
    pub dropdown: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsDropdown>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    pub label: stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsLabel,
    pub numeric: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    pub optional: bool,
    pub text: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsText>,
    /// The type of the field.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PaymentPagesCheckoutSessionCustomFieldsType,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionCustomFieldsBuilder {
    dropdown: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsDropdown>>,
    key: Option<String>,
    label: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsLabel>,
    numeric: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsNumeric>>,
    optional: Option<bool>,
    text: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsText>>,
    type_: Option<PaymentPagesCheckoutSessionCustomFieldsType>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCustomFields {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFields>,
        builder: PaymentPagesCheckoutSessionCustomFieldsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFields> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionCustomFieldsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomFieldsBuilder {
        type Out = PaymentPagesCheckoutSessionCustomFields;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "dropdown" => Ok(Deserialize::begin(&mut self.dropdown)),
                "key" => Ok(Deserialize::begin(&mut self.key)),
                "label" => Ok(Deserialize::begin(&mut self.label)),
                "numeric" => Ok(Deserialize::begin(&mut self.numeric)),
                "optional" => Ok(Deserialize::begin(&mut self.optional)),
                "text" => Ok(Deserialize::begin(&mut self.text)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                dropdown: Deserialize::default(),
                key: Deserialize::default(),
                label: Deserialize::default(),
                numeric: Deserialize::default(),
                optional: Deserialize::default(),
                text: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let dropdown = self.dropdown.take()?;
            let key = self.key.take()?;
            let label = self.label.take()?;
            let numeric = self.numeric.take()?;
            let optional = self.optional.take()?;
            let text = self.text.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { dropdown, key, label, numeric, optional, text, type_ })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomFields {
        type Builder = PaymentPagesCheckoutSessionCustomFieldsBuilder;
    }
};
/// The type of the field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}
impl PaymentPagesCheckoutSessionCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionCustomFieldsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentPagesCheckoutSessionCustomFieldsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentPagesCheckoutSessionCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionCustomFieldsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentPagesCheckoutSessionCustomFieldsType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentPagesCheckoutSessionCustomFieldsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentPagesCheckoutSessionCustomFieldsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentPagesCheckoutSessionCustomFieldsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
