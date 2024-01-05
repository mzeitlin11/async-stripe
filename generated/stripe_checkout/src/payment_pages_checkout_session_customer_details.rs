#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    /// The customer's address after a completed Checkout Session.
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub address: Option<stripe_shared::Address>,
    /// The email associated with the Customer, if one exists, on the Checkout Session after a completed Checkout Session or at time of session expiry.
    /// Otherwise, if the customer has consented to promotional content, this value is the most recent valid email provided by the customer on the Checkout form.
    pub email: Option<String>,
    /// The customer's name after a completed Checkout Session.
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub name: Option<String>,
    /// The customer's phone number after a completed Checkout Session.
    pub phone: Option<String>,
    /// The customer’s tax exempt status after a completed Checkout Session.
    pub tax_exempt: Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>,
    /// The customer’s tax IDs after a completed Checkout Session.
    pub tax_ids: Option<Vec<stripe_checkout::PaymentPagesCheckoutSessionTaxId>>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentPagesCheckoutSessionCustomerDetailsBuilder {
    address: Option<Option<stripe_shared::Address>>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
    tax_exempt: Option<Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>>,
    tax_ids: Option<Option<Vec<stripe_checkout::PaymentPagesCheckoutSessionTaxId>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCustomerDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomerDetails>,
        builder: PaymentPagesCheckoutSessionCustomerDetailsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomerDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentPagesCheckoutSessionCustomerDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomerDetailsBuilder {
        type Out = PaymentPagesCheckoutSessionCustomerDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),
                "email" => Ok(Deserialize::begin(&mut self.email)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "phone" => Ok(Deserialize::begin(&mut self.phone)),
                "tax_exempt" => Ok(Deserialize::begin(&mut self.tax_exempt)),
                "tax_ids" => Ok(Deserialize::begin(&mut self.tax_ids)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                email: Deserialize::default(),
                name: Deserialize::default(),
                phone: Deserialize::default(),
                tax_exempt: Deserialize::default(),
                tax_ids: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;
            let email = self.email.take()?;
            let name = self.name.take()?;
            let phone = self.phone.take()?;
            let tax_exempt = self.tax_exempt.take()?;
            let tax_ids = self.tax_ids.take()?;

            Some(Self::Out { address, email, name, phone, tax_exempt, tax_ids })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomerDetails {
        type Builder = PaymentPagesCheckoutSessionCustomerDetailsBuilder;
    }
};
/// The customer’s tax exempt status after a completed Checkout Session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}
impl PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
