#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingNetworkTokenAddress {
    /// The street address of the cardholder tokenizing the card.
    pub line1: String,
    /// The postal code of the cardholder tokenizing the card.
    pub postal_code: String,
}
#[cfg(feature = "min-ser")]
pub struct IssuingNetworkTokenAddressBuilder {
    line1: Option<String>,
    postal_code: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingNetworkTokenAddress {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenAddress>,
        builder: IssuingNetworkTokenAddressBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenAddress> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingNetworkTokenAddressBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenAddressBuilder {
        type Out = IssuingNetworkTokenAddress;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "line1" => Ok(Deserialize::begin(&mut self.line1)),
                "postal_code" => Ok(Deserialize::begin(&mut self.postal_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { line1: Deserialize::default(), postal_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let line1 = self.line1.take()?;
            let postal_code = self.postal_code.take()?;

            Some(Self::Out { line1, postal_code })
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

    impl ObjectDeser for IssuingNetworkTokenAddress {
        type Builder = IssuingNetworkTokenAddressBuilder;
    }
};
