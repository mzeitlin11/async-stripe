#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BillingDetails {
    /// Billing address.
    pub address: Option<stripe_shared::Address>,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
    /// Billing phone number (including extension).
    pub phone: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct BillingDetailsBuilder {
    address: Option<Option<stripe_shared::Address>>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BillingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingDetails>,
        builder: BillingDetailsBuilder,
    }

    impl Visitor for Place<BillingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: BillingDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BillingDetailsBuilder {
        type Out = BillingDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),
                "email" => Ok(Deserialize::begin(&mut self.email)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "phone" => Ok(Deserialize::begin(&mut self.phone)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { address: Deserialize::default(), email: Deserialize::default(), name: Deserialize::default(), phone: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;
            let email = self.email.take()?;
            let name = self.name.take()?;
            let phone = self.phone.take()?;

            Some(Self::Out { address, email, name, phone })
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

    impl ObjectDeser for BillingDetails {
        type Builder = BillingDetailsBuilder;
    }
};
