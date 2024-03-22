#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasurySharedResourceBillingDetails {
    pub address: stripe_shared::Address,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct TreasurySharedResourceBillingDetailsBuilder {
    address: Option<stripe_shared::Address>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasurySharedResourceBillingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasurySharedResourceBillingDetails>,
        builder: TreasurySharedResourceBillingDetailsBuilder,
    }

    impl Visitor for Place<TreasurySharedResourceBillingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasurySharedResourceBillingDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasurySharedResourceBillingDetailsBuilder {
        type Out = TreasurySharedResourceBillingDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),
                "email" => Ok(Deserialize::begin(&mut self.email)),
                "name" => Ok(Deserialize::begin(&mut self.name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { address: Deserialize::default(), email: Deserialize::default(), name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;
            let email = self.email.take()?;
            let name = self.name.take()?;

            Some(Self::Out { address, email, name })
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

    impl ObjectDeser for TreasurySharedResourceBillingDetails {
        type Builder = TreasurySharedResourceBillingDetailsBuilder;
    }
};
